use anyhow::Result;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Render, Subscription};
use http_client::HttpClient;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

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

/// Enhanced central coordinator entity for the stock trading system
pub struct TradingManager {
    data_service: Entity<DataService>,
    websocket_service: Entity<WebSocketService>,
    mock_data_service: Entity<MockDataService>,
    active_symbol: Option<String>,
    subscribed_symbols: std::collections::HashSet<String>,
    auto_subscribe_enabled: bool,
    _subscriptions: Vec<Subscription>,
}

impl TradingManager {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Entity<Self> {
        let data_service = DataService::new(http_client, cx);
        let websocket_service = WebSocketService::new(cx);
        let mock_data_service = MockDataService::new(cx);
        
        cx.new(|cx| {
            let mut manager = Self {
                data_service,
                websocket_service,
                mock_data_service,
                active_symbol: None,
                subscribed_symbols: std::collections::HashSet::new(),
                auto_subscribe_enabled: true,
                _subscriptions: Vec::new(),
            };
            
            // Set up service references
            manager.setup_service_integration(cx);
            
            manager
        })
    }
    
    /// Set up integration between services
    fn setup_service_integration(&mut self, cx: &mut Context<Self>) {
        // Set WebSocket service reference in DataService
        self.data_service.update(cx, |data_service, _| {
            data_service.set_websocket_service(self.websocket_service.clone());
            data_service.set_mock_data_service(self.mock_data_service.clone());
        });
        
        // Subscribe to WebSocket events
        let websocket_subscription = cx.subscribe(&self.websocket_service, |this, _websocket, event, cx| {
            this.handle_websocket_event(event.clone(), cx);
        });
        self._subscriptions.push(websocket_subscription);
        
        // Subscribe to data service events
        let data_subscription = cx.subscribe(&self.data_service, |this, _data_service, event, cx| {
            this.handle_data_event(event.clone(), cx);
        });
        self._subscriptions.push(data_subscription);
        
        // Subscribe to mock data events
        let mock_subscription = cx.subscribe(&self.mock_data_service, |this, _mock_service, event, cx| {
            this.handle_mock_data_event(event.clone(), cx);
        });
        self._subscriptions.push(mock_subscription);
    }
    
    /// Handle WebSocket events
    fn handle_websocket_event(&mut self, event: WebSocketEvent, cx: &mut Context<Self>) {
        match event {
            WebSocketEvent::Connected => {
                // Re-subscribe to all symbols when WebSocket connects
                self.resubscribe_all_symbols(cx);
                cx.emit(TradingEvent::WebSocketConnected);
            }
            WebSocketEvent::Disconnected => {
                cx.emit(TradingEvent::WebSocketDisconnected);
            }
            WebSocketEvent::MessageReceived(message) => {
                // Forward WebSocket messages to DataService for processing
                if let Err(error) = self.data_service.update(cx, |data_service, cx| {
                    data_service.handle_websocket_message(message, cx)
                }) {
                    error.log_err(); // Proper error handling
                }
            }
            WebSocketEvent::ConnectionError(error) => {
                cx.emit(TradingEvent::DataServiceError(error));
            }
            _ => {} // Handle other events as needed
        }
    }
    
    /// Handle data service events
    fn handle_data_event(&mut self, event: DataEvent, cx: &mut Context<Self>) {
        match event {
            DataEvent::MarketDataReceived(market_data) => {
                cx.emit(TradingEvent::MarketDataUpdated(market_data));
            }
            DataEvent::TradeReceived(trade) => {
                // Convert trade to order for compatibility
                if let Ok(order) = self.convert_trade_to_order(trade) {
                    cx.emit(TradingEvent::OrderPlaced(order));
                }
            }
            DataEvent::ErrorOccurred(error) => {
                cx.emit(TradingEvent::DataServiceError(error));
            }
            _ => {} // Handle other events as needed
        }
    }
    
