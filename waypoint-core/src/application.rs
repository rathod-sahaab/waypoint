use core::fmt::Debug;

use embedded_graphics::prelude::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, WebColors},
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
};

pub struct Application<'dt, DT> {
    display: &'dt mut DT,
}

impl<'dt, DT, E> Application<'dt, DT>
where
    DT: DrawTarget<Color = Rgb565, Error = E>,
    E: Debug,
{
    pub fn new(display: &'dt mut DT) -> Self {
        Self { display }
    }

    pub fn start(&mut self, mut cb: impl FnMut(&DT)) -> ! {
        self.draw_frame();

        cb(self.display);
        loop {}
    }

    fn draw_frame(&mut self) {
        self.display.clear(Rgb565::CSS_SKY_BLUE).unwrap();

        let yoffset = 100;

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(3)
            .stroke_color(Rgb565::CSS_WHITE)
            .build();

        // screen outline for the round 1.28 inch Waveshare display
        Circle::new(Point::new(1, 1), 238)
            .into_styled(style)
            .draw(self.display)
            .unwrap();

        // triangle
        Triangle::new(
            Point::new(50, 32 + yoffset),
            Point::new(50 + 32, 32 + yoffset),
            Point::new(50 + 16, yoffset),
        )
        .into_styled(style)
        .draw(self.display)
        .unwrap();

        // square
        Rectangle::new(Point::new(110, yoffset), Size::new_equal(32))
            .into_styled(style)
            .draw(self.display)
            .unwrap();

        // circle
        Circle::new(Point::new(170, yoffset), 32)
            .into_styled(style)
            .draw(self.display)
            .unwrap();
    }
}
