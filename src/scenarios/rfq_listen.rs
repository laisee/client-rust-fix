use log::{error, info};
use native_tls::TlsStream;
use quickfix::Message;
use std::{env::var, io::{ErrorKind, Read, Write}, net::TcpStream, thread::sleep, time::Duration};

pub fn rfq_listen_fix(mut tls_stream: TlsStream<TcpStream>, rfq: Message) {

    info!("Executing RFQ listen scenario");
    println!("Executing RFQ listen scenario");

    match tls_stream.write(rfq.to_fix_string() .expect("Error while sending RFQ listen message").as_bytes()) { 
        Ok(byte_count) => println!("Sent {rfq:?} with {byte_count:?} bytes ... "),
        Err(error) => println!("Error while sending order msg {error:?} ")
    };

    let mut count: u32 = 0;
    let limit_str = var("PT_LISTEN_EPOCH").expect("Error - PT_LISTEN_EPOCH must be set in .env file");
    let limit: u32 = limit_str.parse::<u32>().unwrap();

    loop {
        println!("RFQ:Listen - checking for new response");
        count += 1;
        if count > limit {
            break;
        }
        println!("RFQ:Listen - listen epoch {count} of {limit}");
        let mut buffer2 = [0; 1024];
        //let byte_count: usize = tls_stream.read(&mut buffer2).expect("Error reading bytes from RFQ responses"); 
        match tls_stream.read(&mut buffer2) {
            Ok(byte_count) => {
                if byte_count > 0 {
                    // Process the read bytes
                    let response = String::from_utf8_lossy(&buffer2[..byte_count]);
                    println!("RFQ:Listen  {byte_count} bytes: {response:?}");
                }
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                // WouldBlock indicates that the read operation would block (non-blocking mode)
                error!("RFQ:Listen ... would block, continuing...");
                println!("RFQ:Listen ... would block, continuing...");
                continue;
            }
            Err(ref error) if error.kind() == ErrorKind::TimedOut => {
                // TimedOut indicates that the read operation timed out
                error!("RFQ:Listen - Read timed out, continuing...");
                println!("RFQ:Listen - Read timed out, continuing...");
                continue;
            }
            Err(error) => {
                // Handle other errors
                error!("RFQ:Listen - error reading from stream: {error:?}");
                eprintln!("RFQ:Listen - error reading from stream: {error:?}");
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }
}