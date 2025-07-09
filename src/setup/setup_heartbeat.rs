use log::info;
use native_tls::TlsStream;
use std::io::Write;
use std::net::TcpStream;
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread::{self, spawn},
    time::Duration,
};

use crate::messages::factory::FixMessageFactory;
use crate::config::Settings;

#[allow(dead_code)]
pub(crate) fn exec(
    apikey: String,
    tls_stream: Arc<Mutex<TlsStream<TcpStream>>>,
    seqnum: Arc<Mutex<u32>>,
    settings: &Settings,
) -> Result<bool, Box<dyn Error>> {
    let heartbeat_interval = settings.heartbeat_interval;
    let heartbeat_count = settings.heartbeat_count;

    info!(
        "Heartbeat configured with interval: {}s, count: {}",
        heartbeat_interval, heartbeat_count
    );

    // Create a copy of the sequence number reference for the thread
    let seqnum_copy = Arc::clone(&seqnum);
    let tls_copy = Arc::clone(&tls_stream);
    let api_clone = apikey.clone();
    let target = settings.target_comp_id.clone();

    // Spawn a thread that periodically sends heartbeat messages
    let _handle = spawn(move || {
        let mut count: u64 = 0;
        while count < heartbeat_count {
            let mut num = seqnum_copy.lock().unwrap();
            let hb_msg = FixMessageFactory::heartbeat(api_clone.clone(), *num, &target);
            if let Ok(msg) = hb_msg {
                if let Ok(mut stream) = tls_copy.lock() {
                    let _ = stream.write(msg.to_fix_string().unwrap().as_bytes());
                }
            }
            *num += 1;
            count += 1;
            drop(num);
            thread::sleep(Duration::from_secs(heartbeat_interval));
        }
    });
    Ok(true)
}
