use crate::devices::bmp280::registers::Register;
use crate::devices::bmp280::status::{CalibrationData, Status};
use crate::devices::bmp280::{Config, registers};
use defmt::info;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c;
use hardware_abstraction::{Pressure, PressureSensor, Temperature, TemperatureSensor};
use crate::devices::common::Error;

pub struct BMP280<I2C> {
    i2c: I2C,
    config: Config,
    calibration_data: CalibrationData,
}

type Result<T, E = Error> = core::result::Result<T, E>;

impl<I2C> BMP280<I2C>
where
    I2C: i2c::I2c,
{
    pub async fn new(mut i2c: I2C, config: Config) -> Result<Self> {
        let mut buffer = [0u8; 24];
        i2c.write_read(
            config.address.into(),
            &[Register::CalibrationData.into()],
            &mut buffer,
        )
            .await?;

        let calib = CalibrationData::from(&buffer);
        info!("{}", calib);

        Timer::after(Duration::from_millis(2)).await; //Wait for the device
        let mut value = Self {
            i2c,
            config,
            calibration_data: CalibrationData::from(&buffer),
        };
        value.upload_config().await?;
        Ok(value)
    }

    async fn upload_config(&mut self) -> Result<()> {
        let config = &self.config;
        let standby: u8 = config.standby_time.into();
        let filter: u8 = config.filter.into();
        let config_value = standby << 5 | filter << 5;

        let oversampling_temp: u8 = config.temperature.into();
        let oversampling_pressure: u8 = config.pressure.into();
        let mode = 0b11; //Normal mode
        let ctrl_value = oversampling_temp << 5 | oversampling_pressure << 2 | mode;

        self.write_u8(Register::Config, config_value).await?;
        self.write_u8(Register::Ctrl, ctrl_value).await?;

        Ok(())
    }
    
    #[allow(unused)]
    pub async fn read_id(&mut self) -> Result<u8> {
        self.read_8(Register::Id).await
    }

    #[allow(unused)]
    pub async fn reset(&mut self) -> Result<()> {
        self.write_u8(Register::Reset, registers::RESET_VALUE).await
    }

    #[allow(unused)]
    pub async fn read_status(&mut self) -> Result<Status> {
        let value = self.read_8(Register::Status).await?;
        Ok(Status {
            measuring: value & 0b0001 > 0,
            im_update: value & 0b1000 > 0,
        })
    }

    async fn write_u8(&mut self, register: Register, value: u8) -> Result<()> {
        self.i2c
            .write(self.config.address.into(), &[register.into(), value])
            .await?;
        Ok(())
    }

    async fn read_8(&mut self, register: Register) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.config.address.into(), &[register.into()], &mut buffer)
            .await?;
        Ok(buffer[0])
    }

    async fn read_24(&mut self, register: Register) -> Result<u32> {
        let mut buffer = [0u8; 3];
        self.i2c
            .write_read(self.config.address.into(), &[register.into()], &mut buffer)
            .await?;
        Ok(((buffer[0] as u32) << 12) | ((buffer[1] as u32) << 4) | ((buffer[2] as u32) >> 4))
    }
}

impl<I2C> TemperatureSensor for BMP280<I2C>
where
    I2C: i2c::I2c,
{
    type Error = Error;
    async fn read_value(&mut self) -> core::result::Result<Temperature, Self::Error> {
        let raw_t = self.read_24(Register::Temperature).await? as i32;
        let compensated = self.calibration_data.compensate_temperature(raw_t);
        Ok(Temperature::from_deg_c_100(compensated))
    }
}

impl<I2C> PressureSensor for BMP280<I2C>
where
    I2C: i2c::I2c,
{
    type Error = Error;
    async fn read_value(&mut self) -> core::result::Result<Pressure, Self::Error> {
        let raw_t = self.read_24(Register::Temperature).await? as i32;
        let raw_p = self.read_24(Register::Pressure).await? as i32;
        let compensated = self.calibration_data.compensate_pressure(raw_p, raw_t);
        Ok(Pressure::from_q248_100(compensated))
    }
}
