use log::{error,info};
use native_tls::TlsStream;
use quickfix::{FieldMap, Message};
use quickfix_msg44::field_types::Side;
use std::{io::{ErrorKind, Read, Write}, net::TcpStream, option::Option::Some, thread::sleep, time::Duration};
use crate::factory::FixMessageFactory;
use crate::utils::get_attr;

pub fn send_single_order(apikey: &str,  tls_stream: &mut TlsStream<TcpStream>, order: Message, seqnum: u32, is_cancel_order: Option<bool> ) {

    // assign parameter for cancel orders as a bool with default == 'true'
    let is_cancel_order = is_cancel_order.unwrap_or(true); 

    println!("Executing add/cancel single order scenario");

    // send the new order
    match tls_stream.write(order.to_fix_string().unwrap().as_bytes()) { 
        Ok(byte_count) => {
            println!("Sent Single Order {order:?} with {byte_count:?} bytes ... ");
        },
        Err(error) => {
            println!("Error while sending Single Order {error:?} ");
            error!("Error while sending Single Order {error:?} ")
        }
    };

    // 
    // set count for iterating on new order messages
    // 
    let mut count: u32 = 1; 
    let mut is_order_confirmed_as_new = false;
    let limit: u32 = 10; // TODO - take this value from .env config file
    loop {
        println!("Count/Limit [{count}/{limit} Is Order confirmed as 'New' {is_order_confirmed_as_new}");
        if count >= limit || is_order_confirmed_as_new {
            break;
        }
        count += 1;

        let orig_cl_order_id: String = order.get_field(11).unwrap();
        let mut exch_order_id = String::new();
        println!("SingleOrder: waiting for response for Order {orig_cl_order_id:?}");
        let mut buffer = [0; 1024];
        match tls_stream.read(&mut buffer) {
            Ok(bytes) => {
                let resp: String = String::from_utf8(buffer[..bytes].to_vec()).expect("Error loading new Order responses message");

                // Split response string into individual FIX messages if received
                let messages = split_fix_messages(&resp);

                // Process 1 ... many received FIX messages
                for message in messages.iter() {
                    let client_order_id = get_attr(message, "11"); 
                    info!("New Order[Client Order Id: {:?}] status [{:?}] response {:?}", client_order_id, get_attr(message, "39"), message.replace("\x01","|"));
                    println!("New Order[Client Order Id: {:?}] status [{:?}] response {:?}", get_attr(message, "11"), get_attr(message, "39"), message.replace("\x01","|"));
                    //
                    // check if we received a 'New' status for order placed above
                    //
                    if orig_cl_order_id == client_order_id && get_attr(message, "39") == "0" {
                        exch_order_id = get_attr(message, "37"); 
                        info!("Order[Client order Id: {:?}] Exchange order Id [{:?}] confirmed with 'New' status", client_order_id, exch_order_id);
                        //
                        // Order was confirmed('New') => can now send cancel msg iif is_cancel_order == 'true'. 
                        // n.b. order will be cancelled if session API key has "cancel-on-session-close" flag selected/
                        // 
                        is_order_confirmed_as_new = true; 
                    }
                }
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                println!("WouldBlock error: Retrying in 5000 ms ...");
                sleep(Duration::from_millis(5000));
            },
            Err(e) => panic!("Error reading bytes from cancel Order responses: {:?}", e),
        };

        //
        // now cancel the new order using the client(our) generated order Id
        // - if flag was passed in with 'true' and order status is 'New'
        //
        println!("Cancel: {is_cancel_order:?} Client Order: {orig_cl_order_id} Exchange Order {exch_order_id}");
        if is_cancel_order &  is_order_confirmed_as_new {
            let symbol = order.get_field(55).unwrap();
            let side_char = Some(get_attr(order.to_fix_string().as_ref().expect("Extracting 'side' from Fix message failed"), "34"));
            let side: Side = match side_char.as_deref() {
                Some("1") => Side::Buy,
                Some("2") => Side::Sell,
                None =>  {
                    panic!("Error - Side value is equal to 'None'");
                },
                _ => {
                    panic!("Error - invalid Side value: {:?}", side_char);
                }
            };

            info!("using Seqnum {} for FixMsg::cancel_order", seqnum);
            let cancel_msg: Message = FixMessageFactory::cancel_order(apikey, &orig_cl_order_id,  &exch_order_id, side, &symbol, seqnum ,format!("Cancel order {orig_cl_order_id}")).unwrap();
            info!("Cancel Id: {:?}] \n{:?}", &orig_cl_order_id, cancel_msg.to_fix_string().expect("Error converting cancel msg").replace("\x01","|"));

            // senf the cancel order
            match tls_stream.write(cancel_msg.to_fix_string().unwrap().as_bytes()) { 
                Ok(byte_count) => println!("Sent Cancel msg with {byte_count:?} bytes ... "),
                Err(error) => println!("Error while sending order msg {error:?} ")
            };
        
            let mut count: u32 = 1;
            const LIMIT: u32 = 10;
            'main_loop: while count < LIMIT {
                let mut buffer = [0; 1024];
                match tls_stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let resp: String = String::from_utf8(buffer[..bytes_read].to_vec()).expect("Error loading new Order responses message");

                        // Split the string into individual FIX messages
                        let messages = split_fix_messages(&resp);

                        // Process each FIX message
                        for message in messages.iter() {
                            //
                            // TODO - replace if ... with match 
                            //
                            if get_attr(message, "35") == "F" {
                                info!("Received Cancel [{:?}] response {:?}", orig_cl_order_id, message.replace("\x01", "|"));
                                println!("\nReceived Cancel [{:?}] response {:?}", orig_cl_order_id, message.replace("\x01", "|"));
                                break;
                            } 
                            else if get_attr(message, "35") == "8" {

                                if get_attr(message, "41") == orig_cl_order_id  {
                                    info!("\nExecution Report with status '{:?}\n [{:?}] ", get_attr(message, "39"), message.replace("\x01", "|"));
                                    println!("\nExecution Report with status '{:?}\n [{:?}] ", get_attr(message, "39"), message.replace("\x01", "|"));

                                    // Cancelled status == "4"
                                    // attempt to cancel trade was a success
                                    if get_attr(message, "39") == "4" {
                                        println!("Order [{orig_cl_order_id:?}] cancelled - status == '4'");
                                        break 'main_loop;
                                    } 
                                } else {
                                    println!("Execution report for Client Order [{:?}] received \n {:?}", get_attr(message, "41"), message.replace("\x01", "|"));
                                }

                            }
                            else if get_attr(message, "35") == "9" {
                                info!("Cancel Reject Msg [{:?}] with Status {:?} ", message.replace("\x01", "|"), get_attr(message, "39"));
                                println!("\nCancel Reject Msg [{:?}] with Status {:?} ", message.replace("\x01", "|"), get_attr(message, "39"));
                            } else {
                                // Some other msg - continue the loop until cancel done or loop finished
                                println!("\nOther Msg [{:?}] with Status {:?} ", message.replace("\x01", "|"), get_attr(message, "39"));
                            }
                        }
                    },
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        println!("Netwprk WouldBlock error: retrying in 5000 ms ...");
                        sleep(Duration::from_millis(5000));
                    },
                    Err(e) => panic!("Error reading bytes from cancel Order responses: {:?}", e),
                };
                // sleep for 2000ms
                info!(" -> Sleeping for 5000 ms while awaiting Cancel Trade response [{count:?}/{LIMIT:?}]");
                println!(" -> Sleeping for 5000 ms while awaiting Cancel Trade response [{count:?}/{LIMIT:?}]");
                sleep(Duration::from_millis(5000));
                count+=1;
            }
            info!("Exiting from Send Single Leg order & Cancel loop [{count:?}/{LIMIT:?}]");
            println!("Exiting Single Leg Add/Cancel Order loop after {count:?} of {LIMIT:?} msg checks");
            break;
        }
    }
}

pub fn send_multiple_orders(apikey: &str, mut tls_stream: TlsStream<TcpStream>, orders: Vec<Message>, seqnum: u32 , cancel: bool) {
    info!("Add-multiple-orders -> TLS: {:?}", tls_stream);
    for order in orders {
        info!("Sending multi/set order to be executed: {:?}", order);
        send_single_order(apikey, &mut tls_stream, order, seqnum, Some(cancel));
    }
}

fn split_fix_messages(fix_messages: &str) -> Vec<String> {
    // Split the messages based on the "8=" tag which signifies the beginning of each message
    let mut messages: Vec<String> = fix_messages
        .split("8=FIX.4.4\x01")
        .filter(|s| !s.is_empty())
        .map(|s| format!("8=FIX.4.4\x01{}", s))
        .collect();

    // Removing any potential trailing separators
    for message in &mut messages {
        if message.ends_with('\x01') {
            message.pop();
        }
    }
    messages
}