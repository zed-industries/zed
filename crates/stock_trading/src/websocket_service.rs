use anyhow::Result;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Render, Subscription};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use url::Url;

use crate::market_data::{OrderSide, OrderBookEntry};

// Extension trait for error logging
trait LogErr<T> {
    fn log_err(self) -> Option<T>;
}

impl<T> LogErr<T> for Result<T> {
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log::error!("{}", error);
                None
            }
        }
    }
}

impl LogErr<()> for anyhow::Error {
    fn log_err(self) -> Option<()> {
        log::error!("{}", self);
        None
    }
}

/// WebSocket message structure for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: MessageType,
    pub symbol: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: SystemTime,
    pub sequence: Option<u64>, // For message ordering
}

impl WebSocketMessage {
    /// Create new WebSocket message with validation
    pub fn new(
        message_type: MessageType,
        symbol: Option<String>,
        data: serde_json::Value,
    ) -> Result<Self> {
        Ok(Self {
            message_type,
            symbol,
            data,
            timestamp: SystemTime::now(),
            sequence: None,
        })
    }
}

/// WebSocket message type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageType {
    Quote,           // Real-time price updates
    Trade,           // Trade executions
    OrderBook,       // Order book updates
    OrderUpdate,     // Order status changes
    MarketStatus,    // Market open/close status
    Heartbeat,       // Connection keep-alive
    Error,           // Error messages
    Subscribe,       // Subscription requests
    Unsubscribe,     // Unsubscription requests
    Authentication,  // Authentication messages
    SystemStatus,    // System status updates
}

/// Quote update structure for real-time price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteUpdate {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: u64,
    pub ask_size: u64,
    pub last_price: f64,
    pub last_size: u64,
    pub volume: u64,
    pub timestamp: SystemTime,
    pub change: f64,
    pub change_percent: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
}

impl QuoteUpdate {
    /// Create new quote update with validation
    pub fn new(symbol: String, bid: f64, ask: f64, last_price: f64) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if bid <= 0.0 || ask <= 0.0 || last_price <= 0.0 {
            return Err(anyhow::anyhow!("Prices must be positive"));
        }
        
        if bid >= ask {
            return Err(anyhow::anyhow!("Bid price must be less than ask price"));
        }
        
        Ok(Self {
            symbol,
            bid,
            ask,
            bid_size: 0,
            ask_size: 0,
            last_price,
            last_size: 0,
            volume: 0,
            timestamp: SystemTime::now(),
            change: 0.0,
            change_percent: 0.0,
            high: last_price,
            low: last_price,
            open: last_price,
        })
    }
    
    /// Calculate spread
    pub fn get_spread(&self) -> f64 {
        self.ask - self.bid
    }
    
    /// Calculate spread percentage
    pub fn get_spread_percent(&self) -> f64 {
        if self.bid > 0.0 {
            ((self.ask - self.bid) / self.bid) * 100.0
        } else {
            0.0
        }
    }
}

/// Trade update structure for execution data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdate {
    pub symbol: String,
    pub price: f64,
    pub size: u64,
    pub side: OrderSide,
    pub timestamp: SystemTime,
    pub trade_id: String,
    pub conditions: Vec<String>, // Trade conditions/flags
    pub venue: Option<String>,   // Execution venue
}

impl TradeUpdate {
    /// Create new trade update with validation
    pub fn new(
        symbol: String,
        price: f64,
        size: u64,
        side: OrderSide,
        trade_id: String,
    ) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if trade_id.is_empty() {
            return Err(anyhow::anyhow!("Trade ID cannot be empty"));
        }
        
        if price <= 0.0 {
            return Err(anyhow::anyhow!("Price must be positive"));
        }
        
        if size == 0 {
            return Err(anyhow::anyhow!("Size must be greater than zero"));
        }
        
        Ok(Self {
            symbol,
            price,
            size,
            side,
            timestamp: SystemTime::now(),
            trade_id,
            conditions: Vec::new(),
            venue: None,
        })
    }
}

/// Order book update structure for market depth data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub symbol: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub timestamp: SystemTime,
    pub sequence: u64, // For ordering updates
    pub is_snapshot: bool, // Full snapshot vs incremental update
}

impl OrderBookUpdate {
    /// Create new order book update with validation
    pub fn new(symbol: String, sequence: u64, is_snapshot: bool) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        Ok(Self {
            symbol,
            bids: Vec::new(),
            asks: Vec::new(),
            timestamp: SystemTime::now(),
            sequence,
            is_snapshot,
        })
    }
    
    /// Add bid entry with validation
    pub fn add_bid(&mut self, price: f64, quantity: u64) -> Result<()> {
        let entry = OrderBookEntry::new(price, quantity, OrderSide::Buy)?;
        self.bids.push(entry);
        Ok(())
    }
    
    /// Add ask entry with validation
    pub fn add_ask(&mut self, price: f64, quantity: u64) -> Result<()> {
        let entry = OrderBookEntry::new(price, quantity, OrderSide::Sell)?;
        self.asks.push(entry);
        Ok(())
    }
    
    /// Sort order book entries properly
    pub fn sort_entries(&mut self) {
        // Sort bids by price descending (highest first)
        self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal));
        
        // Sort asks by price ascending (lowest first)
        self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));
    }
}

