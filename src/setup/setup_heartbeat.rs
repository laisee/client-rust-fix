use std::{error::Error, thread::{self, spawn}, time::Duration, sync::{Arc, Mutex}, env::var};
use log::info;

#[allow(dead_code)]
pub(crate) fn exec(seqnum: Arc<Mutex<u32>>) -> Result<bool, Box<dyn Error>> {
    // Get heartbeat interval from environment variable or use default
    let heartbeat_interval = match var("PT_HEARTBEAT_INTERVAL") {
        Ok(val) => val.parse::<u64>().unwrap_or(60),
        Err(_) => 60, // Default to 60 seconds if not specified
    };
    
    // Get heartbeat count from environment variable or use default
    let heartbeat_count = match var("PT_HEARTBEAT_COUNT") {
        Ok(val) => val.parse::<u32>().unwrap_or(5),
        Err(_) => 5, // Default to 5 if not specified
    };
    
    info!("Heartbeat configured with interval: {}s, count: {}", heartbeat_interval, heartbeat_count);
    
    // Create a copy of the sequence number reference for the thread
    let seqnum_copy = Arc::clone(&seqnum);
    
    // Spawn a thread that periodically sends heartbeat messages
    let _handle = spawn(move || {
        let mut count: u32 = 0;
        while count < heartbeat_count {
            let mut num = seqnum_copy.lock().unwrap();
            println!("In heartbeat thread: count = {} of {}", count, heartbeat_count);
            count += 1;
            *num += 1;
            println!("Sequence number in thread is now: {}", *num);
            drop(num); // Explicitly drop the lock before sleeping
            thread::sleep(Duration::from_secs(heartbeat_interval));
        }
    });
    Ok(true)
}
