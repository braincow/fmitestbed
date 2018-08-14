use std::collections::HashMap;
use datapoint::Datapoint;
use reqwest::get;
use regex::Regex;

pub fn parse_testbed() -> HashMap<String, Datapoint> {
    // fetch HTML source for later parsing
    let mut res = match get("http://testbed.fmi.fi/") {
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