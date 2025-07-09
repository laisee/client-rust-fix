use log::{debug, info};
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub side: String,
    pub order_type: String,
    pub scenario: String,
    pub server: String,
    pub heartbeat_interval: u64,
    pub heartbeat_count: u64,
    pub listen_epoch: u64,
    pub publish_epoch: u64,
    pub cancel_order: bool,
    pub log_file: String,
    pub target_comp_id: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Use debug logging instead of println
        debug!("Reading PT_SYMBOL: {:?}", env::var("PT_SYMBOL"));
        debug!("Reading PT_PRICE: {:?}", env::var("PT_PRICE"));
        debug!("Reading PT_QUANTITY: {:?}", env::var("PT_QUANTITY"));
        
        Ok(Self {
            symbol: env::var("PT_SYMBOL").unwrap_or_else(|_| "SOL-USD".to_string()),
            price: env::var("PT_PRICE")
                .unwrap_or_else(|_| "388.0".to_string())
                .parse()?,
            quantity: env::var("PT_QUANTITY")
                .unwrap_or_else(|_| "0.2".to_string())
                .parse()?,
            side: env::var("PT_SIDE").unwrap_or_else(|_| "SELL".to_string()),
            order_type: env::var("PT_ORDER_TYPE").unwrap_or_else(|_| "LIMIT".to_string()),
            scenario: env::var("PT_SCENARIO").unwrap_or_else(|_| "ORDER".to_string()),
            server: env::var("PT_SERVER").unwrap_or_else(|_| "api.wss.test.power.trade".to_string()),
            heartbeat_interval: env::var("PT_HEARTBEAT_INTERVAL")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            heartbeat_count: env::var("PT_HEARTBEAT_COUNT")
                .unwrap_or_else(|_| "20".to_string())
                .parse()?,
            listen_epoch: env::var("PT_LISTEN_EPOCH")
                .unwrap_or_else(|_| "21".to_string())
                .parse()?,
            publish_epoch: env::var("PT_PUBLISH_EPOCH")
                .unwrap_or_else(|_| "11".to_string())
                .parse()?,
            cancel_order: env::var("PT_CANCEL_ORDER")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            log_file: env::var("PT_LOG_FILE").unwrap_or_else(|_| "app.log".to_string()),
            target_comp_id: env::var("PT_TARGET_COMP_ID").unwrap_or_else(|_| "PT-OE".to_string()),
        })
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate price
        if self.price <= 0.0 {
            return Err("Price must be positive".into());
        }
        
        // Validate quantity
        if self.quantity <= 0.0 {
            return Err("Quantity must be positive".into());
        }
        
        // Validate side
        match self.side.to_uppercase().as_str() {
            "BUY" | "SELL" => {}
            _ => return Err(format!("Invalid side: {}", self.side).into()),
        }
        
        // Validate order type
        match self.order_type.to_uppercase().as_str() {
            "MARKET" | "LIMIT" => {}
            _ => return Err(format!("Invalid order type: {}", self.order_type).into()),
        }
        
        // Validate scenario
        match self.scenario.to_uppercase().as_str() {
            "ORDER" | "ORDERS" | "RFQ_QUOTE" | "RFQ_LISTEN" => {}
            _ => return Err(format!("Invalid scenario: {}", self.scenario).into()),
        }
        
        // Validate server
        if self.server.is_empty() {
            return Err("Server address cannot be empty".into());
        }
        
        // Validate epochs
        if self.listen_epoch == 0 {
            return Err("Listen epoch cannot be zero".into());
        }
        
        if self.publish_epoch == 0 {
            return Err("Publish epoch cannot be zero".into());
        }
        
        // Validate heartbeat settings
        if self.heartbeat_interval == 0 {
            return Err("Heartbeat interval cannot be zero".into());
        }
        
        if self.heartbeat_count == 0 {
            return Err("Heartbeat count cannot be zero".into());
        }
        
        // Validate log_file
        if self.log_file.is_empty() {
            return Err("Log file path cannot be empty".into());
        }
        
        // Validate target_comp_id
        if self.target_comp_id.is_empty() {
            return Err("Target CompID cannot be empty".into());
        }
        
        Ok(())
    }
}

pub fn load_config() -> Result<Settings, Box<dyn std::error::Error>> {
    // Load .env file if present
    match dotenvy::dotenv() {
        Ok(path) => info!("Loaded environment from {}", path.display()),
        Err(e) => debug!("Could not load .env file: {}", e),
    }
    
    // Load settings with proper error handling
    let settings = Settings::from_env()?;
    
    // Validate settings
    settings.validate()?;
    
    // Log successful configuration loading
    info!("Configuration loaded successfully");
    debug!("Using configuration: {:?}", settings);
    
    Ok(settings)
}
