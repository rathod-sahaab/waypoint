use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use waypoint::application::Application;

fn main() -> ! {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size {
        width: 240,
        height: 240,
    });

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Waypoint mock", &output_settings);
    let display_update = |display: &SimulatorDisplay<Rgb565>| window.update(display);

    let mut app = Application::new(&mut display);

    app.start(display_update)
}
