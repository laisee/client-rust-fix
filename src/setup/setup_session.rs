use jwtk::ecdsa::EcdsaPrivateKey;
use log::{info, error};
use native_tls::TlsStream;
use std::{error::Error, io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}};
use quickfix::Message;
use crate::{factory::FixMessageFactory, utils::get_attr, increment_seqnum};

#[allow(clippy::type_complexity)]
pub(crate) fn exec(apikey: &str, pkey: EcdsaPrivateKey, tls_stream: &mut TlsStream<TcpStream>) ->  Result<(bool, Arc<Mutex<u32>>), Box<dyn Error>> {

    let mut status: bool = false;

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //
    // Create Fix LOGON Message using environment settings
    //
    let logon_msg: Message = match FixMessageFactory::new_logon( apikey.to_string(), pkey) {
        Ok(logon_msg) => {
            println!("Created new Fix Logon msg : {:?}", logon_msg);
            logon_msg
        },
        Err(error) => {
            error!("Error creating Fix Logon message -> {error:?}");
            return Err(Box::new(error));
        }
    };

    //
    // Send Fix LOGON message to server via TLS channel 
    //
    match tls_stream.write( logon_msg.to_fix_string().expect("Error converting Logon Msg to Bytes").as_bytes()) {
        Ok(byte_count) => { println!("Sent {byte_count} bytes"); }
        Err(error) => { println!("Error while sending msg {error} "); }
    };
    println!("Checking response to LOGON msg ...");

    //
    // setup buffer to hold the logon msg response
    //
    let mut buffer = [0; 1024];

    //
    // Read response to LOGON request msg
    //
    match tls_stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                let mut islogin_complete = false;
                while !islogin_complete {
                    let response:String = String::from_utf8_lossy(&buffer[..bytes_read]).replace('\x01', "|");
                    //
                    // Check Msg type
                    //
                    let msg_type: String = get_attr(&response, "35");
                    match msg_type.as_str() {
                        "A" => {
                            info!("Received Logon response: {response}");
                            println!("Received Logon response: {response}");
                            islogin_complete = true;
                        },
                        "D" => {
                            info!("Received New Order(Single) response: {response}");
                            println!("Received New Order(Single) response: {response}");
                        },
                        "3" => {
                            info!("Received Reject response: {response}");
                            println!("Received Reject response: {response}");
                        },
                        "8" => {
                            info!("Received Exec Report response: {response}");
                            println!("Received Exec Report response: {response}");
                        },
                        _ => {
                            info!("Received different response type: {response}");
                            println!("Received different response type: {response}");
                        }
                    };
                    //
                    // Keep reading messages if login not completed
                    //
                    if !islogin_complete {
                        let _ = tls_stream.read(&mut buffer);
                    } 
                }
                if islogin_complete { // update status if login was completed Ok
                     status = true 
                };
            } else {
                error!("No response received from server for Logon request");
                println!("No response received from server for Logon request");
            }
        },
        Err(e) => {
            error!("Failed to read from stream: {e}");
            panic!("Failed to read from stream: {e}"); // panic since nothing else can be done here
        }
    }

    let seqnum: Arc<Mutex<u32>> = Arc::new(Mutex::new(1)); // Sequence is LOGON = 1 means running only a single sceniario here
    let seqnum_copy: Arc<Mutex<u32>> = Arc::clone(&seqnum);
    increment_seqnum(seqnum_copy.clone());
    Ok((status, seqnum))
}