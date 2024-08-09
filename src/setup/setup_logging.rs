use std::{error::Error, fs::File};

use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};

pub(crate) fn exec() ->  Result<bool, Box<dyn Error>> {
    //
    // Setup logging
    // - initialize the logging file
    //   TODO: replace hardcoded name('app.log') with value from env settings
    //
    CombinedLogger::init(vec![WriteLogger::new(LevelFilter::Info, Config::default(), File::create("app.log").unwrap())]).unwrap();

    Ok(true)
}