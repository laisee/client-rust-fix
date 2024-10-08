use chrono::Utc;
use jwtk::{ecdsa::{EcdsaPrivateKey, EcdsaPublicKey}, sign, HeaderAndClaims};
use log::{info, error};
use native_tls::{Certificate, TlsConnector, TlsStream};
use quickfix_msg44::field_types::{OrdType, Side};
use std::{env::var, fs::File, io::Read, net::TcpStream, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};
use serde_json::{Value, Map};
use tungstenite::{client::IntoClientRequest, connect, http::HeaderValue, Message};
use url::Url;

#[allow(dead_code)]
pub fn get_now() -> u64 {
     // current datetime as seconds since 1970, used in scenarios executed below
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!!").as_secs()
}
#[allow(dead_code)]
pub fn get_attr(msg: &str, mytag: &str) -> String {
    for field in msg.replace('\x01', "|").split('|') {
        let mut parts: std::str::SplitN<char> = field.splitn(2, '=');
        if let (Some(tag), Some(value)) = (parts.next(), parts.next()) {
            if tag == mytag {
                return value.to_string();
            }
        }
    }
    String::from("")
}

/// Execute WS request
///
/// # Panics
///
/// Panics if
/// - api key not found in config file
/// - api sceret not found in config file
/// - server not found in config file
#[allow(dead_code)]
pub fn execute_ws_request(msg: &str) {

    assert!(!msg.is_empty(), "Error - message string is empty");

    // load target server based on env setting
    let ws_server = var("PT_WS_SERVER")
        .expect("PT_WS_SERVER must be set in the environment or .env file");
    info!("connecting to {:?}", ws_server);

    // load Power.Trade API key && private key from env
    let api_key= var("PT_WS_API_KEY")
        .expect("PT_WS_API_KEY must be set in the environment or .env file");
    info!("PT_API_KEY: {:?}", api_key);


    let binding = var("PT_WS_API_SECRET")
        .expect("PT_WS_API_SECRET must be set in the environment or .env file");
    let api_secret= binding.as_bytes();

    let url = match Url::parse(&ws_server) {
        Ok(url) => url,
        Err(error) => {
            panic!("Error parsing server address {:?} -> {:?}", &ws_server, error);
        }
    };

    // generate JWT token for authenticating at server side
    let token: String = generate_access_token(&api_key, EcdsaPrivateKey::from_pem(api_secret).expect("error copnverting from PEM"), url.as_ref());

    // log first X chars to assist with debugging issues
    info!("Token generated for account {:?}\n{:?} ", api_key, token.clone().truncate(50));

    // setup WS request with required Power.Trade header
    let mut req = url.into_client_request().unwrap();
    req.headers_mut().append("X-Power-Trade", HeaderValue::from_str(&token).unwrap());

    info!("Request headers: {:?}", req.headers());
    info!("Request body: {:?}", req.body());

    info!("Connecting to Power.Trade server: {}", &ws_server);
    println!("Connecting to Power.Trade server: {}", &ws_server);

    let (mut socket, response) = connect(req).unwrap();
    info!("Response from server {:?} -> {:?}", ws_server, response.status());

    info!("Power.Trade websocket client now active for server {}", &ws_server);
    println!("Power.Trade websocket client now active for server {}", &ws_server);

    //let result = connect(req);
    //let (mut socket, response) = match result {
    //    Ok((socket, response)) => (socket, response),
    //    Err(error) => {
    //        println!("Error connecting to WS server: {:?}", error);
    //        return;
    //    }
    //};

    // send RFQ subscription message
    let message: Message = Message::text(msg.to_string());

    match socket.write(message.clone()) {
        Ok(result) => {
            socket.flush().expect("Error while flushing WS socket");
            info!("Success writng message {} to server with result {:?} ", message.clone(), result);
        },
        Err(error) => {
            error!("Error {:?} while writng message {} to server ", error,  message.clone());
        }

    };

    // setup loop for checking received messages
    // n.b. to be replaced by event-driven code
    let mut count: i32 = 0;
    const MAX_EPOCH: i32 = 10;

    //
    // loop on message receive -> TODO replace by event-driven style
    //
    loop {
        sleep( Duration::from_secs(2));
        let msg: Message = socket.read().expect("Error while reading WS channel");
        info!("Received msg: {}", msg);
        info!("Power.Trade websocket client sleeping [{} of {} epochs]", count, MAX_EPOCH);
        count += 1;
        if count >= MAX_EPOCH {
            println!("Power.Trade websocket client closing after count of {count} epochs exceeded");
            info!("\nPower.Trade websocket client closing after count of {count} epochs exceeded\n");
            break;
        } else {
            info!("Power.Trade websocket client waiting after count of {count} epochs vs max {MAX_EPOCH} ");
        }
        print!("Sleeping 2 secs before checking for messages again");
    }
}

