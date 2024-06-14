#![allow(clippy::needless_return)]

use quickfix_msg44::{field_types::{ClOrdID, OrdType, OrderQty, Price, Side, SubscriptionRequestType, Symbol, TimeInForce, TransactTime}, NewOrderMultileg, NewOrderSingle, OrderCancelRequest, RFQRequest};
use log::{error, info};
use quickfix::{Message, QuickFixError};
use jwtk::ecdsa::EcdsaPrivateKey;
use std::env::var;
use crate::utils::{side_as_int, order_type_to_char, generate_access_token, generate_order_id, generate_transact_time};

#[allow(dead_code)]
#[allow(unused)]
pub struct WSMessageFactory;
impl WSMessageFactory {
    pub fn _new_rfq_request(cl_ord_id: ClOrdID, symbol: Symbol, side: Side , order_qty: OrderQty, price: Price, order_type: OrdType ,text: String ) -> String {
        info!("Cient Order Id -> {}", cl_ord_id);
        info!("Side -> {:?}", side);
        info!("Symbol -> {:?}", symbol);
        info!("Cient Order Type -> {:?}", order_type);
        let json_str: String = format!(
            r#"{{
                "new_order": {{
                    "market_id": "0",
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

pub struct FixMessageFactory;
impl FixMessageFactory {

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
    pub fn new_single_leg_order( apikey: String, now: u64, price: f64, quantity: f64, symbol: Symbol,side: Side, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {

        let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
        let message_type: char = 'D';                      // MsgType       [35]
        let client_order_id = now;                    // ClOrdId       [11]
        let ts: String = generate_transact_time();         // SendingTime   [52]
        let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
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

        let template: String = format!("8={}\x0135={}\x0134={}\x0111={}\x0138={}\x0140={:?}\x0144={}\x0149={}\x0152={}\x0154={:?}\x0155={}\x0156={}\x0159={}\x0160={}\x01", begin_string, message_type, seqnum, client_order_id, quantity, order_type, price, apikey, ts, side, symbol, target_comp_id, time_in_force, ts);
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
    pub fn new_logon( apikey: String, my_key: EcdsaPrivateKey,) -> Result<Message, QuickFixError> {

        let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
        let message_type: char = 'A';                      // MsgType       [35]
        let seqnum: u32 = 1;                               // MessageSeqNum [34]
        let ts: String = generate_transact_time();         // SendingTime   [52]
        let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
        let encrypt_method: u32 = 0;                       // EncryptMethod [98] - use config value TODO
        let heartbeat: i64 = 30;                           // HeartBtInt   [108] - use config value TODO
        let reset_seqnum: char = 'Y';                      // ResetSeqNum  [141] - use config value TODO
        let uri: String = var("API_URI").unwrap_or("api.wss.test.power.trade/v1/feeds".to_string()); // refactor this
        
        let jwt: String = generate_access_token(&apikey.clone(), my_key, &uri);
            
        let template: String = format!("8={}\x0135={}\x0134={}\x0149={}\x0156={}\x0152={}\x0198={}\x01108={}\x01141={}\x01554={}\x01", begin_string, message_type, seqnum, apikey, target_comp_id, ts, encrypt_method, heartbeat, reset_seqnum, jwt);
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
    pub fn _new_single_leg_order(cl_ord_id: ClOrdID, symbol: Symbol, side: Side , order_qty: OrderQty, price: Price, order_type: OrdType ,text: String ) -> Result<NewOrderSingle, QuickFixError> {

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
    pub fn new_rfq_quote(now: &str, apikey: &str, symbol: Symbol, side: Side, order_qty: OrderQty, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {

        //
        // RFQ fields based on FIX 4.4 specification for new single order(MsgType='D') with 
        //
        let begin_string: String = "FIX.4.4".to_string();          // BeginString       [8] TODO - take this from .env file (or faster lookup TBD)
        let message_type: String = String::from("D");              // MsgType 'R'      [35]
        let seqnum: u32 = seqnum;                                  // SeqNum           [34]
        let client_order_id: String = now.to_string();             // ClOrdID          [11]       
        let order_quantity: OrderQty = order_qty;                  // Order Qty        [38] TODO - check alignment with qty values based on instrument rules
        let order_type: char = order_type_to_char(order_type);     // OrdType          [40]
        let sender_comp_id: String = apikey.to_string();           // SendCompId       [49]
        let sending_time: String = generate_transact_time();       // ts               [52]
        let side_int:u32 = side_as_int(side);                      // Side             [54]
        // accept symbol from parameter `symbol`                      Symbol           [55]
        let target_comp_id = "PT-OE";                        // target comp Id   [56] TODO - take from .env file
        let time_in_force: u32 = 1;                                // TimeInForce      [59] '1' = 'GTC'
        let transact_time: String = generate_transact_time();      // ts               [60]
        let symbols_sfx: String = "none".to_string();              // SymbolSfx        [65]

        let template: String = format!("8={}\x0135={}\x0134={}\x0111={}\x0138={}\x0140={}\x0149={}\x0152={}\x0154={:?}\x0155={}\x0156={}\x0159={}\x0160={}\x0165={}\x01", begin_string, message_type, seqnum, client_order_id, order_quantity, order_type, sender_comp_id, sending_time, side_int, symbol, target_comp_id, time_in_force, transact_time, symbols_sfx);
        info!("RFQ Msg as string: {}", template.to_string());

        // 
        // two fields are generated when QuickFix Message is generated from string
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
    ///
    /// `new_rfq_sub`
    /// TODO - add subscribe topic list to parameters
    /// 
    pub fn new_rfq_sub(topics: Vec<String>) -> Result<RFQRequest, QuickFixError> {

        assert!(!topics.is_empty());
        let rfq_req_id = generate_order_id().to_string();
        let mut msg: Result<RFQRequest, QuickFixError> = RFQRequest::try_new( rfq_req_id.clone());
        match msg {
            Ok(ref mut msg) => {
                let _ = msg.set_subscription_request_type(SubscriptionRequestType::Snapshot);
                println!("Created new request {:?}", msg);
            }
            Err(error) => {
                println!("Failed on creating new order {:?}", error);
                return Err(error);
            }
        }
        msg
    }
}
