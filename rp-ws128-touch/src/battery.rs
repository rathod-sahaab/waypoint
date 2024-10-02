use dsp::filters::kalman::KalmanFilter;
use dsp::filters::kalman::StateTransition;
use dsp::filters::kalman::TFloatSquare;
use dsp::filters::kalman::TFloatVector;
use embassy_rp::adc::{Adc, Channel};
use waypoint::battery::Battery;

pub struct AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    min: f32,
    max: f32,
    adc: Adc,
    batt_pin: SRC,
    volts: f32,
    kalman_filter: KalmanFilter<'static, 1, 1>,
}

impl<SRC> AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    pub fn new_lipo(adc: Adc, batt_pin: SRC) -> Self {
        let kalman_filter = KalmanFilter::new(
            TFloatVector::<1>::from_element(4.0),
            TFloatSquare::<1>::from_element(2.0),
            StateTransition::<1>::Matrix(TFloatSquare::<1>::new(1.0)),
            TFloatSquare::<1>::from_element(1.0),
            TFloatSquare::<1>::from_element(0.0),
            TFloatSquare::<1>::from_element(2.0),
        );

        Self {
            min: 3.6,
            max: 4.2,
            volts: 4.0,
            adc,
            batt_pin,
            kalman_filter,
        }
    }

    fn raw_volts(&mut self) -> f32 {
        let raw: u16 = self.adc.read(&mut self.batt_pin).unwrap();
        let float: f32 = raw.into();
        float / 100f32
    }
}

impl<SRC> Battery for AdcBattery<SRC>
where
    SRC: Channel<Adc, ID = u8>,
{
    fn update(&mut self) {
        let volts: f32 = self.raw_volts();

        self.kalman_filter.predict(1.0);
        let filtered = self
            .kalman_filter
            .update(TFloatVector::<1>::from_element(volts));

        self.volts = if let Some(filtered) = filtered {
            *filtered.get(0).unwrap()
        } else {
            self.volts
        };
    }

    /// filtered volts
    fn volts(&mut self) -> f32 {
        self.volts
    }

    fn percentage(&mut self) -> f32 {
        let range = self.max - self.min;
        let remaining = self.volts() - self.min;

        100f32 * (remaining / range)
    }
}
