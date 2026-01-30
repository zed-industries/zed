use anyhow::Result;
use gpui::{App, Context, Entity, EventEmitter, Render, Subscription, Task};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::market_data::*;
use crate::websocket_service::*;

/// Mock data service for development and testing
pub struct MockDataService {
    stock_data: HashMap<String, MockStockData>,
    simulation_active: bool,
    update_interval: Duration,
    price_volatility: f64,
    _simulation_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

/// Internal mock stock data structure
#[derive(Debug, Clone)]
struct MockStockData {
    symbol: String,
    name: String,
    base_price: f64,
    current_price: f64,
    volume: u64,
    market_cap: u64,
    sector: String,
    last_update: SystemTime,
    price_history: Vec<f64>,
    trend_direction: f64, // -1.0 to 1.0
    volatility: f64,
}

impl MockStockData {
    /// Create new mock stock data with realistic values
    fn new(symbol: String, name: String, base_price: f64, market_cap: u64, sector: String) -> Self {
        Self {
            symbol,
            name,
            base_price,
            current_price: base_price,
            volume: thread_rng().gen_range(1_000_000..10_000_000),
            market_cap,
            sector,
            last_update: SystemTime::now(),
            price_history: vec![base_price],
            trend_direction: 0.0,
            volatility: thread_rng().gen_range(0.01..0.05), // 1-5% volatility
        }
    }
    
    /// Generate next price using random walk with mean reversion
    fn generate_next_price(&mut self) -> f64 {
        let mut rng = thread_rng();
        
        // Random walk component
        let random_change = rng.gen_range(-1.0..1.0) * self.volatility;
        
        // Mean reversion component (pull towards base price)
        let mean_reversion = (self.base_price - self.current_price) * 0.001;
        
        // Trend component
        let trend_change = self.trend_direction * 0.0001;
        
        // Update trend direction occasionally
        if rng.gen_bool(0.1) {
            self.trend_direction = rng.gen_range(-1.0..1.0);
        }
        
        // Calculate new price
        let price_change = random_change + mean_reversion + trend_change;
        let new_price = self.current_price * (1.0 + price_change);
        
        // Ensure price stays positive and reasonable
        let new_price = new_price.max(self.base_price * 0.1).min(self.base_price * 10.0);
        
        self.current_price = new_price;
        self.price_history.push(new_price);
        
        // Keep history manageable
        if self.price_history.len() > 1000 {
            self.price_history.remove(0);
        }
        
        self.last_update = SystemTime::now();
        new_price
    }
    
    /// Generate realistic volume
    fn generate_volume(&mut self) -> u64 {
        let mut rng = thread_rng();
        let base_volume = self.volume;
        let volume_change = rng.gen_range(0.5..2.0);
        (base_volume as f64 * volume_change) as u64
    }
    
    /// Convert to MarketData structure
    fn to_market_data(&self) -> Result<MarketData> {
        let previous_price = self.price_history.get(self.price_history.len().saturating_sub(2))
            .copied()
            .unwrap_or(self.base_price);
        
        let change = self.current_price - previous_price;
        let change_percent = if previous_price > 0.0 {
            (change / previous_price) * 100.0
        } else {
            0.0
        };
        
        // Calculate day high/low from recent history
        let recent_history = &self.price_history[self.price_history.len().saturating_sub(100)..];
        let day_high = recent_history.iter().fold(0.0f64, |a, &b| a.max(b));
        let day_low = recent_history.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        Ok(MarketData {
            symbol: self.symbol.clone(),
            current_price: self.current_price,
            change,
            change_percent,
            volume: self.volume,
            market_cap: Some(self.market_cap),
            high_52w: Some(self.base_price * 1.5),
            low_52w: Some(self.base_price * 0.5),
            timestamp: self.last_update,
            market_status: MockDataService::get_current_market_status(),
            previous_close: previous_price,
            day_high,
            day_low,
            average_volume: Some(self.volume),
            bid: Some(self.current_price * 0.999),
            ask: Some(self.current_price * 1.001),
            bid_size: Some(thread_rng().gen_range(100..1000)),
            ask_size: Some(thread_rng().gen_range(100..1000)),
        })
    }
}

impl MockDataService {
    /// Create new mock data service with realistic stock data
    pub fn new(cx: &mut App) -> Entity<Self> {
        let mut service = Self {
            stock_data: HashMap::new(),
            simulation_active: false,
            update_interval: Duration::from_millis(1000), // 1 second updates
            price_volatility: 0.02, // 2% volatility
            _simulation_task: None,
            _subscriptions: Vec::new(),
        };
        
        // Initialize with realistic stock data
        service.initialize_stock_data();
        
        cx.new_entity(service)
    }
    
