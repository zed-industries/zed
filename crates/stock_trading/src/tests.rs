use anyhow::Result;
use gpui::TestAppContext;
use std::time::{Duration, SystemTime};

use crate::market_data::*;
use crate::websocket_service::*;
use crate::mock_data_service::*;
use crate::*;

/// Property 12: Order Validation
/// **Validates: Requirements 5.2**
/// 
/// Property: For any order parameters entered, the system should validate them 
/// before submission, accepting valid orders and rejecting invalid ones
#[gpui::test]
async fn property_order_validation(cx: &mut TestAppContext) {
    // Test valid order creation
    let valid_order = Order::new(
        "ORDER_001".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        100,
        Some(150.0),
        TimeInForce::Day,
    );
    
    assert!(valid_order.is_ok(), "Valid order should be created successfully");
    
    let order = valid_order.unwrap();
    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.quantity, 100);
    assert_eq!(order.price, Some(150.0));
    assert!(order.is_active());
    assert!(!order.is_complete());
    
    // Test invalid order creation - empty ID
    let invalid_order_empty_id = Order::new(
        "".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        100,
        Some(150.0),
        TimeInForce::Day,
    );
    
    assert!(invalid_order_empty_id.is_err(), "Order with empty ID should be rejected");
    
    // Test invalid order creation - empty symbol
    let invalid_order_empty_symbol = Order::new(
        "ORDER_002".to_string(),
        "".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        100,
        Some(150.0),
        TimeInForce::Day,
    );
    
    assert!(invalid_order_empty_symbol.is_err(), "Order with empty symbol should be rejected");
    
    // Test invalid order creation - zero quantity
    let invalid_order_zero_quantity = Order::new(
        "ORDER_003".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        0,
        Some(150.0),
        TimeInForce::Day,
    );
    
    assert!(invalid_order_zero_quantity.is_err(), "Order with zero quantity should be rejected");
    
    // Test invalid order creation - limit order without price
    let invalid_order_no_price = Order::new(
        "ORDER_004".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        100,
        None,
        TimeInForce::Day,
    );
    
    assert!(invalid_order_no_price.is_err(), "Limit order without price should be rejected");
    
    // Test invalid order creation - negative price
    let invalid_order_negative_price = Order::new(
        "ORDER_005".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        100,
        Some(-150.0),
        TimeInForce::Day,
    );
    
    assert!(invalid_order_negative_price.is_err(), "Order with negative price should be rejected");
    
    // Test market order (should not require price)
    let valid_market_order = Order::new(
        "ORDER_006".to_string(),
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Market,
        100,
        None,
        TimeInForce::Day,
    );
    
    assert!(valid_market_order.is_ok(), "Valid market order should be created successfully");
}

#[gpui::test]
async fn test_market_data_validation(cx: &mut TestAppContext) {
    // Test valid market data creation
    let valid_data = MarketData::new("AAPL".to_string(), 150.0);
    assert!(valid_data.is_ok(), "Valid market data should be created successfully");
    
    let data = valid_data.unwrap();
    assert_eq!(data.symbol, "AAPL");
    assert_eq!(data.current_price, 150.0);
    assert_eq!(data.previous_close, 150.0);
    
    // Test spread calculation
    let mut data_with_bid_ask = data.clone();
    data_with_bid_ask.bid = Some(149.5);
    data_with_bid_ask.ask = Some(150.5);
    
    let spread = data_with_bid_ask.get_spread();
    assert!(spread.is_some());
    assert_eq!(spread.unwrap(), 1.0);
    
    let spread_percent = data_with_bid_ask.get_spread_percent();
    assert!(spread_percent.is_some());
    assert!((spread_percent.unwrap() - 0.6688963210702341).abs() < 0.001); // Approximately 0.67%
    
    // Test invalid market data creation - empty symbol
    let invalid_data_empty_symbol = MarketData::new("".to_string(), 150.0);
    assert!(invalid_data_empty_symbol.is_err(), "Market data with empty symbol should be rejected");
    
    // Test invalid market data creation - negative price
    let invalid_data_negative_price = MarketData::new("AAPL".to_string(), -150.0);
    assert!(invalid_data_negative_price.is_err(), "Market data with negative price should be rejected");
}

