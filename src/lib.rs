// src/lib.rs 

pub mod common {

    use std::sync::{Arc, Mutex};
    use log::info;

    /// increment_sequence
    ///
    /// # Panics
    ///
    /// Panics if None
    pub fn increment_seqnum(seqnum: Arc<Mutex<u32>> ) -> u32 {
        let mut num = seqnum.lock().unwrap();
        *num += 1;
        info!("Seqnum incremented to {}", *num);
        *num
    }
}