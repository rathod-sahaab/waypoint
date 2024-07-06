use display_interface::DisplayError;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};

pub struct Display<'dt, DT> {
    pub width: u16,
    pub height: u16,
    pub draw_target: &'dt mut DT,
}

impl <'dt, DT> Display <'dt, DT>
    where DT: DrawTarget<Color = Rgb565, Error = DisplayError>{

    pub fn new(width: u16, height: u16, draw_target: &'dt mut DT)-> Self {
        Self {
            width,
            height,
            draw_target,
        }
    }
}
