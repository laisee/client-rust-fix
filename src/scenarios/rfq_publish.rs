use log::info;
use native_tls::TlsStream;
use quickfix::Message;
use std::{io::{ErrorKind, Read, Write}, net::TcpStream, thread::sleep, time::Duration};
use crate::utils::execute_ws_request;

pub fn rfq_publish_fix(mut tls_stream: TlsStream<TcpStream>, rfq: Message) {

    info!("Executing RFQ publish scenario");
    println!("Executing RFQ publish scenario");

    // set 10 second timerout on reads
    tls_stream.get_ref().set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout");

    match tls_stream.write(rfq.to_fix_string() .expect("Error while sending RFQ listen message") .as_bytes()) { 
        Ok(byte_count) => println!("Sent {:?} with {:?} bytes ... ", rfq.to_fix_string(), byte_count),
        Err(error) => println!("Error while sending order msg {:?} ", error)
    };

    let mut count: u32 = 0;
    let limit: u32 = 10; // TODO - take from .env config
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
                    let response = String::from_utf8_lossy(&buffer2[..byte_count]);
                    println!("RFQ-Publish  {} bytes: {:?}", byte_count, response);
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // WouldBlock indicates that the read operation would block (non-blocking mode)
                println!("RFQ-Publish ... would block, continuing...");
                continue;
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                // TimedOut indicates that the read operation timed out
                println!("RFQ-Publish - Read timed out, continuing...");
                continue;
            }
            Err(e) => {
                // Handle other errors
                eprintln!("RFQ-Publish - error reading from stream: {:?}", e);
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }
}

pub fn _rfq_publish_ws(rfq: String) {
    info!("Sending RFQ -> {rfq}");
    execute_ws_request(rfq);
}
