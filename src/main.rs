#[allow(clippy::too_many_lines)]

#[path = "messages/factory.rs"]
mod factory;

#[path = "scenarios/rfq_publish.rs"]
mod rfq;

#[path = "scenarios/single_order_add_cancel.rs"]
mod order;

#[path = "messages/utils/mod.rs"]
mod utils;

use clap::{Arg, ArgAction, Command, value_parser, ValueEnum};
use factory::{get_logon_msg, get_single_order_msg, WSMessageFactory};
use jwtk::ecdsa::EcdsaPrivateKey;
use log::{error, info};
use native_tls::TlsStream;
use quickfix_msg44::field_types::{OrdType, Side};
use rfq::rfq_publish_ws;
use order::add_cancel_single_order;
use std::{env::var, fs::File, io::{Read, Write}, net::TcpStream, process::ExitCode, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use utils::{get_pkey, setup_connection};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Environment {
    Development,
    Test,
    Production
}

fn main() -> ExitCode {

    let version_info = "version 0.1.9 built on 1/6/2024";
    info!("Starting Fix client for power.trade [{version_info}]");

     // check ENV to be run && set env config file name based on ENV settings
    let matches: clap::ArgMatches = Command::new("Power.Trade Websocket Client")
    .version(version_info)
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

    // 
    // Setup logging
    // - initialize the logging file
    //   TODO: replace hardcoded name('app.log') with value from env settings
    //
    CombinedLogger::init(vec![WriteLogger::new(LevelFilter::Info, Config::default(), File::create("app.log").unwrap())]).unwrap();


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
    // load API key for selected environment
    //
    let apikey: String = var("PT_API_KEY").expect("PT_API_KEY must be set in the environment or .env file");
    info!("Using API Key: {apikey}");

    //
    // retrieve private key from file
    //
    let pkey: EcdsaPrivateKey = get_pkey();

    //
    // >> Initiate TLS Stream to handle messaging to/from power.trade server
    //
    let mut tls_stream: TlsStream<TcpStream> = setup_connection();

    //
    // Set timestamp using epoch time (secs since 01-01-1970) for JWT Claims (iat, exp,...)
    //
    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!!")
        .as_secs();

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //
    // Create order which is a SELL at high price so will not be executed, shows adding & removing orders
    // 
    //let single_leg_order = get_single_order_msg(String::from("121312312"), String::from("SOL_USD"), Side::Buy, 12312.2312, 888.0, OrdType::Limit, String::from("Some text")).expect("Error creating message");
    let single_leg_order: quickfix::Message = get_single_order_msg(apikey.clone(), now, 488.00, 1.00,String::from("SOL-USD"), 1,  1, 2).unwrap();
    println!("Created new single order using MsgFactory: {single_leg_order:?}");

    // 
    // create RFQ subscription msg for demonstrationg RFQ quote flow 
    //
    let rfq = WSMessageFactory::new_rfq_request( now.to_string(), String::from("SOL-USD"), Side::Buy, 1.50, 88.50, OrdType::Limit, apikey.clone());
    info!("Created new RFQ subscription from WS MsgFactory: {:?}", rfq);

    //
    // Create Fix LOGON Message using environment settings
    //
    let msg: quickfix::Message = match get_logon_msg(apikey.clone(), now, pkey) {
        Ok(msg) => msg,
        Err(error) => {
            error!("Error creating LOGON message -> {:?}", error);
            return ExitCode::from(1); // no way to continue - abort app
        }
    };

    //
    // Send Fix LOGON message to power.trade server via TLS channel 
    //
    match tls_stream.write(
        msg.to_fix_string()
            .expect("Error sending LOGON msg")
            .as_bytes(),
    ) {
        Ok(byte_count) => {
            println!("Sent {byte_count} bytes ... ");
        }
        Err(error) => {
            println!("Error while sending msg {error} ");
        }
    };

    sleep(Duration::from_secs(2)); // TODO - remove this sleep after testing done

    // Buffer to hold the response
    println!("Checking response to LOGON msg ...");
    let mut buffer = [0; 1024];

    let scenario: String =  var("PT_SCENARIO").expect("PT_SCENARIO must be set in the environment or .env file");
    println!("Using Scenario : {scenario}");
    info!("Using Scenario : {scenario}");
    //
    // Read response from server to LOGON request msg
    //
    match tls_stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {

                // Print response as a string
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received response: {response}");

                match scenario.as_str() {
                    "RFQ" => {
                        //execute_ws_request(rfq);
                        rfq_publish_ws(rfq);
                    }, 
                    "ORDER" => {
                        add_cancel_single_order(tls_stream, single_leg_order);
                    },
                    _ => {
                        panic!("Error - no valid scenario defined to execute");
                    }
                }
            } else {
                println!("No response received from server");
            }
        }
        Err(e) => {
            error!("Failed to read from stream: {e}");
            panic!("Failed to read from stream: {e}"); // panic since nothing else can be done here
        }
    }
    println!("Sleeping for 20 seconds ...");
    sleep(Duration::from_secs(20));

    ExitCode::from(0) // return 0 as SUCCESS status to calling exvironment
}
