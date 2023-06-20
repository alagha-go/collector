pub mod structs;
pub mod handle;

use serde::{Serialize, Deserialize};
use lambda_runtime::LambdaEvent;
use crate::prelude::{Result};
use serde_json::{Value};
use structs::*;
use handle::*;

pub async fn handle(event: LambdaEvent<Data>) -> Result<Value> {
    match event.payload {
        Data::None => Ok(Value::Null),
        Data::Type(typ) => collection(typ).await,
        Data::HttpRequest(req) => http_data(req).await,
        Data::Queue(queue) => queue_data(queue).await
    }
}