/// connect
///
/// used to carry out low level setup for opening TLS connection to power.trade server
///
/// # Panics
///
/// Panics if certificates or settings missing
///
#[allow(dead_code)]
pub fn setup_tls_connection() -> TlsStream<TcpStream> {

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
   //
   // >> Assign/Load Settings
   //
   let host: String = var("PT_SERVER").expect("Error while retrieving PT_SERVER from .env file");
   let port: String = String::from("2021"); // TODO - move this to .env vonfig
   info!("Connecting to Host {:?}", host);
   let server: String = format!("{host}:{port}");
   info!("Connecting to Endpoint {server:?}");

   //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
   //
   // Load the public certificate from a file
   //
   let pubkey_path = var("PT_PUBKEY_FILE").expect("Error while retrieving PT_PUBKEY_FILE from .env file"); //"private_cert.pem"; // TODO - take from env
   info!("PUBKEY Path: {pubkey_path}");
   let mut cert_file: File = File::open(pubkey_path).expect("Unable to open certificate file"); // TODO - from .env
   let mut cert_data: Vec<u8> = Vec::new();
   cert_file
       .read_to_end(&mut cert_data)
       .expect("Unable to read certificate file");

   //
   // Create Certificate object from the certificate data loaded from file
   //
   let cert = match Certificate::from_pem(&cert_data) {
       Ok(cert) => {
           info!("Found valid cert in file {cert_file:?}");
           cert
       }
       Err(err) => {
           error!("Error loading cert from PEM -> {err:?}");
           panic!("Error loading cert from PEM -> {err:?}");
       }
   };

   //
   // Build instance of TLS connector
   //
   let connector: TlsConnector = TlsConnector::builder()
       .danger_accept_invalid_certs(true)
       .add_root_certificate(cert)
       .build()
       .expect("Failed to build TLS connector");
   info!("TLS Connection -> {connector:?}");

   //
   // Connect to power.trade server over TCP
   //
   let stream = TcpStream::connect(&server).expect("Failed to connect to server");
   info!("TLS Stream connecting to -> {server}");

   //
   // Setup TLS channel on top of TCP connection
   //
   let tls_stream = connector
       .connect(&server, stream)
       .expect("Failed to establish TLS session");

   // set 5 second timerout on reads
   tls_stream.get_ref().set_read_timeout(Some(Duration::new(5, 0))).expect("Failed to set read timeout");

   info!("TLS Stream -> {tls_stream:?}");

   tls_stream

}

/// `get_pkey`
///
/// # Panics
///
/// Panics if private key cannot be loaded as `EcdsaPrivateKey` object
///
#[allow(dead_code)]
pub fn get_pkey() -> EcdsaPrivateKey {

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //
    // >> Read private key from file and load into 'EcdsaPrivateKey' object
    //
    // Read the PEM file
    //
    let pem_path: String = var("PT_PEM_FILE").expect("Error while retrieving PT_PEM_FILE from .env file");
    let mut file = File::open(pem_path).expect("File name error");
    let mut pem_bytes: Vec<u8> = Vec::<u8>::new();
    let size = file.read_to_end(&mut pem_bytes).expect("Error reading from file containing PEM content");
    info!("Read {} bytes into string from PEM file '{:?}'", size, file);

    let pem = String::from_utf8(pem_bytes.clone()).expect("Error converting bytes to string");

    //
    // load string taken from PEM file into a private (ecdsa) key
    //
    let key: EcdsaPrivateKey = match process_key(&pem) {
        Ok(key) => {
            info!("Priv key: {:?}", key);
            key
        },
        Err(error) => {
            error!("Failed to process key: {error}");
            panic!("Error - failed to process PEM key: {error:?} "); // no way to continue, abort app
        }
    };
    key
}

///
///  `generate_jwt`
///
#[allow(dead_code)]
pub fn generate_jwt(apikey: String, now: u64, uri: String, my_key: EcdsaPrivateKey) -> String {
    let binding: HeaderAndClaims<Map<String, Value>> = HeaderAndClaims::new_dynamic();
    let mut claims: HeaderAndClaims<Map<String, Value>> = binding;

    claims
        .set_iat_now()
        .set_exp_from_now(Duration::from_secs(18000))
        .insert("client", "api")
        .insert("uri", uri)
        .insert("nonce", now)
        .insert("sub", apikey)
        .set_iss(String::from("app.power.trade"))
        .header_mut().alg ="ES256".to_string().into();

    let token: String = match sign(&mut claims, &my_key) {
        Ok(token) => {
            token
        },
        Err(error) => {
            error!("Error converting to private key: {error}");
            return String::new();
        }
    };
    token
}

