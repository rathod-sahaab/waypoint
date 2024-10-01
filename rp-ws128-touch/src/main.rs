#![no_std]
#![no_main]

use {
    defmt::{info, unwrap},
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_rp::gpio::{Level, Output},
    embassy_time::{Duration, Timer},
    panic_probe as _,
}; // global logger

#[embassy_executor::task]
async fn blinker(mut led: Output<'static>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(500))));
}
