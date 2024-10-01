use alloc::format;
use core::fmt::Debug;
use embedded_graphics::mono_font::iso_8859_7::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, WebColors},
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
};

use crate::battery::Battery;

///
/// DT: dipslay_target
/// BAT: battery
pub struct Application<'dt, DT, BAT> {
    display: &'dt mut DT,
    battery: BAT,
}

impl<'dt, DT, E, BAT> Application<'dt, DT, BAT>
where
    DT: DrawTarget<Color = Rgb565, Error = E>,
    E: Debug,
    BAT: Battery,
{
    pub fn new(display: &'dt mut DT, battery: BAT) -> Self {
        Self { display, battery }
    }

    pub fn start(&mut self) -> ! {
        for _ in 0..50 {
            self.update();
        }
        self.redraw();
        loop {
            self.update();
            self.draw_frame();
        }
    }

    pub fn start_with_callback(&mut self, mut cb: impl FnMut(&DT)) -> ! {
        self.redraw();
        loop {
            self.update();
            self.draw_frame();

            cb(self.display);
        }
    }

    fn update(&mut self) {
        self.battery.update();
    }

    fn redraw(&mut self) {
        self.display.clear(Rgb565::CSS_LIGHT_SLATE_GRAY).unwrap();
    }
    fn draw_frame(&mut self) {
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

        let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

        Text::with_alignment(
            format!(
                "{:.2}% @{:.2}V",
                self.battery.percentage(),
                self.battery.volts()
            )
            .as_str(),
            Point::new(120, 60 + yoffset),
            text_style,
            Alignment::Center,
        )
        .draw(self.display)
        .unwrap();
    }
}
