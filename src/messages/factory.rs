#![warn(unused)]
#![allow(clippy::needless_return)]

use chrono::Utc;
use quickfix_msg44::{NewOrderSingle, OrderCancelRequest, NewOrderMultileg, RFQRequest };
use quickfix_msg44::field_types::{ClOrdID, OrdType, OrderQty, Price, Side, SubscriptionRequestType, Symbol, TimeInForce, TransactTime};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use quickfix::{Message, QuickFixError};
use jwtk::ecdsa::EcdsaPrivateKey;
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{generate_jwt, generate_order_id, generate_transact_time};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRFQ {
    market_id: String,
    side: String,
    order_type: String,
    time_in_force: String,
    quantity: String,
    price: String,
    client_order_id: String,
    timestamp: String,
    symbol: String,
    user_tag: String,
}

impl NewRFQ {
    //
    // Method to initialize RFQ structure
    //
    fn _new(market_id: String, side: String, quantity: f64, price: f64, symbol: String, text: String) -> Self {
        let client_order_id: String = format!("{}", Self::_current_timestamp());
        let timestamp: String = format!("{}", Self::_current_timestamp());
        
        NewRFQ {
            market_id,
            side,
            order_type: "LIMIT".to_string(),
            time_in_force: "GTC".to_string(),
            quantity: format!("{}", quantity),
            price: format!("{}", price),
            client_order_id,
            timestamp,
            symbol,
            user_tag: text,
        }
    }
    // Helper method to get current timestamp
    fn _current_timestamp() -> u64 {
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_epoch.as_micros() as u64
    }
    // Method to export structure as JSON string
    pub fn _to_json_string(&self) -> Result<String, Error> {
        match serde_json::to_string(self) {
            Ok(json_str) => return Ok(json_str),
            Err(error) => return Err(error)
        };
    }
    pub fn _from_json_str(json_str: &str) -> Result<NewRFQ, Error> {
        info!("Received JSON: {}", json_str);
        match serde_json::from_str(json_str) {
            Ok(new_rfq) => return Ok(new_rfq),
            Err(error) => return Err(error)
        };
    }
}

pub struct WSMessageFactory;

impl WSMessageFactory {
    pub fn new_rfq_request(cl_ord_id: ClOrdID, symbol: Symbol, side: Side , order_qty: OrderQty, price: Price, order_type: OrdType ,text: String ) -> String {
        info!("Cient Order Id -> {}", cl_ord_id);
        info!("Side -> {:?}", side);
        info!("Symbol -> {:?}", symbol);
        info!("Cient Order Type -> {:?}", order_type);
        let json_str: String = format!(
            r#"{{
                "new_order": {{
                    "market_id": "none",
                    "side": "{:?}",
                    "order_type": "{:?}",
                    "time_in_force": "GTC",
                    "quantity": "{}",
                    "price": "{}",
                    "recv_window": "2", 
                    "client_order_id": "{}",
                    "timestamp": "{}",
                    "symbol": "{}",
                    "user_tag": "{}"
                }}
            }}"#,
            side,
            order_type,
            order_qty,
            price,
            cl_ord_id,
            generate_transact_time(),
            symbol,
            text
        );
        json_str // NewRFQ::from_json_str(&json_str).expect(format!("Error loading New RFQ from string: {:?}", json_str).as_str());
    }
}

pub struct _FixMessageFactory;

