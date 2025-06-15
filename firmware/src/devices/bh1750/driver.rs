use crate::devices::bh1750::config::{Config, OperationMode, ResolutionMode};
use crate::devices::bh1750::error::Error;
use crate::devices::bh1750::registers::Command;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c;
use hardware_abstraction::LightSensor;

pub struct BH1750<I2C> {
    i2c: I2C,
    config: Config,
}

type Result<T, E = Error> = core::result::Result<T, E>;

impl<I2C> BH1750<I2C>
where
    I2C: i2c::I2c,
{
    pub async fn new(i2c: I2C, config: Config) -> Result<Self> {
        let mut value = Self { i2c, config };

        value.upload_config().await?;

        Ok(value)
    }

    async fn upload_config(&mut self) -> Result<()> {
        let command = match self.config.operation_mode {
            OperationMode::OneTime => Command::PowerDown,
            OperationMode::Continuously => match self.config.resolution_mode {
                ResolutionMode::UltraHighResolution => Command::ContinuouslyUltraHighResolution,
                ResolutionMode::HighResolution => Command::ContinuouslyHighResolution,
                ResolutionMode::LowResolution => Command::ContinuouslyLowResolution,
            },
        };

        self.write_register(command).await.map_err(Error::from)?;

        Ok(())
    }

    async fn write_register(&mut self, command: Command) -> Result<()> {
        self.i2c
            .write(self.config.address.into(), &[command.into()])
            .await?;

        Timer::after(self.get_delay_after_command(command)).await;

        Ok(())
    }

    fn get_delay_after_command(&self, command: Command) -> Duration {
        let duration_ms = match command {
            Command::PowerDown => 1,
            Command::PowerUp => 1,
            Command::Reset => 1,
            Command::ContinuouslyHighResolution => 180,
            Command::ContinuouslyUltraHighResolution => 180,
            Command::ContinuouslyLowResolution => 24,
            Command::OneTimeHighResolution => 24,
            Command::OneTimeUltraHighResolution => 24,
            Command::OneTimeLowResolution => 24,
        };

        Duration::from_millis(duration_ms)
    }

    async fn read_u16(&mut self) -> Result<u16> {
        let mut buffer = [0u8; 2];
        self.i2c
            .read(self.config.address.into(), &mut buffer)
            .await?;
        Ok(u16::from_le_bytes([buffer[1], buffer[0]]))
    }
}

impl<I2C> LightSensor for BH1750<I2C>
where
    I2C: i2c::I2c,
{
    type Error = Error;

    async fn read_value(&mut self) -> Result<u16> {
        if self.config.operation_mode == OperationMode::OneTime {
            let command = match self.config.resolution_mode {
                ResolutionMode::UltraHighResolution => Command::OneTimeUltraHighResolution,
                ResolutionMode::HighResolution => Command::OneTimeHighResolution,
                ResolutionMode::LowResolution => Command::OneTimeLowResolution,
            };
            
            self.write_register(command).await?;
        }
        
        self.read_u16().await
    }
}
