use display_interface::DisplayError;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::{Rgb565, WebColors}, primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle}};
use embedded_graphics::prelude::*;

use crate::display::Display;

pub struct Application<'dt,DT> {
    display: Display<'dt, DT>,
}


impl <'dt, DT> Application <'dt, DT>
    where DT: DrawTarget<Color = Rgb565, Error = DisplayError>{

    pub fn new(display: Display<'dt, DT>) -> Self {
        Self {
            display
        }
    }

    pub fn start(&mut self) -> ! {
        self.draw_frame();

        loop {

        }
    }

    fn draw_frame(&mut self) {
        self.display.draw_target.clear(Rgb565::CSS_SKY_BLUE).unwrap();

        let yoffset = 100;

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(3)
            .stroke_color(Rgb565::CSS_WHITE)
            .build();

        // screen outline for the round 1.28 inch Waveshare display
        Circle::new(Point::new(1, 1), 238)
            .into_styled(style)
            .draw(self.display.draw_target)
            .unwrap();

        // triangle
        Triangle::new(
            Point::new(50, 32 + yoffset),
            Point::new(50 + 32, 32 + yoffset),
            Point::new(50 + 8, yoffset),
        )
        .into_styled(style)
        .draw(self.display.draw_target)
        .unwrap();

        // square
        Rectangle::new(Point::new(110, yoffset), Size::new_equal(32))
            .into_styled(style)
            .draw(self.display.draw_target)
            .unwrap();

        // circle
        Circle::new(Point::new(170, yoffset), 32)
            .into_styled(style)
            .draw(self.display.draw_target)
            .unwrap();
    }
}