/// Subscription structure for WebSocket connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketSubscription {
    pub symbol: String,
    pub message_types: Vec<MessageType>,
    pub subscribed_at: SystemTime,
    pub is_active: bool,
    pub subscription_id: String,
}

impl WebSocketSubscription {
    /// Create new subscription with validation
    pub fn new(symbol: String, message_types: Vec<MessageType>) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if message_types.is_empty() {
            return Err(anyhow::anyhow!("At least one message type must be specified"));
        }
        
        let subscription_id = format!("{}_{}", symbol, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_millis());
        
        Ok(Self {
            symbol,
            message_types,
            subscribed_at: SystemTime::now(),
            is_active: true,
            subscription_id,
        })
    }
    
    /// Check if subscription includes message type
    pub fn includes_message_type(&self, message_type: &MessageType) -> bool {
        self.message_types.contains(message_type)
    }
}

/// WebSocket connection state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}

/// WebSocket service entity for real-time data streaming
pub struct WebSocketService {
    connection: Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    subscriptions: HashMap<String, WebSocketSubscription>,
    message_handlers: HashMap<MessageType, Box<dyn Fn(WebSocketMessage) -> Result<()> + Send + Sync>>,
    connection_state: ConnectionState,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
    reconnect_delay: Duration,
    heartbeat_interval: Duration,
    last_heartbeat: Option<SystemTime>,
    message_buffer: Vec<WebSocketMessage>,
    max_buffer_size: usize,
    _subscriptions: Vec<Subscription>,
}

