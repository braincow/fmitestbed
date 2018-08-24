use gio;
use gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Builder, Image};

use std::env::args;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

struct Datapoints {
    datapoints: HashMap<String, Datapoint>
}
impl Datapoints {
    fn new() -> Datapoints {
        // return empty hashmap
        Datapoints { datapoints: HashMap::new() }
    }

    fn tick(&mut self) -> Continue {
        // download fmi data
        self.datapoints = parse_testbed();

        // we could return gtk::Continue(false) to stop our refreshing data
        gtk::Continue(true)
    }
}

pub struct TestbedGui {
    application: Application,
    datapoints: RefCell<Datapoints>
}
impl TestbedGui {
    pub fn new() -> Rc<TestbedGui> {
        let application = gtk::Application::new("me.bcow.fmitestbed",
            gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        let instance = Rc::new(TestbedGui { application: application, datapoints: RefCell::new(Datapoints::new()) });
        // executes the update every five minutes
        instance.datapoints.borrow().tick();
        //gtk::timeout_add_seconds(5 * 60, instance.datapoints.borrow().tick);

        instance
    }

    pub fn run(&self) {
        // run Gtk main loop
        self.application.run(&args().collect::<Vec<_>>());
    }
}