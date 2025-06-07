#[cfg(test)]
mod jwt_tests {
    use super::*;
    use jwtk::ecdsa::EcdsaPrivateKey;

    fn setup_test_key() -> EcdsaPrivateKey {
        let test_pem = "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEIFxrWIO1BpNTG1oiLRKc9fPHUuF3PHtx9RgNJAdZWhDBoAoGCCqGSM49\nAwEHoUQDQgAEdqr9eg/9hxsGwHatmOMyAQX2PfW1uj8RA4xKy3e4mUTO0+nUPlwl\n1ZqiYMy8ciFFRu6Jj7WDU/lVrqB1HvnCgQ==\n-----END EC PRIVATE KEY-----";
        EcdsaPrivateKey::from_pem(test_pem.as_bytes()).expect("Test key creation failed")
    }

    #[test]
    fn test_generate_jwt_structure() {
        let key = setup_test_key();
        let api_key = "test_api_key".to_string();
        let now = 1234567890;
        let uri = "wss://test.example.com".to_string();
        let token = generate_jwt(api_key.clone(), now, uri.clone(), key);
        assert!(!token.is_empty());
        assert_eq!(token.split('.').count(), 3);
    }

    #[test]
    fn test_generate_access_token() {
        let key = setup_test_key();
        let token = generate_access_token("api", key, "wss://example.com");
        assert!(!token.is_empty());
        assert_eq!(token.split('.').count(), 3);
    }
}
