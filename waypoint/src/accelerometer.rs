pub struct Acceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Accelerometer {
    fn acceleration() -> Acceleration;
}
