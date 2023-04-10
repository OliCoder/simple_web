mod conf;

use std::{path::PathBuf, error::Error};
use glob::glob;
use clap::Parser;
use crate::conf::Config as LocalConfig;
use config::{Config, File};

#[derive(Parser, Debug)]
#[command(author="jingui.chen", version, about="simple_web", long_about = None)]
pub struct Args {
    /// Config file path
    #[arg(short, long, value_name = "FILE")]
    conf: PathBuf,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8
}

pub fn parse() -> Result<LocalConfig, impl Error> {
    let args = Args::parse();
    let config = Config::builder()
        .add_source(
            glob(&format!("{}/*", args.conf.to_str().unwrap()))
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .build()?;
    config.try_deserialize::<LocalConfig>()
}