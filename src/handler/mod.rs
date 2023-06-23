pub mod structs;
pub mod handle;
#[cfg(test)]
mod test;

use serde::{Serialize, Deserialize};
use lambda_runtime::LambdaEvent;
use crate::prelude::{Result};
use serde_json::{Value, Map};
use structs::*;
use handle::*;

pub async fn handle(event: LambdaEvent<String>) -> Result<Value> {
    let data: Data = serde_json::from_str(&event.payload)?;
    match data {
        Data::None => Ok(Value::Null),
        Data::Type(typ) => collection(typ).await,
        Data::HttpRequest(req) => http_data(req).await,
        Data::Queue(queue) => queue_data(queue).await
    }
}