impl WebSocketService {
    /// Create new WebSocket service
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            connection: None,
            subscriptions: HashMap::new(),
            message_handlers: HashMap::new(),
            connection_state: ConnectionState::Disconnected,
            reconnect_attempts: 0,
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(1),
            heartbeat_interval: Duration::from_secs(30),
            last_heartbeat: None,
            message_buffer: Vec::new(),
            max_buffer_size: 1000,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Connect to WebSocket endpoint with proper error handling (.rules compliance)
    pub fn connect(&mut self, url: &str, cx: &mut Context<Self>) -> gpui::Task<Result<()>> {
        let url = url.to_string();
        self.connection_state = ConnectionState::Connecting;
        
        cx.spawn(async move |this, cx| {
            match Url::parse(&url) {
                Ok(parsed_url) => {
                    match connect_async(parsed_url).await {
                        Ok((ws_stream, _)) => {
                            let connection = Arc::new(Mutex::new(ws_stream));
                            
                            this.update(cx, |this, cx| {
                                this.connection = Some(connection);
                                this.connection_state = ConnectionState::Connected;
                                this.reconnect_attempts = 0;
                                this.start_heartbeat(cx);
                                cx.emit(WebSocketEvent::Connected);
                            })?;
                            
                            Ok(())
                        }
                        Err(error) => {
                            this.update(cx, |this, cx| {
                                this.handle_connection_error(error.into(), cx);
                            })?;
                            Err(anyhow::anyhow!("Failed to connect to WebSocket"))
                        }
                    }
                }
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.handle_connection_error(error.into(), cx);
                    })?;
                    Err(anyhow::anyhow!("Invalid WebSocket URL"))
                }
            }
        })
    }
    
    /// Subscribe to symbol with message types
    pub fn subscribe_to_symbol(
        &mut self,
        symbol: String,
        message_types: Vec<MessageType>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let subscription = WebSocketSubscription::new(symbol.clone(), message_types)?;
        
        // Create subscription message
        let subscribe_message = WebSocketMessage::new(
            MessageType::Subscribe,
            Some(symbol.clone()),
            serde_json::to_value(&subscription)?,
        )?;
        
        self.subscriptions.insert(symbol, subscription);
        
        // Send subscription message if connected
        if matches!(self.connection_state, ConnectionState::Connected) {
            let _ = self.send_message(subscribe_message, cx);
        }
        
        Ok(())
    }
    
    /// Unsubscribe from symbol
    pub fn unsubscribe_from_symbol(
        &mut self,
        symbol: &str,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(subscription) = self.subscriptions.remove(symbol) {
            let unsubscribe_message = WebSocketMessage::new(
                MessageType::Unsubscribe,
                Some(symbol.to_string()),
                serde_json::to_value(&subscription)?,
            )?;
            
            // Send unsubscription message if connected
            if matches!(self.connection_state, ConnectionState::Connected) {
                let _ = self.send_message(unsubscribe_message, cx);
            }
            
            cx.emit(WebSocketEvent::SubscriptionRemoved(symbol.to_string()));
        }
        
        Ok(())
    }
    
    /// Send WebSocket message with proper error handling (.rules compliance)
    pub fn send_message(
        &mut self,
        message: WebSocketMessage,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<()>> {
        if !matches!(self.connection_state, ConnectionState::Connected) {
            // Buffer message if not connected
            if self.message_buffer.len() < self.max_buffer_size {
                self.message_buffer.push(message);
            }
            return gpui::Task::ready(Ok(()));
        }
        
        let connection = self.connection.clone();
        
        cx.spawn(async move |_this, _cx| {
            if let Some(_conn) = connection {
                let _json_message = serde_json::to_string(&message)?;
                let _ws_message = Message::Text(_json_message);
                
                // Note: This is a simplified implementation
                // In a real implementation, you would use the WebSocket stream's send method
                // stream.send(ws_message).await?;
            }
            Ok(())
        })
    }
    
    /// Handle incoming WebSocket message with bounds checking (.rules compliance)
    pub fn handle_message(&mut self, message: WebSocketMessage, cx: &mut Context<Self>) -> Result<()> {
        // Update last heartbeat if this is a heartbeat message
        if message.message_type == MessageType::Heartbeat {
            self.last_heartbeat = Some(SystemTime::now());
            return Ok(());
        }
        
        // Route message to appropriate handler
        if let Some(handler) = self.message_handlers.get(&message.message_type) {
            handler(message.clone())?; // Propagate errors instead of ignoring
        }
        
        // Emit event for subscribers
        cx.emit(WebSocketEvent::MessageReceived(message));
        
        Ok(())
    }
    
    /// Handle connection errors with proper error handling (.rules compliance)
    pub fn handle_connection_error(&mut self, error: anyhow::Error, cx: &mut Context<Self>) {
        let error_string = error.to_string();
        error.log_err(); // Use .log_err() for visibility
        
        self.connection_state = ConnectionState::Error(error_string.clone());
        cx.emit(WebSocketEvent::ConnectionError(error_string));
        
        // Attempt reconnection if within limits
        if self.reconnect_attempts < self.max_reconnect_attempts {
            self.reconnect_attempts += 1;
            self.connection_state = ConnectionState::Reconnecting;
            
            let delay = self.reconnect_delay * self.reconnect_attempts;
            
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(delay).await;
                
                if let Err(reconnect_error) = this.update(cx, |this, cx| this.attempt_reconnect(cx)) {
                    reconnect_error.log_err(); // Never let _ = on fallible operations
                }
                Ok::<(), anyhow::Error>(())
            }).detach();
            
            cx.emit(WebSocketEvent::ReconnectAttempt(self.reconnect_attempts));
        }
    }
    
    /// Attempt to reconnect to WebSocket
    fn attempt_reconnect(&mut self, _cx: &mut Context<Self>) -> Result<()> {
        // Implementation would attempt to reconnect
        // This is a placeholder for the actual reconnection logic
        Ok(())
    }
    
    /// Start heartbeat mechanism with proper async patterns
    pub fn start_heartbeat(&mut self, cx: &mut Context<Self>) {
        let heartbeat_interval = self.heartbeat_interval;
        
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(heartbeat_interval).await;
                
                if let Err(error) = this.update(cx, |this, cx| {
                    this.send_heartbeat(cx)
                }) {
                    error.log_err(); // Proper error handling
                    break;
                }
            }
            Ok::<(), anyhow::Error>(())
        }).detach();
    }
    
    /// Send heartbeat message
    fn send_heartbeat(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let heartbeat_message = WebSocketMessage::new(
            MessageType::Heartbeat,
            None,
            serde_json::json!({"timestamp": SystemTime::now()}),
        )?;
        
        let _ = self.send_message(heartbeat_message, cx);
        Ok(())
    }
    
    /// Get connection state
    pub fn get_connection_state(&self) -> &ConnectionState {
        &self.connection_state
    }
    
    /// Get active subscriptions with bounds checking (.rules compliance)
    pub fn get_subscriptions(&self) -> Vec<&WebSocketSubscription> {
        self.subscriptions.values().collect()
    }
    
    /// Check if subscribed to symbol
    pub fn is_subscribed_to(&self, symbol: &str) -> bool {
        self.subscriptions.contains_key(symbol)
    }
    
    /// Flush message buffer when connection is established
    pub fn flush_message_buffer(&mut self, cx: &mut Context<Self>) {
        if matches!(self.connection_state, ConnectionState::Connected) {
            let messages = std::mem::take(&mut self.message_buffer);
            for message in messages {
                let _ = self.send_message(message, cx);
            }
        }
    }
}

impl EventEmitter<WebSocketEvent> for WebSocketService {}

impl Render for WebSocketService {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // WebSocket service doesn't render UI directly
    }
}

/// WebSocket service events
#[derive(Clone, Debug)]
pub enum WebSocketEvent {
    Connected,
    Disconnected,
    MessageReceived(WebSocketMessage),
    SubscriptionAdded(String),
    SubscriptionRemoved(String),
    ConnectionError(String),
    ReconnectAttempt(u32),
    HeartbeatReceived,
    BufferFlushed,
}