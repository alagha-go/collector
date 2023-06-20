pub mod person;

use crate::prelude::{Result, collecting_date, laoad_ids, StdError, split};
use self::person::structs::Person;
use futures::future::join_all;
use rayon::iter::Either;
use rayon::prelude::*;


const URL: &str = "http://files.tmdb.org/p/exports/person_ids_";
const ROUNDSIZE: usize = 500;

pub async fn ids() -> Result<Vec<Vec<u32>>> {
    let url =  String::new() + URL + &collecting_date() + ".json.gz";
    let ids = laoad_ids(&url).await?;
    Ok(split(ids.into_iter(), ROUNDSIZE))
}

pub async fn collect(ids: &[u32]) -> (Vec<(u32, StdError)>, Vec<Person>) {
    let mut futures = Vec::new();
    for id in ids {
        futures.push(Person::from_imdb_json(*id))
    }
    let results = join_all(futures).await;
    let mut index = 0;
    let mut jsons = Vec::new();
    let mut error_ids = Vec::new();
    results.into_iter().map(|result| {
        match result {
            Ok(json) => jsons.push((json, ids[index])),
            Err(err) => error_ids.push((ids[index], err))
        }
        index+=1;
    });
    let (mut errors, people): (Vec<_>, Vec<_>) = jsons.par_iter().map(|(json, id)| {
        match Person::from_json(json) {
            Ok(person) => Either::Right(person),
            Err(err) => Either::Left((*id, err))
        }
    }).collect();
    error_ids.append(&mut errors);
    (error_ids, people)
}