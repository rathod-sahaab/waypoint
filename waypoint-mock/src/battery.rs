use waypoint::battery::Battery;

pub struct MockBattery {
    min: f32,
    max: f32,
}

impl MockBattery {
    pub fn new() -> MockBattery {
        Self { min: 3.6, max: 4.2 }
    }
}

impl Battery for MockBattery {
    fn volts(&mut self) -> f32 {
        3.9
    }

    fn percentage(&mut self) -> f32 {
        let range = self.max - self.min;
        let remaining = self.volts() - self.min;
        100f32 * remaining / range
    }
}
