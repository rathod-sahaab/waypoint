use dsp::filters::kalman::KalmanFilter;
use dsp::filters::kalman::StateTransition;
use dsp::filters::kalman::TFloatSquare;
use dsp::filters::kalman::TFloatVector;
use waypoint::battery::Battery;

///
/// Adc battery reader with kalman filter to reduce the noise
///
pub struct AdcBattery<'a, BatteryFn>
where
    BatteryFn: FnMut() -> f32,
{
    /// Range of battery operation
    min: f32,
    max: f32,

    battery_fn: &'a mut BatteryFn,

    /// Store voltage guess
    volts: f32,
    kalman_filter: KalmanFilter<'static, 1, 1>,
}

impl<'a, BatteryFn> AdcBattery<'a, BatteryFn>
where
    BatteryFn: FnMut() -> f32,
{
    pub fn new_lipo(battery_fn: &'a mut BatteryFn) -> Self {
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
            battery_fn,
            kalman_filter,
        }
    }
}

impl<'a, BatteryFn> Battery for AdcBattery<'a, BatteryFn>
where
    BatteryFn: FnMut() -> f32,
{
    fn update(&mut self) {
        let volts: f32 = (self.battery_fn)();

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
