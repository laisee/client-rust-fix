#[cfg(test)]
mod utils_tests {
    use super::*;
    use jwtk::ecdsa::EcdsaPrivateKey;
    
    #[test]
    fn test_get_now_returns_valid_timestamp() {
        let now = get_now();
        assert!(now > 0, "Timestamp should be greater than 0");
    }
    
    #[test]
    fn test_generate_order_id_is_unique() {
        let id1 = generate_order_id();
        let id2 = generate_order_id();
        assert_ne!(id1, id2, "Generated order IDs should be unique");
    }
    
    #[test]
    fn test_side_as_int_conversions() {
        use quickfix_msg44::field_types::Side;
        assert_eq!(side_as_int(Side::Buy), 1);
        assert_eq!(side_as_int(Side::Sell), 2);
    }
    
    #[test]
    fn test_order_type_to_char_conversions() {
        use quickfix_msg44::field_types::OrdType;
        assert_eq!(order_type_to_char(OrdType::Market), '1');
        assert_eq!(order_type_to_char(OrdType::Limit), '2');
    }
    
    #[test]
    fn test_process_key_with_valid_pem() {
        let valid_pem = "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEILULtd0+Rx4/vMF/cVZ/SYU6Fo/HlgQF==\n-----END EC PRIVATE KEY-----";
        // Mock the key processing - this would need proper test keys
        // let result = process_key(valid_pem);
        // assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_key_with_invalid_pem() {
        let invalid_pem = "NOT A VALID PEM";
        let result = process_key(invalid_pem);
        assert!(result.is_err());
    }
}