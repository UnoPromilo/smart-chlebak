#![no_std]
#![no_main]

use embassy_rp::watchdog::Watchdog;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c;
use embassy_rp::pwm::PwmBatch;
use embassy_time::Duration;
use {defmt_rtt as _, panic_probe as _};


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
    loop {}
}
