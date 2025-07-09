use client_rust_fix::config::Settings;
use serial_test::serial;
use std::env;

fn setup() {
    // Clear all relevant environment variables before each test
    env::remove_var("PT_SYMBOL");
    env::remove_var("PT_PRICE");
    env::remove_var("PT_QUANTITY");
    env::remove_var("PT_SIDE");
    env::remove_var("PT_ORDER_TYPE");
    env::remove_var("PT_LOG_FILE");
    env::remove_var("PT_HEARTBEAT_INTERVAL");
    env::remove_var("PT_HEARTBEAT_COUNT");
    env::remove_var("PT_TARGET_COMP_ID");
    env::remove_var("PT_SERVER");
    env::remove_var("PT_LISTEN_EPOCH");
    env::remove_var("PT_PUBLISH_EPOCH");
    env::remove_var("PT_CANCEL_ORDER");
    env::remove_var("PT_SCENARIO");
}

#[test]
#[serial]
fn test_settings_from_env_defaults() {
    setup();
    let settings = Settings::from_env().expect("load settings");
    // Update the expected values to match the actual defaults in Settings::from_env()
    assert_eq!(settings.symbol, "SOL-USD");
    assert_eq!(settings.price, 388.0);
    assert_eq!(settings.quantity, 0.20);
    assert_eq!(settings.side.to_lowercase(), "sell");
    assert_eq!(settings.order_type.to_lowercase(), "limit");
    assert_eq!(settings.log_file, "app.log");
    assert_eq!(settings.heartbeat_interval, 60);
    assert_eq!(settings.heartbeat_count, 20);
    assert_eq!(settings.target_comp_id, "PT-OE");
}

#[test]
#[serial]
fn test_settings_custom_values() {
    setup();
    // Set environment variables for this test
    env::set_var("PT_SYMBOL", "BTC-USD");
    env::set_var("PT_PRICE", "42.0");
    env::set_var("PT_QUANTITY", "2.5");
    env::set_var("PT_SIDE", "Buy");
    env::set_var("PT_ORDER_TYPE", "Market");
    env::set_var("PT_LOG_FILE", "test.log");
    env::set_var("PT_HEARTBEAT_INTERVAL", "3");
    env::set_var("PT_HEARTBEAT_COUNT", "2");
    env::set_var("PT_TARGET_COMP_ID", "TEST");

    // Print the environment variables to verify they're set correctly
    println!("PT_SYMBOL: {:?}", env::var("PT_SYMBOL"));
    println!("PT_PRICE: {:?}", env::var("PT_PRICE"));

    let settings = Settings::from_env().unwrap();

    // Verify all settings match expected values
    assert_eq!(settings.symbol, "BTC-USD");
    assert_eq!(settings.price, 42.0);
    assert_eq!(settings.quantity, 2.5);
    assert_eq!(settings.side, "Buy");
    assert_eq!(settings.order_type, "Market");
    assert_eq!(settings.log_file, "test.log");
    assert_eq!(settings.heartbeat_interval, 3);
    assert_eq!(settings.heartbeat_count, 2);
    assert_eq!(settings.target_comp_id, "TEST");
}

#[test]
#[serial]
fn test_settings_invalid_price() {
    setup();
    env::set_var("PT_PRICE", "abc");
    assert!(Settings::from_env().is_err());
}

#[test]
#[serial]
fn test_settings_invalid_heartbeat_interval() {
    setup();
    env::set_var("PT_HEARTBEAT_INTERVAL", "xyz");
    assert!(Settings::from_env().is_err());
}

#[test]
#[serial]
fn test_settings_invalid_scenario() {
    setup();
    env::set_var("PT_SCENARIO", "INVALID_SCENARIO");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Invalid scenario"));
    }
}

#[test]
#[serial]
fn test_settings_empty_server() {
    setup();
    env::set_var("PT_SERVER", "");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Server address cannot be empty"));
    }
}

#[test]
#[serial]
fn test_settings_zero_listen_epoch() {
    setup();
    env::set_var("PT_LISTEN_EPOCH", "0");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Listen epoch cannot be zero"));
    }
}

#[test]
#[serial]
fn test_settings_zero_publish_epoch() {
    setup();
    env::set_var("PT_PUBLISH_EPOCH", "0");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Publish epoch cannot be zero"));
    }
}

