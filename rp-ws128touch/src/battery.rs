use waypoint::battery::Battery;

pub struct AdcBattery<Read>
where
    Read: FnMut() -> f32,
{
    min: f32,
    max: f32,
    read: Read,
}

impl<Read> AdcBattery<Read>
where
    Read: FnMut() -> f32,
{
    pub fn new_lipo(read: Read) -> Self {
        Self {
            min: 3.6,
            max: 4.2,
            read,
        }
    }
}

impl<Read> Battery for AdcBattery<Read>
where
    Read: FnMut() -> f32,
{
    fn volts(&mut self) -> f32 {
        (self.read)()
    }

    fn percentage(&mut self) -> f32 {
        let range = self.max - self.min;
        let remaining = self.volts() - self.min;

        100f32 * remaining / range
    }
}
