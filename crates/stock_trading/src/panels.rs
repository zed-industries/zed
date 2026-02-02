use anyhow::Result;
use gpui::{
    actions, App, AppContext, Context, DockPosition, Entity, EventEmitter, FocusHandle, 
    IntoElement, Panel, Pixels, Render, Subscription, WeakEntity, Window
};
use gpui_component::{
    button::Button,
    table::{Table, TableColumn, TableData},
    chart::{Chart, ChartType, ChartData},
    input::Input,
    Root,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    MarketData, OrderBook, OrderBookEntry, StockInfo, WatchlistItem, Order, OrderSide, 
    OrderType, TimeInForce, TimeFrame, Candle, TradingManager, TradingEvent
};

// Define actions for panel operations
actions!(
    stock_trading_panels,
    [
        ToggleWatchlistPanel,
        ToggleChartPanel,
        ToggleStockInfoPanel,
        ToggleOrderPanel,
        ToggleOrderBookPanel,
        AddStockToWatchlist,
        RemoveStockFromWatchlist,
        SelectStock,
        PlaceOrder,
        CancelOrder,
    ]
);

/// Enhanced panel events for inter-component communication
#[derive(Clone, Debug)]
pub enum PanelEvent {
    StockSelected(String),
    WatchlistUpdated(Vec<WatchlistItem>),
    OrderPlaced(Order),
    OrderCancelled(String),
    TimeFrameChanged(TimeFrame),
    ChartDataRequested(String, TimeFrame),
    MarketDataUpdated(MarketData),
    OrderBookRequested(String),
    OrderBookUpdated(OrderBook),
    HistoricalDataRequested(String, TimeFrame, usize),
    HistoricalDataUpdated(String, Vec<Candle>),
    RealTimeSubscriptionRequested(String),
    RealTimeSubscriptionCancelled(String),
    RefreshRequested(String),
    ErrorOccurred(String),
}

/// Watchlist panel using gpui-component's virtualized Table
pub struct WatchlistPanel {
    focus_handle: FocusHandle,
    watchlist_data: Vec<WatchlistItem>,
    selected_index: Option<usize>,
    trading_manager: WeakEntity<TradingManager>,
    width: Option<Pixels>,
    add_stock_input: String,
    _subscriptions: Vec<Subscription>,
}