#[gpui::test]
async fn test_order_book_validation(cx: &mut TestAppContext) {
    // Test valid order book creation
    let valid_order_book = OrderBook::new("AAPL".to_string());
    assert!(valid_order_book.is_ok(), "Valid order book should be created successfully");
    
    let mut order_book = valid_order_book.unwrap();
    assert_eq!(order_book.symbol, "AAPL");
    assert!(order_book.bids.is_empty());
    assert!(order_book.asks.is_empty());
    
    // Test adding valid entries
    let bid_entry = OrderBookEntry::new(149.5, 100, OrderSide::Buy);
    assert!(bid_entry.is_ok(), "Valid bid entry should be created successfully");
    order_book.bids.push(bid_entry.unwrap());
    
    let ask_entry = OrderBookEntry::new(150.5, 200, OrderSide::Sell);
    assert!(ask_entry.is_ok(), "Valid ask entry should be created successfully");
    order_book.asks.push(ask_entry.unwrap());
    
    // Test spread calculation
    order_book.calculate_spread();
    assert_eq!(order_book.spread, 1.0);
    assert!((order_book.spread_percent - 0.6688963210702341).abs() < 0.001);
    
    // Test best bid/ask
    assert_eq!(order_book.get_best_bid(), Some(149.5));
    assert_eq!(order_book.get_best_ask(), Some(150.5));
    
    // Test invalid order book creation - empty symbol
    let invalid_order_book = OrderBook::new("".to_string());
    assert!(invalid_order_book.is_err(), "Order book with empty symbol should be rejected");
    
    // Test invalid order book entry - zero price
    let invalid_entry_zero_price = OrderBookEntry::new(0.0, 100, OrderSide::Buy);
    assert!(invalid_entry_zero_price.is_err(), "Order book entry with zero price should be rejected");
    
    // Test invalid order book entry - zero quantity
    let invalid_entry_zero_quantity = OrderBookEntry::new(149.5, 0, OrderSide::Buy);
    assert!(invalid_entry_zero_quantity.is_err(), "Order book entry with zero quantity should be rejected");
}

#[gpui::test]
async fn test_portfolio_validation(cx: &mut TestAppContext) {
    // Test valid portfolio creation
    let valid_portfolio = Portfolio::new("ACCOUNT_001".to_string(), 100_000.0);
    assert!(valid_portfolio.is_ok(), "Valid portfolio should be created successfully");
    
    let portfolio = valid_portfolio.unwrap();
    assert_eq!(portfolio.account_id, "ACCOUNT_001");
    assert_eq!(portfolio.cash_balance, 100_000.0);
    assert_eq!(portfolio.total_value, 100_000.0);
    assert!(portfolio.positions.is_empty());
    
    // Test invalid portfolio creation - empty account ID
    let invalid_portfolio_empty_id = Portfolio::new("".to_string(), 100_000.0);
    assert!(invalid_portfolio_empty_id.is_err(), "Portfolio with empty account ID should be rejected");
    
    // Test invalid portfolio creation - negative cash balance
    let invalid_portfolio_negative_cash = Portfolio::new("ACCOUNT_002".to_string(), -100_000.0);
    assert!(invalid_portfolio_negative_cash.is_err(), "Portfolio with negative cash balance should be rejected");
}

#[gpui::test]
async fn test_position_validation(cx: &mut TestAppContext) {
    // Test valid position creation
    let valid_position = Position::new("AAPL".to_string(), 100, 150.0);
    assert!(valid_position.is_ok(), "Valid position should be created successfully");
    
    let position = valid_position.unwrap();
    assert_eq!(position.symbol, "AAPL");
    assert_eq!(position.quantity, 100);
    assert_eq!(position.average_cost, 150.0);
    assert_eq!(position.cost_basis, 15_000.0);
    
    // Test valid short position
    let valid_short_position = Position::new("TSLA".to_string(), -50, 250.0);
    assert!(valid_short_position.is_ok(), "Valid short position should be created successfully");
    
    let short_position = valid_short_position.unwrap();
    assert_eq!(short_position.quantity, -50);
    assert_eq!(short_position.cost_basis, 12_500.0); // abs(quantity) * price
    
    // Test invalid position creation - empty symbol
    let invalid_position_empty_symbol = Position::new("".to_string(), 100, 150.0);
    assert!(invalid_position_empty_symbol.is_err(), "Position with empty symbol should be rejected");
    
    // Test invalid position creation - zero quantity
    let invalid_position_zero_quantity = Position::new("AAPL".to_string(), 0, 150.0);
    assert!(invalid_position_zero_quantity.is_err(), "Position with zero quantity should be rejected");
    
    // Test invalid position creation - zero/negative average cost
    let invalid_position_zero_cost = Position::new("AAPL".to_string(), 100, 0.0);
    assert!(invalid_position_zero_cost.is_err(), "Position with zero average cost should be rejected");
    
    let invalid_position_negative_cost = Position::new("AAPL".to_string(), 100, -150.0);
    assert!(invalid_position_negative_cost.is_err(), "Position with negative average cost should be rejected");
}

