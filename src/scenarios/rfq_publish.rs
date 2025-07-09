use crate::messages::utils::execute_ws_request;
use log::{info, error, debug};
use native_tls::TlsStream;
use quickfix::Message;
use std::{
    env::var,
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use crate::config::Settings;

#[allow(dead_code)]
pub fn rfq_publish_fix(
    tls_stream: Arc<Mutex<TlsStream<TcpStream>>>, 
    rfq: Message,
    settings: Option<&Settings>
) {
    info!("Executing RFQ publish scenario");

    match tls_stream.lock().unwrap().write(
        rfq.to_fix_string()
            .expect("Error while sending RFQ publish message")
            .as_bytes(),
    ) {
        Ok(byte_count) => info!("Sent RFQ with {} bytes", byte_count),
        Err(error) => error!("Error while sending order msg: {:?}", error),
    };

    let mut count: u32 = 0;
    let limit: u32 = if let Some(settings) = settings {
        settings.publish_epoch as u32
    } else {
        let limit_str = var("PT_PUBLISH_EPOCH")
            .expect("Error - PT_PUBLISH_EPOCH must be set in .env file");
        limit_str.parse::<u32>().unwrap()
    };

    loop {
        debug!("RFQ-Publish - checking for new response");
        count += 1;
        if count > limit {
            break;
        }
        debug!("RFQ-Publish - listen epoch {count} of {limit}");
        let mut buffer2 = [0; 1024];
        match tls_stream.lock().unwrap().read(&mut buffer2) {
            Ok(byte_count) => {
                if byte_count > 0 {
                    // Process the read bytes
                    let response =
                        String::from_utf8_lossy(&buffer2[..byte_count]).replace("\x01", "|");
                    info!("RFQ-Publish received {} bytes: {}", byte_count, response);
                }
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                // WouldBlock indicates that the read operation would block (non-blocking mode)
                debug!("RFQ-Publish ... would block, continuing...");
                continue;
            }
            Err(ref error) if error.kind() == ErrorKind::TimedOut => {
                // TimedOut indicates that the read operation timed out
                debug!("RFQ-Publish - Read timed out, continuing...");
                continue;
            }
            Err(error) => {
                // Handle other errors
                error!("RFQ-Publish - error reading from stream: {:?}", error);
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }
}

#[allow(dead_code)]
pub fn _rfq_publish_ws(rfq: String) {
    info!("Sending RFQ -> {}", rfq);
    execute_ws_request(&rfq);
}
