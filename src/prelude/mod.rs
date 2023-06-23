#[cfg(test)]
mod test;
pub mod que;

use chrono::{Duration, Datelike};
use futures::{io::{self, BufReader, ErrorKind},prelude::*};
use async_compression::futures::bufread::GzipDecoder;
use aws_sdk_sqs::{self as sqs, Client as Sqs};
use serde::{Serialize, Deserialize};
use static_init::{dynamic};
use aws_config::SdkConfig;
use tokio::sync::OnceCell;
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
#[dynamic]
pub static QUEUEURL: String = std::env::var("QUEUEURL").expect("QUEUEURL not provided");
#[dynamic]
pub static SQSCONCURRENCY: usize = (std::env::var("SQSCONCURRENCY").expect("SQSCONCURRENCY not provided")).parse().expect("provide SQSCONCURRENCY is invalid");

pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T> = std::result::Result<T, StdError>;

static CONFIG: OnceCell<SdkConfig> = OnceCell::const_new();
static SQS: OnceCell<Sqs> = OnceCell::const_new();


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


pub fn split<T, I: Iterator<Item = T>>(iter: I, limit: usize) -> Vec<Vec<T>> {
    let mut iter = iter.peekable();
    let mut rounds = Vec::new();
    while let Some(item) = iter.peek() {
        let mut round = Vec::new();
        for _ in 0..limit {
            match iter.next() {
                Some(item) => round.push(item),
                None => break
            }
        }
        rounds.push(round)
    }
    rounds
}

pub async fn config() -> &'static SdkConfig {
    CONFIG.get_or_init(||async {aws_config::load_from_env().await}).await
}

pub async fn sqs() -> &'static Sqs {
    let config = config().await;
    SQS.get_or_init(|| async {Sqs::new(config)}).await
}