#[gpui::test]
async fn test_candle_validation(cx: &mut TestAppContext) {
    let timestamp = SystemTime::now();
    
    // Test valid candle creation
    let valid_candle = Candle::new(timestamp, 100.0, 105.0, 95.0, 102.0, 1_000_000);
    assert!(valid_candle.is_ok(), "Valid candle should be created successfully");
    
    let candle = valid_candle.unwrap();
    assert_eq!(candle.open, 100.0);
    assert_eq!(candle.high, 105.0);
    assert_eq!(candle.low, 95.0);
    assert_eq!(candle.close, 102.0);
    assert_eq!(candle.volume, 1_000_000);
    assert!(candle.is_bullish());
    assert!(!candle.is_bearish());
    
    // Test typical price calculation
    let typical_price = candle.typical_price();
    assert_eq!(typical_price, (105.0 + 95.0 + 102.0) / 3.0);
    
    // Test bearish candle
    let bearish_candle = Candle::new(timestamp, 100.0, 105.0, 95.0, 98.0, 1_000_000);
    assert!(bearish_candle.is_ok());
    let bearish = bearish_candle.unwrap();
    assert!(!bearish.is_bullish());
    assert!(bearish.is_bearish());
    
    // Test invalid candle creation - negative prices
    let invalid_candle_negative_open = Candle::new(timestamp, -100.0, 105.0, 95.0, 102.0, 1_000_000);
    assert!(invalid_candle_negative_open.is_err(), "Candle with negative open should be rejected");
    
    // Test invalid candle creation - high < low
    let invalid_candle_high_low = Candle::new(timestamp, 100.0, 95.0, 105.0, 102.0, 1_000_000);
    assert!(invalid_candle_high_low.is_err(), "Candle with high < low should be rejected");
    
    // Test invalid candle creation - high < open
    let invalid_candle_high_open = Candle::new(timestamp, 110.0, 105.0, 95.0, 102.0, 1_000_000);
    assert!(invalid_candle_high_open.is_err(), "Candle with high < open should be rejected");
    
    // Test invalid candle creation - low > close
    let invalid_candle_low_close = Candle::new(timestamp, 100.0, 105.0, 104.0, 102.0, 1_000_000);
    assert!(invalid_candle_low_close.is_err(), "Candle with low > close should be rejected");
}

#[gpui::test]
async fn test_websocket_message_validation(cx: &mut TestAppContext) {
    // Test valid WebSocket message creation
    let data = serde_json::json!({"test": "data"});
    let valid_message = WebSocketMessage::new(
        MessageType::Quote,
        Some("AAPL".to_string()),
        data.clone(),
    );
    assert!(valid_message.is_ok(), "Valid WebSocket message should be created successfully");
    
    let message = valid_message.unwrap();
    assert_eq!(message.message_type, MessageType::Quote);
    assert_eq!(message.symbol, Some("AAPL".to_string()));
    assert_eq!(message.data, data);
    
    // Test message without symbol (valid for some message types)
    let valid_message_no_symbol = WebSocketMessage::new(
        MessageType::Heartbeat,
        None,
        serde_json::json!({"timestamp": "now"}),
    );
    assert!(valid_message_no_symbol.is_ok(), "WebSocket message without symbol should be valid for heartbeat");
}

#[gpui::test]
async fn test_quote_update_validation(cx: &mut TestAppContext) {
    // Test valid quote update creation
    let valid_quote = QuoteUpdate::new("AAPL".to_string(), 149.5, 150.5, 150.0);
    assert!(valid_quote.is_ok(), "Valid quote update should be created successfully");
    
    let quote = valid_quote.unwrap();
    assert_eq!(quote.symbol, "AAPL");
    assert_eq!(quote.bid, 149.5);
    assert_eq!(quote.ask, 150.5);
    assert_eq!(quote.last_price, 150.0);
    
    // Test spread calculations
    assert_eq!(quote.get_spread(), 1.0);
    assert!((quote.get_spread_percent() - 0.6688963210702341).abs() < 0.001);
    
    // Test invalid quote update creation - empty symbol
    let invalid_quote_empty_symbol = QuoteUpdate::new("".to_string(), 149.5, 150.5, 150.0);
    assert!(invalid_quote_empty_symbol.is_err(), "Quote update with empty symbol should be rejected");
    
    // Test invalid quote update creation - negative prices
    let invalid_quote_negative_bid = QuoteUpdate::new("AAPL".to_string(), -149.5, 150.5, 150.0);
    assert!(invalid_quote_negative_bid.is_err(), "Quote update with negative bid should be rejected");
    
    // Test invalid quote update creation - bid >= ask
    let invalid_quote_bid_ask = QuoteUpdate::new("AAPL".to_string(), 150.5, 149.5, 150.0);
    assert!(invalid_quote_bid_ask.is_err(), "Quote update with bid >= ask should be rejected");
}

