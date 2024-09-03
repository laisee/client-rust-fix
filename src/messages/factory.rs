#![allow(clippy::needless_return)]

use quickfix_msg44::{field_types::{ClOrdID, OrdType, OrderQty, Price, Side, SubscriptionRequestType, Symbol, TransactTime}, NewOrderMultileg, OrderCancelRequest, RFQRequest};
use log::{error, info};
use quickfix::{Message, QuickFixError};
use jwtk::ecdsa::EcdsaPrivateKey;
use std::{env::var, time::{SystemTime, UNIX_EPOCH}};
use crate::utils::{get_now, side_as_int, order_type_to_char, generate_access_token, generate_order_id, generate_ts};

#[allow(dead_code)]
#[allow(unused)]
pub struct WSMessageFactory;
impl WSMessageFactory {
    #[allow(dead_code)]
    pub fn new_rfq_request(cl_ord_id: ClOrdID, symbol: Symbol, side: Side , order_qty: OrderQty, price: Price, order_type: OrdType ,text: &str ) -> String {
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
            generate_ts(0),
            symbol,
            text
        );
        json_str // NewRFQ::from_json_str(&json_str).expect(format!("Error loading New RFQ from string: {:?}", json_str).as_str());
    }
}

///
/// `FixMsgFactory`
/// 
/// Panic if: 
/// apikey missing or empty
/// `now` is 0 or missing or empty
/// `cl_order_id` is missing or empty
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
    pub fn new_single_leg_order( apikey: String, price: f64, quantity: f64, symbol: Symbol,side: Side, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {

        let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
        let message_type: char = 'D';                      // MsgType       [35]
        let client_order_id = get_now();              // ClOrdId       [11]
        let order_type: char = order_type_to_char(order_type); // OrdType   [40]
        let side_int:u32 = side_as_int(side);              // Side          [54]
        let ts: String = generate_ts(0);        // SendingTime   [52]
        let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
        let time_in_force: u32 = 1;                        // TimeInForce   [59] - '1' = GTC

        // Body
        // msg.append_pair(11,   now)         -> ClOrdID
        // msg.append_pair(38,   quantity)
        // msg.append_pair(40,   2)           -> order type ["market" = 1, "limit" = 2]
        // msg.append_pair(44,   price)
        // msg.append_pair(54,   1)           -> Side ["1" = "Buy", "2" = "Sell"]
        // msg.append_pair(55,   symbol)
        // msg.append_pair(59,   TimeInForce) -> TimeInforce ["1" = "GTC"]
        // msg.append_pair(60,   format_epoch_time(now))

        let template: String = format!("8={begin_string}\x0135={message_type}\x0134={seqnum}\x0111={client_order_id}\x0138={quantity}\x0140={order_type}\x0144={price}\x0149={apikey}\x0152={ts}\x0154={side_int}\x0155={symbol}\x0156={target_comp_id}\x0159={time_in_force}\x0160={ts}\x01");
        println!("Single Order Msg as string: {template}");

        // 
        // two fields are generated when Message is created
        // * body length -> a.k.a. Fix BodyLength with number code [9]
        // * check sum   -> a.k.a. Fix CheckSum with number code [10]
        //
        let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
        match msg {
            Ok(ref msg) => {
                println!("Created Single Leg Order message OK -> {:?}", msg.to_fix_string());
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
        let ts: String = generate_ts(0);        // SendingTime   [52]
        let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
        let encrypt_method: u32 = 0;                       // EncryptMethod [98] - use config value TODO
        let heartbeat: i64 = 3600;                         // HeartBtInt   [108] - use config value TODO
        let reset_seqnum: char = 'Y';                      // ResetSeqNum  [141] - use config value TODO
        let uri: String = var("API_URI").unwrap_or("api.wss.test.power.trade/v1/feeds".to_string()); // refactor this
        
        let jwt: String = generate_access_token(&apikey.clone(), my_key, &uri);
            
        let template: String = format!("8={begin_string}\x0135={message_type}\x0134={seqnum}\x0149={apikey}\x0156={target_comp_id}\x0152={ts}\x0198={encrypt_method}\x01108={heartbeat}\x01141={reset_seqnum}\x01554={jwt}\x01");
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
    pub fn cancel_order(apikey: &str, orig_cl_order_id: &str, exch_order_id: &str, side: Side, symbol: &str, seqnum: u32, text: String) -> Result<Message, QuickFixError> {
       
        let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
        let message_type: char = 'F';                      // MsgType       [35]
        let ts: String = generate_ts(0);        // SendingTime   [52]
        let side_int:u32 = side_as_int(side);              // Side          [54]
        let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
        let cl_order_id: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!!").as_secs();
       
        let template: String = format!("8={begin_string}\x0135={message_type}\x0134={seqnum}\x0137={exch_order_id}\x0149={apikey}\x0156={target_comp_id}\x0111={cl_order_id}\x0141={orig_cl_order_id}\x0152={ts}\x0154={side_int}\x0155={symbol}\x0158={text}\x0160={ts}\x01");
        info!("Order Cancel Msg as string: {:?}", template.to_string());
        
        // two fields are generated when Message is created
        // body length -> BodyLength  [9]
        // check sum   -> CheckSum   [10]
        let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
        match msg {
            Ok(ref msg) => {
                info!("Created order Cancel message -> {:?}", msg.to_fix_string());
            }
            Err(ref error) => {
                error!("Error while creating Order Cancel error {:?}", error);
            }
        }
        msg
    }
    #[allow(dead_code)]
    pub fn new_cancel_order_single(orig_cl_ord_id: ClOrdID, cl_ord_id: &str, side: Side, symbol: String, text: String ) -> Result<OrderCancelRequest, QuickFixError> {
        let result: Result<OrderCancelRequest, QuickFixError> = OrderCancelRequest::try_new(
            orig_cl_ord_id.to_string(),
            cl_ord_id.to_string(),
            side,
            generate_ts(0)
        ); 
        match result {
            Ok(mut order) => {
                //let _ = order.set_sender_comp_id(apikey);
                let _ = order.set_symbol(symbol);
                //let _ = order.set_sending_time(generate_transact_time());
                //let _ = order.set_target_comp_id("PT-OE");
                //let _ = order.set_sequence_number(3);
                let _ = order.set_text(text);
                println!("Created Cancel nsg \n {order:?}");
                Ok(order)
            }
            Err(error) => {
                println!("Failed on creating cancel order {error:?}");
                Err(error)
            }
        }       
    }
    #[allow(dead_code)]
    pub fn new_order_multi(cl_ord_id: ClOrdID, side: Side, transact_time: TransactTime,  order_type: OrdType) -> Result<NewOrderMultileg, QuickFixError> {
        NewOrderMultileg::try_new(cl_ord_id, side, transact_time, order_type)
    }

    #[allow(dead_code)]
    pub fn new_rfq_quote(apikey: &str, symbol: Symbol, side: Side, order_qty: OrderQty, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {
        //
        // RFQ fields based on FIX 4.4 specification for new single order(MsgType='D') with 
        //
        let begin_string: String = "FIX.4.4".to_string();          // BeginString       [8] TODO - take this from .env file (or faster lookup TBD)
        let message_type: String = String::from("D");              // MsgType 'R'      [35]
        let seqnum: u32 = seqnum;                                  // SeqNum           [34]
        let client_order_id: String = generate_ts(0);             // ClOrdID          [11]       
        let order_quantity: OrderQty = order_qty;                  // Order Qty        [38] TODO - check alignment with qty values based on instrument rules
        let order_type: char = order_type_to_char(order_type);     // OrdType          [40]
        let sender_comp_id: String = apikey.to_string();           // SendCompId       [49]
        let sending_time: String = generate_ts(0);       // ts               [52]
        let side_int:u32 = side_as_int(side);                      // Side             [54]
        // accept symbol using incoming parameter `symbol`            Symbol           [55]
        let target_comp_id = "PT-OE";                        // target comp Id   [56] TODO - take from .env file
        let time_in_force: u32 = 1;                                // TimeInForce      [59] '1' = 'GTC'
        let transact_time: String = generate_ts(0);      // ts               [60]
        let symbols_sfx: String = "none".to_string();              // SymbolSfx        [65]
    
        let template: String = format!("8={begin_string}\x0135={message_type}\x0134={seqnum}\x0111={client_order_id}\x0138={order_quantity}\x0140={order_type}\x0149={sender_comp_id}\x0152={sending_time}\x0154={side_int}\x0155={symbol}\x0156={target_comp_id}\x0159={time_in_force}\x0160={transact_time}\x0165={symbols_sfx}\x01");
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
    #[allow(dead_code)]
    pub fn new_rfq_sub(topics: Vec<String>) -> Result<RFQRequest, QuickFixError> {
        assert!(!topics.is_empty());
        let rfq_req_id = generate_order_id().to_string();
        let mut msg: Result<RFQRequest, QuickFixError> = RFQRequest::try_new( rfq_req_id.clone());
        match msg {
            Ok(ref mut msg) => {
                let _ = msg.set_subscription_request_type(SubscriptionRequestType::Snapshot);
                info!("Created new request {msg:?}");
            }
            Err(error) => {
                error!("Failed on creating new order {error:?}");
                return Err(error);
            }
        }
        msg
    }
    #[allow(dead_code)]
    pub fn heartbeat(apikey: String, seqnum: u32, target_comp_id: &str) -> Result<Message, QuickFixError> {

        let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
        let message_type: char = '0';                      // MsgType       [35]
        let ts: String = generate_ts(0);         // ts            [52]

        // 8=FIX.4.2|9=49|35=0|34=4|49=SENDER_COMP_ID|56=TARGET_COMP_ID|52=20230624-14:30:00.000|10=128|
        let template: String = format!("8={begin_string}\x0135={message_type}\x0134={seqnum}\x0149={apikey}\x0152={ts}\x0156={target_comp_id}\x01");
        info!("Heartbeat Msg as string: {}", template.to_string());

        // 
        // two fields are generated when Message is created
        // * body length -> a.k.a. Fix BodyLength with number code [9]
        // * check sum   -> a.k.a. Fix CheckSum with number code [10]
        //
        let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
        match msg {
            Ok(ref msg) => {
                info!("Created Heartbeat message -> {:?}", msg.to_fix_string());
            }
            Err(error) => {
                error!("Error while creating Heartbeat Msg {:?}", error);
                return Err(error);
            }
        }
        msg
    }
}
