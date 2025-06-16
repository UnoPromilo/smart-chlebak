#![no_std]
#![no_main]

use crate::devices::{bh1750, bmp280};
use core::cell::RefCell;
use defmt::{info, warn};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::{i2c, spi};
use embassy_sync::blocking_mutex::NoopMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use embedded_hal::spi::Operation::DelayNs;
use embedded_sdmmc::SdCard;
use embedded_sdmmc::sdcard::CardType;
use hardware_abstraction::TemperatureSensor;
use hardware_abstraction::{LightSensor, PressureSensor};
use static_cell::StaticCell;

use crate::devices::bh1750::BH1750;
use crate::devices::bmp280::BMP280;
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

    static I2C_BUS: StaticCell<
        Mutex<NoopRawMutex, i2c::I2c<embassy_rp::peripherals::I2C1, i2c::Async>>,
    > = StaticCell::new();
    let i2c_config = i2c::Config::default();
    let i2c = i2c::I2c::new_async(p.I2C1, p.PIN_15, p.PIN_14, Irqs, i2c_config);
    let i2c_bus = I2C_BUS.init(Mutex::new(i2c));

    let i2c_device_bh1750 = I2cDevice::new(i2c_bus);
    let mut bh1750 = BH1750::new(i2c_device_bh1750, bh1750::Config::default())
        .await
        .expect("Failed to initialize BH1750");

    let i2c_device_bmp280 = I2cDevice::new(i2c_bus);
    let mut bmp280 = BMP280::new(i2c_device_bmp280, bmp280::Config::default())
        .await
        .expect("Failed to initialize BMP280");

    static SPI_BUS: StaticCell<
        NoopMutex<RefCell<spi::Spi<embassy_rp::peripherals::SPI1, spi::Blocking>>>,
    > = StaticCell::new();
    let mut spi_config = spi::Config::default();
    spi_config.frequency = 400_000;
    let spi = spi::Spi::new_blocking(p.SPI1, p.PIN_10, p.PIN_11, p.PIN_12, spi_config);
    let spi_bus = NoopMutex::new(RefCell::new(spi));
    let spi_bus = SPI_BUS.init(spi_bus);
    let cs_pin1 = Output::new(p.PIN_13, Level::Low);

    let spi_dev1 = SpiDevice::new(spi_bus, cs_pin1);
    let sdcard = SdCard::new(spi_dev1, embassy_time::Delay);

    loop {
        Timer::after(Duration::from_millis(100)).await;
        try_log_lux_value(&mut bh1750).await;
        try_log_press_value(&mut bmp280).await;
        try_log_temp_value(&mut bmp280).await;
    }
}

async fn try_log_lux_value<L: LightSensor>(sensor: &mut L) {
    let result = sensor.read_value().await;
    match result {
        Ok(value) => {
            info!("LUX value: {}lux", value);
        }
        Err(_) => {
            warn!("Error during reading");
        }
    }
}

async fn try_log_press_value<L: PressureSensor>(sensor: &mut L) {
    let result = sensor.read_value().await;
    match result {
        Ok(value) => {
            info!("Pressure value: {}hPa", value.to_hpa());
        }
        Err(_) => {
            warn!("Error during reading");
        }
    }
}

async fn try_log_temp_value<L: TemperatureSensor>(sensor: &mut L) {
    let result = sensor.read_value().await;
    match result {
        Ok(value) => {
            info!("Temperature value: {}°C", value.to_deg_c());
        }
        Err(_) => {
            warn!("Error during reading");
        }
    }
}
