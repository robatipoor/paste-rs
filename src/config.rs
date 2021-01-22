use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: ConfigApp = {
        let home = std::env::var("HOME").unwrap();
        confy::load(&format!("{}/.config/paste-rs/config",home)).unwrap()
    };
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigApp {
    pub database_url: String,
    pub base_url: String,
    pub random_code_len : usize,
    pub create_database :bool,
}
