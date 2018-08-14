extern crate fmitestbed;

use fmitestbed::gui::TestbedGui;

fn main() {
    // build ui and start gtk main loop in app
    let gui = TestbedGui::new();
    gui.run();
}