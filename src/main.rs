extern crate gio;
extern crate gtk;
extern crate fmitestbed;

use fmitestbed::parser::parse_testbed;

mod example {
    use gio;
    use gtk;

    use gio::prelude::*;
    use gtk::prelude::*;

    use gtk::{ApplicationWindow, Builder, Image};

    use std::env::args;

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

    pub fn build_ui(application: &gtk::Application) {
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

    pub fn main() {
        let application = gtk::Application::new("me.bcow.fmitestbed",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}

fn main() {
    // download fmi data
    let datapoints = parse_testbed();
    // test pixbuf conversion
    for key in datapoints.keys() {
        println!("get pixbuf for {}", key);
        let _pixbuf = datapoints.get(key).unwrap().image_as_pixbuf();
    }
    example::main();
}