use std::error::Error;

use quickfix_msg44::field_types::{OrdType, Side};
//
// Default values for new order, new rfq quotes below. To be added to trading_config struct for use in testing
// TODO - assign values fron .env file
// 
const SYMBOL: &str = "SOL-USD";
const PRICE: f64 = 388.00; 
const QUANTITY: f64 = 2.00; 
const SIDE: Side = Side::Sell;
const ORDERTYPE: OrdType = OrdType::Limit;

// 
// Placeholder for common trading settings and parameters, taken from ENV or file
//
pub(crate) fn exec() ->  Result<bool , Box<dyn Error>> {
    assert!(PRICE > 0.0);
    assert!(QUANTITY > 0.0);
    assert!(SIDE == Side::Buy || SIDE == Side::Sell, "SIDE should be either Buy or Sell");
    assert!(ORDERTYPE == OrdType::Limit || ORDERTYPE == OrdType::Market, "ORDERTYPE should be either Limit or Market");
    assert_eq!(SYMBOL, "SOL-USD");
    Ok(true)
}
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//
// Create order which is a SELL at high price which will not be executed
// 
// - generate timestamp using epoch time (secs since 01-01-1970) for JWT Claims (iat, exp,...)
//
//increment_seqnum(seqnum_clone.clone());
//let num = seqnum.lock().unwrap();
//println!("Seqnum is now {}", *num);
//let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!!").as_secs();
//let order_msg: Message = FixMessageFactory::new_single_leg_order(apikey.clone(), now, PRICE, QUANTITY, SYMBOL.to_string(), SIDE, ORDERTYPE, *num).unwrap();
//drop(num);
//println!("Created new single leg order msg using FixMsgFactory  {order_msg:?}");