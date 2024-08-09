use std::error::Error;

pub(crate) fn exec() ->  Result<bool, Box<dyn Error>> {
    // Spawn a new thread to modify the sequence number
    //let handle = thread::spawn(move || {
    //    let mut count: u32 = 0;
    //    while count < 5 {
    //        let mut num = seqnum_copy.lock().unwrap();
    //        println!("In heartbeat thread: count = {} of 5", count);
    //        count += 1;
    //        *num += 1;
    //        println!("Sequence number in thread is now: {}", *num);
    //        drop(num); // Explicitly drop the lock before sleeping
    //        thread::sleep(Duration::from_secs(60));
    //    }
    //});
    // close down heartbeat thread iif exists
    //if !handle.is_finished() {
    //    println!("Closing down heartbeat thread");
    //    handle.join().unwrap();
    // }
    Ok(true)
}