    /// Initialize realistic stock data for major stocks
    fn initialize_stock_data(&mut self) {
        let stocks = vec![
            ("AAPL", "Apple Inc.", 175.0, 2_800_000_000_000, "Technology"),
            ("GOOGL", "Alphabet Inc.", 140.0, 1_800_000_000_000, "Technology"),
            ("MSFT", "Microsoft Corporation", 380.0, 2_900_000_000_000, "Technology"),
            ("TSLA", "Tesla Inc.", 250.0, 800_000_000_000, "Automotive"),
            ("AMZN", "Amazon.com Inc.", 150.0, 1_500_000_000_000, "Consumer Discretionary"),
            ("NVDA", "NVIDIA Corporation", 500.0, 1_200_000_000_000, "Technology"),
            ("META", "Meta Platforms Inc.", 320.0, 800_000_000_000, "Technology"),
        ];
        
        for (symbol, name, price, market_cap, sector) in stocks {
            let mock_data = MockStockData::new(
                symbol.to_string(),
                name.to_string(),
                price,
                market_cap,
                sector.to_string(),
            );
            self.stock_data.insert(symbol.to_string(), mock_data);
        }
    }
    
    /// Start price simulation with proper error handling (.rules compliance)
    pub fn start_simulation(&mut self, cx: &mut Context<Self>) {
        if self.simulation_active {
            return;
        }
        
        self.simulation_active = true;
        let update_interval = self.update_interval;
        
        let task = cx.spawn(|this, mut cx| async move {
            loop {
                cx.background_executor().timer(update_interval).await;
                
                if let Err(error) = this.update(&mut cx, |this, cx| {
                    this.update_all_prices(cx)
                }) {
                    error.log_err(); // Proper error handling
                    break;
                }
            }
        });
        
        self._simulation_task = Some(task);
        cx.emit(MockDataEvent::SimulationStarted);
    }
    
    /// Stop price simulation
    pub fn stop_simulation(&mut self, cx: &mut Context<Self>) {
        self.simulation_active = false;
        self._simulation_task = None;
        cx.emit(MockDataEvent::SimulationStopped);
    }
    
    /// Update all stock prices with proper error handling (.rules compliance)
    fn update_all_prices(&mut self, cx: &mut Context<Self>) -> Result<()> {
        for (symbol, stock_data) in &mut self.stock_data {
            stock_data.generate_next_price();
            stock_data.volume = stock_data.generate_volume();
            
            // Emit market data update event
            let market_data = stock_data.to_market_data()?;
            cx.emit(MockDataEvent::MarketDataUpdated(symbol.clone(), market_data));
        }
        
        Ok(())
    }
    
    /// Get market data for symbol with bounds checking (.rules compliance)
    pub fn get_market_data(&self, symbol: &str) -> Option<MarketData> {
        self.stock_data.get(symbol)
            .and_then(|data| data.to_market_data().ok())
    }
    
    /// Get all available symbols
    pub fn get_available_symbols(&self) -> Vec<String> {
        self.stock_data.keys().cloned().collect()
    }
    
    /// Generate historical data for timeframe
    pub fn generate_historical_data(
        &self,
        symbol: &str,
        timeframe: TimeFrame,
        periods: usize,
    ) -> Result<Vec<Candle>> {
        let stock_data = self.stock_data.get(symbol)
            .ok_or_else(|| anyhow::anyhow!("Symbol not found: {}", symbol))?;
        
        let mut candles = Vec::new();
        let mut rng = thread_rng();
        let interval_seconds = timeframe.to_seconds();
        let mut current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .saturating_sub(interval_seconds * periods as u64);
        
        let mut current_price = stock_data.base_price;
        
        for _ in 0..periods {
            let timestamp = UNIX_EPOCH + Duration::from_secs(current_time);
            
            // Generate OHLC data
            let open = current_price;
            let volatility = stock_data.volatility;
            
            // Generate high and low within reasonable bounds
            let high_change = rng.gen_range(0.0..volatility);
            let low_change = rng.gen_range(-volatility..0.0);
            
            let high = open * (1.0 + high_change);
            let low = open * (1.0 + low_change);
            
            // Generate close price
            let close_change = rng.gen_range(-volatility..volatility);
            let close = open * (1.0 + close_change);
            let close = close.max(low).min(high); // Ensure close is within high/low
            
            // Generate volume
            let volume = rng.gen_range(100_000..1_000_000);
            
            let candle = Candle::new(timestamp, open, high, low, close, volume)?;
            candles.push(candle);
            
            current_price = close;
            current_time += interval_seconds;
        }
        
        Ok(candles)
    }
    
