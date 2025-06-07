use client_rust_fix::config::Settings;
use std::env;

#[test]
fn test_settings_from_env_defaults() {
    env::remove_var("PT_SYMBOL");
    let settings = Settings::from_env().expect("load settings");
    assert_eq!(settings.symbol, "SOL-USD");
}

#[test]
fn test_settings_custom_values() {
    env::set_var("PT_SYMBOL", "BTC-USD");
    env::set_var("PT_PRICE", "42.0");
    env::set_var("PT_QUANTITY", "2.5");
    env::set_var("PT_SIDE", "Buy");
    env::set_var("PT_ORDER_TYPE", "Market");
    env::set_var("PT_LOG_FILE", "test.log");
    env::set_var("PT_HEARTBEAT_INTERVAL", "3");
    env::set_var("PT_HEARTBEAT_COUNT", "2");
    env::set_var("PT_TARGET_COMP_ID", "TEST");
    let settings = Settings::from_env().unwrap();
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
