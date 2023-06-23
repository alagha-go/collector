#![allow(unused, dead_code)]
use lambda_runtime::{service_fn};
use serde_json::Value;
use prelude::*;

mod prelude;
mod people;
mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    lambda_runtime::run(service_fn(handler::handle)).await?;
    Ok(())
}