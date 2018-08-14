extern crate reqwest;
extern crate regex;
extern crate url;
extern crate chrono;
extern crate image;

use regex::Regex;
use url::Url;
use chrono::NaiveDateTime;

struct Datapoint {
    _url: Url,
    timestamp: NaiveDateTime,
    image: image::DynamicImage
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

    fn image(&self) -> &image::DynamicImage {
        &self.image
    }

    fn timestamp(&self) -> &chrono::NaiveDateTime {
        &self.timestamp
    }
}

fn parse_testbed() -> Vec<Datapoint> {
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
    let mut datapoints: Vec<Datapoint> = Vec::new();
    let matrix = url_re.captures_iter(&body).zip(timestamp_re.captures_iter(&body));
    for (url, timestamp) in matrix {
        println!("{} {}", &timestamp[1], &url[1]);
        datapoints.push(Datapoint::new(url[1].to_string(), timestamp[1].to_string()).unwrap());
    }
    datapoints
}

fn main() {
    // download fmi data
    let datapoints = parse_testbed();
}