    /// Generate mock order book data
    pub fn generate_order_book(&self, symbol: &str) -> Result<OrderBook> {
        let stock_data = self.stock_data.get(symbol)
            .ok_or_else(|| anyhow::anyhow!("Symbol not found: {}", symbol))?;
        
        let mut order_book = OrderBook::new(symbol.to_string())?;
        let mut rng = thread_rng();
        
        let current_price = stock_data.current_price;
        let spread_percent = 0.001; // 0.1% spread
        
        // Generate bid entries (below current price)
        for i in 0..10 {
            let price_offset = (i + 1) as f64 * spread_percent;
            let price = current_price * (1.0 - price_offset);
            let quantity = rng.gen_range(100..1000);
            
            let entry = OrderBookEntry::new(price, quantity, OrderSide::Buy)?;
            order_book.bids.push(entry);
        }
        
        // Generate ask entries (above current price)
        for i in 0..10 {
            let price_offset = (i + 1) as f64 * spread_percent;
            let price = current_price * (1.0 + price_offset);
            let quantity = rng.gen_range(100..1000);
            
            let entry = OrderBookEntry::new(price, quantity, OrderSide::Sell)?;
            order_book.asks.push(entry);
        }
        
        order_book.calculate_spread();
        order_book.timestamp = SystemTime::now();
        
        Ok(order_book)
    }
    
    /// Generate mock portfolio data
    pub fn generate_mock_portfolio(&self, account_id: String) -> Result<Portfolio> {
        let mut portfolio = Portfolio::new(account_id, 100_000.0)?; // $100k cash
        let mut rng = thread_rng();
        
        // Add some random positions
        let symbols: Vec<_> = self.stock_data.keys().take(3).collect();
        for symbol in symbols {
            if let Some(stock_data) = self.stock_data.get(*symbol) {
                let quantity = rng.gen_range(10..100) as i64;
                let average_cost = stock_data.base_price * rng.gen_range(0.9..1.1);
                
                let position = Position::new(symbol.clone(), quantity, average_cost)?;
                portfolio.positions.push(position);
            }
        }
        
        // Calculate portfolio values
        let market_data: HashMap<String, MarketData> = self.stock_data
            .iter()
            .filter_map(|(symbol, data)| {
                data.to_market_data().ok().map(|md| (symbol.clone(), md))
            })
            .collect();
        
        portfolio.calculate_total_value(&market_data);
        
        Ok(portfolio)
    }
    
    /// Get current market status based on time
    fn get_current_market_status() -> MarketStatus {
        // Simplified market status - in reality this would check actual market hours
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let hours = (duration_since_epoch.as_secs() / 3600) % 24;
        
        match hours {
            0..=8 => MarketStatus::Closed,
            9..=15 => MarketStatus::Open,
            16..=20 => MarketStatus::AfterHours,
            _ => MarketStatus::Closed,
        }
    }
    
    /// Set update interval for simulation
    pub fn set_update_interval(&mut self, interval: Duration) {
        self.update_interval = interval;
    }
    
    /// Set price volatility for simulation
    pub fn set_price_volatility(&mut self, volatility: f64) {
        self.price_volatility = volatility.max(0.001).min(0.1); // Clamp between 0.1% and 10%
        
        // Update volatility for all stocks
        for stock_data in self.stock_data.values_mut() {
            stock_data.volatility = self.price_volatility;
        }
    }
    
