pub trait Battery {
    fn update(&mut self);
    fn volts(&mut self) -> f32;
    fn percentage(&mut self) -> f32;
}