#[allow(dead_code)]
pub fn generate_access_token(api_key: &str, key: EcdsaPrivateKey, uri: &str) -> String {

    info!("Loading private key for account {}", api_key);
    //let key: EcdsaPrivateKey = match EcdsaPrivateKey::from_pem(pkey.as_bytes()) {
    //    Ok(my_key) => {
    //        my_key
    //    }
    //    Err(e) => {
    //        // replace with error handling for invalid/missing private key
    //        error!("Error while loading private key -> {e}");
    //        panic!("Error while loading private key for account {api_key}");
    //    }
    //};
    let binding: HeaderAndClaims<Map<String, Value>> = HeaderAndClaims::new_dynamic();
    let mut claims: HeaderAndClaims<Map<String, Value>> = binding;

    claims
        .set_iat_now()
        .set_exp_from_now(Duration::from_secs(18000))
        .insert("client", "api".to_owned())
        .insert("sub", api_key.to_owned())
        .insert("uri", uri)
        .insert("nonce",  Utc::now().timestamp())
        .set_iss(String::from("app.power.trade"))
        .header_mut().alg ="ES256".to_string().into();

    info!("Adding claims {:?} to signed JWT for account {}", claims, api_key);

    let token: String = match sign( &mut claims, &key) {
        Ok(token) => {
            info!("JWT signed Ok with private key");
            token
        }
        Err(e) => {
            error!("Error signing JWT with private key: {}", e);
            "ERROR-Gen-JWT".to_string()
        }
    };
    token
}

#[allow(dead_code)]
pub fn process_key(pem: &str) -> Result<EcdsaPrivateKey, String> {
    match EcdsaPrivateKey::from_pem(pem.as_bytes()) {
        Ok(key) => {
            info!("Private Key:\n{:?}\n", key);
            Ok(key)
        }
        Err(e) => {
            error!("Error converting to private key: {e}");
            Err(format!("Error converting to private key: {e}"))
        }
    }
}

#[allow(dead_code)]
pub fn generate_order_id() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}

/// `generate_pubkey`
///  - given a private key (ES256) will generate the public key
///
/// # Panics
///
/// Panics if ...
///
#[allow(dead_code)]
fn _generate_pubkey(mykey: EcdsaPrivateKey) -> EcdsaPublicKey {
    let newpem: String = mykey.public_key_to_pem().expect("Error generating PEM file format from string");
    let pk = EcdsaPublicKey::from_pem(newpem.as_bytes()).expect("Error creating public key from PEM");
    pk
}

#[allow(dead_code)]
pub fn generate_ts(add_hours: i64) -> String {
    use chrono::prelude::*;
    use chrono::Duration;
    let now: DateTime<Utc> = Utc::now();
    let future_time = if add_hours != 0 {
        now + Duration::hours(add_hours)
    } else {
        now
    };
    future_time.format("%Y%m%d-%H:%M:%S%.9f").to_string()
}

#[allow(dead_code)]
pub(crate) fn order_type_to_char(order_type: OrdType) -> char {
    match order_type {
        OrdType::Market => '1',
        OrdType::Limit => '2',
        OrdType::Stop => '3',
        OrdType::StopLimit => '4',
        OrdType::WithOrWithout => '6',
        OrdType::LimitOrBetter => '7',
        OrdType::LimitWithOrWithout => '8',
        OrdType::OnBasis => '9',
        OrdType::PreviouslyQuoted => 'D',
        OrdType::PreviouslyIndicated => 'E',
        OrdType::ForexSwap => 'G',
        OrdType::Funari => 'I',
        OrdType::MarketIfTouched => 'J',
        OrdType::MarketWithLeftOverAsLimit => 'K',
        OrdType::PreviousFundValuationPoint => 'L',
        OrdType::NextFundValuationPoint => 'M',
        OrdType::Pegged => 'P',
    }
}

