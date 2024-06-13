use log::info;
use native_tls::TlsStream;
use quickfix::Message;
use std::{io::{Read, Write}, net::TcpStream, thread::sleep, time::Duration};
use crate::utils::execute_ws_request;

pub fn _rfq_publish_fix(mut tls_stream: TlsStream<TcpStream>, rfq: Message) {

    println!("Executing RFQ Listen scenario");

    match tls_stream.write(rfq.to_fix_string()
        .expect("Error while sending RFQ listen message")
        .as_bytes()) { 
        Ok(byte_count) => println!("Sent {:?} with {:?} bytes ... ", rfq.to_fix_string(), byte_count),
        Err(error) => println!("Error while sending order msg {:?} ", error)
    };

    let mut count: u32 = 1;
    let limit: u32 = 1000; // TODO - take from .env config
    loop {
        let mut buffer2 = [0; 1024];
        let byte_count: usize = tls_stream.read(&mut buffer2).expect("Error reading bytes from RFQ responses"); 
        let resp: String = String::from_utf8(buffer2[..byte_count].to_vec()).expect("Error loading RFQ responses message");
        println!("RFQ-Listener - received response: {resp}\n");

        if count > limit {
            break;
        }
        count += 1;
        sleep(Duration::from_secs(1));
    }
}

pub fn rfq_publish_ws(rfq: String) {
    info!("RFQ -> {rfq}");
    execute_ws_request(rfq);
}
