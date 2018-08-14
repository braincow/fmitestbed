extern crate reqwest;
extern crate regex;
extern crate url;
extern crate chrono;
extern crate image;
extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;

use gdk_pixbuf::{Pixbuf, Colorspace};
use regex::Regex;
use url::Url;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use image::{DynamicImage, GenericImage};

#[cfg(feature = "gtk_3_10")]
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

struct Datapoint {
    _url: Url,
    timestamp: NaiveDateTime,
    image: DynamicImage
}
impl Datapoint {
    fn new(url: String, timestamp: String) -> Result<Datapoint, Box<::std::error::Error>> {
        // parse url string into object representation
        let url: Url = match Url::parse(&url) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("url parse error: {} {}", err, url);
                return Err(Box::new(err))
            }
        };
        // parse timestamp into object representation
        let timestamp = match NaiveDateTime::parse_from_str(&timestamp, "%Y%m%d%H%M") {
            Ok(timestamp) => timestamp,
            Err(err) => {
                eprintln!("timestamp parse error: {} {}", err, timestamp);
                return Err(Box::new(err))
            }
        };
        // download the actual image data ...
        let mut res = match reqwest::get(&url.to_string()) {
            Ok(mut res) => res,
            Err(err) => {
                eprintln!("image download error: {} {}", err, url.to_string());
                panic!("{}", err);
            }
        };
        // ... and parse it from memory
        let mut image_buf: Vec<u8> = Vec::new();
        res.copy_to(&mut image_buf)?;
        let image = match image::load_from_memory_with_format(
                image_buf.as_slice(), image::ImageFormat::PNG) {
            Ok(image) => image,
            Err(err) => {
                eprintln!("Error on parsing image date {} {}", err, url);
                return Err(Box::new(err))
            }
        };
        // return Datapoint
        Ok(Datapoint { _url: url, timestamp: timestamp, image: image })
    }

    fn image(&self) -> &DynamicImage {
        &self.image
    }

    fn image_as_pixbuf(&self, alpha: bool) -> Pixbuf {
        let mut channels: i32 = 3;
        if alpha {
            channels += 1;
        }
        let pixbuf = Pixbuf::new_from_vec(
            self.image.raw_pixels(),
            Colorspace::Rgb,
            false,
            8,
            self.image.width() as i32,
            self.image.height() as i32,
            channels * self.image.width() as i32);
        pixbuf
    }

    fn timestamp(&self) -> &chrono::NaiveDateTime {
        &self.timestamp
    }
}

fn parse_testbed() -> HashMap<String, Datapoint> {
    // fetch HTML source for later parsing
    let mut res = match reqwest::get("http://testbed.fmi.fi/") {
        Ok(res) => res,
        Err(err) => {
            panic!("{}", err);
        }
    };
    //eprintln!("Status: {}", res.status());
    //eprintln!("Headers:\n{:?}", res.headers());
    let body: String = res.text().unwrap();
    let url_re = Regex::new(r"(https://.?.img.fmi.fi/php/img.php[\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])").unwrap();
    let timestamp_re = Regex::new(r"(\d{12})").unwrap();
    // zip iterators from both regexp searches into one and loop through them simultaneously
    let mut datapoints = HashMap::new();
    let matrix = url_re.captures_iter(&body).zip(timestamp_re.captures_iter(&body));
    for (url, timestamp) in matrix {
        println!("{} {}", &timestamp[1], &url[1]);
        let datapoint = Datapoint::new(url[1].to_string(), timestamp[1].to_string()).unwrap();
        datapoints.insert(String::from(&timestamp[1]), datapoint);
    }
    datapoints
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    // download fmi data
    let datapoints = parse_testbed();
    // test pixbuf conversion
    for key in datapoints.keys() {
        println!("get pixbuf for {}", key);
        let pixbuf = datapoints.get(key).unwrap().image_as_pixbuf(false);
    }
    example::main();
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    eprintln!("This program requires GTK 3.10 or later");
    eprintln!("Did you forget to build with `--features gtk_3_10`?");
}