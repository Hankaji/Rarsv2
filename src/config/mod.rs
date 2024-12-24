use std::fs;

use bar_conf::BarConf;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

pub mod bar_conf;
mod utils;

lazy_static! {
    pub static ref CONFIG: Config = Config::init(None);
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub bar: BarConf,
}

impl Config {
    const DEFAULT_CONF_DIR: &str = "config/rars.json5";

    fn init(path: Option<&str>) -> Self {
        let config_dir = match path {
            Some(path) => path,
            None => Self::DEFAULT_CONF_DIR,
        };

        let config_file = fs::read_to_string(config_dir).expect("Config file not found!");
        // println!("Rars config: {:?}", json5::from_str::<Config>(&config_file).unwrap());

        json5::from_str::<Config>(&config_file).expect("Cant parse configuration file")
    }
}
