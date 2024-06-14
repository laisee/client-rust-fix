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
use factory::FixMessageFactory;
use jwtk::ecdsa::EcdsaPrivateKey;
use log::{error, info};
use order::add_cancel_single_order;
use native_tls::TlsStream;
use quickfix::Message;
use quickfix_msg44::field_types::{OrdType, Side};
use rfq::rfq_publish_fix;
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
    // load Scenario setting from Env
    //
    let scenario: String =  var("PT_SCENARIO").expect("PT_SCENARIO must be set in the environment or .env file");
    println!("Using Scenario : {scenario}");
    info!("Using Scenario : {scenario}");

    //
    // retrieve private key from local pem file
    //
    let pkey: EcdsaPrivateKey = get_pkey();

    //
    // >> Initiate TLS Stream to handle messaging to/from power.trade server
    //
    let mut tls_stream: TlsStream<TcpStream> = setup_connection();


    //
    // Default values for new order, new rfq quotes below
    // TODO - assign values fron .env file
    // 
    static SYMBOL: &str = "SOL-USD";
    const PRICE: f64 = 388.00; 
    const QUANTITY: f64 = 5.00; 
    const SIDE: Side = Side::Sell;
    const ORDERTYPE: OrdType = OrdType::Limit;
    let seqnum: u32 = 2; // Sequence is LOGON + 1 means we expect to run single sceniario here. extend seqnum logic if needed later

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //
    // Create order which is a SELL at high price which will not be executed
    // 
    // - generate timestamp using epoch time (secs since 01-01-1970) for JWT Claims (iat, exp,...)
    //
    let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!!").as_secs();
    let order_msg: Message = FixMessageFactory::new_single_leg_order(apikey.clone(), now, PRICE, QUANTITY, SYMBOL.to_string(), SIDE, ORDERTYPE, seqnum).unwrap();
    println!("Created new single leg order msg using FixMsgFactory  {order_msg:?}");

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 
    // create RFQ subscription msg for demonstrationg RFQ quote flow 
    //
    // array of topica(symbols) to subscribe for RFQ updates
    let topics: Vec<String> = ["ETH-USD", "SOL-USD", "DOGE"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let rfq_sub_msg: Message = FixMessageFactory::new_rfq_sub(topics).unwrap().into();
    println!("Created new RFQ subscribe msg using FixMsgFactory: {:?}", rfq_sub_msg);


    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 
    // Create RFQ quote msg for creating new RFQ quote for a symbol
    //
    let rfq_quote_msg: Message = FixMessageFactory::new_rfq_quote(&now.to_string(), &apikey, SYMBOL.to_string(), SIDE, QUANTITY,ORDERTYPE, seqnum).unwrap();
    println!("Created new RFQ publish msg using FixMsgFactory: {:?}", rfq_quote_msg);

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //
    // Create Fix LOGON Message using environment settings
    //
    let logon_msg: Message = match FixMessageFactory::new_logon( apikey.clone(), pkey) {
        Ok(logon_msg) => {
            info!("Created new Fix Logon msg : {:?}", logon_msg);
            logon_msg
        },
        Err(error) => {
            error!("Error creating Fix Logon message -> {:?}", error);
            return ExitCode::from(1); // no way to continue - abort app
        }
    };

    // Send Fix LOGON message to server via TLS channel 
    match tls_stream.write( logon_msg.to_fix_string().expect("Error converting Logon Msg to Bytes").as_bytes()) {
        Ok(byte_count) => { println!("Sent {byte_count} bytes"); }
        Err(error) => { println!("Error while sending msg {error} "); }
    };

    // setup buffer to hold the logon msg response
    println!("Checking response to LOGON msg ...");
    let mut buffer = [0; 1024];

    //
    // Read response to LOGON request msg
    //
    match tls_stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Message received with {bytes_read} bytes");
            if bytes_read > 0 {

                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                info!("Received Logon response: {response}");
                println!("Received Logon response: {response}");

                //
                // Received logon response, now execute next step for sample client functionality
                //
                match scenario.as_str() {
                    "ORDER" => {
                        // publish new limit order & listen for response msg
                        add_cancel_single_order(tls_stream, order_msg);
                    },
                    "RFQ" => {
                        // publish RFQ quote request & listen for response msg
                        rfq_publish_fix(tls_stream, rfq_quote_msg);
                    }, 
                    "LISTEN" => {
                        //
                        // publish RFQ subscription request
                        // TODO - retrieve set of coins to listen for (.env)
                        // TODO - implement and test subcribe/listen/unsubscribe 
                        //
                        rfq_publish_fix(tls_stream, rfq_sub_msg);
                    },
                    _ => {
                        panic!("Error - no valid scenario defined to execute");
                    }
                }
            } else {
                info!("No response received from server for Logon request");
                println!("No response received from server for Logon request");
            }
        }
        Err(e) => {
            error!("Failed to read from stream: {e}");
            panic!("Failed to read from stream: {e}"); // panic since nothing else can be done here
        }
    }
    println!("Sleeping for 40 seconds to allow inspecting data, reviewing UI updates. To be removed ...");
    sleep(Duration::from_secs(40));
    ExitCode::from(0) // return 0 as SUCCESS status to calling exvironment
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
