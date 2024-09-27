#![no_std]
#![no_main]

use bsp::entry;

use defmt::*;
use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use waveshare_rp2040_lcd_0_96 as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    fugit::RateExtU32,
    pac, pwm,
    sio::Sio,
    watchdog::Watchdog,
};

extern crate alloc;
use embedded_alloc::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

mod battery;
use crate::battery::AdcBattery;

#[entry]
fn main() -> ! {
    info!("Program start");

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    info!("Memory init");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let dc_pin = pins.gp8.into_push_pull_output();
    let spi_cs = pins.gp9.into_push_pull_output();

    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_miso = pins.gp12.into_function::<hal::gpio::FunctionSpi>();
    let lcd_sclk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_miso, lcd_sclk));

    let lcd_rst = pins
        .gp13
        .into_push_pull_output_in_state(hal::gpio::PinState::High);

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        8_000_000u32.Hz(),
        embedded_hal::spi::MODE_0,
    );

    let spi_interface = SPIInterface::new(spi, dc_pin, spi_cs);

    // let mut led_pin = pins.gpio25.into_push_pull_output();

    // initialize PWM for backlight
    let pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM4
    let mut pwm = pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    // Output channel B on PWM4 to GPIO 25
    let mut channel = pwm.channel_b;
    channel.output_to(pins.gp25);

    let mut display = gc9a01a::GC9A01A::new(spi_interface, lcd_rst, channel);

    // Bring out of reset
    display.reset(&mut delay).unwrap();
    // Turn on backlight
    display.set_backlight(15000);
    // Initialize registers
    display.initialize(&mut delay).unwrap();

    let adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let batt_pin = hal::adc::AdcPin::new(pins.gp29).unwrap();

    let battery = AdcBattery::new_lipo(adc, batt_pin);

    let mut app = waypoint::application::Application::new(&mut display, battery);

    app.start()
}

// End of file