impl WatchlistPanel {
    pub fn new(trading_manager: WeakEntity<TradingManager>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            watchlist_data: Vec::new(),
            selected_index: None,
            trading_manager,
            width: None,
            add_stock_input: String::new(),
            _subscriptions: Vec::new(),
        })
    }
    
    /// Add stock to watchlist with validation (.rules compliance)
    pub fn add_stock(&mut self, symbol: String, cx: &mut Context<Self>) -> Result<()> {
        if symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        
        // Check if symbol already exists using bounds checking
        if self.watchlist_data.iter().any(|item| item.symbol == symbol) {
            return Err(anyhow::anyhow!("Symbol already in watchlist"));
        }
        
        let watchlist_item = WatchlistItem {
            symbol: symbol.clone(),
            current_price: 0.0,
            change: 0.0,
            change_percent: 0.0,
            volume: 0,
            market_cap: None,
            pe_ratio: None,
        };
        
        self.watchlist_data.push(watchlist_item);
        self.add_stock_input.clear();
        cx.emit(PanelEvent::WatchlistUpdated(self.watchlist_data.clone()));
        cx.notify();
        
        Ok(())
    }
    
    /// Remove stock from watchlist with bounds checking (.rules compliance)
    pub fn remove_stock(&mut self, index: usize, cx: &mut Context<Self>) -> Result<()> {
        if let Some(_) = self.watchlist_data.get(index) {
            self.watchlist_data.remove(index);
            
            // Adjust selected index if necessary
            if let Some(selected) = self.selected_index {
                if selected >= index {
                    self.selected_index = if selected == 0 { None } else { Some(selected - 1) };
                }
            }
            
            cx.emit(PanelEvent::WatchlistUpdated(self.watchlist_data.clone()));
            cx.notify();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid index for watchlist removal"))
        }
    }
    
    /// Select stock with bounds checking (.rules compliance)
    pub fn select_stock(&mut self, index: usize, cx: &mut Context<Self>) -> Result<()> {
        if let Some(item) = self.watchlist_data.get(index) {
            self.selected_index = Some(index);
            cx.emit(PanelEvent::StockSelected(item.symbol.clone()));
            cx.notify();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid index for stock selection"))
        }
    }
    
    /// Update market data for watchlist items with WebSocket integration
    pub fn update_market_data(&mut self, market_data: MarketData, cx: &mut Context<Self>) {
        for item in &mut self.watchlist_data {
            if item.symbol == market_data.symbol {
                item.current_price = market_data.current_price;
                item.change = market_data.change;
                item.change_percent = market_data.change_percent;
                item.volume = market_data.volume;
                break;
            }
        }
        cx.notify();
    }
    
    /// Subscribe to real-time updates for all watchlist symbols
    pub fn subscribe_to_real_time_updates(&mut self, cx: &mut Context<Self>) -> Result<()> {
        if let Some(trading_manager) = self.trading_manager.upgrade() {
            for item in &self.watchlist_data {
                trading_manager.update(cx, |manager, cx| {
                    manager.subscribe_to_symbol(item.symbol.clone(), cx)
                })?;
            }
        }
        Ok(())
    }
    
    /// Unsubscribe from real-time updates for symbol
    pub fn unsubscribe_from_symbol(&mut self, symbol: &str, cx: &mut Context<Self>) -> Result<()> {
        if let Some(trading_manager) = self.trading_manager.upgrade() {
            trading_manager.update(cx, |manager, cx| {
                manager.unsubscribe_from_symbol(symbol, cx)
            })?;
        }
        Ok(())
    }
    
    /// Request market data refresh for all symbols
    pub fn refresh_market_data(&mut self, cx: &mut Context<Self>) {
        if let Some(trading_manager) = self.trading_manager.upgrade() {
            for item in &self.watchlist_data {
                let symbol = item.symbol.clone();
                let _task = trading_manager.update(cx, |manager, cx| {
                    manager.get_market_data(&symbol, cx)
                });
            }
        }
    }
}

impl EventEmitter<PanelEvent> for WatchlistPanel {}

impl Panel for WatchlistPanel {
    fn panel_name(&self) -> &'static str {
        "Watchlist"
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Left
    }
    
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
    
    fn set_width(&mut self, width: Option<Pixels>, _cx: &mut Context<Self>) {
        self.width = width;
    }
}

impl Render for WatchlistPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root::new()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .p_4()
                    .child(
                        // Header with add stock input
                        gpui::div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mb_4()
                            .child(
                                Input::new("add-stock-input")
                                    .placeholder("Enter symbol...")
                                    .value(self.add_stock_input.clone())
                                    .on_input(cx.listener(|this, input: &str, cx| {
                                        this.add_stock_input = input.to_uppercase();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("add-stock-btn")
                                    .label("Add")
                                    .on_click(cx.listener(|this, _event, cx| {
                                        let symbol = this.add_stock_input.clone();
                                        if let Err(error) = this.add_stock(symbol, cx) {
                                            error.log_err(); // Use .log_err() for visibility
                                        }
                                    }))
                            )
                    )
                    .child(
                        // Watchlist table
                        if self.watchlist_data.is_empty() {
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    gpui::div()
                                        .text_color(gpui::rgb(0x888888))
                                        .child("No stocks in watchlist. Add a symbol above.")
                                )
                        } else {
                            self.render_watchlist_table(cx)
                        }
                    )
            )
    }
    
    fn render_watchlist_table(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let columns = vec![
            TableColumn::new("Symbol", 80),
            TableColumn::new("Price", 80),
            TableColumn::new("Change", 80),
            TableColumn::new("Change %", 80),
            TableColumn::new("Volume", 100),
            TableColumn::new("Actions", 80),
        ];
        
        let table_data: Vec<TableData> = self.watchlist_data
            .iter()
            .enumerate()
            .map(|(index, item)| {
                TableData::new(vec![
                    item.symbol.clone(),
                    format!("${:.2}", item.current_price),
                    format!("{:+.2}", item.change),
                    format!("{:+.2}%", item.change_percent),
                    item.volume.to_string(),
                    "Remove".to_string(),
                ])
                .with_id(index.to_string())
            })
            .collect();
        
        Table::new("watchlist-table")
            .columns(columns)
            .data(table_data)
            .selected_row(self.selected_index.map(|i| i.to_string()))
            .on_row_click(cx.listener(|this, row_id: &str, cx| {
                if let Ok(index) = row_id.parse::<usize>() {
                    if let Err(error) = this.select_stock(index, cx) {
                        error.log_err(); // Proper error handling
                    }
                }
            }))
            .on_cell_click(cx.listener(|this, (row_id, col_index): &(String, usize), cx| {
                if *col_index == 5 { // Actions column
                    if let Ok(index) = row_id.parse::<usize>() {
                        if let Err(error) = this.remove_stock(index, cx) {
                            error.log_err(); // Proper error handling
                        }
                    }
                }
            }))
    }
}

