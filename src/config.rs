use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub side: String,
    pub order_type: String,
    pub log_file: String,
    pub heartbeat_interval: u64,
    pub heartbeat_count: u32,
    pub target_comp_id: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            symbol: env::var("PT_SYMBOL").unwrap_or_else(|_| "SOL-USD".to_string()),
            price: env::var("PT_PRICE")
                .unwrap_or_else(|_| "388.0".to_string())
                .parse()?,
            quantity: env::var("PT_QUANTITY")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()?,
            side: env::var("PT_SIDE").unwrap_or_else(|_| "Sell".to_string()),
            order_type: env::var("PT_ORDER_TYPE").unwrap_or_else(|_| "Limit".to_string()),
            log_file: env::var("PT_LOG_FILE").unwrap_or_else(|_| "app.log".to_string()),
            heartbeat_interval: env::var("PT_HEARTBEAT_INTERVAL")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            heartbeat_count: env::var("PT_HEARTBEAT_COUNT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            target_comp_id: env::var("PT_TARGET_COMP_ID").unwrap_or_else(|_| "PT-OE".to_string()),
        })
    }
    
    // Add a new method to create Settings with custom values for testing
    #[allow(dead_code)]
    pub fn new(
        symbol: &str,
        price: f64,
        quantity: f64,
        side: &str,
        order_type: &str,
        log_file: &str,
        heartbeat_interval: u64,
        heartbeat_count: u32,
        target_comp_id: &str,
    ) -> Self {
        Self {
            symbol: symbol.to_string(),
            price,
            quantity,
            side: side.to_string(),
            order_type: order_type.to_string(),
            log_file: log_file.to_string(),
            heartbeat_interval,
            heartbeat_count,
            target_comp_id: target_comp_id.to_string(),
        }
    }
}
