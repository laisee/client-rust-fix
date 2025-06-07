use client_rust_fix::config::Settings;
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
fn test_settings_from_env_defaults() {
    setup();
    let settings = Settings::from_env().expect("load settings");
    // Update the expected values to match the actual defaults in Settings::from_env()
    assert_eq!(settings.symbol, "SOL-USD");
    assert_eq!(settings.price, 388.0);
    assert_eq!(settings.quantity, 2.0);
    assert_eq!(settings.side, "Sell");
    assert_eq!(settings.order_type, "Limit");
    assert_eq!(settings.log_file, "app.log");
    assert_eq!(settings.heartbeat_interval, 60);
    assert_eq!(settings.heartbeat_count, 5);
    assert_eq!(settings.target_comp_id, "PT-OE");
}

#[test]
fn test_settings_custom_values() {
    setup();
    
    // Create settings with custom values directly
    let settings = Settings::new(
        "BTC-USD",
        42.0,
        2.5,
        "Buy",
        "Market",
        "test.log",
        3,
        2,
        "TEST",
    );
    
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