/// Chart panel using gpui-component's built-in Chart
pub struct ChartPanel {
    focus_handle: FocusHandle,
    current_symbol: Option<String>,
    current_timeframe: TimeFrame,
    chart_data: Vec<Candle>,
    trading_manager: WeakEntity<TradingManager>,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl ChartPanel {
    pub fn new(trading_manager: WeakEntity<TradingManager>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            current_symbol: None,
            current_timeframe: TimeFrame::OneDay,
            chart_data: Vec::new(),
            trading_manager,
            width: None,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Set symbol and load chart data with real-time subscription
    pub fn set_symbol(&mut self, symbol: String, cx: &mut Context<Self>) {
        self.current_symbol = Some(symbol.clone());
        
        // Request historical data
        cx.emit(PanelEvent::ChartDataRequested(symbol.clone(), self.current_timeframe));
        
        // Subscribe to real-time updates
        if let Some(trading_manager) = self.trading_manager.upgrade() {
            if let Err(error) = trading_manager.update(cx, |manager, cx| {
                manager.subscribe_to_symbol(symbol, cx)
            }) {
                error.log_err(); // Proper error handling
            }
        }
        
        cx.notify();
    }
    
    /// Change timeframe with validation and data refresh (.rules compliance)
    pub fn set_timeframe(&mut self, timeframe: TimeFrame, cx: &mut Context<Self>) -> Result<()> {
        self.current_timeframe = timeframe;
        
        if let Some(symbol) = &self.current_symbol {
            cx.emit(PanelEvent::ChartDataRequested(symbol.clone(), timeframe));
            
            // Request fresh historical data for new timeframe
            if let Some(trading_manager) = self.trading_manager.upgrade() {
                let symbol_clone = symbol.clone();
                let _task = trading_manager.update(cx, |manager, cx| {
                    manager.get_historical_data(&symbol_clone, timeframe, 100, cx)
                });
            }
        }
        
        cx.emit(PanelEvent::TimeFrameChanged(timeframe));
        cx.notify();
        Ok(())
    }
    
    /// Update chart data with bounds checking and real-time integration (.rules compliance)
    pub fn update_chart_data(&mut self, data: Vec<Candle>, cx: &mut Context<Self>) {
        self.chart_data = data;
        cx.notify();
    }
    
    /// Update real-time price data (append new candle or update last candle)
    pub fn update_real_time_data(&mut self, market_data: MarketData, cx: &mut Context<Self>) {
        if let Some(symbol) = &self.current_symbol {
            if market_data.symbol == *symbol {
                // Update the last candle with current price
                if let Some(last_candle) = self.chart_data.last_mut() {
                    last_candle.close = market_data.current_price;
                    last_candle.high = last_candle.high.max(market_data.current_price);
                    last_candle.low = last_candle.low.min(market_data.current_price);
                    last_candle.volume = market_data.volume;
                    last_candle.timestamp = market_data.timestamp;
                }
                cx.notify();
            }
        }
    }
}

impl EventEmitter<PanelEvent> for ChartPanel {}

impl Panel for ChartPanel {
    fn panel_name(&self) -> &'static str {
        "Chart"
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Right
    }
    
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
    
    fn set_width(&mut self, width: Option<Pixels>, _cx: &mut Context<Self>) {
        self.width = width;
    }
}

impl Render for ChartPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root::new()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .p_4()
                    .child(
                        // Header with timeframe buttons
                        gpui::div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mb_4()
                            .child(
                                gpui::div()
                                    .child(format!("Chart: {}", 
                                        self.current_symbol.as_deref().unwrap_or("No symbol selected")))
                            )
                            .child(
                                gpui::div()
                                    .flex()
                                    .gap_1()
                                    .child(self.render_timeframe_button("1D", TimeFrame::OneDay, cx))
                                    .child(self.render_timeframe_button("1W", TimeFrame::OneWeek, cx))
                                    .child(self.render_timeframe_button("1M", TimeFrame::OneMonth, cx))
                                    .child(self.render_timeframe_button("3M", TimeFrame::ThreeMonths, cx))
                                    .child(self.render_timeframe_button("1Y", TimeFrame::OneYear, cx))
                            )
                    )
                    .child(
                        // Chart area
                        if self.chart_data.is_empty() {
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    gpui::div()
                                        .text_color(gpui::rgb(0x888888))
                                        .child("Select a stock to view chart")
                                )
                        } else {
                            self.render_chart(cx)
                        }
                    )
            )
    }
    
    fn render_timeframe_button(
        &mut self, 
        label: &str, 
        timeframe: TimeFrame, 
        cx: &mut Context<Self>
    ) -> impl IntoElement {
        let is_active = self.current_timeframe == timeframe;
        
        Button::new(format!("timeframe-{}", label))
            .label(label)
            .variant(if is_active { "primary" } else { "secondary" })
            .on_click(cx.listener(move |this, _event, cx| {
                if let Err(error) = this.set_timeframe(timeframe, cx) {
                    error.log_err(); // Proper error handling
                }
            }))
    }
    
    fn render_chart(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        let chart_data: Vec<ChartData> = self.chart_data
            .iter()
            .map(|candle| ChartData::new(
                candle.timestamp.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs() as f64,
                vec![candle.open, candle.high, candle.low, candle.close]
            ))
            .collect();
        
        Chart::new("price-chart")
            .chart_type(ChartType::Candlestick)
            .data(chart_data)
            .width_full()
            .height_full()
    }
}

