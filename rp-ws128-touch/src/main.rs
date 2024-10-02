#![no_std]
#![no_main]

use {
    // battery::AdcBattery,
    defmt::{info, unwrap},
    defmt_rtt as _,
    embassy_embedded_hal::shared_bus::{
        asynch::spi::SpiDeviceWithConfig, blocking::spi::SpiDeviceWithConfig,
    },
    embassy_executor::Spawner,
    embassy_rp::{
        // bind_interrupts,
        gpio::{Level, Output},
        spi::{self, Spi},
    },
    embassy_time::{Duration, Timer},
    panic_probe as _,
};

#[embassy_executor::task]
async fn blinker(mut led: Output<'static>, interval: Duration) {
    loop {
        info!("loop");
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

// pub mod battery;

const DISPLAY_FREQ: u32 = 8_000_000;
const TOUCH_FREQ: u32 = 200_000;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // bind_interrupts!(struct Irqs {
    //     ADC_IRQ_FIFO => InterruptHandler;
    // });
    //
    // let adc_battery = AdcBattery::new_lipo(battery_fn);

    let lcd_dc = p.PIN_8;
    let lcd_cs = p.PIN_9;
    let lcd_mosi = p.PIN_11;
    let lcd_miso = p.PIN_12;
    let lcd_clk = p.PIN_10;
    let lcd_rst = p.PIN_13;
    let lcd_bl = p.PIN_25;

    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;

    let spi = Spi::new_blocking(p.SPI1, lcd_clk, lcd_mosi, lcd_miso, display_config.clone());

    let display_spi =
        SpiDeviceWithConfig::new(&spi, Output::new(lcd_cs, Level::High), display_config);
    // SpiDeviceWithConfig

    let led = Output::new(p.PIN_25, Level::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(500))));
}
