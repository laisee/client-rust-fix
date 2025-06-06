use client_rust_fix::messages::factory::FixMessageFactory;
use client_rust_fix::messages::utils::{generate_order_id, side_as_int};
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
        api_key, price, quantity, symbol, side, order_type, seqnum
    );
    
    assert!(order_result.is_ok());
    
    if let Ok(order) = order_result {
        let order_str = order.to_fix_string().unwrap();
        
        // Verify essential fields
        assert!(order_str.contains("35=D"));  // NewOrderSingle
        assert!(order_str.contains("54=1"));  // Side=Buy
        assert!(order_str.contains("40=2"));  // OrdType=Limit
        assert!(order_str.contains("55=BTC-USD"));  // Symbol
    }
}

// More integration tests would be added here