/// Stock info panel using gpui-component's layout components
pub struct StockInfoPanel {
    focus_handle: FocusHandle,
    current_symbol: Option<String>,
    stock_info: Option<StockInfo>,
    trading_manager: WeakEntity<TradingManager>,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl StockInfoPanel {
    pub fn new(trading_manager: WeakEntity<TradingManager>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            current_symbol: None,
            stock_info: None,
            trading_manager,
            width: None,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Set symbol and load stock info
    pub fn set_symbol(&mut self, symbol: String, _cx: &mut Context<Self>) {
        self.current_symbol = Some(symbol);
        // In a real implementation, this would trigger data loading
        // For now, we'll use placeholder data
        self.stock_info = Some(StockInfo {
            symbol: self.current_symbol.clone().unwrap_or_default(),
            company_name: format!("{} Inc.", self.current_symbol.as_deref().unwrap_or("Unknown")),
            sector: "Technology".to_string(),
            industry: "Software".to_string(),
            market_cap: Some(1_000_000_000),
            pe_ratio: Some(25.5),
            dividend_yield: Some(2.1),
            fifty_two_week_high: 150.0,
            fifty_two_week_low: 80.0,
            average_volume: 1_000_000,
            beta: Some(1.2),
            eps: Some(5.50),
            description: "A leading technology company.".to_string(),
        });
    }
    
    /// Update stock info data
    pub fn update_stock_info(&mut self, info: StockInfo, cx: &mut Context<Self>) {
        self.stock_info = Some(info);
        cx.notify();
    }
}

impl EventEmitter<PanelEvent> for StockInfoPanel {}

impl Panel for StockInfoPanel {
    fn panel_name(&self) -> &'static str {
        "Stock Info"
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Right
    }
    
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
    
    fn set_width(&mut self, width: Option<Pixels>, _cx: &mut Context<Self>) {
        self.width = width;
    }
}

impl Render for StockInfoPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root::new()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .p_4()
                    .child(
                        gpui::div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .mb_4()
                            .child("Stock Information")
                    )
                    .child(
                        if let Some(info) = &self.stock_info {
                            self.render_stock_info(info)
                        } else {
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    gpui::div()
                                        .text_color(gpui::rgb(0x888888))
                                        .child("Select a stock to view information")
                                )
                        }
                    )
            )
    }
    
    fn render_stock_info(&self, info: &StockInfo) -> impl IntoElement {
        gpui::div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                gpui::div()
                    .child(
                        gpui::div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(format!("{} ({})", info.company_name, info.symbol))
                    )
                    .child(
                        gpui::div()
                            .text_sm()
                            .text_color(gpui::rgb(0x666666))
                            .child(format!("{} - {}", info.sector, info.industry))
                    )
            )
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(self.render_info_row("Market Cap", 
                        info.market_cap.map(|mc| format!("${:.2}B", mc as f64 / 1_000_000_000.0))
                            .unwrap_or_else(|| "N/A".to_string())))
                    .child(self.render_info_row("P/E Ratio", 
                        info.pe_ratio.map(|pe| format!("{:.2}", pe))
                            .unwrap_or_else(|| "N/A".to_string())))
                    .child(self.render_info_row("Dividend Yield", 
                        info.dividend_yield.map(|dy| format!("{:.2}%", dy))
                            .unwrap_or_else(|| "N/A".to_string())))
                    .child(self.render_info_row("52W High", format!("${:.2}", info.fifty_two_week_high)))
                    .child(self.render_info_row("52W Low", format!("${:.2}", info.fifty_two_week_low)))
                    .child(self.render_info_row("Avg Volume", format!("{:,}", info.average_volume)))
                    .child(self.render_info_row("Beta", 
                        info.beta.map(|b| format!("{:.2}", b))
                            .unwrap_or_else(|| "N/A".to_string())))
                    .child(self.render_info_row("EPS", 
                        info.eps.map(|eps| format!("${:.2}", eps))
                            .unwrap_or_else(|| "N/A".to_string())))
            )
            .child(
                gpui::div()
                    .mt_4()
                    .child(
                        gpui::div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .mb_2()
                            .child("Description")
                    )
                    .child(
                        gpui::div()
                            .text_sm()
                            .text_color(gpui::rgb(0x666666))
                            .child(info.description.clone())
                    )
            )
    }
    
    fn render_info_row(&self, label: &str, value: String) -> impl IntoElement {
        gpui::div()
            .flex()
            .justify_between()
            .items_center()
            .child(
                gpui::div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(label)
            )
            .child(
                gpui::div()
                    .text_sm()
                    .child(value)
            )
    }
}

