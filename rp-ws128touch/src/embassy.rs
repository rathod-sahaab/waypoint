#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Pull;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use display_interface_spi::SPIInterface;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
// use waveshare_rp2040_lcd_0_96 as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

// extern crate alloc;
// use embedded_alloc::Heap;

// #[global_allocator]
// static ALLOCATOR: Heap = Heap::empty();

use crate::battery::AdcBattery;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut adc = Adc::new(p.ADC, Irqs, Config::default());

    let mut p26 = Channel::new_pin(p.PIN_26, Pull::None);
    let mut p27 = Channel::new_pin(p.PIN_27, Pull::None);
    let mut p28 = Channel::new_pin(p.PIN_28, Pull::None);
    let mut ts = Channel::new_temp_sensor(p.ADC_TEMP_SENSOR);

    loop {
        let level = adc.read(&mut p26).await.unwrap();
        info!("Pin 26 ADC: {}", level);
        let level = adc.read(&mut p27).await.unwrap();
        info!("Pin 27 ADC: {}", level);
        let level = adc.read(&mut p28).await.unwrap();
        info!("Pin 28 ADC: {}", level);
        let temp = adc.read(&mut ts).await.unwrap();
        info!("Temp: {} degrees", convert_to_celsius(temp));
        Timer::after_secs(1).await;
    }
}

fn convert_to_celsius(raw_temp: u16) -> f32 {
    // According to chapter 4.9.5. Temperature Sensor in RP2040 datasheet
    let temp = 27.0 - (raw_temp as f32 * 3.3 / 4096.0 - 0.706) / 0.001721;
    let sign = if temp < 0.0 { -1.0 } else { 1.0 };
    let rounded_temp_x10: i16 = ((temp * 10.0) + 0.5 * sign) as i16;
    (rounded_temp_x10 as f32) / 10.0
}
