
use log::info;
use quickfix_msg44::field_types::{OrdType, Side};
use std::error::Error;
use quickfix::Message;

use crate::factory::FixMessageFactory;

#[allow(clippy::type_complexity)]
pub(crate) fn exec(apikey: &str, seqnum_latest: u32) -> Result<(bool,Message), Box<dyn Error>> {

    // Default values for new rfq quote below
    // TODO - assign values fron .env file
    // 
    static SYMBOL: &str = "BTC-USD";
    const QUANTITY: f64 = 2.00; 
    const SIDE: Side = Side::Sell;
    const ORDERTYPE: OrdType = OrdType::Limit;
    // create RFQ subscription msg for demonstrationg RFQ quote flow 
    //
    // array of topica(symbols) to subscribe for RFQ updates
    //
    let _topics: Vec<String> = ["ETH-USD", "SOL-USD", "DOGE-USD"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    // Create RFQ subscribe msg for listening to published RFQ quotes 
    //
    //let rfq_sub_msg: Message = FixMessageFactory::new_rfq_sub(topics).unwrap().into();
    //info!("Created new RFQ Subscribe msg using FixMsgFactory: {rfq_sub_msg:?}");

    // Create RFQ quote msg for creating new RFQ quote for a symbol
    //
    let rfq_quote_msg: Message = FixMessageFactory::new_rfq_quote( apikey, SYMBOL.to_string(), SIDE, QUANTITY, ORDERTYPE, seqnum_latest ).unwrap();
    info!("Created new RFQ Quote msg using FixMsgFactory: {rfq_quote_msg:?}");

    Ok((true, rfq_quote_msg))
}