impl _FixMessageFactory {
    pub fn _new_order_single(cl_ord_id: ClOrdID, symbol: Symbol, side: Side , order_qty: OrderQty, price: Price, order_type: OrdType ,text: String ) -> Result<NewOrderSingle, QuickFixError> {

        let mut order: Option<NewOrderSingle> = None;
        let result: Result<NewOrderSingle, QuickFixError> = NewOrderSingle::try_new(
            cl_ord_id.to_string(),
            side,
            generate_transact_time(),
            order_type
        );
        match result {
            Ok(mut ord) => {
                let _ = ord.set_order_qty(order_qty);
                let _ = ord.set_price(price);
                let _ = ord.set_symbol(symbol.to_string());
                let _ = ord.set_text(text);
                let _ = ord.set_time_in_force(TimeInForce::GoodTillCancel);
                order = Some(ord);
                println!("Created new order {:?}", order );
            }
            Err(err) => {
                println!("Failed on creating new order {:?}", err);
            }
        }
        Ok(order.expect("Expected a valid New Single Order but received 'None' value"))
    }
    pub fn _new_cancel_order_single(orig_cl_ord_id: ClOrdID, cl_ord_id: ClOrdID, side: Side, text: String ) -> Result<OrderCancelRequest, QuickFixError> {
        let result: Result<OrderCancelRequest, QuickFixError> = OrderCancelRequest::try_new(
            orig_cl_ord_id.to_string(),
            cl_ord_id.to_string(),
            side,
            generate_transact_time()
        ); 
        match result {
            Ok(mut order) => {
                let _ = order.set_text(text);
                println!("Created cancel order {:?}", order );
                Ok(order)
            }
            Err(err) => {
                println!("Failed on creating cancel order {:?}", err);
                Err(err)
            }
        }       
    }
    pub fn _new_order_multi(cl_ord_id: ClOrdID, side: Side, transact_time: TransactTime,  order_type: OrdType) -> Result<NewOrderMultileg, QuickFixError> {
        NewOrderMultileg::try_new(cl_ord_id, side, transact_time, order_type)
    }
    pub fn _new_rfq_msg(now: String) -> Result<Message, QuickFixError> {

        // Define the RFQ fields according to FIX 4.4 specification
        let begin_string: String = "FIX.4.4".to_string();          // BeginString       [8]
        let message_type: String = String::from("R");              // MsgType 'R'      [35]
        let seqnum: u32 = 2;                                       // Seq Num 2        [34]
        let order_quantity: OrderQty = 1000.00;                    // Order Qty        [38]       
        let order_type: i32 = 1;                                   // OrdType          [40]       
        let sender_comp_id = "83ff7bddbcf42437250ca268bf0d644f";
        let sending_time: String = TransactTime::from(Utc::now().to_string()); // ts   [52]
        let side: i32 = 1;                                         // Side             [54]
        let symbol: String = String::from("ETH-USD");              // Symbol           [55]
        let target_comp_id = "PT-OE";                        // target comp Id   [56]
        //let time_in_force: u32 = 1;                              // TimeInForce      [59]
        let transact_time: String = TransactTime::from(Utc::now().to_string()); // ts  [60]
        let quote_reqd_id: String = now;                           // QuoteReqId      [131]
        let no_related_symbol: u32 = 1;                            // NoRelatedSymbol [146]

        let template: String = format!("8={}\x0135={}\x0134={}\x0138={}\x0140={}\x0149={}\x0152={}\x0154={:?}\x0155={}\x0156={}\x0160={}\x01131={}\x01146={}\x01", begin_string, message_type, seqnum, order_quantity, order_type, sender_comp_id, sending_time, side, symbol, target_comp_id, transact_time, quote_reqd_id, no_related_symbol);
        info!("RFQ Msg as string: {}", template.to_string());

        // 
        // two fields are generated when Message is created
        // * body length -> a.k.a. Fix BodyLength with number code [9]
        // * check sum   -> a.k.a. Fix CheckSum with number code [10]
        //
        let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
        match msg {
            Ok(ref msg) => {
                info!("Created message OK -> {:?}", msg.to_fix_string());
            }
            Err(ref error) => {
                error!("Create Msg error {:?}", error);
            }
        }
        msg
    }
    pub fn _get_rfq_request() -> Result<RFQRequest, QuickFixError> {
        let rfq_req_id = generate_order_id().to_string();
        let result = RFQRequest::try_new(
            rfq_req_id.clone()
        );
        match result {
            Ok(mut request) => {
                let _ = request.set_subscription_request_type(SubscriptionRequestType::Snapshot);
                println!("Created new request {:?}", request);
            }
            Err(err) => {
                println!("Failed on creating new order {:?}", err);
            }
        }
        RFQRequest::try_new(rfq_req_id)
    }
}

