// src/lib.rs
pub mod messages {
    pub mod factory;
    pub mod utils;
}
pub mod scenarios {
    pub mod rfq_listen;
    pub mod rfq_publish;
    pub mod single_leg_order;
    #[cfg(test)]
    mod tests;
}
pub mod config;
pub mod setup;
