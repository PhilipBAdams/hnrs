#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate html2text;

use chrono::prelude::*;
use chrono::serde::ts_seconds;
use chrono::Duration;
use chrono::DateTime;
use html2text::from_read;

static TEXT_COLS : usize = 120;

fn format_date(date : DateTime<Utc>) -> String {
    let time_difference = Utc::now().signed_duration_since(date);
    assert!(Duration::minutes(1) < time_difference);

    match time_difference.num_minutes() {
        m if m < 3 => String::from("just now"),
        3...60 => format!("{} minutes ago", time_difference.num_minutes()),
        60 ...360 => format!("{} hours and {} minutes ago", time_difference.num_hours(), (time_difference - Duration::hours(time_difference.num_hours())).num_minutes()),
        360 ... 1440 => format!("{} hours ago", time_difference.num_hours()),
        1440 ... 14400 => format!("{} days ago", time_difference.num_days()),
        14400 ... 100800 => format!("{} weeks ago", time_difference.num_weeks()),
        _ => format!("on {}", date.format("%B %e, %Y").to_string()),
    }
}

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
    url : Option<String>,
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
    println!("{:?}", get_item(8863).unwrap());
    println!("{:?}", get_item(2921983).unwrap());
    println!("{:?}", get_item(121003).unwrap());
    println!("{:?}", get_item(192327).unwrap());
    println!("{:?}", get_item(126809).unwrap());
    println!("{:?}", get_item(160705).unwrap());
    println!("{}", render_compact(get_item(8863).unwrap()));

    let best = reqwest::get("https://hacker-news.firebaseio.com/v0/beststories.json");

    match best {
        Ok(mut best) => {
            let text = &best.text().unwrap();
            let bestids : Vec<u32> = serde_json::from_str(text).unwrap();
            println!("{:?}", bestids);
            for i, id in bestids {
                println!("{}", render_compact(get_item(id).unwrap()));
            }
        }
        _ => ()
    }

    
    
}



fn get_item(id: u32) -> Result<Item,String> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let res = reqwest::get(&url);
    match res {
        Ok(mut res) => {
            if res.status().is_success() {
                let text = &res.text().unwrap();
                let item : Item = serde_json::from_str(text).unwrap();
                
                Ok(item)
            } else {
                Err(format!("Error while parsing url.\nRecieved {}", res.status()))
            }
        },
        Err(_) => {
            Err(format!("Failed to parse URL"))
        }
    }
}

fn render_text(text : &str) -> String {
    from_read(text.as_bytes(), TEXT_COLS)
}

fn render_compact(item : Item) -> String {
    format!("{}\n by {} {}", item.title.unwrap(), item.by, format_date(item.time))
}
