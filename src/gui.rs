use gio;
use gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Builder, Image};

use std::env::args;
use std::collections::BTreeMap;
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

pub struct TestbedGui {
    application: Application,
    datapoints: Rc<RefCell<BTreeMap<String, Datapoint>>>,
    datapoint_image: Rc<RefCell<Image>>,
    datapoint_position: Rc<RefCell<usize>>
}
impl TestbedGui {
    pub fn new() -> TestbedGui {
        let application = gtk::Application::new("me.bcow.fmitestbed",
            gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

        let img_datapoint: Rc<RefCell<Image>> = Rc::new(RefCell::new(Image::new()));

        let img_datapoint_clone = img_datapoint.clone();
        application.connect_startup(move |app| {
            // added during compilation (macro)
            let glade_src = include_str!("fmitestbed.glade");
            let builder = Builder::new_from_string(glade_src);

            let main_window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get main_window");
            let img_dbz: Image = builder.get_object("img_dBZ").expect("Could'nt get img_dBZ");
            *img_datapoint_clone.borrow_mut() = builder.get_object("img_datapoint").expect("Could'nt get img_datapoint");
            // loaded during runtime
            img_dbz.set_from_file("resources/dbz.png");

            main_window.set_application(app);
            main_window.connect_delete_event(clone!(main_window => move |_, _| {
                main_window.destroy();
                Inhibit(false)
            }));

            main_window.show_all();
        });
        application.connect_activate(|_| {});

        TestbedGui { application: application,
            datapoints: Rc::new(RefCell::new(parse_testbed())),
            datapoint_image: img_datapoint,
            datapoint_position: Rc::new(RefCell::new(0)) }
    }

    pub fn run(&self) {
        // create a reference cycle clone of the datapoints in our struct and refresh data
        let datapoints_clone = self.datapoints.clone();
        let datapoints_position_clone = self.datapoint_position.clone();
        let tick = move || {
            //println!("tick");
            // refresh the FMI testbed data
            *datapoints_clone.borrow_mut() = parse_testbed();
            *datapoints_position_clone.borrow_mut() = 0;
            gtk::Continue(true)
        };
        gtk::timeout_add_seconds(5 * 60, tick);

        // create a new clone (again)
        let datapoints_clone = self.datapoints.clone();
        let datapoints_position_clone = self.datapoint_position.clone();
        let img_datapoint_clone = self.datapoint_image.clone();
        let tock = move || {
            //println!("tock");
            let map = &*datapoints_clone.borrow();

            let mut position: usize = 0;
            let old_position = *datapoints_position_clone.borrow_mut();
            for (_key, val) in map {
                if position > old_position {
                    //println!("get pixbuf for {}", _key);
                    let pixbuf = val.image_as_pixbuf();
                    *&img_datapoint_clone.borrow_mut().set_from_pixbuf(&pixbuf);
                    break;
                }
                position = position + 1;
            }
            //println!("{}", position);
            if position == 14 {
                // when finding the end roll back to begin
                position = 0;
            }
            *datapoints_position_clone.borrow_mut() = position;
            gtk::Continue(true)
        };
        gtk::timeout_add_seconds(1, tock);

        // run Gtk main loop
        self.application.run(&args().collect::<Vec<_>>());
    }
}