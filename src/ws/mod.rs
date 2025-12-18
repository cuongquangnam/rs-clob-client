#![expect(
    clippy::module_name_repetitions,
    reason = "Re-exported names intentionally match their modules for API clarity"
)]

pub mod client;
pub mod config;
pub mod connection;
pub mod error;
pub mod messages;
pub mod subscription;

// Re-export commonly used types
pub use client::WebSocketClient;
pub use config::{ReconnectConfig, WebSocketConfig};
pub use error::WsError;
pub use messages::{
    AuthPayload, BookUpdate, LastTradePrice, OrderMessage, OrderStatus, PriceChange,
    SubscriptionRequest, TickSizeChange, TradeMessage, WsMessage,
};