#[gpui::test]
async fn test_websocket_subscription_validation(cx: &mut TestAppContext) {
    // Test valid subscription creation
    let message_types = vec![MessageType::Quote, MessageType::Trade];
    let valid_subscription = WebSocketSubscription::new("AAPL".to_string(), message_types.clone());
    assert!(valid_subscription.is_ok(), "Valid WebSocket subscription should be created successfully");
    
    let subscription = valid_subscription.unwrap();
    assert_eq!(subscription.symbol, "AAPL");
    assert_eq!(subscription.message_types, message_types);
    assert!(subscription.is_active);
    assert!(subscription.includes_message_type(&MessageType::Quote));
    assert!(subscription.includes_message_type(&MessageType::Trade));
    assert!(!subscription.includes_message_type(&MessageType::OrderBook));
    
    // Test invalid subscription creation - empty symbol
    let invalid_subscription_empty_symbol = WebSocketSubscription::new("".to_string(), message_types.clone());
    assert!(invalid_subscription_empty_symbol.is_err(), "WebSocket subscription with empty symbol should be rejected");
    
    // Test invalid subscription creation - empty message types
    let invalid_subscription_empty_types = WebSocketSubscription::new("AAPL".to_string(), vec![]);
    assert!(invalid_subscription_empty_types.is_err(), "WebSocket subscription with empty message types should be rejected");
}

#[gpui::test]
async fn test_gpui_component_integration(cx: &mut TestAppContext) {
    // Test TradingManager entity creation and basic functionality
    let http_client = http_client::ReqwestClient::new().unwrap();
    let trading_manager = TradingManager::new(std::sync::Arc::new(http_client), cx);
    
    // Test setting active symbol
    trading_manager.update(cx, |manager, cx| {
        manager.set_active_symbol("AAPL".to_string(), cx);
        assert_eq!(manager.get_active_symbol(), Some(&"AAPL".to_string()));
    });
    
    // Test DataService entity creation
    let http_client = http_client::ReqwestClient::new().unwrap();
    let data_service = DataService::new(std::sync::Arc::new(http_client), cx);
    
    // Test data service caching
    data_service.update(cx, |service, cx| {
        let market_data = MarketData::new("AAPL".to_string(), 150.0).unwrap();
        let result = service.process_market_data(market_data, cx);
        assert!(result.is_ok(), "Processing valid market data should succeed");
        
        // Test cached data retrieval
        let cached_data = service.get_cached_data("AAPL");
        assert!(cached_data.is_some(), "Cached data should be available");
        assert_eq!(cached_data.unwrap().symbol, "AAPL");
    });
    
    // Test WebSocketService entity creation
    let websocket_service = WebSocketService::new(cx);
    
    websocket_service.update(cx, |service, cx| {
        // Test subscription management
        let result = service.subscribe_to_symbol(
            "AAPL".to_string(),
            vec![MessageType::Quote, MessageType::Trade],
            cx,
        );
        assert!(result.is_ok(), "Valid subscription should succeed");
        
        assert!(service.is_subscribed_to("AAPL"), "Should be subscribed to AAPL");
        assert!(!service.is_subscribed_to("GOOGL"), "Should not be subscribed to GOOGL");
        
        // Test unsubscription
        let unsubscribe_result = service.unsubscribe_from_symbol("AAPL", cx);
        assert!(unsubscribe_result.is_ok(), "Unsubscription should succeed");
        assert!(!service.is_subscribed_to("AAPL"), "Should no longer be subscribed to AAPL");
    });
    
    // Test MockDataService entity creation and functionality
    let mock_data_service = MockDataService::new(cx);
    
    mock_data_service.update(cx, |service, _cx| {
        // Test getting available symbols
        let symbols = service.get_available_symbols();
        assert!(!symbols.is_empty(), "Mock data service should have available symbols");
        assert!(symbols.contains(&"AAPL".to_string()), "Should include AAPL");
        
        // Test getting market data
        let market_data = service.get_market_data("AAPL");
        assert!(market_data.is_some(), "Should have market data for AAPL");
        
        let data = market_data.unwrap();
        assert_eq!(data.symbol, "AAPL");
        assert!(data.current_price > 0.0, "Price should be positive");
        
        // Test historical data generation
        let historical_result = service.generate_historical_data("AAPL", TimeFrame::OneDay, 30);
        assert!(historical_result.is_ok(), "Historical data generation should succeed");
        
        let historical_data = historical_result.unwrap();
        assert_eq!(historical_data.len(), 30, "Should generate 30 candles");
        
        // Verify candle data integrity
        for candle in &historical_data {
            assert!(candle.open > 0.0, "Open price should be positive");
            assert!(candle.high >= candle.open, "High should be >= open");
            assert!(candle.low <= candle.open, "Low should be <= open");
            assert!(candle.high >= candle.close, "High should be >= close");
            assert!(candle.low <= candle.close, "Low should be <= close");
            assert!(candle.volume > 0, "Volume should be positive");
        }
        
        // Test order book generation
        let order_book_result = service.generate_order_book("AAPL");
        assert!(order_book_result.is_ok(), "Order book generation should succeed");
        
        let order_book = order_book_result.unwrap();
        assert_eq!(order_book.symbol, "AAPL");
        assert!(!order_book.bids.is_empty(), "Should have bid entries");
        assert!(!order_book.asks.is_empty(), "Should have ask entries");
        
        // Verify order book integrity
        for i in 1..order_book.bids.len() {
            assert!(
                order_book.bids[i-1].price >= order_book.bids[i].price,
                "Bids should be sorted by price descending"
            );
        }
        
        for i in 1..order_book.asks.len() {
            assert!(
                order_book.asks[i-1].price <= order_book.asks[i].price,
                "Asks should be sorted by price ascending"
            );
        }
        
        // Test portfolio generation
        let portfolio_result = service.generate_mock_portfolio("TEST_ACCOUNT".to_string());
        assert!(portfolio_result.is_ok(), "Portfolio generation should succeed");
        
        let portfolio = portfolio_result.unwrap();
        assert_eq!(portfolio.account_id, "TEST_ACCOUNT");
        assert!(portfolio.cash_balance > 0.0, "Should have positive cash balance");
        assert!(portfolio.total_value > 0.0, "Should have positive total value");
    });
}

