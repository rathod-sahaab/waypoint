pub trait Battery {
    fn volts(&mut self) -> f32;
    fn percentage(&mut self) -> f32;
}