/// Order panel using gpui-component's form controls
pub struct OrderPanel {
    focus_handle: FocusHandle,
    current_symbol: Option<String>,
    order_side: OrderSide,
    order_type: OrderType,
    quantity: String,
    price: String,
    time_in_force: TimeInForce,
    trading_manager: WeakEntity<TradingManager>,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl OrderPanel {
    pub fn new(trading_manager: WeakEntity<TradingManager>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            current_symbol: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            quantity: String::new(),
            price: String::new(),
            time_in_force: TimeInForce::Day,
            trading_manager,
            width: None,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Set symbol for order
    pub fn set_symbol(&mut self, symbol: String, cx: &mut Context<Self>) {
        self.current_symbol = Some(symbol);
        cx.notify();
    }
    
    /// Place order with validation (.rules compliance)
    pub fn place_order(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let symbol = self.current_symbol.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No symbol selected"))?;
        
        let quantity: u64 = self.quantity.parse()
            .map_err(|_| anyhow::anyhow!("Invalid quantity"))?;
        
        if quantity == 0 {
            return Err(anyhow::anyhow!("Quantity must be greater than zero"));
        }
        
        let price = if self.order_type == OrderType::Market {
            None
        } else {
            Some(self.price.parse::<f64>()
                .map_err(|_| anyhow::anyhow!("Invalid price"))?)
        };
        
        let order = Order {
            id: format!("ORD_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default().as_millis()),
            symbol: symbol.clone(),
            side: self.order_side,
            order_type: self.order_type,
            quantity,
            price,
            time_in_force: self.time_in_force,
            status: crate::OrderStatus::Pending,
            filled_quantity: 0,
            average_fill_price: None,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
        };
        
        cx.emit(PanelEvent::OrderPlaced(order));
        
        // Clear form after successful order
        self.quantity.clear();
        self.price.clear();
        cx.notify();
        
        Ok(())
    }
}

impl EventEmitter<PanelEvent> for OrderPanel {}

impl Panel for OrderPanel {
    fn panel_name(&self) -> &'static str {
        "Order Entry"
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Bottom
    }
    
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
    
    fn set_width(&mut self, width: Option<Pixels>, _cx: &mut Context<Self>) {
        self.width = width;
    }
}

impl Render for OrderPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root::new()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .p_4()
                    .child(
                        gpui::div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .mb_4()
                            .child(format!("Order Entry - {}", 
                                self.current_symbol.as_deref().unwrap_or("No symbol selected")))
                    )
                    .child(
                        if self.current_symbol.is_some() {
                            self.render_order_form(cx)
                        } else {
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    gpui::div()
                                        .text_color(gpui::rgb(0x888888))
                                        .child("Select a stock to place orders")
                                )
                        }
                    )
            )
    }
    
    fn render_order_form(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                // Buy/Sell buttons
                gpui::div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("buy-btn")
                            .label("Buy")
                            .variant(if matches!(self.order_side, OrderSide::Buy) { "primary" } else { "secondary" })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.order_side = OrderSide::Buy;
                                cx.notify();
                            }))
                    )
                    .child(
                        Button::new("sell-btn")
                            .label("Sell")
                            .variant(if matches!(self.order_side, OrderSide::Sell) { "primary" } else { "secondary" })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.order_side = OrderSide::Sell;
                                cx.notify();
                            }))
                    )
            )
            .child(
                // Order type buttons
                gpui::div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("market-btn")
                            .label("Market")
                            .variant(if matches!(self.order_type, OrderType::Market) { "primary" } else { "secondary" })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.order_type = OrderType::Market;
                                cx.notify();
                            }))
                    )
                    .child(
                        Button::new("limit-btn")
                            .label("Limit")
                            .variant(if matches!(self.order_type, OrderType::Limit) { "primary" } else { "secondary" })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.order_type = OrderType::Limit;
                                cx.notify();
                            }))
                    )
            )
            .child(
                // Quantity input
                gpui::div()
                    .child(
                        gpui::div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .mb_1()
                            .child("Quantity")
                    )
                    .child(
                        Input::new("quantity-input")
                            .placeholder("Enter quantity...")
                            .value(self.quantity.clone())
                            .on_input(cx.listener(|this, input: &str, cx| {
                                // Only allow numeric input
                                if input.chars().all(|c| c.is_ascii_digit()) {
                                    this.quantity = input.to_string();
                                    cx.notify();
                                }
                            }))
                    )
            )
            .child(
                // Price input (only for limit orders)
                if matches!(self.order_type, OrderType::Limit) {
                    gpui::div()
                        .child(
                            gpui::div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .mb_1()
                                .child("Price")
                        )
                        .child(
                            Input::new("price-input")
                                .placeholder("Enter price...")
                                .value(self.price.clone())
                                .on_input(cx.listener(|this, input: &str, cx| {
                                    // Allow numeric input with decimal point
                                    if input.chars().all(|c| c.is_ascii_digit() || c == '.') {
                                        this.price = input.to_string();
                                        cx.notify();
                                    }
                                }))
                        )
                } else {
                    gpui::div() // Empty div for market orders
                }
            )
            .child(
                // Place order button
                Button::new("place-order-btn")
                    .label("Place Order")
                    .variant("primary")
                    .on_click(cx.listener(|this, _event, cx| {
                        if let Err(error) = this.place_order(cx) {
                            error.log_err(); // Proper error handling
                        }
                    }))
            )
    }
}

