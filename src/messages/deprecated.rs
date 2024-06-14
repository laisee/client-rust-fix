
#[derive(Serialize, Deserialize, Debug)]
pub struct _NewRFQ {
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
impl _NewRFQ {
    //
    // Method to initialize RFQ structure
    //
    fn _new(market_id: String, side: String, quantity: f64, price: f64, symbol: String, text: String) -> Self {
        let client_order_id: String = format!("{}", Self::_current_timestamp());
        let timestamp: String = format!("{}", Self::_current_timestamp());
        
        _NewRFQ {
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
    pub fn _from_json_str(json_str: &str) -> Result<_NewRFQ, Error> {
        info!("Received JSON: {}", json_str);
        match serde_json::from_str(json_str) {
            Ok(new_rfq) => return Ok(new_rfq),
            Err(error) => return Err(error)
        };
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
pub fn _get_logon_msg( apikey: String, now: u64, my_key: EcdsaPrivateKey,) -> Result<Message, QuickFixError> {
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
pub fn _get_single_order_msg( apikey: String, now: u64, price: f64, quantity: f64, symbol: Symbol,side: Side, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {
  

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

pub fn _get_rfq_quote_msg( apikey: String, now: u64, _price: f64, quantity: f64, symbol: Symbol, side: Side, order_type: OrdType, seqnum: u32) -> Result<Message, QuickFixError> {

    let begin_string: String = "FIX.4.4".to_string();  // BeginString    [8]
    let message_type: char = 'D';                      // MsgType       [35]
    let client_order_id = now;                    // ClOrdId       [11]
    let order_type: char = order_type_to_char(order_type);// OrdType    [40]
    let source_comp_id = apikey.clone();       // SourceCompID  [49]
    let ts: String = generate_transact_time();         // SendingTime   [52]
    let side_int:u32 = side_as_int(side);              // Side          [54]
    let target_comp_id = "PT-OE";                // TargetCompID  [56] - use config value TODO
    let time_in_force: u32 = 1;                        // TimeInForce   [59] - '1' = 'GTC'
    let symbols_sfx: String = "none".to_string();      // SymbolSfx     [65]
    let _quote_request_id: u64 = now;                  // QuoteReqId   [131]

    // this field is how RFQ quotes differ from normal orders on power.trade
    //
    // market_id is 'none' or '' => RFQ / indicative order
    // market_id is  0 => firm order
    //
    let market_id: String = "none".to_string();         // MarketID    [1301]

    let template: String = format!("8={}\x0135={}\x0134={}\x0111={}\x0138={}\x0140={}\x0149={}\x0156={}\x0152={}\x0154={:?}\x0155={}\x0159={}\x0160={}\x0165={}\x011301={:?}\x01", begin_string, message_type, seqnum, client_order_id, quantity, side_int, source_comp_id, target_comp_id, ts, symbol, time_in_force, ts, symbols_sfx, market_id);
    info!("RFQ Quote Msg as string: {}", template.to_string());

    // 
    // two fields are generated when Message is created
    // * body length -> a.k.a. Fix BodyLength with number code [9]
    // * check sum   -> a.k.a. Fix CheckSum with number code [10]
    //
    let msg: Result<Message, QuickFixError> = Message::try_from_text(&template);
    match msg {
        Ok(ref msg) => {
            info!("Created RFQ message OK -> {:?}", msg.to_fix_string());
        }
        Err(error) => {
            error!("Create RFQ Msg error {:?}", error);
            return Err(error);
        }
    }
    msg
}