mod battery;

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use waypoint::application::Application;

use battery::MockBattery;

fn main() -> ! {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size {
        width: 240,
        height: 240,
    });

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Waypoint mock", &output_settings);
    let display_update = |display: &SimulatorDisplay<Rgb565>| window.update(display);

    let mock_battery = MockBattery::new();

    let mut app = Application::new(&mut display, mock_battery);

    app.start_with_callback(display_update)
}
