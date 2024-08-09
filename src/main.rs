#[allow(clippy::too_many_lines)]

#[path = "messages/factory.rs"]
mod factory;

#[path = "scenarios/rfq_listen.rs"]
mod listen;

#[path = "scenarios/rfq_publish.rs"]
mod publish;

#[path = "scenarios/single_leg_order.rs"]
mod single_leg_order;

#[path = "messages/utils/mod.rs"]
mod utils;

pub(crate) mod setup;

use clap::ValueEnum;
use client_rust_fix::common::increment_seqnum;
use factory::FixMessageFactory;
use log::{error,info};
use native_tls::TlsStream;
use publish::rfq_publish_fix;
use quickfix::Message;
use quickfix_msg44::field_types::{OrdType, Side};
use setup::{setup_env, setup_heartbeat, setup_keys, setup_logging, setup_rfq, setup_session, setup_trading};
use single_leg_order::{send_single_order, send_multiple_orders};
use std::{io::Write, net::TcpStream, option::Option::Some, process::ExitCode, thread::sleep, time::Duration};
use utils::setup_tls_connection;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Environment {
    Development,
    Test,
    Production
}
pub fn main() -> ExitCode {
    let version = "version 0.1.9 built on 1/6/2024";
    info!("Starting Fix client for power.trade [{version}]");
    const SUCCESS: u8 = 0;
    const FAILURE: u8 = 1;

    // read env vars and default settings
    let (status, scenario) = setup_env::exec().unwrap();
    if !status {
        println!("Error while setting up 'env'");
        return ExitCode::from(FAILURE);
    }

    // setup logging style and level
    if !setup_logging::exec().unwrap() {
        println!("Error while setting up 'logging'");
        return ExitCode::from(FAILURE);
    }

    // setup heartbeat process used for maintaining Fix connection
    if !setup_heartbeat::exec().unwrap() {
        println!("Error while setting up 'heartbeat'");
        return ExitCode::from(FAILURE);
    }

    // read and initialize keys used for Fix session and power.trade trading
    let (status, apikey, pkey ) = setup_keys::exec().unwrap();
    if !status {
        println!("Error while setting up 'keys'");
        return ExitCode::from(FAILURE);
    }

    // Initiate TLS Stream to handle messaging to/from power.trade server
    let mut tls_stream: TlsStream<TcpStream> = setup_tls_connection();

    let (status, seqnum) = setup_session::exec(&apikey, pkey.clone(), &mut tls_stream).unwrap();
    if !status {
        println!("Error while setting up 'session'");
        return ExitCode::from(FAILURE);
    } else {
        println!("Seqnum initialized with value {:?}", seqnum.lock().unwrap() );
    }

    // setup common trading settings and defaults
    if !setup_trading::exec().unwrap() {
        println!("Error while setting up 'trading'");
        return ExitCode::from(FAILURE);
    }

    //
    // Execute assigned scenario now session is opened
    //
    match scenario.as_str()  {
        "ORDER" => {
            // TODO - take these values from setup_trading call
            const PRICE: f64 = 388.00; 
            const QUANTITY: f64 = 2.00; 
            const SIDE: Side = Side::Sell;
            const ORDERTYPE: OrdType = OrdType::Limit;
            let symbol: String = "SOL-USD".to_string();

            //
            // publish new limit single leg order, listen for response msg and cancel (if cancel_order == 'true')
            // use current seqnum(latest) for new order
            let mut seqnum_latest = *seqnum.lock().unwrap() ;
            let order_msg = FixMessageFactory::new_single_leg_order(apikey.clone(), PRICE, QUANTITY, symbol, SIDE,ORDERTYPE, seqnum_latest).unwrap();

            // - increment seqnum for use in cancel order
            seqnum_latest = increment_seqnum(seqnum.clone()); 
            info!("Sequence number incremented to {:?} after sending New Order message {:?}", seqnum_latest, order_msg);
            send_single_order(&apikey.clone(),  &mut tls_stream, order_msg.clone(), seqnum_latest, Some(true));
        },
        "ORDERS" => {
            //
            // publish new set of limit single leg orders, listen for response msg and cancel (if cancel_order == 'true')
            //
            const PRICE: f64 = 388.00; 
            const QUANTITY: f64 = 2.00; 
            const SIDE: Side = Side::Sell;
            const ORDERTYPE: OrdType = OrdType::Limit;
            let symbol: String = "SOL-USD".to_string();

            // use current seqnum(latest) for new order
            let mut seqnum_latest = *seqnum.lock().unwrap() ;
            let order_msg = FixMessageFactory::new_single_leg_order(apikey.clone(), PRICE, QUANTITY, symbol, SIDE,ORDERTYPE, seqnum_latest).unwrap();
            let orders: Vec<Message> = vec![order_msg.clone()];

            // increment seqnum for cancel of new order 
            seqnum_latest = increment_seqnum(seqnum.clone()); 
            info!("Sequence number incremented to {:?} after sending New Order message {:?}", seqnum_latest, order_msg.clone());

            // TODO - enhance send_nultiple to manage seqnums
            send_multiple_orders(&apikey, tls_stream, orders, seqnum_latest, true);
        },
        "RFQ_QUOTE" => {
            //
            // publish RFQ quote request & listen for response msgs
            //
            // use current seqnum(latest) for new quote
            let mut seqnum_latest = *seqnum.lock().unwrap();

            let (status, rfq_quote_msg ) = setup_rfq::exec(&apikey, seqnum_latest).unwrap();
            if !status {
                error!("Error while setting up 'rfq'");
                return ExitCode::from(FAILURE);
            } else {
                seqnum_latest = increment_seqnum(seqnum.clone()); 
                info!("Sequence number incremented to {:?} after sending RFQ message {:?}", seqnum_latest, rfq_quote_msg);
            }
            info!("Sending RFQ Quote {:?}", rfq_quote_msg);
            println!("Sending RFQ Quote {:?}", rfq_quote_msg);
            rfq_publish_fix(tls_stream, rfq_quote_msg);
        }, 
        "RFQ_LISTEN" => {
            //
            // publish RFQ subscription request
            // TODO - implement and test subcribe/listen/unsubscribe 
            // TODO - refactor setup to return either quote/listem msg
            //

            // use current seqnum(latest) for new quote
            let mut seqnum_latest = *seqnum.lock().unwrap();

            let (status, rfq_subscribe_msg) = setup_rfq::exec(&apikey, seqnum_latest).unwrap();
            if !status {
                println!("Error while setting up 'rfq'");
                return ExitCode::from(FAILURE);
            } else {
                seqnum_latest = increment_seqnum(seqnum.clone()); 
                info!("Sequence number incremented to {:?} after sending RFQ message {:?}", seqnum_latest, rfq_subscribe_msg);
            }
            info!("Sending RFQ Listen {:?}", rfq_subscribe_msg);
            println!("Sending RFQ Listen {:?}", rfq_subscribe_msg);
            rfq_publish_fix(tls_stream, rfq_subscribe_msg);
            },
        _ => {
            panic!("Error - no valid scenario defined to execute. Value provided was '{scenario}'");
        }
    }
    ExitCode::from(SUCCESS) // return SUCCESS(0) status to calling exvironment
}

fn _send_heartbeat(apikey: String, seqnum: u32, mut tls_stream: TlsStream<TcpStream>) {

    println!("Hello from spawned thread for seqnum {seqnum}");
    let heartbeat_msg: Message = match FixMessageFactory::heartbeat(apikey, seqnum, "PT-OE") {
        Ok(heartbeat_msg) => {
            info!("Created new Fix heartbeat msg : {:?}", heartbeat_msg);
            heartbeat_msg
        },
        Err(error) => {
            error!("Error creating Fix Logon message -> {error:?}");
            return;
        }
    };
    match tls_stream.write( heartbeat_msg.to_fix_string().expect("Error converting Logon Msg to Bytes").as_bytes()) {
        Ok(byte_count) => { println!("Sent {byte_count} bytes for heartbeat"); }
        Err(error) => { println!("Error while sending msg {error} "); }
    };
    sleep(Duration::from_millis(500));
}
