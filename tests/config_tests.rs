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
}

#[test]
#[serial]
fn test_settings_from_env_defaults() {
    setup();
    let settings = Settings::from_env().expect("load settings");
    assert_eq!(settings.symbol, "SOL-USD");
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
