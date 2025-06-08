use std::time as duration;
use std::thread::sleep;

use client_rust_fix::messages::{factory::FixMessageFactory, utils::{generate_order_id, side_as_int}};

use quickfix_msg44::field_types::{OrdType, Side};

#[test]
fn test_create_and_validate_order() {
    // This would be an integration test that creates an order message
    // and validates its structure without sending it

    let api_key = "test_api_key".to_string();
    let price = 100.0;
    let quantity = 1.0;
    let symbol = "BTC-USD".to_string();
    let side = Side::Buy;
    let order_type = OrdType::Limit;
    let seqnum = 2;

    let order_result = FixMessageFactory::new_single_leg_order(
        api_key, price, quantity, symbol, side, order_type, seqnum,
    );

    assert!(order_result.is_ok());

    if let Ok(order) = order_result {
        let order_str = order.to_fix_string().unwrap();

        // Verify essential fields
        assert!(order_str.contains("35=D")); // NewOrderSingle
        assert!(order_str.contains("54=1")); // Side=Buy
        assert!(order_str.contains("40=2")); // OrdType=Limit
        assert!(order_str.contains("55=BTC-USD")); // Symbol
    }
}

#[test]
fn test_generate_order_id() {
    // Test that generate_order_id returns a unique ID each time
    let id1 = generate_order_id();
    sleep(duration::Duration::from_millis(100));
    let id2 = generate_order_id();
    
    // IDs should be different
    assert_ne!(id1, id2, "Generated order IDs should be unique");
    
    // IDs should be non-zero
    assert!(id1 > 0, "Order ID should be greater than 0");
    assert!(id2 > 0, "Order ID should be greater than 0");
    
    // Print the IDs for debugging
    println!("ID1: {}, ID2: {}", id1, id2);
}

#[test]
fn test_side_as_int() {
    // Test conversion of Side enum to integer values
    assert_eq!(side_as_int(Side::Buy), 1);
    assert_eq!(side_as_int(Side::Sell), 2);
    assert_eq!(side_as_int(Side::BuyMinus), 3);
    assert_eq!(side_as_int(Side::SellPlus), 4);
    assert_eq!(side_as_int(Side::SellShort), 5);
    assert_eq!(side_as_int(Side::SellShortExempt), 6);
    assert_eq!(side_as_int(Side::Undisclosed), 7);
    assert_eq!(side_as_int(Side::Cross), 8);
    assert_eq!(side_as_int(Side::CrossShort), 9);
    assert_eq!(side_as_int(Side::CrossShortExempt), 10);
    assert_eq!(side_as_int(Side::AsDefined), 11);
    assert_eq!(side_as_int(Side::Opposite), 12);
    assert_eq!(side_as_int(Side::Subscribe), 13);
    assert_eq!(side_as_int(Side::Redeem), 14);
    assert_eq!(side_as_int(Side::Lend), 15);
    assert_eq!(side_as_int(Side::Borrow), 16);
}