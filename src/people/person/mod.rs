pub mod structs;
pub mod person;
pub mod gender;
#[cfg(test)]
pub mod test;

use serde::{Serialize, Deserialize};
use chrono::naive::NaiveDate;
use crate::prelude::*;
use structs::*;

const PERSONURL: &str = "http://api.themoviedb.org/3/person/ID?api_key=APIKEY";