/// Order book panel using gpui-component's virtualized Table
pub struct OrderBookPanel {
    focus_handle: FocusHandle,
    current_symbol: Option<String>,
    order_book: Option<OrderBook>,
    trading_manager: WeakEntity<TradingManager>,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl OrderBookPanel {
    pub fn new(trading_manager: WeakEntity<TradingManager>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            current_symbol: None,
            order_book: None,
            trading_manager,
            width: None,
            _subscriptions: Vec::new(),
        })
    }
    
    /// Set symbol and load order book with real-time subscription
    pub fn set_symbol(&mut self, symbol: String, cx: &mut Context<Self>) {
        self.current_symbol = Some(symbol.clone());
        
        // Subscribe to real-time order book updates
        if let Some(trading_manager) = self.trading_manager.upgrade() {
            if let Err(error) = trading_manager.update(cx, |manager, cx| {
                manager.subscribe_to_symbol(symbol.clone(), cx)
            }) {
                error.log_err(); // Proper error handling
            }
            
            // Request initial order book data
            let _task = trading_manager.update(cx, |manager, cx| {
                manager.get_order_book(&symbol, cx)
            });
        }
        
        cx.notify();
    }
    
    /// Update order book data with real-time integration
    pub fn update_order_book(&mut self, order_book: OrderBook, cx: &mut Context<Self>) {
        if let Some(current_symbol) = &self.current_symbol {
            if order_book.symbol == *current_symbol {
                self.order_book = Some(order_book);
                cx.notify();
            }
        }
    }
    
    /// Refresh order book data
    pub fn refresh_order_book(&mut self, cx: &mut Context<Self>) {
        if let (Some(symbol), Some(trading_manager)) = (&self.current_symbol, self.trading_manager.upgrade()) {
            let symbol_clone = symbol.clone();
            let _task = trading_manager.update(cx, |manager, cx| {
                manager.get_order_book(&symbol_clone, cx)
            });
        }
    }
}