    /// Handle mock data service events
    fn handle_mock_data_event(&mut self, event: MockDataEvent, cx: &mut Context<Self>) {
        match event {
            MockDataEvent::MarketDataUpdated(_symbol, market_data) => {
                // Forward to main data flow
                cx.emit(TradingEvent::MarketDataUpdated(market_data));
            }
            _ => {} // Handle other events as needed
        }
    }
    
    /// Set active symbol with automatic subscription
    pub fn set_active_symbol(&mut self, symbol: String, cx: &mut Context<Self>) -> Result<()> {
        // Auto-subscribe to real-time updates if enabled
        if self.auto_subscribe_enabled {
            self.subscribe_to_symbol(symbol.clone(), cx)?;
        }
        
        self.active_symbol = Some(symbol.clone());
        cx.emit(TradingEvent::SymbolSelected(symbol));
        Ok(())
    }
    
    /// Subscribe to symbol for real-time updates
    pub fn subscribe_to_symbol(&mut self, symbol: String, cx: &mut Context<Self>) -> Result<()> {
        if self.subscribed_symbols.contains(&symbol) {
            return Ok(()); // Already subscribed
        }
        
        self.subscribed_symbols.insert(symbol.clone());
        
        // Subscribe via DataService
        self.data_service.update(cx, |data_service, cx| {
            data_service.subscribe_to_symbol(symbol, cx)
        })?;
        
        Ok(())
    }
    
    /// Unsubscribe from symbol
    pub fn unsubscribe_from_symbol(&mut self, symbol: &str, cx: &mut Context<Self>) -> Result<()> {
        if !self.subscribed_symbols.remove(symbol) {
            return Ok(()); // Not subscribed
        }
        
        // Unsubscribe via DataService
        self.data_service.update(cx, |data_service, cx| {
            data_service.unsubscribe_from_symbol(symbol, cx)
        })?;
        
        Ok(())
    }
    
    /// Re-subscribe to all symbols (useful after reconnection)
    fn resubscribe_all_symbols(&mut self, cx: &mut Context<Self>) {
        let symbols: Vec<String> = self.subscribed_symbols.iter().cloned().collect();
        
        for symbol in symbols {
            if let Err(error) = self.data_service.update(cx, |data_service, cx| {
                data_service.subscribe_to_symbol(symbol, cx)
            }) {
                error.log_err(); // Log but continue with other symbols
            }
        }
    }
    
    /// Get market data for symbol
    pub fn get_market_data(&mut self, symbol: &str, cx: &mut Context<Self>) -> gpui::Task<Result<MarketData>> {
        self.data_service.update(cx, |data_service, cx| {
            data_service.get_market_data(symbol, cx)
        })
    }
    
    /// Get historical data for symbol
    pub fn get_historical_data(
        &mut self,
        symbol: &str,
        timeframe: TimeFrame,
        periods: usize,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<Vec<Candle>>> {
        self.data_service.update(cx, |data_service, cx| {
            data_service.get_historical_data(symbol, timeframe, periods, cx)
        })
    }
    
    /// Get order book for symbol
    pub fn get_order_book(&mut self, symbol: &str, cx: &mut Context<Self>) -> gpui::Task<Result<OrderBook>> {
        self.data_service.update(cx, |data_service, cx| {
            data_service.get_order_book(symbol, cx)
        })
    }
    
    /// Toggle between mock and live data
    pub fn set_use_mock_data(&mut self, use_mock: bool, cx: &mut Context<Self>) {
        self.data_service.update(cx, |data_service, cx| {
            data_service.set_use_mock_data(use_mock, cx);
        });
    }
    
    /// Start mock data simulation
    pub fn start_mock_simulation(&mut self, cx: &mut Context<Self>) {
        self.mock_data_service.update(cx, |mock_service, cx| {
            mock_service.start_simulation(cx);
        });
    }
    
    /// Stop mock data simulation
    pub fn stop_mock_simulation(&mut self, cx: &mut Context<Self>) {
        self.mock_data_service.update(cx, |mock_service, cx| {
            mock_service.stop_simulation(cx);
        });
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self, cx: &Context<Self>) -> CacheStats {
        self.data_service.read(cx).get_cache_stats()
    }
    
    /// Convert trade update to order (for event compatibility)
    fn convert_trade_to_order(&self, trade: TradeUpdate) -> Result<Order> {
        Order::new(
            format!("ORDER_{}", trade.trade_id),
            trade.symbol,
            trade.side,
            OrderType::Market,
            trade.size,
            Some(trade.price),
            TimeInForce::Day,
        )
    }
    
    /// Enable/disable auto-subscription for active symbols
    pub fn set_auto_subscribe(&mut self, enabled: bool) {
        self.auto_subscribe_enabled = enabled;
    }
    
    /// Get active symbol
    pub fn get_active_symbol(&self) -> Option<&String> {
        self.active_symbol.as_ref()
    }
    
    /// Get subscribed symbols
    pub fn get_subscribed_symbols(&self) -> Vec<String> {
        self.subscribed_symbols.iter().cloned().collect()
    }
    
    /// Check if symbol is subscribed
    pub fn is_subscribed(&self, symbol: &str) -> bool {
        self.subscribed_symbols.contains(symbol)
    }
}

impl EventEmitter<TradingEvent> for TradingManager {}

impl Render for TradingManager {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Minimal render implementation
    }
}

