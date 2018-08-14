use gio;
use gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Builder, Image};

use std::env::args;
use std::collections::HashMap;
use datapoint::Datapoint;
use parser::parse_testbed;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    // added during compilation (macro)
    let glade_src = include_str!("fmitestbed.glade");
    let builder = Builder::new_from_string(glade_src);

    let main_window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get main_window");
    let img_dbz: Image = builder.get_object("img_dBZ").expect("Could'nt get img_dBZ");
    // loaded during runtime
    img_dbz.set_from_file("resources/dbz.png");

    main_window.set_application(application);
    main_window.connect_delete_event(clone!(main_window => move |_, _| {
        main_window.destroy();
        Inhibit(false)
    }));

    main_window.show_all();
}

pub struct TestbedGui {
    application: Application,
    datapoints: HashMap<String, Datapoint>
}
impl TestbedGui {
    pub fn new() -> TestbedGui {
        let application = gtk::Application::new("me.bcow.fmitestbed",
            gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});
        
        TestbedGui { application: application, datapoints: HashMap::new() }
    }

    pub fn run(&self) {
        // we are using a closure to capture the data
        //let mut datapoints: HashMap<String, Datapoint> = HashMap::new();
        let tick = move || {
            // download fmi data
            self.datapoints = parse_testbed();
            // test pixbuf conversion
            //for key in self.datapoints.keys() {
            //    println!("get pixbuf for {}", key);
            //    let _pixbuf = self.datapoints.get(key).unwrap().image_as_pixbuf();
            //}

            // we could return gtk::Continue(false) to stop our refreshing data
            gtk::Continue(true)
        };

        // executes the closure once every five minutes
        gtk::timeout_add_seconds(5 * 60, tick);

        // run Gtk main loop
        self.application.run(&args().collect::<Vec<_>>());
    }
}