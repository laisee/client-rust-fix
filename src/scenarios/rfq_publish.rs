use crate::utils::execute_ws_request;
use log::info;
use native_tls::TlsStream;
use quickfix::Message;
use std::{env::var, io::{ErrorKind, Read, Write}, net::TcpStream, thread::sleep, time::Duration};

pub fn rfq_publish_fix(mut tls_stream: TlsStream<TcpStream>, rfq: Message) {

    info!("Executing RFQ publish scenario");
    println!("Executing RFQ publish scenario");

    match tls_stream.write(rfq.to_fix_string() .expect("Error while sending RFQ listen message").as_bytes()) { 
        Ok(byte_count) => println!("Sent {rfq:?} with {byte_count:?} bytes ... "),
        Err(error) => println!("Error while sending order msg {error:?} ")
    };

    let mut count: u32 = 0;
    let limit_str = var("PT_PUBLISH_EPOCH").expect("Error - PT_PUBLISH_EPOCH must be set in .env file");
    let limit: u32 = limit_str.parse::<u32>().unwrap();
    loop {
        println!("RFQ-Publish - checking for new response");
        count += 1;
        if count > limit {
            break;
        }
        println!("RFQ-Publish - listen epoch {count} of {limit}");
        let mut buffer2 = [0; 1024];
        //let byte_count: usize = tls_stream.read(&mut buffer2).expect("Error reading bytes from RFQ responses"); 
        match tls_stream.read(&mut buffer2) {
            Ok(byte_count) => {
                if byte_count > 0 {
                    // Process the read bytes
                    let response = String::from_utf8_lossy(&buffer2[..byte_count]).replace("\x01","|");
                    println!("RFQ-Publish  {byte_count} bytes: {response:?}");
                }
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                // WouldBlock indicates that the read operation would block (non-blocking mode)
                println!("RFQ-Publish ... would block, continuing...");
                continue;
            }
            Err(ref error) if error.kind() == ErrorKind::TimedOut => {
                // TimedOut indicates that the read operation timed out
                println!("RFQ-Publish - Read timed out, continuing...");
                continue;
            }
            Err(error) => {
                // Handle other errors
                eprintln!("RFQ-Publish - error reading from stream: {error:?}");
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }
}

#[allow(dead_code)]
pub fn _rfq_publish_ws(rfq: String) {
    info!("Sending RFQ -> {rfq}");
    execute_ws_request(&rfq);
}