/// Enhanced core trading events for inter-component communication
#[derive(Clone, Debug)]
pub enum TradingEvent {
    SymbolSelected(String),
    MarketDataUpdated(MarketData),
    OrderPlaced(Order),
    OrderCancelled(String),
    WebSocketConnected,
    WebSocketDisconnected,
    DataServiceError(String),
    HistoricalDataUpdated(String, Vec<Candle>),
    OrderBookUpdated(String, OrderBook),
    SymbolSubscribed(String),
    SymbolUnsubscribed(String),
    CacheStatsUpdated(CacheStats),
    SimulationStarted,
    SimulationStopped,
}

/// Enhanced data service entity for managing market data with WebSocket integration
pub struct DataService {
    http_client: Arc<dyn HttpClient>,
    websocket_service: Option<Entity<WebSocketService>>,
    mock_data_service: Option<Entity<MockDataService>>,
    cache: HashMap<String, CachedMarketData>,
    historical_cache: HashMap<String, HashMap<TimeFrame, Vec<Candle>>>,
    order_book_cache: HashMap<String, OrderBook>,
    cache_duration: Duration,
    use_mock_data: bool,
    subscribed_symbols: std::collections::HashSet<String>,
    auto_refresh_enabled: bool,
    refresh_interval: Duration,
    _cleanup_task: Option<gpui::Task<()>>,
    _refresh_task: Option<gpui::Task<()>>,
}

/// Enhanced cached market data with metadata
#[derive(Debug, Clone)]
struct CachedMarketData {
    data: MarketData,
    cached_at: Instant,
    source: DataSource,
    access_count: u32,
    last_accessed: Instant,
}

/// Data source enumeration
#[derive(Debug, Clone, PartialEq)]
enum DataSource {
    WebSocket,
    Http,
    Mock,
    Cache,
}

impl DataService {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let mut service = Self {
                http_client,
                websocket_service: None,
                mock_data_service: None,
                cache: HashMap::new(),
                historical_cache: HashMap::new(),
                order_book_cache: HashMap::new(),
                cache_duration: Duration::from_secs(60), // 1 minute cache
                use_mock_data: true, // Default to mock data for development
                subscribed_symbols: std::collections::HashSet::new(),
                auto_refresh_enabled: true,
                refresh_interval: Duration::from_secs(30), // 30 second refresh
                _cleanup_task: None,
                _refresh_task: None,
            };
            
            // Start background tasks
            service.start_cleanup_task(cx);
            service.start_refresh_task(cx);
            