#[gpui::test]
async fn test_timeframe_functionality(cx: &mut TestAppContext) {
    // Test TimeFrame enum functionality
    assert_eq!(TimeFrame::OneMinute.to_seconds(), 60);
    assert_eq!(TimeFrame::FiveMinutes.to_seconds(), 300);
    assert_eq!(TimeFrame::OneHour.to_seconds(), 3600);
    assert_eq!(TimeFrame::OneDay.to_seconds(), 86400);
    
    assert_eq!(TimeFrame::OneMinute.display_name(), "1m");
    assert_eq!(TimeFrame::FiveMinutes.display_name(), "5m");
    assert_eq!(TimeFrame::OneHour.display_name(), "1h");
    assert_eq!(TimeFrame::OneDay.display_name(), "1d");
}

#[gpui::test]
async fn test_watchlist_item_functionality(cx: &mut TestAppContext) {
    // Test WatchlistItem creation and updates
    let watchlist_item = WatchlistItem::new("AAPL".to_string(), "Apple Inc.".to_string());
    assert!(watchlist_item.is_ok(), "Valid watchlist item should be created successfully");
    
    let mut item = watchlist_item.unwrap();
    assert_eq!(item.symbol, "AAPL");
    assert_eq!(item.name, "Apple Inc.");
    assert_eq!(item.current_price, 0.0);
    
    // Test market data update
    let market_data = MarketData::new("AAPL".to_string(), 150.0).unwrap();
    item.update_market_data(&market_data);
    
    assert_eq!(item.current_price, 150.0);
    assert_eq!(item.change, 0.0);
    assert_eq!(item.volume, 0);
    
    // Test invalid watchlist item creation
    let invalid_item_empty_symbol = WatchlistItem::new("".to_string(), "Apple Inc.".to_string());
    assert!(invalid_item_empty_symbol.is_err(), "Watchlist item with empty symbol should be rejected");
    
    let invalid_item_empty_name = WatchlistItem::new("AAPL".to_string(), "".to_string());
    assert!(invalid_item_empty_name.is_err(), "Watchlist item with empty name should be rejected");
}