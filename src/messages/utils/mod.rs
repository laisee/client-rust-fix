#![warn(unused)]

use chrono::DateTime;

use chrono::Utc;
use log::{error,info};
use native_tls::{Certificate, TlsConnector, TlsStream};
use tungstenite::{client::IntoClientRequest, connect, http::HeaderValue, Message};
use std::{env::var, fs::File, io::Read, net::TcpStream, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};
use jwtk::{ecdsa::{EcdsaPrivateKey, EcdsaPublicKey}, sign, HeaderAndClaims};
use serde_json::{Value, Map};
use url::Url;

/// Execute WS request 
///
/// # Panics
///
/// Panics if 
/// - api key not found in config file
/// - api sceret not found in config file
/// - server not found in config file
#[allow(dead_code)]
pub fn execute_ws_request(msg: String) {

    assert!(!msg.is_empty(), "Error - message string is empty");

    // load target server based on env setting
    let ws_server = var("PT_WS_SERVER")
        .expect("PT_WS_SERVER must be set in the environment or .env file");
    info!("connecting to {:?}", ws_server);

    // load Power.Trade API key && private key from env
    let api_key= var("PT_WS_API_KEY")
        .expect("PT_WS_API_KEY must be set in the environment or .env file");
    info!("PT_API_KEY: {:?}", api_key);
    
    let api_secret= var("PT_WS_API_SECRET")
        .expect("PT_WS_API_SECRET must be set in the environment or .env file");

    let url = match Url::parse(&ws_server) {
        Ok(url) => url,
        Err(error) => {
            panic!("Error parsing server address {:?} -> {:?}", &ws_server, error);
        }
    };

    // generate JWT token for authenticating at server side
    let token: String = generate_access_token(&api_key, api_secret);

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
    let message: Message = Message::text(&msg);

    match socket.write(message.clone()) {
        Ok(result) => {
            socket.flush().expect("Error while flushing ws stream");
            println!("Success writng message {} to server with result {:?} ", message.clone(), result);
        },
        Err(error) => {
            println!("Error {:?} while writng message {} to server ", error,  message.clone());
        }

    };

    // setup loop for checking received messages 
    // n.b. to be replaced by event-driven code
    let mut count: i32 = 0;
    const MAX_EPOCH: i32 = 5;
    
    //
    // loop on message receive -> TODO replace by event-driven style
    //
    loop {
        sleep( Duration::from_secs(10));
        let msg: Message = socket.read().expect("Error while reading WS channel");
        info!("Received msg: {}", msg);
        info!("Power.Trade websocket client sleeping [{} of {} epochs]", count, MAX_EPOCH);
        count += 1;
        if count >= MAX_EPOCH {
            println!("Power.Trade websocket client closing after count of {count} epochs exceeded");
            info!("\nPower.Trade websocket client closing after count of {count} epochs exceeded\n");
            break;
        } else {
            println!("Power.Trade websocket client waiting after count of {count} epochs vs max {MAX_EPOCH} ");   
        }
        print!("Sleeping 10 secs before checking for messages again");
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
pub fn setup_connection() -> TlsStream<TcpStream> {

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
   println!("PUBKEY Path: {pubkey_path}");
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
   println!("TLS Connection -> {connector:?}");

   //
   // Connect to power.trade server over TCP
   //
   let stream = TcpStream::connect(&server).expect("Failed to connect to server");
   println!("TLS Stream connecting to -> {server}");

   //
   // Setup TLS channel on top of TCP connection
   //
   let tls_stream = connector
       .connect(&server, stream)
       .expect("Failed to establish TLS session");
   println!("TLS Stream -> {tls_stream:?}");

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
        .header_mut()
        .typ = Some("JWT".to_string());

    let token: String = match sign(&mut claims, &my_key) {
        Ok(token) => token,
        Err(error) => {
            println!("Error converting to private key: {error}");
            String::new()
        }
    };
    token
}

#[allow(dead_code)]
pub fn generate_access_token(api_key: &str, pkey: String) -> String {

    info!("Loading private key for account {}", api_key);
    let key: EcdsaPrivateKey = match EcdsaPrivateKey::from_pem(pkey.as_bytes()) {
        Ok(my_key) => {
            my_key
        }
        Err(e) => {
            // replace with error handling for invalid/missing private key
            error!("Error while loading private key -> {e}");
            panic!("Error while loading private key for account {api_key}");
        }
    };
    let binding: HeaderAndClaims<Map<String, Value>> = HeaderAndClaims::new_dynamic();
    let mut claims: HeaderAndClaims<Map<String, Value>> = binding;

    claims
        .set_iat_now()
        .set_exp_from_now(Duration::from_secs(18000))
        .insert("client", "api".to_owned())
        .insert("sub", api_key.to_owned())
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
            println!("Error converting to private key: {e}");
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
pub fn generate_transact_time() -> String {
    // Get the current date and time in UTC
    let utc: DateTime<Utc> = Utc::now();
    // Format the date and time as a string
    utc.to_rfc3339()
}
