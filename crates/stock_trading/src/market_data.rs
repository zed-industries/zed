use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Enhanced market data structure with comprehensive information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub current_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub market_cap: Option<u64>,
    pub high_52w: Option<f64>,
    pub low_52w: Option<f64>,
    pub timestamp: SystemTime,
    pub market_status: MarketStatus,
    pub previous_close: f64,
    pub day_high: f64,
    pub day_low: f64,
    pub average_volume: Option<u64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub bid_size: Option<u64>,
    pub ask_size: Option<u64>,
}

impl MarketData {
    /// Create new market data with validation (.rules compliance)
    pub fn new(symbol: String, current_price: f64) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if current_price < 0.0 {
            return Err(anyhow::anyhow!("Price cannot be negative"));
        }
        
        Ok(Self {
            symbol,
            current_price,
            change: 0.0,
            change_percent: 0.0,
            volume: 0,
            market_cap: None,
            high_52w: None,
            low_52w: None,
            timestamp: SystemTime::now(),
            market_status: MarketStatus::Closed,
            previous_close: current_price,
            day_high: current_price,
            day_low: current_price,
            average_volume: None,
            bid: None,
            ask: None,
            bid_size: None,
            ask_size: None,
        })
    }
    
    /// Calculate spread with bounds checking (.rules compliance)
    pub fn get_spread(&self) -> Option<f64> {
        match (self.bid, self.ask) {
            (Some(bid), Some(ask)) if ask > bid => Some(ask - bid),
            _ => None,
        }
    }
    
    /// Calculate spread percentage with proper error handling
    pub fn get_spread_percent(&self) -> Option<f64> {
        match (self.bid, self.ask) {
            (Some(bid), Some(ask)) if ask > bid && bid > 0.0 => {
                Some(((ask - bid) / bid) * 100.0)
            }
            _ => None,
        }
    }
}

/// Market status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketStatus {
    PreMarket,
    Open,
    Closed,
    AfterHours,
    Holiday,
}

/// Order book structure with bid/ask spreads and order counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookEntry>, // Sorted by price descending
    pub asks: Vec<OrderBookEntry>, // Sorted by price ascending
    pub timestamp: SystemTime,
    pub spread: f64,
    pub spread_percent: f64,
    pub sequence_number: u64, // For ordering updates
}

impl OrderBook {
    /// Create new order book with validation
    pub fn new(symbol: String) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        Ok(Self {
            symbol,
            bids: Vec::new(),
            asks: Vec::new(),
            timestamp: SystemTime::now(),
            spread: 0.0,
            spread_percent: 0.0,
            sequence_number: 0,
        })
    }
    
    /// Get best bid price with bounds checking (.rules compliance)
    pub fn get_best_bid(&self) -> Option<f64> {
        self.bids.first().map(|entry| entry.price)
    }
    
    /// Get best ask price with bounds checking (.rules compliance)
    pub fn get_best_ask(&self) -> Option<f64> {
        self.asks.first().map(|entry| entry.price)
    }
    
    /// Calculate current spread
    pub fn calculate_spread(&mut self) {
        match (self.get_best_bid(), self.get_best_ask()) {
            (Some(bid), Some(ask)) if ask > bid => {
                self.spread = ask - bid;
                self.spread_percent = if bid > 0.0 {
                    ((ask - bid) / bid) * 100.0
                } else {
                    0.0
                };
            }
            _ => {
                self.spread = 0.0;
                self.spread_percent = 0.0;
            }
        }
    }
}

/// Order book entry with price, quantity, and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: u64,
    pub side: OrderSide,
    pub order_count: u32, // Number of orders at this price level
    pub timestamp: SystemTime,
}

impl OrderBookEntry {
    /// Create new order book entry with validation
    pub fn new(price: f64, quantity: u64, side: OrderSide) -> Result<Self> {
        if price <= 0.0 {
            return Err(anyhow::anyhow!("Price must be positive"));
        }
        
        if quantity == 0 {
            return Err(anyhow::anyhow!("Quantity must be greater than zero"));
        }
        
        Ok(Self {
            price,
            quantity,
            side,
            order_count: 1,
            timestamp: SystemTime::now(),
        })
    }
}

/// Portfolio structure for account management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub account_id: String,
    pub total_value: f64,
    pub cash_balance: f64,
    pub positions: Vec<Position>,
    pub day_change: f64,
    pub day_change_percent: f64,
    pub total_return: f64,
    pub total_return_percent: f64,
    pub buying_power: f64,
    pub margin_used: f64,
    pub last_updated: SystemTime,
}

impl Portfolio {
    /// Create new portfolio with validation
    pub fn new(account_id: String, cash_balance: f64) -> Result<Self> {
        if account_id.is_empty() {
            return Err(anyhow::anyhow!("Account ID cannot be empty"));
        }
        
        if cash_balance < 0.0 {
            return Err(anyhow::anyhow!("Cash balance cannot be negative"));
        }
        
        Ok(Self {
            account_id,
            total_value: cash_balance,
            cash_balance,
            positions: Vec::new(),
            day_change: 0.0,
            day_change_percent: 0.0,
            total_return: 0.0,
            total_return_percent: 0.0,
            buying_power: cash_balance,
            margin_used: 0.0,
            last_updated: SystemTime::now(),
        })
    }
    