    /// Check if simulation is active
    pub fn is_simulation_active(&self) -> bool {
        self.simulation_active
    }
}

impl EventEmitter<MockDataEvent> for MockDataService {}

impl Render for MockDataService {
    fn render(&mut self, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Mock data service doesn't render UI directly
    }
}

/// Mock WebSocket service for real-time data simulation
pub struct MockWebSocketService {
    subscriptions: HashMap<String, WebSocketSubscription>,
    simulation_active: bool,
    update_interval: Duration,
    price_volatility: f64,
    connection_state: ConnectionState,
    mock_data_service: Option<Entity<MockDataService>>,
    _simulation_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl MockWebSocketService {
    /// Create new mock WebSocket service
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new_entity(Self {
            subscriptions: HashMap::new(),
            simulation_active: false,
            update_interval: Duration::from_millis(500), // 500ms updates
            price_volatility: 0.02,
            connection_state: ConnectionState::Disconnected,
            mock_data_service: None,
            _simulation_task: None,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Set mock data service reference
    pub fn set_mock_data_service(&mut self, service: Entity<MockDataService>) {
        self.mock_data_service = Some(service);
    }
    
    /// Simulate WebSocket connection
    pub fn simulate_connect(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.connection_state = ConnectionState::Connecting;
        
        cx.spawn(|this, mut cx| async move {
            // Simulate connection delay
            cx.background_executor().timer(Duration::from_millis(100)).await;
            
            this.update(&mut cx, |this, cx| {
                this.connection_state = ConnectionState::Connected;
                cx.emit(WebSocketEvent::Connected);
                this.start_simulation(cx);
            })?;
            
            Ok(())
        })
    }
    
    /// Simulate WebSocket disconnection
    pub fn simulate_disconnect(&mut self, cx: &mut Context<Self>) {
        self.connection_state = ConnectionState::Disconnected;
        self.stop_simulation();
        cx.emit(WebSocketEvent::Disconnected);
    }
    
    /// Subscribe to symbol for mock updates
    pub fn subscribe_to_symbol(
        &mut self,
        symbol: String,
        message_types: Vec<MessageType>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let subscription = WebSocketSubscription::new(symbol.clone(), message_types)?;
        self.subscriptions.insert(symbol.clone(), subscription);
        cx.emit(WebSocketEvent::SubscriptionAdded(symbol));
        Ok(())
    }
    
    /// Unsubscribe from symbol
    pub fn unsubscribe_from_symbol(&mut self, symbol: &str, cx: &mut Context<Self>) -> Result<()> {
        if self.subscriptions.remove(symbol).is_some() {
            cx.emit(WebSocketEvent::SubscriptionRemoved(symbol.to_string()));
        }
        Ok(())
    }
    
    /// Start real-time data simulation
    pub fn start_simulation(&mut self, cx: &mut Context<Self>) {
        if self.simulation_active {
            return;
        }
        
        self.simulation_active = true;
        let update_interval = self.update_interval;
        
        let task = cx.spawn(|this, mut cx| async move {
            loop {
                cx.background_executor().timer(update_interval).await;
                
                if let Err(error) = this.update(&mut cx, |this, cx| {
                    this.generate_mock_updates(cx)
                }) {
                    error.log_err(); // Proper error handling
                    break;
                }
            }
        });
        
        self._simulation_task = Some(task);
    }
    
    /// Stop simulation
    pub fn stop_simulation(&mut self) {
        self.simulation_active = false;
        self._simulation_task = None;
    }
    
    /// Generate mock WebSocket updates
    fn generate_mock_updates(&mut self, cx: &mut Context<Self>) -> Result<()> {
        if !matches!(self.connection_state, ConnectionState::Connected) {
            return Ok(());
        }
        
        for (symbol, subscription) in &self.subscriptions {
            // Generate different types of updates based on subscription
            for message_type in &subscription.message_types {
                match message_type {
                    MessageType::Quote => {
                        let quote_update = self.generate_mock_quote_update(symbol)?;
                        let message = WebSocketMessage::new(
                            MessageType::Quote,
                            Some(symbol.clone()),
                            serde_json::to_value(quote_update)?,
                        )?;
                        cx.emit(WebSocketEvent::MessageReceived(message));
                    }
                    MessageType::Trade => {
                        if thread_rng().gen_bool(0.3) { // 30% chance of trade update
                            let trade_update = self.generate_mock_trade_update(symbol)?;
                            let message = WebSocketMessage::new(
                                MessageType::Trade,
                                Some(symbol.clone()),
                                serde_json::to_value(trade_update)?,
                            )?;
                            cx.emit(WebSocketEvent::MessageReceived(message));
                        }
                    }
                    MessageType::OrderBook => {
                        if thread_rng().gen_bool(0.2) { // 20% chance of order book update
                            let order_book_update = self.generate_mock_order_book_update(symbol)?;
                            let message = WebSocketMessage::new(
                                MessageType::OrderBook,
                                Some(symbol.clone()),
                                serde_json::to_value(order_book_update)?,
                            )?;
                            cx.emit(WebSocketEvent::MessageReceived(message));
                        }
                    }
                    _ => {} // Handle other message types as needed
                }
            }
        }
        
        Ok(())
    }
    
    /// Generate mock quote update
    fn generate_mock_quote_update(&self, symbol: &str) -> Result<QuoteUpdate> {
        let mut rng = thread_rng();
        
        // Use realistic base prices for known symbols
        let base_price = match symbol {
            "AAPL" => 175.0,
            "GOOGL" => 140.0,
            "MSFT" => 380.0,
            "TSLA" => 250.0,
            "AMZN" => 150.0,
            "NVDA" => 500.0,
            "META" => 320.0,
            _ => 100.0,
        };
        
        let price_variation = base_price * self.price_volatility * rng.gen_range(-1.0..1.0);
        let current_price = base_price + price_variation;
        
        let spread = current_price * 0.001; // 0.1% spread
        let bid = current_price - spread / 2.0;
        let ask = current_price + spread / 2.0;
        
        let mut quote = QuoteUpdate::new(symbol.to_string(), bid, ask, current_price)?;
        
        quote.bid_size = rng.gen_range(100..1000);
        quote.ask_size = rng.gen_range(100..1000);
        quote.last_size = rng.gen_range(10..100);
        quote.volume = rng.gen_range(1_000_000..10_000_000);
        quote.change = price_variation;
        quote.change_percent = (price_variation / base_price) * 100.0;
        quote.high = current_price * 1.02;
        quote.low = current_price * 0.98;
        quote.open = base_price;
        
        Ok(quote)
    }
    
    /// Generate mock trade update
    fn generate_mock_trade_update(&self, symbol: &str) -> Result<TradeUpdate> {
        let mut rng = thread_rng();
        
        let base_price = match symbol {
            "AAPL" => 175.0,
            "GOOGL" => 140.0,
            "MSFT" => 380.0,
            "TSLA" => 250.0,
            "AMZN" => 150.0,
            "NVDA" => 500.0,
            "META" => 320.0,
            _ => 100.0,
        };
        
        let price_variation = base_price * self.price_volatility * rng.gen_range(-0.5..0.5);
        let trade_price = base_price + price_variation;
        let trade_size = rng.gen_range(10..1000);
        let side = if rng.gen_bool(0.5) { OrderSide::Buy } else { OrderSide::Sell };
        let trade_id = format!("{}_{}", symbol, SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis());
        
        TradeUpdate::new(symbol.to_string(), trade_price, trade_size, side, trade_id)
    }
    
    /// Generate mock order book update
    fn generate_mock_order_book_update(&self, symbol: &str) -> Result<OrderBookUpdate> {
        let mut rng = thread_rng();
        let sequence = rng.gen_range(1000..9999);
        
        let mut update = OrderBookUpdate::new(symbol.to_string(), sequence, false)?;
        
        let base_price = match symbol {
            "AAPL" => 175.0,
            "GOOGL" => 140.0,
            "MSFT" => 380.0,
            "TSLA" => 250.0,
            "AMZN" => 150.0,
            "NVDA" => 500.0,
            "META" => 320.0,
            _ => 100.0,
        };
        
        // Add a few bid/ask updates
        for i in 0..3 {
            let price_offset = (i + 1) as f64 * 0.01;
            let bid_price = base_price * (1.0 - price_offset);
            let ask_price = base_price * (1.0 + price_offset);
            let quantity = rng.gen_range(100..1000);
            
            update.add_bid(bid_price, quantity)?;
            update.add_ask(ask_price, quantity)?;
        }
        
        update.sort_entries();
        
        Ok(update)
    }
    
    /// Get connection state
    pub fn get_connection_state(&self) -> &ConnectionState {
        &self.connection_state
    }
    
    /// Set update interval
    pub fn set_update_interval(&mut self, interval: Duration) {
        self.update_interval = interval;
    }
    
    /// Set price volatility
    pub fn set_price_volatility(&mut self, volatility: f64) {
        self.price_volatility = volatility.max(0.001).min(0.1);
    }
}

impl EventEmitter<WebSocketEvent> for MockWebSocketService {}

impl Render for MockWebSocketService {
    fn render(&mut self, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div() // Mock WebSocket service doesn't render UI directly
    }
}

/// Mock data service events
#[derive(Clone, Debug)]
pub enum MockDataEvent {
    SimulationStarted,
    SimulationStopped,
    MarketDataUpdated(String, MarketData),
    HistoricalDataGenerated(String, Vec<Candle>),
    OrderBookGenerated(String, OrderBook),
    PortfolioGenerated(Portfolio),
}