use std::{error::Error, fs::File};

use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};

#[allow(dead_code)]
pub(crate) fn exec(log_file: &str) -> Result<bool, Box<dyn Error>> {
    //
    // Setup logging
    // - initialize the logging file
    //   TODO: replace hardcoded name('app.log') with value from env settings
    //
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(log_file)?,
    )])?;

    Ok(true)
}