    /// Calculate total portfolio value with bounds checking (.rules compliance)
    pub fn calculate_total_value(&mut self, market_data: &HashMap<String, MarketData>) {
        let mut total_position_value = 0.0;
        
        for position in &mut self.positions {
            if let Some(data) = market_data.get(&position.symbol) {
                position.current_price = data.current_price;
                position.market_value = position.quantity as f64 * data.current_price;
                position.unrealized_pnl = position.market_value - (position.quantity as f64 * position.average_cost);
                position.unrealized_pnl_percent = if position.average_cost > 0.0 {
                    (position.unrealized_pnl / (position.quantity as f64 * position.average_cost)) * 100.0
                } else {
                    0.0
                };
                total_position_value += position.market_value;
            }
        }
        
        self.total_value = self.cash_balance + total_position_value;
        self.last_updated = SystemTime::now();
    }
}

/// Position structure for individual stock holdings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: i64, // Negative for short positions
    pub average_cost: f64,
    pub current_price: f64,
    pub market_value: f64,
    pub unrealized_pnl: f64,
    pub unrealized_pnl_percent: f64,
    pub day_change: f64,
    pub day_change_percent: f64,
    pub cost_basis: f64,
    pub last_updated: SystemTime,
}

impl Position {
    /// Create new position with validation
    pub fn new(symbol: String, quantity: i64, average_cost: f64) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if quantity == 0 {
            return Err(anyhow::anyhow!("Quantity cannot be zero"));
        }
        
        if average_cost <= 0.0 {
            return Err(anyhow::anyhow!("Average cost must be positive"));
        }
        
        let cost_basis = (quantity.abs() as f64) * average_cost;
        
        Ok(Self {
            symbol,
            quantity,
            average_cost,
            current_price: average_cost,
            market_value: cost_basis,
            unrealized_pnl: 0.0,
            unrealized_pnl_percent: 0.0,
            day_change: 0.0,
            day_change_percent: 0.0,
            cost_basis,
            last_updated: SystemTime::now(),
        })
    }
}

/// Stock information structure for fundamental data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<u64>,
    pub pe_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub beta: Option<f64>,
    pub eps: Option<f64>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub employees: Option<u64>,
    pub headquarters: Option<String>,
    pub founded_year: Option<u32>,
}

impl StockInfo {
    /// Create new stock info with validation
    pub fn new(symbol: String, name: String, exchange: String) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if name.is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }
        
        if exchange.is_empty() {
            return Err(anyhow::anyhow!("Exchange cannot be empty"));
        }
        
        Ok(Self {
            symbol,
            name,
            exchange,
            sector: None,
            industry: None,
            market_cap: None,
            pe_ratio: None,
            dividend_yield: None,
            beta: None,
            eps: None,
            description: None,
            website: None,
            employees: None,
            headquarters: None,
            founded_year: None,
        })
    }
}

/// Trade structure for execution records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: u64,
    pub price: f64,
    pub commission: f64,
    pub timestamp: SystemTime,
    pub execution_venue: Option<String>,
    pub trade_type: TradeType,
}

impl Trade {
    /// Create new trade with validation
    pub fn new(
        id: String,
        order_id: String,
        symbol: String,
        side: OrderSide,
        quantity: u64,
        price: f64,
    ) -> Result<Self> {
        if id.is_empty() {
            return Err(anyhow::anyhow!("Trade ID cannot be empty"));
        }
        
        if order_id.is_empty() {
            return Err(anyhow::anyhow!("Order ID cannot be empty"));
        }
        
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if quantity == 0 {
            return Err(anyhow::anyhow!("Quantity must be greater than zero"));
        }
        
        if price <= 0.0 {
            return Err(anyhow::anyhow!("Price must be positive"));
        }
        
        Ok(Self {
            id,
            order_id,
            symbol,
            side,
            quantity,
            price,
            commission: 0.0,
            timestamp: SystemTime::now(),
            execution_venue: None,
            trade_type: TradeType::Regular,
        })
    }
}

/// Order side enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
    SellShort,
}

/// Trade type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeType {
    Regular,
    Extended,
    PreMarket,
    AfterHours,
}

/// Time in force enumeration for order duration management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeInForce {
    Day,        // Good for day
    GTC,        // Good till cancelled
    IOC,        // Immediate or cancel
    FOK,        // Fill or kill
    GTD(SystemTime), // Good till date
}

/// Order type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    TrailingStop,
    TrailingStopLimit,
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
    Replaced,
}

