use std::error::Error;
use std::env::var;
use quickfix_msg44::field_types::{OrdType, Side};

// 
// Placeholder for common trading settings and parameters, taken from ENV or file
//
#[allow(dead_code)]
pub(crate) fn exec() ->  Result<bool , Box<dyn Error>> {
    //
    // Default values for new order, new rfq quotes below. To be added to trading_config struct for use in testing
    // TODO - assign values from .env file
    // 
    let symbol = var("PT_SYMBOL").unwrap_or("SOL-USD".to_string());
    let price: f64 = var("PT_PRICE").unwrap_or("388.00".to_string()).parse().unwrap_or(388.00);
    let quantity: f64 = var("PT_QUANTITY").unwrap_or("2.00".to_string()).parse().unwrap_or(2.00);
    let side_str = var("PT_SIDE").unwrap_or("Sell".to_string());
    let side: Side = if side_str == "Buy" { Side::Buy } else { Side::Sell };
    let order_type_str = var("PT_ORDER_TYPE").unwrap_or("Limit".to_string());
    let order_type: OrdType = if order_type_str == "Market" { OrdType::Market } else { OrdType::Limit };

    assert!(price > 0.0);
    assert!(quantity > 0.0);
    assert!(side == Side::Buy || side == Side::Sell, "SIDE should be either Buy or Sell");
    assert!(symbol == "SOL-USD", "Symbol should be equal to 'SOL-USD'");
    assert!(order_type == OrdType::Limit || order_type == OrdType::Market, "ORDERTYPE should be either Limit or Market");
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
