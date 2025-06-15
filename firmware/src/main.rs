#![no_std]
#![no_main]

use crate::devices::bh1750;
use crate::devices::bh1750::BH1750;
use defmt::{Format, info, warn};
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c;
use embassy_time::{Duration, Timer};
use hardware_abstraction::LightSensor;

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

mod devices;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // TODO(safety): Implement commutation watchdog timeout
    // If angle read or update loop fails, shut off motor to avoid damage
    // let mut watchdog = Watchdog::new(p.WATCHDOG);
    // watchdog.start(Duration::from_millis(100));

    let i2c_config = i2c::Config::default();
    let i2c = i2c::I2c::new_async(p.I2C1, p.PIN_15, p.PIN_14, Irqs, i2c_config);
    let mut bh1750 = BH1750::new(i2c, bh1750::Config::default())
        .await
        .expect("Failed to initialize BH1750");
    loop {
        try_log_value(&mut bh1750).await;
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn try_log_value<L: LightSensor>(bh1750: &mut L)
where
    <L as LightSensor>::Error: Format,
{
    let result = bh1750.read_value().await;
    match result {
        Ok(value) => {
            info!("Read value: {}", value);
        }
        Err(error) => {
            warn!("Error during reading, {}", error);
        }
    }
}
