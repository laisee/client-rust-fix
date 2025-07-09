use std::{error::Error, fs::File};
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};
use crate::config::Settings;

#[allow(dead_code)]
pub(crate) fn exec(settings: &Settings) -> Result<bool, Box<dyn Error>> {
    //
    // Setup logging
    // - initialize the logging file using the value from settings
    //
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(&settings.log_file)?,
    )])?;

    Ok(true)
}
