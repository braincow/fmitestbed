extern crate fmitestbed;

use fmitestbed::parser::parse_testbed;
use fmitestbed::gui::TestbedGui;

fn main() {
    // download fmi data
    let datapoints = parse_testbed();
    // test pixbuf conversion
    for key in datapoints.keys() {
        println!("get pixbuf for {}", key);
        let _pixbuf = datapoints.get(key).unwrap().image_as_pixbuf();
    }
    // build ui and start gtk main loop in app
    let gui = TestbedGui::new();
    gui.run();
}