#[test]
#[serial]
fn test_settings_zero_heartbeat_count() {
    setup();
    env::set_var("PT_HEARTBEAT_COUNT", "0");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Heartbeat count cannot be zero"));
    }
}

#[test]
#[serial]
fn test_settings_server_and_epochs() {
    setup();
    // Set environment variables for this test
    env::set_var("PT_SERVER", "custom.api.server.com");
    env::set_var("PT_LISTEN_EPOCH", "42");
    env::set_var("PT_PUBLISH_EPOCH", "24");
    
    let settings = Settings::from_env().unwrap();
    
    // Verify server and epoch settings
    assert_eq!(settings.server, "custom.api.server.com");
    assert_eq!(settings.listen_epoch, 42);
    assert_eq!(settings.publish_epoch, 24);
}

#[test]
#[serial]
fn test_settings_cancel_order() {
    setup();
    // Test with cancel_order set to false
    env::set_var("PT_CANCEL_ORDER", "false");
    
    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.cancel_order, false);
    
    // Test with cancel_order set to true
    setup();
    env::set_var("PT_CANCEL_ORDER", "true");
    
    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.cancel_order, true);
}

#[test]
#[serial]
fn test_settings_invalid_cancel_order() {
    setup();
    env::set_var("PT_CANCEL_ORDER", "not_a_boolean");
    assert!(Settings::from_env().is_err());
}

#[test]
#[serial]
fn test_settings_log_file() {
    setup();
    // Test with custom log file
    env::set_var("PT_LOG_FILE", "custom.log");
    
    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.log_file, "custom.log");
}

#[test]
#[serial]
fn test_settings_validate_empty_log_file() {
    setup();
    
    // Set environment variables with an empty log file
    env::set_var("PT_LOG_FILE", "");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Log file path cannot be empty"));
    }
}

#[test]
#[serial]
fn test_settings_validate_empty_target_comp_id() {
    setup();
    
    // Set environment variables with an empty target comp ID
    env::set_var("PT_TARGET_COMP_ID", "");
    
    let settings = Settings::from_env().unwrap();
    let result = settings.validate();
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Target CompID cannot be empty"));
    }
}

#[test]
#[serial]
fn test_settings_target_comp_id() {
    setup();
    
    // Test with custom target comp ID
    env::set_var("PT_TARGET_COMP_ID", "CUSTOM-ID");
    
    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.target_comp_id, "CUSTOM-ID");
    
    // Validate should pass with valid target comp ID
    assert!(settings.validate().is_ok());
}

#[test]
#[serial]
fn test_settings_validate_all_fields() {
    setup();
    
    // Set all environment variables with valid values
    env::set_var("PT_SYMBOL", "BTC-USD");
    env::set_var("PT_PRICE", "100.0");
    env::set_var("PT_QUANTITY", "1.0");
    env::set_var("PT_SIDE", "BUY");
    env::set_var("PT_ORDER_TYPE", "LIMIT");
    env::set_var("PT_SCENARIO", "ORDER");
    env::set_var("PT_SERVER", "test.server.com");
    env::set_var("PT_HEARTBEAT_INTERVAL", "30");
    env::set_var("PT_HEARTBEAT_COUNT", "10");
    env::set_var("PT_LISTEN_EPOCH", "42");
    env::set_var("PT_PUBLISH_EPOCH", "24");
    env::set_var("PT_CANCEL_ORDER", "true");
    env::set_var("PT_LOG_FILE", "test.log");
    env::set_var("PT_TARGET_COMP_ID", "TEST-ID");
    
    let settings = Settings::from_env().unwrap();
    
    // Validate should pass with all valid fields
    assert!(settings.validate().is_ok());
    
    // Check that all fields have the expected values
    assert_eq!(settings.symbol, "BTC-USD");
    assert_eq!(settings.price, 100.0);
    assert_eq!(settings.quantity, 1.0);
    assert_eq!(settings.side, "BUY");
    assert_eq!(settings.order_type, "LIMIT");
    assert_eq!(settings.scenario, "ORDER");
    assert_eq!(settings.server, "test.server.com");
    assert_eq!(settings.heartbeat_interval, 30);
    assert_eq!(settings.heartbeat_count, 10);
    assert_eq!(settings.listen_epoch, 42);
    assert_eq!(settings.publish_epoch, 24);
    assert_eq!(settings.cancel_order, true);
    assert_eq!(settings.log_file, "test.log");
    assert_eq!(settings.target_comp_id, "TEST-ID");
}
