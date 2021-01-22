use std::{error,result};

pub mod util;
pub mod config;
pub mod paste;
pub mod db;
pub mod note;
pub mod handler;

use crate::paste::run;

pub type Result<T =()> = result::Result<T,Box<dyn error::Error + Send + Sync + 'static >>;

#[async_std::main]
async fn main(){
    if let Err (e) = run().await{
        eprintln!("exit with error message {}",e);
    }
}