            service
        })
    }
    
    /// Set WebSocket service for real-time updates
    pub fn set_websocket_service(&mut self, websocket_service: Entity<WebSocketService>) {
        self.websocket_service = Some(websocket_service);
    }
    
    /// Set mock data service for development
    pub fn set_mock_data_service(&mut self, mock_data_service: Entity<MockDataService>) {
        self.mock_data_service = Some(mock_data_service);
    }
    
    /// Toggle between mock and real data
    pub fn set_use_mock_data(&mut self, use_mock: bool, cx: &mut Context<Self>) {
        self.use_mock_data = use_mock;
        
        // Clear cache when switching data sources
        self.cache.clear();
        self.historical_cache.clear();
        self.order_book_cache.clear();
        
        cx.emit(DataEvent::DataSourceChanged(if use_mock { "Mock".to_string() } else { "Live".to_string() }));
        cx.notify();
    }
    
    /// Subscribe to real-time updates for symbol
    pub fn subscribe_to_symbol(&mut self, symbol: String, cx: &mut Context<Self>) -> Result<()> {
        if self.subscribed_symbols.contains(&symbol) {
            return Ok(()); // Already subscribed
        }
        
        self.subscribed_symbols.insert(symbol.clone());
        
        // Subscribe via WebSocket if available
        if let Some(websocket_service) = &self.websocket_service {
            let message_types = vec![MessageType::Quote, MessageType::Trade, MessageType::OrderBook];
            
            websocket_service.update(cx, |ws, cx| {
                ws.subscribe_to_symbol(symbol.clone(), message_types, cx)
            })?;
        }
        
        cx.emit(DataEvent::SymbolSubscribed(symbol));
        Ok(())
    }
    
    /// Unsubscribe from real-time updates for symbol
    pub fn unsubscribe_from_symbol(&mut self, symbol: &str, cx: &mut Context<Self>) -> Result<()> {
        if !self.subscribed_symbols.remove(symbol) {
            return Ok(()); // Not subscribed
        }
        
        // Unsubscribe via WebSocket if available
        if let Some(websocket_service) = &self.websocket_service {
            websocket_service.update(cx, |ws, cx| {
                ws.unsubscribe_from_symbol(symbol, cx)
            })?;
        }
        
        cx.emit(DataEvent::SymbolUnsubscribed(symbol.to_string()));
        Ok(())
    }
    
    /// Get market data with intelligent caching and fallback (.rules compliance)
    pub fn get_market_data(&mut self, symbol: &str, cx: &mut Context<Self>) -> gpui::Task<Result<MarketData>> {
        let symbol = symbol.to_string();
        
        // Check cache first with bounds checking
        if let Some(cached) = self.cache.get_mut(&symbol) {
            cached.access_count += 1;
            cached.last_accessed = Instant::now();
            
            // Return cached data if still fresh
            if cached.cached_at.elapsed() < self.cache_duration {
                cached.data.clone(); // Update source to cache
                return gpui::Task::ready(Ok(cached.data.clone()));
            }
        }
        
        // Fetch fresh data
        if self.use_mock_data {
            self.fetch_mock_market_data(symbol, cx)
        } else {
            self.fetch_live_market_data(symbol, cx)
        }
    }
    
    /// Fetch market data from mock service
    fn fetch_mock_market_data(&mut self, symbol: String, cx: &mut Context<Self>) -> gpui::Task<Result<MarketData>> {
        if let Some(mock_service) = &self.mock_data_service {
            let mock_service = mock_service.clone();
            
            cx.spawn(async move |this, cx| {
                let market_data_option = mock_service.update(cx, |service, _| {
                    service.get_market_data(&symbol)
                })?;
                
                let market_data = market_data_option.ok_or_else(|| anyhow::anyhow!("Symbol not found in mock data: {}", symbol))?;
                
                // Cache the data
                this.update(cx, |this, cx| {
                    this.cache_market_data(market_data.clone(), DataSource::Mock, cx)
                })?;
                
                Ok(market_data)
            })
        } else {
            gpui::Task::ready(Err(anyhow::anyhow!("Mock data service not available")))
        }
    }
    
    /// Fetch market data from live sources (HTTP fallback)
    fn fetch_live_market_data(&mut self, _symbol: String, cx: &mut Context<Self>) -> gpui::Task<Result<MarketData>> {
        // For now, return an error as we don't have live data integration yet
        // In a real implementation, this would make HTTP requests to financial APIs
        cx.spawn(async move |_this, _cx| {
            Err(anyhow::anyhow!("Live data not implemented yet. Use mock data for development."))
        })
    }
    
    /// Cache market data with metadata
    fn cache_market_data(&mut self, data: MarketData, source: DataSource, cx: &mut Context<Self>) -> Result<()> {
        let validated_data = self.validate_market_data(data)?;
        
        let cached_data = CachedMarketData {
            data: validated_data.clone(),
            cached_at: Instant::now(),
            source,
            access_count: 1,
            last_accessed: Instant::now(),
        };
        
        self.cache.insert(validated_data.symbol.clone(), cached_data);
        cx.emit(DataEvent::MarketDataReceived(validated_data));
        cx.notify();
        
        Ok(())
    }
    
    /// Get historical data with caching
    pub fn get_historical_data(
        &mut self,
        symbol: &str,
        timeframe: TimeFrame,
        periods: usize,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<Vec<Candle>>> {
        let symbol = symbol.to_string();
        
        // Check cache first with bounds checking
        if let Some(symbol_cache) = self.historical_cache.get(&symbol) {
            if let Some(cached_data) = symbol_cache.get(&timeframe) {
                if cached_data.len() >= periods {
                    // Return cached data if sufficient
                    let result = cached_data.iter().take(periods).cloned().collect();
                    return gpui::Task::ready(Ok(result));
                }
            }
        }
        
        // Fetch fresh historical data
        if self.use_mock_data {
            self.fetch_mock_historical_data(symbol, timeframe, periods, cx)
        } else {
            self.fetch_live_historical_data(symbol, timeframe, periods, cx)
        }
    }
    
    /// Fetch historical data from mock service
    fn fetch_mock_historical_data(
        &mut self,
        symbol: String,
        timeframe: TimeFrame,
        periods: usize,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<Vec<Candle>>> {
        if let Some(mock_service) = &self.mock_data_service {

            
            let mock_service = mock_service.clone();
            
            cx.spawn(async move |this, cx| {
                let historical_data = mock_service.update(cx, |service, _| {
                    service.generate_historical_data(&symbol, timeframe, periods)
                })?;
                
                // Cache the data
                this.update(cx, |this, cx| {
                    this.cache_historical_data(symbol.clone(), timeframe, historical_data.clone(), cx)
                })?;
                
                Ok(historical_data)
            })
        } else {
            gpui::Task::ready(Err(anyhow::anyhow!("Mock data service not available")))
        }
    }
    
    /// Fetch historical data from live sources
    fn fetch_live_historical_data(
        &mut self,
        _symbol: String,
        _timeframe: TimeFrame,
        _periods: usize,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<Vec<Candle>>> {
        cx.spawn(async move |_this, _cx| {
            Err(anyhow::anyhow!("Live historical data not implemented yet. Use mock data for development."))
        })
    }
    
    /// Cache historical data
    fn cache_historical_data(
        &mut self,
        symbol: String,
        timeframe: TimeFrame,
        data: Vec<Candle>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.historical_cache
            .entry(symbol.clone())
            .or_insert_with(HashMap::new)
            .insert(timeframe, data.clone());
        
        cx.emit(DataEvent::HistoricalDataReceived(symbol, data));
        Ok(())
    }
    
    /// Get order book data
    pub fn get_order_book(&mut self, symbol: &str, cx: &mut Context<Self>) -> gpui::Task<Result<OrderBook>> {
        let symbol = symbol.to_string();
        
        // Check cache first
        if let Some(cached_order_book) = self.order_book_cache.get(&symbol) {
            // Check if data is still fresh (order books update frequently)
            let order_book_cache_duration = Duration::from_secs(5); // 5 second cache for order books
            if cached_order_book.timestamp.elapsed().unwrap_or(Duration::MAX) < order_book_cache_duration {
                return gpui::Task::ready(Ok(cached_order_book.clone()));
            }
        }
        
        // Fetch fresh order book data
        if self.use_mock_data {
            self.fetch_mock_order_book(symbol, cx)
        } else {
            self.fetch_live_order_book(symbol, cx)
        }
    }
    
    /// Fetch order book from mock service
    fn fetch_mock_order_book(&mut self, symbol: String, cx: &mut Context<Self>) -> gpui::Task<Result<OrderBook>> {
        if let Some(mock_service) = &self.mock_data_service {
            let mock_service = mock_service.clone();
            
            cx.spawn(async move |this, cx| {
                let order_book = mock_service.update(cx, |service, _| {
                    service.generate_order_book(&symbol)
                })?;
                
                // Cache the order book
                this.update(cx, |this, _cx| {
                    this.order_book_cache.insert(symbol, order_book.clone());
                })?;
                
                Ok(order_book)
            })
        } else {
            gpui::Task::ready(Err(anyhow::anyhow!("Mock data service not available")))
        }
    }
    
    /// Fetch order book from live sources
    fn fetch_live_order_book(&mut self, _symbol: String, cx: &mut Context<Self>) -> gpui::Task<Result<OrderBook>> {
        cx.spawn(async move |_this, _cx| {
            Err(anyhow::anyhow!("Live order book data not implemented yet. Use mock data for development."))
        })
    }
    
    /// Handle WebSocket message updates
    pub fn handle_websocket_message(&mut self, message: WebSocketMessage, cx: &mut Context<Self>) -> Result<()> {
        match message.message_type {
            MessageType::Quote => {
                if let Some(_symbol) = &message.symbol {
                    let quote_update: QuoteUpdate = serde_json::from_value(message.data)?;
                    self.process_quote_update(quote_update, cx)?;
                }
            }
            MessageType::Trade => {
                if let Some(_symbol) = &message.symbol {
                    let trade_update: TradeUpdate = serde_json::from_value(message.data)?;
                    self.process_trade_update(trade_update, cx)?;
                }
            }
            MessageType::OrderBook => {
                if let Some(_symbol) = &message.symbol {
                    let order_book_update: OrderBookUpdate = serde_json::from_value(message.data)?;
                    self.process_order_book_update(order_book_update, cx)?;
                }
            }
            _ => {} // Handle other message types as needed
        }
        
        Ok(())
    }
    
    /// Process quote update from WebSocket
    fn process_quote_update(&mut self, quote: QuoteUpdate, cx: &mut Context<Self>) -> Result<()> {
        // Convert quote update to market data
        let market_data = MarketData {
            symbol: quote.symbol.clone(),
            current_price: quote.last_price,
            change: quote.change,
            change_percent: quote.change_percent,
            volume: quote.volume,
            market_cap: None, // Not provided in quote update
            high_52w: None,
            low_52w: None,
            timestamp: quote.timestamp,
            market_status: MarketStatus::Open, // Assume open if receiving updates
            previous_close: quote.open,
            day_high: quote.high,
            day_low: quote.low,
            average_volume: None,
            bid: Some(quote.bid),
            ask: Some(quote.ask),
            bid_size: Some(quote.bid_size),
            ask_size: Some(quote.ask_size),
        };
        
        self.cache_market_data(market_data, DataSource::WebSocket, cx)?;
        Ok(())
    }
    
    /// Process trade update from WebSocket
    fn process_trade_update(&mut self, trade: TradeUpdate, cx: &mut Context<Self>) -> Result<()> {
        // Update last trade price in cached market data if available
        if let Some(cached) = self.cache.get_mut(&trade.symbol) {
            cached.data.current_price = trade.price;
            cached.data.volume += trade.size;
            cached.data.timestamp = trade.timestamp;
            cached.cached_at = Instant::now();
            cached.source = DataSource::WebSocket;
            
            cx.emit(DataEvent::TradeReceived(trade));
        }
        
        Ok(())
    }
    
    /// Process order book update from WebSocket
    fn process_order_book_update(&mut self, update: OrderBookUpdate, cx: &mut Context<Self>) -> Result<()> {
        // Create or update order book
        let mut order_book = self.order_book_cache
            .get(&update.symbol)
            .cloned()
            .unwrap_or_else(|| OrderBook::new(update.symbol.clone()).unwrap_or_else(|_| {
                // Fallback to empty order book if creation fails
                OrderBook {
                    symbol: update.symbol.clone(),
                    bids: Vec::new(),
                    asks: Vec::new(),
                    timestamp: SystemTime::now(),
                    spread: 0.0,
                    spread_percent: 0.0,
                    sequence_number: 0,
                }
            }));
        
        if update.is_snapshot {
            // Full snapshot - replace all data
            order_book.bids = update.bids;
            order_book.asks = update.asks;
        } else {
            // Incremental update - merge with existing data
            // This is a simplified implementation
            order_book.bids.extend(update.bids);
            order_book.asks.extend(update.asks);
            
            // Sort and limit to top levels
            order_book.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal));
            order_book.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal));
            
            order_book.bids.truncate(20); // Keep top 20 levels
            order_book.asks.truncate(20);
        }
        
        order_book.timestamp = update.timestamp;
        order_book.sequence_number = update.sequence;
        order_book.calculate_spread();
        
        self.order_book_cache.insert(update.symbol.clone(), order_book.clone());
        cx.emit(DataEvent::OrderBookUpdated(update.symbol, order_book));
        
        Ok(())
    }
    
    /// Start background cleanup task
    fn start_cleanup_task(&mut self, cx: &mut Context<Self>) {
        let cleanup_interval = Duration::from_secs(300); // 5 minutes
        
        let task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(cleanup_interval).await;
                
                if let Err(error) = this.update(cx, |this, cx| {
                    this.cleanup_stale_data(cx)
                }) {
                    error.log_err(); // Proper error handling
                }
            }
        });
        
        self._cleanup_task = Some(task);
    }
    
    /// Start background refresh task for subscribed symbols
    fn start_refresh_task(&mut self, cx: &mut Context<Self>) {
        if !self.auto_refresh_enabled {
            return;
        }
        
        let refresh_interval = self.refresh_interval;
        
        let task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(refresh_interval).await;
                
                if let Err(error) = this.update(cx, |this, cx| {
                    this.refresh_subscribed_symbols(cx)
                }) {
                    error.log_err(); // Proper error handling
                }
            }
        });
        
        self._refresh_task = Some(task);
    }
    
    /// Refresh data for all subscribed symbols
    fn refresh_subscribed_symbols(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let symbols: Vec<String> = self.subscribed_symbols.iter().cloned().collect();
        
        for symbol in symbols {
            // Trigger refresh by requesting data (will fetch if cache is stale)
            let _task = self.get_market_data(&symbol, cx);
        }
        
        Ok(())
    }
    
    /// Enhanced cleanup with memory management
    pub fn cleanup_stale_data(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let cutoff = Instant::now() - self.cache_duration;
        let max_cache_size = 1000; // Maximum number of cached items
        
        // Remove stale entries
        self.cache.retain(|_, cached| cached.cached_at > cutoff);
        
        // If cache is still too large, remove least recently used items
        if self.cache.len() > max_cache_size {
            let mut entries: Vec<_> = self.cache.iter().map(|(k, v)| (k.clone(), v.last_accessed)).collect();
            entries.sort_by_key(|(_, last_accessed)| *last_accessed);
            
            let to_remove = entries.len() - max_cache_size;
            let symbols_to_remove: Vec<String> = entries.iter().take(to_remove).map(|(symbol, _)| symbol.clone()).collect();
            
            for symbol in symbols_to_remove {
                self.cache.remove(&symbol);
            }
        }
        
        // Clean up historical cache (keep only recent data)
        for symbol_cache in self.historical_cache.values_mut() {
            for candles in symbol_cache.values_mut() {
                candles.truncate(1000); // Keep last 1000 candles per timeframe
            }
        }
        
        // Clean up order book cache (remove old entries)
        let order_book_cutoff = Duration::from_secs(60); // 1 minute for order books
        self.order_book_cache.retain(|_, order_book| {
            order_book.timestamp.elapsed().unwrap_or(Duration::MAX) < order_book_cutoff
        });
        
        cx.emit(DataEvent::CacheCleanupCompleted);
        Ok(())
    }
    
    /// Get cached market data with bounds checking (.rules compliance)
    pub fn get_cached_data(&self, symbol: &str) -> Option<&MarketData> {
        self.cache.get(symbol).map(|cached| &cached.data)
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let total_entries = self.cache.len();
        let total_access_count: u32 = self.cache.values().map(|cached| cached.access_count).sum();
        let websocket_entries = self.cache.values().filter(|cached| cached.source == DataSource::WebSocket).count();
        let mock_entries = self.cache.values().filter(|cached| cached.source == DataSource::Mock).count();
        let http_entries = self.cache.values().filter(|cached| cached.source == DataSource::Http).count();
        
        CacheStats {
            total_entries,
            total_access_count,
            websocket_entries,
            mock_entries,
            http_entries,
            historical_symbols: self.historical_cache.len(),
            order_book_entries: self.order_book_cache.len(),
            subscribed_symbols: self.subscribed_symbols.len(),
        }
    }
    
    /// Validate market data with enhanced checks
    fn validate_market_data(&self, data: MarketData) -> Result<MarketData> {
        if data.symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if data.current_price < 0.0 {
            return Err(anyhow::anyhow!("Price cannot be negative"));
        }
        
        // Additional validation for bid/ask spread
        if let (Some(bid), Some(ask)) = (data.bid, data.ask) {
            if bid >= ask {
                return Err(anyhow::anyhow!("Bid price must be less than ask price"));
            }
        }
        
        // Validate day high/low
        if data.day_high < data.day_low {
            return Err(anyhow::anyhow!("Day high cannot be less than day low"));
        }
        
        Ok(data)
    }
    
    /// Set cache duration
    pub fn set_cache_duration(&mut self, duration: Duration) {
        self.cache_duration = duration;
    }
    
    /// Enable/disable auto refresh
    pub fn set_auto_refresh(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.auto_refresh_enabled = enabled;
        
        if enabled {
            self.start_refresh_task(cx);
        } else {
            self._refresh_task = None;
        }
    }
    
    /// Set refresh interval
    pub fn set_refresh_interval(&mut self, interval: Duration, cx: &mut Context<Self>) {
        self.refresh_interval = interval;
        
        // Restart refresh task with new interval
        if self.auto_refresh_enabled {
            self.start_refresh_task(cx);
        }
    }
}

/// Cache statistics structure
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_access_count: u32,
    pub websocket_entries: usize,
    pub mock_entries: usize,
    pub http_entries: usize,
    pub historical_symbols: usize,
    pub order_book_entries: usize,
    pub subscribed_symbols: usize,
}

impl EventEmitter<DataEvent> for DataService {}

impl Render for DataService {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Data service doesn't render UI directly
    }
}

/// Enhanced data service events
#[derive(Clone, Debug)]
pub enum DataEvent {
    MarketDataReceived(MarketData),
    HistoricalDataReceived(String, Vec<Candle>),
    OrderBookUpdated(String, OrderBook),
    TradeReceived(TradeUpdate),
    ConnectionStatusChanged(bool),
    CacheUpdated(String),
    CacheCleanupCompleted,
    SymbolSubscribed(String),
    SymbolUnsubscribed(String),
    DataSourceChanged(String),
    RefreshCompleted,
    ErrorOccurred(String),
}