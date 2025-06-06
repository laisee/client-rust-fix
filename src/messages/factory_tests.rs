#[cfg(test)]
mod factory_tests {
    use super::*;
    use quickfix_msg44::field_types::{OrdType, Side};
    
    #[test]
    fn test_new_single_leg_order_valid_params() {
        let api_key = "test_api_key".to_string();
        let price = 100.0;
        let quantity = 1.0;
        let symbol = "BTC-USD".to_string();
        let side = Side::Buy;
        let order_type = OrdType::Limit;
        let seqnum = 2;
        
        let result = FixMessageFactory::new_single_leg_order(
            api_key, price, quantity, symbol, side, order_type, seqnum
        );
        
        assert!(result.is_ok());
        
        if let Ok(msg) = result {
            // Verify message fields
            let msg_str = msg.to_fix_string().unwrap();
            assert!(msg_str.contains("35=D"));  // Verify message type is NewOrderSingle
            assert!(msg_str.contains("54=1"));  // Verify side is Buy
            assert!(msg_str.contains("40=2"));  // Verify order type is Limit
        }
    }
    
    #[test]
    fn test_new_single_leg_order_invalid_price() {
        let api_key = "test_api_key".to_string();
        let price = -100.0;  // Invalid price
        let quantity = 1.0;
        let symbol = "BTC-USD".to_string();
        let side = Side::Buy;
        let order_type = OrdType::Limit;
        let seqnum = 2;
        
        let result = FixMessageFactory::new_single_leg_order(
            api_key, price, quantity, symbol, side, order_type, seqnum
        );
        
        // Should fail with negative price
        assert!(result.is_err());
    }
    
    #[test]
    fn test_cancel_order() {
        let api_key = "test_api_key".to_string();
        let orig_cl_order_id = "order123".to_string();
        let order_id = "exchange456".to_string();
        let side = Side::Buy;
        let symbol = "BTC-USD".to_string();
        let seqnum = 3;
        let text = "Cancel test".to_string();
        
        let result = FixMessageFactory::cancel_order(
            &api_key, &orig_cl_order_id, &order_id, side, &symbol, seqnum, text
        );
        
        assert!(result.is_ok());
        
        if let Ok(msg) = result {
            // Verify message fields
            let msg_str = msg.to_fix_string().unwrap();
            assert!(msg_str.contains("35=F"));  // Verify message type is OrderCancelRequest
            assert!(msg_str.contains("41=order123"));  // Verify original client order ID
        }
    }
}