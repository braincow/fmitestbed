use gdk_pixbuf::{Pixbuf, Colorspace};
use url::Url;
use chrono::NaiveDateTime;
use image::{DynamicImage, GenericImage};
use reqwest::get;
use image::load_from_memory;

pub struct Datapoint {
    _url: Url,
    timestamp: NaiveDateTime,
    image: DynamicImage
}
impl Datapoint {
    pub fn new(url: String, timestamp: String) -> Result<Datapoint, Box<::std::error::Error>> {
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
        let mut res = match get(&url.to_string()) {
            Ok(mut res) => res,
            Err(err) => {
                eprintln!("image download error: {} {}", err, url.to_string());
                panic!("{}", err);
            }
        };
        // ... and parse it from memory
        let mut image_buf: Vec<u8> = Vec::new();
        res.copy_to(&mut image_buf)?;
        let image = match load_from_memory(image_buf.as_slice()) {
            Ok(image) => image,
            Err(err) => {
                eprintln!("Error on parsing image date {} {}", err, url);
                return Err(Box::new(err))
            }
        };
        // return Datapoint
        Ok(Datapoint { _url: url, timestamp: timestamp, image: image })
    }

    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    pub fn image_as_pixbuf(&self) -> Pixbuf {
        let pixbuf = Pixbuf::new_from_vec(
            self.image.raw_pixels(),
            Colorspace::Rgb,
            false,
            8,
            self.image.width() as i32,
            self.image.height() as i32,
            3 * self.image.width() as i32);
        pixbuf
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }
}
