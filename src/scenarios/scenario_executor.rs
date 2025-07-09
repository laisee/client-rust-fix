// Update the RFQ functions to pass the settings

fn execute_rfq_quote_scenario(
    apikey: &str,
    tls_arc: Arc<Mutex<TlsStream<TcpStream>>>,
    seqnum: Arc<Mutex<u32>>,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use current seqnum for new quote
    let mut seqnum_latest = *seqnum.lock().unwrap();

    // Create RFQ quote message
    let (status, rfq_quote_msg) = setup_rfq::exec(apikey, seqnum_latest)?;
    if !status {
        error!("Error while setting up 'rfq'");
        return Err("Failed to set up RFQ".into());
    }
    
    // Increment sequence number
    seqnum_latest = increment_seqnum(seqnum.clone());
    info!(
        "Sequence number incremented to {:?} after sending RFQ message",
        seqnum_latest
    );
    
    // Send RFQ quote
    info!("Sending RFQ Quote {:?}", rfq_quote_msg);
    rfq_publish_fix(tls_arc.clone(), rfq_quote_msg, Some(settings));

    Ok(())
}

fn execute_rfq_listen_scenario(
    apikey: &str,
    tls_arc: Arc<Mutex<TlsStream<TcpStream>>>,
    seqnum: Arc<Mutex<u32>>,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use current seqnum for new quote
    let mut seqnum_latest = *seqnum.lock().unwrap();

    // Create RFQ subscribe message
    let (status, rfq_subscribe_msg) = setup_rfq::exec(apikey, seqnum_latest)?;
    if !status {
        error!("Error while setting up 'rfq'");
        return Err("Failed to set up RFQ".into());
    }
    
    // Increment sequence number
    seqnum_latest = increment_seqnum(seqnum.clone());
    info!(
        "Sequence number incremented to {:?} after sending RFQ message",
        seqnum_latest
    );
    
    // Send RFQ listen request
    info!("Sending RFQ Listen {:?}", rfq_subscribe_msg);
    rfq_listen_fix(tls_arc.clone(), rfq_subscribe_msg, Some(settings));

    Ok(())
}

// Update the execute_scenario function signature
pub fn execute_scenario(
    scenario: &str,
    apikey: &str,
    tls_arc: Arc<Mutex<TlsStream<TcpStream>>>,
    seqnum: Arc<Mutex<u32>>,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    match scenario {
        "ORDER" => execute_single_order_scenario(apikey, tls_arc, seqnum, settings),
        "ORDERS" => execute_multiple_orders_scenario(apikey, tls_arc, seqnum, settings),
        "RFQ_QUOTE" => execute_rfq_quote_scenario(apikey, tls_arc, seqnum, settings),
        "RFQ_LISTEN" => execute_rfq_listen_scenario(apikey, tls_arc, seqnum, settings),
        _ => {
            error!("Unknown scenario: {}", scenario);
            Err("Unknown scenario".into())
        }
    }
}