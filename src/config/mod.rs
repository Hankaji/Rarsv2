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
    const DEFAULT_CONF_DIR: [&str; 2] =
        ["config/rars.json5", "/home/hankaji/.config/rars/rars.json5"];

    fn init(path: Option<&str>) -> Self {
        let config_dirs: Vec<&str> = match path {
            Some(path) => vec![path],
            None => Self::DEFAULT_CONF_DIR.to_vec(),
        };

        let mut config_file: Option<String> = None;
        for cfg_dir in &config_dirs {
            if let Ok(cfg_str) = fs::read_to_string(cfg_dir) {
                config_file = Some(cfg_str);
                break;
            }
        }

        // NOTE: Maybe can construct a Config struct with default values in case fail
        // Panic if config file cannot be found
        let Some(config_file) = config_file else {
            panic!("Can't find the config file in specified path \"{config_dirs:?}\"");
        };
        // println!("Rars config: {:?}", json5::from_str::<Config>(&config_file).unwrap());

        json5::from_str::<Config>(&config_file).expect("Cant parse configuration file")
    }
}
