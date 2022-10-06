use std::env;

// Import the flashloan-rs crate
use flashloan_rs::*;

// Run the code asynchronously in a tokio runtime
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TODO:

    // Construct the flashloan builder
    let mut builder = FlashloanBuilder::new();

    // Add multicalls in the flashloan builder to construct a pure arbitrage

    // Execute the flashloan

    println!("Executed flashloan");

    Ok(())
}
