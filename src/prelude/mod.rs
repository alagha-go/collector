#[cfg(test)]
mod test;

use chrono::{Duration, Datelike};
use futures::{io::{self, BufReader, ErrorKind},prelude::*};
use async_compression::futures::bufread::GzipDecoder;
use serde::{Serialize, Deserialize};
use static_init::{dynamic};
use reqwest::Client;


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default)]
pub struct Object {
    id: u32
}


#[dynamic]
pub static CLIENT: Client = Client::new();

#[dynamic]
pub static APIKEY: String = std::env::var("APIKEY").expect("APIKEY not provided");

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;


pub async fn laoad_ids(url: &str) -> Result<Vec<u32>> {
    let response = CLIENT.get(url).send().await?;
    let mut ids = Vec::new();

    let reader = response.bytes_stream().map_err(|e| io::Error::new(ErrorKind::Other, e)).into_async_read();
    let mut decoder = GzipDecoder::new(BufReader::new(reader));
    decoder.multiple_members(true); // supports multiline parsing
    let decoder = BufReader::new(decoder);
    let mut lines_stream = decoder.lines().map(|l| l.unwrap());

    let mut line = lines_stream.next().await.unwrap_or(String::new());

    while line.len() > 0 {
        let object: Object = serde_json::from_str(&line)?;
        ids.push(object.id);
        line.clear();
        line = lines_stream.next().await.unwrap_or(String::new());
    }

    Ok(ids)
}

pub fn str_from_number(number: u32) -> String {
    if number > 9 {
        return number.to_string()
    }
    String::from("0") + &number.to_string()
}


pub fn collecting_date() -> String {
    let mut time = chrono::offset::Utc::now();
    if time.time() < chrono::naive::NaiveTime::from_hms_opt(8,0,0).unwrap() {
        time-=chrono::Duration::days(1);
    }
    let (year, month, day) = (str_from_number(time.year() as u32), str_from_number(time.month()), str_from_number(time.day()));
    month+"_"+&day+"_"+&year
}