use std::{error, result};

pub mod config;
pub mod handlers;
pub mod models;
pub mod paste;
pub mod services;
pub mod utils;

use crate::paste::run;

pub type Result<T = ()> = result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

#[async_std::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("exit with error message {}", e);
    }
}
