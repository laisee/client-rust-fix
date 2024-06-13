use native_tls::TlsStream;
use quickfix::Message;
use std::{io::{Read, Write}, net::TcpStream, thread::sleep, time::Duration};

pub fn add_cancel_single_order(mut tls_stream: TlsStream<TcpStream>, order: Message ) {
    println!("Executing Add/Cancel Single Order scenario");

    // senf the new order
    match tls_stream.write(order.to_fix_string()
        .unwrap()
        .as_bytes()) { 
        Ok(byte_count) => println!("Sent {order:?} with {byte_count:?} bytes ... "),
        Err(error) => println!("Error while sending order msg {error:?} ")
    };
    // 
    // set count for iterating on new order messages
    // 
    let mut count: u32 = 1; 
    let limit: u32 = 3; // TODO - take this value from .env config file
    loop {
        if count > limit {
            break;
        }
        count += 1;

        println!("SingleOrder: waiting for response");
        // define receive buffer and wait for new order response(s)
        let mut buffer2 = [0; 1024];
        let byte_count = tls_stream.read(&mut buffer2).expect("Error reading bytes from new Order responses"); 
        let resp: String = String::from_utf8(buffer2[..byte_count].to_vec()).expect("Error loading new Order responses message");
        println!("SingleOrder::received response: {resp:?}\n");

        // sleep for Y seconds between order sending
        sleep(Duration::from_secs(1));
    }
}
