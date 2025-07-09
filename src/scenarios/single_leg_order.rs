use crate::config::Settings;
use log::{info, error, debug};
use quickfix::Message;
use std::{
    fmt::Debug,
    io::{Read, Write},
    sync::{Arc, Mutex},
};

#[allow(dead_code)]
pub fn send_single_order<S>(
    tls_stream: Arc<Mutex<S>>,
    order_msg: Message,
    settings: Option<&Settings>,
) where
    S: Read + Write + Send + Debug,
{
    // Send the order
    let mut stream_guard = tls_stream.lock().unwrap();
    match stream_guard.write(order_msg.to_fix_string().unwrap().as_bytes()) {
        Ok(byte_count) => info!("Sent Order msg with {} bytes", byte_count),
        Err(error) => error!("Error while sending order msg: {:?}", error),
    };
    
    // Flush the stream to ensure all data is sent
    if let Err(e) = stream_guard.flush() {
        error!("Error flushing stream: {:?}", e);
    }
    
    // Read the response
    let mut buffer = [0; 1024];
    match stream_guard.read(&mut buffer) {
        Ok(byte_count) => {
            if byte_count > 0 {
                let response = String::from_utf8_lossy(&buffer[0..byte_count]);
                info!("Received response: {}", response);
            }
        },
        Err(error) => error!("Error while reading response: {:?}", error),
    }
    
    // Release the lock
    drop(stream_guard);

    // Determine whether to cancel the order based on settings
    let should_cancel = if let Some(settings) = settings {
        settings.cancel_order
    } else {
        // Default to true if no settings provided
        true
    };

    // Rest of the function remains the same, but use should_cancel to determine whether to cancel
    if should_cancel {
        debug!("Order cancellation is enabled, but not implemented yet");
        // Cancel order logic would go here
    }
}

pub fn send_multiple_orders<S>(
    tls_stream: Arc<Mutex<S>>,
    orders: Vec<Message>,
    settings: Option<&Settings>,
) where
    S: Read + Write + Send + Debug,
{
    info!("Sending multiple orders");
    for order in orders {
        debug!("Sending order: {:?}", order);
        // Note: We're not passing apikey and seqnum here as they're not used in the current implementation
        send_single_order(tls_stream.clone(), order, settings);
    }
}

#[allow(dead_code)]
pub(crate) fn split_fix_messages(fix_messages: &str) -> Vec<String> {
    // Split the messages based on the "8=" tag which signifies the beginning of each message
    let mut messages: Vec<String> = fix_messages
        .split("8=FIX.4.4\x01")
        .filter(|s| !s.is_empty())
        .map(|s| format!("8=FIX.4.4\x01{}", s))
        .collect();

    // Removing any potential trailing separators
    for message in &mut messages {
        if message.ends_with('\x01') {
            message.pop();
        }
    }
    messages
}
