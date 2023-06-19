#![allow(unused, dead_code)]
use prelude::*;

mod prelude;
mod people;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", collecting_date());
    Ok(())
}