/// Order structure for order management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: u64,
    pub price: Option<f64>, // None for market orders
    pub stop_price: Option<f64>, // For stop orders
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub filled_quantity: u64,
    pub remaining_quantity: u64,
    pub average_price: Option<f64>,
    pub commission: Option<f64>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub filled_at: Option<SystemTime>,
    pub cancelled_at: Option<SystemTime>,
    pub reject_reason: Option<String>,
}

impl Order {
    /// Create new order with validation
    pub fn new(
        id: String,
        symbol: String,
        side: OrderSide,
        order_type: OrderType,
        quantity: u64,
        price: Option<f64>,
        time_in_force: TimeInForce,
    ) -> Result<Self> {
        if id.is_empty() {
            return Err(anyhow::anyhow!("Order ID cannot be empty"));
        }
        
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if quantity == 0 {
            return Err(anyhow::anyhow!("Quantity must be greater than zero"));
        }
        
        // Validate price for limit orders
        if matches!(order_type, OrderType::Limit | OrderType::StopLimit) && price.is_none() {
            return Err(anyhow::anyhow!("Limit orders must have a price"));
        }
        
        if let Some(p) = price {
            if p <= 0.0 {
                return Err(anyhow::anyhow!("Price must be positive"));
            }
        }
        
        let now = SystemTime::now();
        
        Ok(Self {
            id,
            symbol,
            side,
            order_type,
            quantity,
            price,
            stop_price: None,
            time_in_force,
            status: OrderStatus::Pending,
            filled_quantity: 0,
            remaining_quantity: quantity,
            average_price: None,
            commission: None,
            created_at: now,
            updated_at: now,
            filled_at: None,
            cancelled_at: None,
            reject_reason: None,
        })
    }
    
    /// Check if order is complete
    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Filled | OrderStatus::Cancelled | OrderStatus::Rejected | OrderStatus::Expired
        )
    }
    
    /// Check if order is active
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Pending | OrderStatus::PartiallyFilled
        )
    }
}

/// Candlestick data for chart rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: SystemTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub adjusted_close: Option<f64>, // For dividend adjustments
    pub vwap: Option<f64>, // Volume weighted average price
}

impl Candle {
    /// Create new candle with validation
    pub fn new(
        timestamp: SystemTime,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: u64,
    ) -> Result<Self> {
        if open <= 0.0 || high <= 0.0 || low <= 0.0 || close <= 0.0 {
            return Err(anyhow::anyhow!("All prices must be positive"));
        }
        
        if high < low {
            return Err(anyhow::anyhow!("High price cannot be less than low price"));
        }
        
        if high < open || high < close {
            return Err(anyhow::anyhow!("High price must be >= open and close prices"));
        }
        
        if low > open || low > close {
            return Err(anyhow::anyhow!("Low price must be <= open and close prices"));
        }
        
        Ok(Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            adjusted_close: None,
            vwap: None,
        })
    }
    
    /// Calculate typical price
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
    
    /// Check if candle is bullish
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
    
    /// Check if candle is bearish
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
}

/// Time frame enumeration for chart data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TimeFrame {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
    OneMonth,
}

impl TimeFrame {
    /// Convert timeframe to seconds
    pub fn to_seconds(&self) -> u64 {
        match self {
            TimeFrame::OneMinute => 60,
            TimeFrame::FiveMinutes => 300,
            TimeFrame::FifteenMinutes => 900,
            TimeFrame::ThirtyMinutes => 1800,
            TimeFrame::OneHour => 3600,
            TimeFrame::FourHours => 14400,
            TimeFrame::OneDay => 86400,
            TimeFrame::OneWeek => 604800,
            TimeFrame::OneMonth => 2592000,
        }
    }
    
    /// Get display name for timeframe
    pub fn display_name(&self) -> &'static str {
        match self {
            TimeFrame::OneMinute => "1m",
            TimeFrame::FiveMinutes => "5m",
            TimeFrame::FifteenMinutes => "15m",
            TimeFrame::ThirtyMinutes => "30m",
            TimeFrame::OneHour => "1h",
            TimeFrame::FourHours => "4h",
            TimeFrame::OneDay => "1d",
            TimeFrame::OneWeek => "1w",
            TimeFrame::OneMonth => "1M",
        }
    }
}

/// Watchlist item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistItem {
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub added_at: SystemTime,
    pub notes: Option<String>,
    pub alert_price: Option<f64>,
}

impl WatchlistItem {
    /// Create new watchlist item with validation
    pub fn new(symbol: String, name: String) -> Result<Self> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        if name.is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }
        
        Ok(Self {
            symbol,
            name,
            current_price: 0.0,
            change: 0.0,
            change_percent: 0.0,
            volume: 0,
            added_at: SystemTime::now(),
            notes: None,
            alert_price: None,
        })
    }
    
    /// Update market data for watchlist item
    pub fn update_market_data(&mut self, market_data: &MarketData) {
        if market_data.symbol == self.symbol {
            self.current_price = market_data.current_price;
            self.change = market_data.change;
            self.change_percent = market_data.change_percent;
            self.volume = market_data.volume;
        }
    }
}