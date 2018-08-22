#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate url;
extern crate url_serde;

use url::{Url,};
use serde_json::Value;
use chrono::prelude::*;
use chrono::serde::ts_seconds;




#[derive(Deserialize, Debug)]
enum ItemType {
    job, 
    story,
    comment,
    poll,
    pollopt,
}

fn default_deleted() -> bool {
    false
}

fn default_kids() -> Vec<u32> {
    Vec::new()
}

#[derive(Deserialize, Debug)]
struct Item {
    id : u32,
    by : String,
    #[serde(with = "ts_seconds")]
    time : DateTime<Utc>,
    #[serde(default = "default_kids")]
    kids : Vec<u32>,
    score : Option<u32>,
    title : Option<String>,
    //#[serde(deserialize_with = "url_serde", default)]
    //url : Option<Url>,
    descendants : Option<u32>,
    parent : Option<u32>,
    parts : Option<Vec<u32>>,
    #[serde(default = "default_deleted")]
    deleted : bool,
    #[serde(rename = "type")]
    item_type : ItemType,
    text : Option<String>,
    #[serde(default = "default_deleted")]
    dead : bool,

}

fn main() {
    println!("{}", get_item(8863));
    println!("{}", get_item(2921983));
    println!("{}", get_item(121003));
    println!("{}", get_item(192327));
    println!("{}", get_item(126809));
    println!("{}", get_item(160705));
}

fn get_item(id: u32) -> String {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let res = reqwest::get(&url);
    match res {
        Ok(mut res) => {
            if res.status().is_success() {
                let text = &res.text().unwrap();
                let item : Item = serde_json::from_str(text).unwrap();

                format!("{:?}", item)
            } else {
                format!("Error while parsing url.\nRecieved {}", res.status())
            }
        },
        Err(_) => {
            format!("Failed to parse URL")
        }
    }
}
