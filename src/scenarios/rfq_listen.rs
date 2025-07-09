use std::env::var;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, ErrorKind};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use log::{info, error, debug};
use native_tls::TlsStream;
use quickfix::Message;
use crate::config::Settings;

#[allow(dead_code)]
pub fn rfq_listen_fix(
    tls_stream: Arc<Mutex<TlsStream<TcpStream>>>, 
    rfq: Message,
    settings: Option<&Settings>
) {
    info!("Executing RFQ listen scenario");

    match tls_stream.lock().unwrap().write(
        rfq.to_fix_string()
            .expect("Error while sending RFQ listen message")
            .as_bytes(),
    ) {
        Ok(byte_count) => info!("Sent RFQ with {} bytes", byte_count),
        Err(error) => error!("Error while sending order msg: {:?}", error),
    };

    let mut count: u32 = 0;
    let limit: u32 = if let Some(settings) = settings {
        settings.listen_epoch as u32
    } else {
        let limit_str = var("PT_LISTEN_EPOCH")
            .expect("Error - PT_LISTEN_EPOCH must be set in .env file");
        limit_str.parse::<u32>().unwrap()
    };

    loop {
        debug!("RFQ:Listen - checking for new response");
        count += 1;
        if count > limit {
            break;
        }
        debug!("RFQ:Listen - listen epoch {count} of {limit}");
        let mut buffer2 = [0; 1024];
        match tls_stream.lock().unwrap().read(&mut buffer2) {
            Ok(byte_count) => {
                if byte_count > 0 {
                    // Process the read bytes
                    let response = String::from_utf8_lossy(&buffer2[..byte_count]);
                    info!("RFQ:Listen received {} bytes: {}", byte_count, response);
                }
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                // WouldBlock indicates that the read operation would block (non-blocking mode)
                debug!("RFQ:Listen ... would block, continuing...");
                continue;
            }
            Err(ref error) if error.kind() == ErrorKind::TimedOut => {
                // TimedOut indicates that the read operation timed out
                debug!("RFQ:Listen - Read timed out, continuing...");
                continue;
            }
            Err(error) => {
                // Handle other errors
                error!("RFQ:Listen - error reading from stream: {:?}", error);
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }
}