#[allow(dead_code)]
pub fn side_as_int(side: Side) -> u32 {
    match side {
        Side::Buy => 1,
        Side::Sell => 2,
        Side::BuyMinus => 3,
        Side::SellPlus => 4,
        Side::SellShort => 5,
        Side::SellShortExempt => 6,
        Side::Undisclosed => 7,
        Side::Cross => 8,
        Side::CrossShort => 9,
        Side::CrossShortExempt => 10, // 'A' in FIX is 10 in decimal
        Side::AsDefined => 11,        // 'B' in FIX is 11 in decimal
        Side::Opposite => 12,         // 'C' in FIX is 12 in decimal
        Side::Subscribe => 13,        // 'D' in FIX is 13 in decimal
        Side::Redeem => 14,           // 'E' in FIX is 14 in decimal
        Side::Lend => 15,             // 'F' in FIX is 15 in decimal
        Side::Borrow => 16,           // 'G' in FIX is 16 in decimal
    }
}

#[cfg(test)]
mod fix_msg_tests {

    use quickfix::Message;

    #[test]
    fn test_message_equality() {
        let fix_msg_txt = "8=FIX.4.2|9=12|35=D|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let msg1: Message = Message::try_from_text(&fix_msg_txt).unwrap();
        let msg2: Message = Message::try_from_text(&fix_msg_txt).unwrap();
        assert_eq!(msg1.to_fix_string(), msg2.to_fix_string());
    }
    #[test]
    fn test_message_inequality() {
        let fix_msg_txt1 = "8=FIX.4.2|9=12|35=D|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let fix_msg_txt2 = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let msg1: Message = Message::try_from_text(&fix_msg_txt1).unwrap();
        let msg2: Message = Message::try_from_text(&fix_msg_txt2).unwrap();
        assert_ne!(msg1.to_fix_string(), msg2.to_fix_string());
    }
}

#[cfg(test)]
mod fix_msg_enum_tests {

    use quickfix_msg44::field_types::{OrdType, Side, TimeInForce};
    use crate::utils::{order_type_to_char, side_as_int};

    #[test]
    fn test_time_in_force_day() {
        let tif = TimeInForce::Day;
        assert_eq!(tif, TimeInForce::Day);
    }

    #[test]
    fn test_time_in_force_good_till_cancel() {
        let tif = TimeInForce::GoodTillCancel;
        assert_eq!(tif, TimeInForce::GoodTillCancel);
    }

    #[test]
    fn test_time_in_force_at_the_opening() {
        let tif = TimeInForce::AtTheOpening;
        assert_eq!(tif, TimeInForce::AtTheOpening);
    }

    #[test]
    fn test_time_in_force_immediate_or_cancel() {
        let tif = TimeInForce::ImmediateOrCancel;
        assert_eq!(tif, TimeInForce::ImmediateOrCancel);
    }

    #[test]
    fn test_time_in_force_fill_or_kill() {
        let tif = TimeInForce::FillOrKill;
        assert_eq!(tif, TimeInForce::FillOrKill);
    }

    #[test]
    fn test_time_in_force_good_till_crossing() {
        let tif = TimeInForce::GoodTillCrossing;
        assert_eq!(tif, TimeInForce::GoodTillCrossing);
    }

    #[test]
    fn test_time_in_force_good_till_date() {
        let tif = TimeInForce::GoodTillDate;
        assert_eq!(tif, TimeInForce::GoodTillDate);
    }

    #[test]
    fn test_time_in_force_debug_format() {
        let tif = TimeInForce::Day;
        assert_eq!(format!("{:?}", tif), "Day");
    }

    #[test]
    fn test_time_in_force_equality() {
        assert_eq!(TimeInForce::Day, TimeInForce::Day);
        assert_ne!(TimeInForce::Day, TimeInForce::GoodTillCancel);
    }

    #[test]
    fn test_time_in_force_clone() {
        let tif = TimeInForce::FillOrKill;
        let tif_clone = tif.clone();
        assert_eq!(tif, tif_clone);
    }

    #[test]
    fn test_order_type_market() {
        let order_type: OrdType = OrdType::Market;
        let order_type_char = order_type_to_char(order_type);
        assert_eq!(order_type_char, '1');
    }

    #[test]
    fn test_order_type_limit() {
        let order_type: OrdType = OrdType::Limit;
        let order_type_char = order_type_to_char(order_type);
        assert_eq!(order_type_char, '2');
    }

    #[test]
    fn test_side_buy() {
        let side: Side = Side::Buy;
        let side_int = side_as_int(side);
        assert_eq!(side_int, 1);
    }

    #[test]
    fn test_side_sell() {
        let side: Side = Side::Sell;
        let side_int = side_as_int(side);
        assert_eq!(side_int, 2);
    }
}