use std::error::Error;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command, ValueEnum};
use log::info;
use std::env::var;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Environment {
    Development,
    Test,
    Production
}

pub(crate) fn exec() -> Result<(bool, String), Box<dyn Error>> {
    println!("Initializing env ...");

    // check ENV to be run && set env config file name based on ENV settings
    let matches: ArgMatches = Command::new("Power.Trade Client")
    .about("Handles the environment('ENV') setting")
    .arg(
        Arg::new("env")
            .action(ArgAction::Set)
            .alias("environment")
            .short('e')
            .long("env")
            .required(true)
            .help("Select environment for the WS Client to run against")
            .value_name("pt_env")
            .value_parser(value_parser!(Environment))
     )
    .arg(Arg::new("custom-help")
        .short('?')
        .action(ArgAction::Help)
        .display_order(100)  // Don't sort
        .help("Alt help")
     )
    .get_matches();
    
    // Retrieve the value of env
    let pt_env: &Environment = matches.get_one::<Environment>("env").expect("env is required");

    // Setuo env with associated config from dotenv file
    match pt_env {
        Environment::Development => {
            println!("Environment is set to DEV");
            // Load environment variables from development version of .env file
            dotenv::from_filename(".env.dev").expect("Failed to load env values from file '.env.dev'");
        },
        Environment::Test => {
            println!("Environment is set to TEST");
            // Load environment variables from test version of .env file
            dotenv::from_filename(".env.test").expect("Failed to load env values from file '.env.test'");
        },
        Environment::Production => {
            println!("Environment is set to PROD");
            // Load environment variables from production version of .env file
            dotenv::from_filename(".env.prod").expect("Failed to load env values from file '.env.prod'");
        },
    }

    //
    // load Scenario setting from Env
    //
    let scenario: String = var("PT_SCENARIO").unwrap();
    println!("Executing Scenario : {scenario}");
    info!("Executing Scenario : {scenario}");
    Ok((true, scenario))
}