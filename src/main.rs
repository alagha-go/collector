#![allow(unused, dead_code)]
use prelude::*;
use lambda_runtime::{service_fn};

mod prelude;
mod people;
mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    lambda_runtime::run(service_fn(handler::handle)).await?;
    Ok(())
}