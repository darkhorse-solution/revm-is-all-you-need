mod revm_examples;
use crate::revm_examples::*;
use chrono::{Local};

#[tokio::main]
async fn main() {
    println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string());
    evm_main().await;
    println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string());
    println!("Hello, world!");
}
