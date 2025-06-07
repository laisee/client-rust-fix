use crate::config::Settings;
use quickfix_msg44::field_types::{OrdType, Side};
use std::error::Error;

//
// Placeholder for common trading settings and parameters, taken from ENV or file
//
#[allow(dead_code)]
pub(crate) fn exec(settings: &Settings) -> Result<bool, Box<dyn Error>> {
    //
    // Default values for new order, new rfq quotes below. To be added to trading_config struct for use in testing
    // TODO - assign values from .env file
    //
    let symbol = &settings.symbol;
    let price: f64 = settings.price;
    let quantity: f64 = settings.quantity;
    let side: Side = if settings.side.to_lowercase() == "buy" {
        Side::Buy
    } else {
        Side::Sell
    };
    let order_type: OrdType = if settings.order_type.to_lowercase() == "market" {
        OrdType::Market
    } else {
        OrdType::Limit
    };

    assert!(price > 0.0);
    assert!(quantity > 0.0);
    assert!(
        side == Side::Buy || side == Side::Sell,
        "SIDE should be either Buy or Sell"
    );
    assert!(symbol == "SOL-USD", "Symbol should be equal to 'SOL-USD'");
    assert!(
        order_type == OrdType::Limit || order_type == OrdType::Market,
        "ORDERTYPE should be either Limit or Market"
    );
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