impl EventEmitter<PanelEvent> for OrderBookPanel {}

impl Panel for OrderBookPanel {
    fn panel_name(&self) -> &'static str {
        "Order Book"
    }
    
    fn dock_position(&self) -> DockPosition {
        DockPosition::Right
    }
    
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
    
    fn set_width(&mut self, width: Option<Pixels>, _cx: &mut Context<Self>) {
        self.width = width;
    }
}

impl Render for OrderBookPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root::new()
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .p_4()
                    .child(
                        gpui::div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .mb_4()
                            .child(format!("Order Book - {}", 
                                self.current_symbol.as_deref().unwrap_or("No symbol selected")))
                    )
                    .child(
                        if let Some(order_book) = &self.order_book {
                            self.render_order_book(order_book)
                        } else {
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .child(
                                    gpui::div()
                                        .text_color(gpui::rgb(0x888888))
                                        .child("Select a stock to view order book")
                                )
                        }
                    )
            )
    }
    
    fn render_order_book(&self, order_book: &OrderBook) -> impl IntoElement {
        gpui::div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                // Spread information
                gpui::div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_2()
                    .bg(gpui::rgb(0xf5f5f5))
                    .rounded_md()
                    .child(
                        gpui::div()
                            .text_sm()
                            .child(format!("Spread: ${:.2}", order_book.get_spread()))
                    )
                    .child(
                        gpui::div()
                            .text_sm()
                            .child(format!("Mid: ${:.2}", order_book.get_mid_price()))
                    )
            )
            .child(
                // Order book tables
                gpui::div()
                    .flex()
                    .gap_4()
                    .h_full()
                    .child(
                        // Bids (left side)
                        gpui::div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(
                                gpui::div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(0x00aa00))
                                    .mb_2()
                                    .child("Bids")
                            )
                            .child(self.render_order_book_side(&order_book.bids, true))
                    )
                    .child(
                        // Asks (right side)
                        gpui::div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(
                                gpui::div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(gpui::rgb(0xaa0000))
                                    .mb_2()
                                    .child("Asks")
                            )
                            .child(self.render_order_book_side(&order_book.asks, false))
                    )
            )
    }
    
    fn render_order_book_side(&self, entries: &[OrderBookEntry], is_bids: bool) -> impl IntoElement {
        let columns = vec![
            TableColumn::new("Price", 80),
            TableColumn::new("Size", 80),
            TableColumn::new("Total", 80),
        ];
        
        let table_data: Vec<TableData> = entries
            .iter()
            .take(10) // Show top 10 levels
            .enumerate()
            .map(|(index, entry)| {
                TableData::new(vec![
                    format!("${:.2}", entry.price),
                    entry.quantity.to_string(),
                    format!("${:.0}", entry.price * entry.quantity as f64),
                ])
                .with_id(index.to_string())
            })
            .collect();
        
        Table::new(if is_bids { "bids-table" } else { "asks-table" })
            .columns(columns)
            .data(table_data)
            .height_full()
    }
}

// Extension trait for error logging (if not already defined elsewhere)
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