use embedded_hal_0_2::adc::Channel;
use embedded_hal_0_2::adc::OneShot;
use waveshare_rp2040_lcd_0_96::hal::Adc;
use waypoint::battery::Battery;

pub struct AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    min: f32,
    max: f32,
    adc: Adc,
    batt_pin: SRC,
}

impl<SRC> AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    pub fn new_lipo(adc: Adc, batt_pin: SRC) -> Self {
        Self {
            min: 3.6,
            max: 4.2,
            adc,
            batt_pin,
        }
    }
}

impl<SRC> Battery for AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    fn volts(&mut self) -> f32 {
        let raw: u16 = self.adc.read(&mut self.batt_pin).unwrap();
        let float: f32 = raw.into();
        float / 100f32
    }

    fn percentage(&mut self) -> f32 {
        let range = self.max - self.min;
        let remaining = self.volts() - self.min;

        100f32 * (remaining / range)
    }
}
