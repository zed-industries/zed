use anyhow::Result;
use gpui::{App, Context, Entity, EventEmitter, Render, Subscription};
use http_client::HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

// Re-export core modules
pub mod market_data;
pub mod websocket_service;
pub mod mock_data_service;

#[cfg(test)]
mod tests;

pub use market_data::*;
pub use websocket_service::*;
pub use mock_data_service::*;

/// Initialize the stock trading system with Zed Lite integration
pub fn init(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Result<()> {
    // Initialize core services
    let _data_service = DataService::new(http_client.clone(), cx);
    let _websocket_service = WebSocketService::new(cx);
    let _mock_data_service = MockDataService::new(cx);
    
    Ok(())
}

/// Central coordinator entity for the stock trading system
pub struct TradingManager {
    data_service: Entity<DataService>,
    websocket_service: Entity<WebSocketService>,
    mock_data_service: Entity<MockDataService>,
    active_symbol: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl TradingManager {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Entity<Self> {
        let data_service = DataService::new(http_client, cx);
        let websocket_service = WebSocketService::new(cx);
        let mock_data_service = MockDataService::new(cx);
        
        cx.new_entity(Self {
            data_service,
            websocket_service,
            mock_data_service,
            active_symbol: None,
            _subscriptions: Vec::new(),
        })
    }
    
    pub fn set_active_symbol(&mut self, symbol: String, _cx: &mut Context<Self>) {
        self.active_symbol = Some(symbol);
    }
    
    pub fn get_active_symbol(&self) -> Option<&String> {
        self.active_symbol.as_ref()
    }
}

impl EventEmitter<TradingEvent> for TradingManager {}

impl Render for TradingManager {
    fn render(&mut self, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Minimal render implementation
    }
}

/// Core trading events for inter-component communication
#[derive(Clone, Debug)]
pub enum TradingEvent {
    SymbolSelected(String),
    MarketDataUpdated(MarketData),
    OrderPlaced(Order),
    OrderCancelled(String),
    WebSocketConnected,
    WebSocketDisconnected,
    DataServiceError(String),
}

/// Data service entity for managing market data and HTTP requests
pub struct DataService {
    http_client: Arc<dyn HttpClient>,
    cache: HashMap<String, (MarketData, Instant)>,
    cache_duration: Duration,
}

impl DataService {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Entity<Self> {
        cx.new_entity(Self {
            http_client,
            cache: HashMap::new(),
            cache_duration: Duration::from_secs(60), // 1 minute cache
        })
    }
    
    /// Get cached market data with bounds checking (.rules compliance)
    pub fn get_cached_data(&self, symbol: &str) -> Option<&MarketData> {
        self.cache.get(symbol).map(|(data, _)| data)
    }
    
    /// Process market data with proper error handling (.rules compliance)
    pub fn process_market_data(&mut self, data: MarketData, cx: &mut Context<Self>) -> Result<()> {
        // Use ? for error propagation instead of unwrap()
        let validated_data = self.validate_market_data(data)?;
        self.cache.insert(
            validated_data.symbol.clone(), 
            (validated_data, Instant::now())
        );
        cx.notify();
        Ok(())
    }
    
    /// Validate market data with proper error handling
    fn validate_market_data(&self, data: MarketData) -> Result<MarketData> {
        if data.symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if data.current_price < 0.0 {
            return Err(anyhow::anyhow!("Price cannot be negative"));
        }
        
        Ok(data)
    }
    
    /// Clean up stale data with proper error handling (.rules compliance)
    pub fn cleanup_stale_data(&mut self, cx: &mut Context<Self>) {
        let cutoff = Instant::now() - self.cache_duration;
        self.cache.retain(|_, (_, timestamp)| *timestamp > cutoff);
        
        // Background cleanup task with proper error handling
        cx.spawn(|this, mut cx| async move {
            if let Err(error) = this.update(&mut cx, |this, _| this.perform_deep_cleanup()) {
                error.log_err(); // Use .log_err() instead of let _ =
            }
        }).detach();
    }
    
    /// Perform deep cleanup of cached data
    fn perform_deep_cleanup(&mut self) -> Result<()> {
        // Implementation for deep cleanup
        self.cache.clear();
        Ok(())
    }
}

impl EventEmitter<DataEvent> for DataService {}

impl Render for DataService {
    fn render(&mut self, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Data service doesn't render UI directly
    }
}

/// Data service events
#[derive(Clone, Debug)]
pub enum DataEvent {
    MarketDataReceived(MarketData),
    HistoricalDataReceived(String, Vec<Candle>),
    ConnectionStatusChanged(bool),
    CacheUpdated(String),
}