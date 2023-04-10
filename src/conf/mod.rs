use serde::{Serialize, Deserialize};
use log4rs;

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub port: u16
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pattern: String,
    mode: String,
    level: String,
    dir: String,
    conf_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    log: Log
}

impl Config {
    pub fn init_log(&self) {
        log4rs::init_file(&self.log.conf_file, Default::default()).unwrap()
    }
}