use std::fs;

use bar_conf::BarSettings;
use lazy_static::lazy_static;
use miette::{Diagnostic, NamedSource, SourceSpan};
use serde::Deserialize;
use theme::ThemeSettings;
use thiserror::Error;

pub mod bar_conf;
pub mod theme;
mod utils;

lazy_static! {
    pub static ref CONFIG: Config = Config::init(None);
}

#[derive(Error, Debug, Diagnostic)]
#[error("Configuration parse failed!")]
// #[diagnostic(code(oops::my::bad), url(docsrs))]
struct ConfigError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("{msg}")]
    span: SourceSpan,
    msg: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub theme: ThemeSettings,
    pub bar: BarSettings,
}

impl Config {
    const DEFAULT_CONF_DIR: [&str; 2] = ["config/rars.ron", "/home/hankaji/.config/rars/rars.ron"];

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

        // TODO: Implement a more friendly error reporting upon failure
        match ron::from_str(&config_file) {
            Ok(cfg) => cfg,
            Err(e) => {
                let (line, col) = (e.position.line, e.position.col);

                let lines: Vec<&str> = config_file.split('\n').collect();
                let offset_start: usize = lines[..line - 1] // Take all lines before the target line
                    .iter()
                    .map(|l| l.len() + 1) // Add 1 for the newline character
                    .sum();

                let start = offset_start + col;

                let err = ConfigError {
                    src: NamedSource::new("rars.ron", config_file.clone()),
                    span: (start - 2, 1).into(),
                    msg: match e.code {
                        ron::Error::Message(msg) => msg,
                        _ => format!("{:?}", e.code),
                    },
                };
                panic!("{:?}", miette::Report::new(err));
            }
        }
    }
}