//
// Messages generated manually, outside Fix MessageFactory
// TODO - move the code inside MessageFactory or replace & remove
//
///
/// Retrieve Fix LOGON message
/// 
/// Parameters
/// 
///  - apikey: API key from customers PT account (separate keys for staging/test/prod)  
/// 
///  - now: current UTC timestamp when logon message was generated
/// 
///  - mykey: private key issued for an API Key
///  
pub fn get_logon_msg(
    apikey: String,
    now: u64,
    my_key: EcdsaPrivateKey,
) -> Result<Message, QuickFixError> {
    let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
    let message_type: char = 'A';                      // MsgType       [35]
    let seqnum: u32 = 1;                               // MessageSeqNum [34]
    let ts: String = generate_transact_time();         // SendingTime   [52]
    let target_comp_id = "PT-OE";                      // TargetCompID  [56] - use config value TODO
    let encrypt_method: u32 = 0;                       // EncryptMethod [98] - use config value TODO
    let heartbeat: i64 = 30;                           // HeartBtInt   [108] - use config value TODO
    let reset_seqnum: char = 'Y';                      // ResetSeqNum  [141] - use config value TODO
    let uri: String = var("API_URI").unwrap_or("api.wss.test.power.trade/v1/positions".to_string()); // refactor this

    let jwt: String = generate_jwt(apikey.clone(), now, uri, my_key);
    
    let template: String = format!("8={}\x0135={}\x0134={}\x0149={}\x0152={}\x0156={}\x0198={}\x01108={}\x01141={}\x01554={}\x01", begin_string, message_type, seqnum, apikey, ts, target_comp_id,encrypt_method, heartbeat, reset_seqnum, jwt);
    info!("LOGON Msg as string: {:?}", template.to_string());

    // two fields are generated when Message is created
    // body length -> BodyLength  [9]
    // check sum   -> CheckSum   [10]
    let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
    match msg {
        Ok(ref msg) => {
            info!("Created message OK -> {:?}", msg.to_fix_string());
        }
        Err(ref error) => {
            error!("Create Msg error {:?}", error);
        }
    }
    msg
}
pub fn _get_rfq_quote_msg( apikey: String, now: u64, _price: f64, _quantity: f64, symbol: String, side: u32, _order_type: u32, seqnum: u32) -> Result<Message, QuickFixError> {

    let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
    let message_type: char = 'D';                      // MsgType       [35]
    let client_order_id = now;                         // ClOrdId       [11]
    //let order_type = 2;                              // OrdType       [40]
    let source_comp_id = apikey.clone();               // SourceCompID  [49]
    let ts: String = generate_transact_time();         // SendingTime   [52]
    let target_comp_id = "PT-OE";                      // TargetCompID  [56] - use config value TODO
    let _time_in_force: u32 = 1;                       // TimeInForce   [59]
    let _quote_request_id = now;                       // QuoteReqId   [131]

    //
    // this field is how RFQ quotes differ from normal orders on power.trade
    //
    // market_id is 'none' or '' => RFQ / indicative order
    // market_id is  0 => firm order
    //
    let market_id: String = String::from("none");                    // MarketID    [1301]

    let template: String = format!("8={}\x0135={}\x0134={}\x0111={}\x0140={}\x0149={}\x0156={}\x0152={}\x0154={}\x0155={}\x0160={}\x011301={:?}\x01", begin_string, message_type, seqnum, client_order_id, side, source_comp_id, target_comp_id, ts, side, symbol, ts, market_id);
    info!("RFQ QUOTE Msg as string: {}", template.to_string());

    // 
    // two fields are generated when Message is created
    // * body length -> a.k.a. Fix BodyLength with number code [9]
    // * check sum   -> a.k.a. Fix CheckSum with number code [10]
    //
    let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
    match msg {
        Ok(msg) => {
            info!("Created RFQ message OK -> {:?}", msg.to_fix_string());
            Ok(msg)
        }
        Err(error) => {
            error!("Create RFQ Msg error {:?}", error);
            Err(error)
        }
    }
}

///
/// `get_single_order_msg`
///
/// # Errors
///
/// function will return error if 
/// - price is zero or less on Limit order
/// - price is not aligned to power.trade price levels 
/// - quantity is zero or less
/// - side is not valie (1 - buy or 2 - sell)
/// - symbol is not a traded coin or instrument @ power.trade
/// - order type is not valid (Limit or Market)
/// - seqnum is less than 2 (1 is seqnum for login message)
pub fn get_single_order_msg( apikey: String, now: u64, price: f64, quantity: f64, symbol: String,side: u32, order_type: u32, seqnum: u32) -> Result<Message, QuickFixError> {
  

    let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
    let message_type: char = 'D';                      // MsgType       [35]
    let client_order_id = now;                         // ClOrdId       [11]
    let ts: String = generate_transact_time();         // SendingTime   [52]
    let target_comp_id = "PT-OE";                      // TargetCompID  [56] - use config value TODO
    let time_in_force: u32 = 1;                        // TimeInForce   [59]

    // Body
    // msg.append_pair(11,   now)         -> ClOrdID
    // msg.append_pair(38,   quantity)
    // msg.append_pair(40,   2)           -> order type ["market" = 1, "limit" = 2]
    // msg.append_pair(44,   price)
    // msg.append_pair(54,   1)           -> Side ["1" = "Buy", "2" = "Sell"]
    // msg.append_pair(55,   symbol)
    // msg.append_pair(59,   TimeInForce) -> TimeInforce ["1" = "GTC"]
    // msg.append_pair(60,   format_epoch_time(now))

    let template: String = format!("8={}\x0135={}\x0134={}\x0111={}\x0138={}\x0140={}\x0144={}\x0149={}\x0152={}\x0154={}\x0155={}\x0156={}\x0159={}\x0160={}\x01", begin_string, message_type, seqnum, client_order_id, quantity, order_type, price, apikey, ts, side, symbol, target_comp_id, time_in_force, ts);
    info!("Single Order Msg as string: {}", template.to_string());

    // 
    // two fields are generated when Message is created
    // * body length -> a.k.a. Fix BodyLength with number code [9]
    // * check sum   -> a.k.a. Fix CheckSum with number code [10]
    //
    let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
    match msg {
        Ok(ref msg) => {
            info!("Created Single Leg Order message OK -> {:?}", msg.to_fix_string());
        }
        Err(ref error) => {
            error!("Create Single Leg Order Msg error {:?}", error);
        }
    }
    msg
}
