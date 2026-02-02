# Implementation Plan: Stock Trading System

## Overview

This implementation plan converts the stock trading system design into discrete coding tasks that build incrementally on Zed Lite's existing GPUI architecture. The approach follows Zed's established patterns for entity creation, panel integration, workspace management, and proper error handling using `?` operator and `.log_err()`.

The implementation prioritizes core functionality first (entities and basic data display) before adding advanced features (real-time updates, order management). Each task builds on previous work and includes validation through Zed's testing framework using `TestAppContext` and GPUI test patterns.

## Development Guidelines

- **Follow .rules strictly**: Use `?` for error propagation, `.log_err()` for visibility, never `let _ =` on fallible operations
- **Avoid many small files**: Prefer implementing functionality in existing files unless it's a new logical component (.rules)
- **No mod.rs files**: Use direct file paths like `src/panels.rs` instead of `src/panels/mod.rs` (.rules)
- **Use full words**: No abbreviations in variable names (e.g., `market_data` not `mkt_data`) (.rules)
- **Prioritize correctness**: Code correctness and clarity over speed/efficiency unless specified (.rules)
- **Variable shadowing**: Use for clarity in async contexts to minimize borrowed reference lifetimes (.rules)
- **GPUI patterns**: Use `cx.spawn()` and `cx.background_spawn()` for async operations
- **Entity management**: Proper `Context<T>`, `EventEmitter`, and `Render` trait implementations
- **Build tools**: Use `./script/clippy` instead of `cargo clippy`, `cargo nextest run` for tests (AGENTS.md)
- **Comments**: Only explain "why" for tricky/non-obvious code, no organizational comments (.rules)
- **Error handling**: Never use `unwrap()` or panic; use bounds checking with `.get()` for indexing operations (.rules)
- **License compliance**: Set `publish = false` in Cargo.toml to avoid license errors (.rules)

## Tasks

- [x] 1. Set up stock trading crate structure with gpui-component integration
  - Create `crates/stock_trading/` directory with proper Cargo.toml
  - Set library path to `stock_trading.rs` in Cargo.toml (avoid default `lib.rs`) (.rules compliance)
  - Add gpui-component dependency from GitHub repository
  - Set `publish = false` and proper license metadata to avoid license errors (.rules compliance)
  - Define core data structures for market data and orders with proper error types
  - Set up module exports and integrate with Zed Lite's HTTP client
  - Add dependencies: `gpui`, `gpui-component`, `anyhow`, `serde`, `futures`, `http_client`
  - Use full words for all variable names (no abbreviations) (.rules compliance)
  - Prioritize code correctness and clarity over speed/efficiency (.rules compliance)
  - _Requirements: 8.1, 8.4, 9.4_

- [x]* 1.1 Write GPUI property test for data structure validation using gpui-component
  - Use `TestAppContext` and `cx.background_executor().timer()` instead of `smol::Timer::after()` (AGENTS.md compliance)
  - Test gpui-component integration with Root component
  - Follow .rules: never use `unwrap()`, use `?` for error propagation
  - **Property 12: Order Validation**
  - **Validates: Requirements 5.2**

- [x] 1.2 Implement comprehensive financial data models with WebSocket support
  - Define enhanced MarketData structure with market status, day high/low, previous close
  - Create OrderBook structure with bid/ask spreads and order counts
  - Implement Portfolio and Position structures for account management
  - Add StockInfo structure for fundamental data (PE ratio, market cap, sector)
  - Define Trade structure for execution records
  - Add TimeInForce enum for order duration management
  - **Create WebSocket message structures for real-time updates**
  - **Define QuoteUpdate, TradeUpdate, OrderBookUpdate structures**
  - **Add Subscription management for WebSocket connections**
  - Use full words for all field names (e.g., `market_data` not `mkt_data`) (.rules compliance)
  - Include proper error handling for data validation (.rules compliance)
  - _Requirements: All data-related requirements_

- [x] 1.3 Create MockDataService and MockWebSocketService for development
  - Implement MockDataService with realistic stock data for AAPL, GOOGL, MSFT, TSLA, AMZN, NVDA, META
  - Generate realistic price movements using mathematical models (random walk, mean reversion)
  - Create mock order book data with proper bid/ask spreads
  - Implement historical data generation for all supported timeframes
  - Add mock portfolio data with positions and P&L calculations
  - Include market status simulation (pre-market, open, closed, after-hours)
  - **Implement MockWebSocketService for real-time data simulation**
  - **Add configurable update intervals and price volatility simulation**
  - **Generate realistic quote updates, trade executions, and order book changes**
  - Use proper error handling for mock data generation (.rules compliance)
  - Support real-time data simulation with configurable update intervals
  - _Requirements: 8.1, 8.2, 8.3_

- [x] 1.4 Implement WebSocket service for real-time data updates
  - Create WebSocketService entity with proper GPUI integration
  - Implement connection management with automatic reconnection
  - Add subscription management for symbols and message types
  - Implement message parsing and routing with proper error handling
  - Add heartbeat mechanism for connection keep-alive
  - Support multiple WebSocket endpoints and failover
  - Use `cx.background_spawn()` for WebSocket operations (.rules compliance)
  - Implement proper error propagation with `?` operator, never `unwrap()` (.rules compliance)
  - Use `.log_err()` for connection error visibility (.rules compliance)
  - Add bounds checking for message parsing (.rules compliance)
  - _Requirements: 8.1, 8.2, 8.7, 10.1, 10.2_

- [ ]* 1.5 Write property tests for WebSocket functionality
  - Test WebSocket connection and reconnection logic
  - Validate message serialization/deserialization
  - Test subscription management and message routing
  - Verify error handling for connection failures
  - Test mock WebSocket service simulation accuracy
  - Use `TestAppContext` and proper async testing patterns (AGENTS.md compliance)
  - Follow .rules: never use `unwrap()`, use `?` for error propagation
  - **Property 24: Stale Data Refresh** (via WebSocket updates)
  - **Property 29: Comprehensive Error Handling**
  - **Property 30: Rate Limiting Management**
  - **Validates: Requirements 8.2, 8.3, 10.1, 10.2**
- [ ]* 1.6 Write property tests for financial data models
  - Test data model serialization/deserialization with various inputs
  - Validate price calculation accuracy (P&L, spreads, percentages)
  - Test order book consistency (bid prices < ask prices)
  - Verify portfolio calculations (total value, unrealized P&L)
  - Test WebSocket message format consistency
  - Use `TestAppContext` and proper error handling (.rules compliance)
  - **Property 23: Data Caching Efficiency**
  - **Property 25: Memory Management**
  - **Validates: Requirements 8.1, 8.5**

- [-] 2. Implement all trading panels using gpui-component widgets
  - [x] 2.1 Create panels.rs with enhanced UI components from gpui-component
    - Implement WatchlistPanel using gpui-component's virtualized Table for stock list
    - Implement ChartPanel using gpui-component's built-in Chart for K-line display
    - Implement StockInfoPanel using gpui-component's layout and display components
    - Implement OrderPanel using gpui-component's Button, Input, and form controls
    - Implement OrderBookPanel using gpui-component's virtualized Table for bid/ask data
    - Follow Zed's preference for fewer, more substantial files instead of many small files (.rules compliance)
    - Add proper GPUI entities with `Context<Self>` usage and focus handling
    - Use gpui-component's Root component as the base for all panels
    - Use `EventEmitter<PanelEvent>` for inter-component communication
    - Use full words for all variable names (e.g., `watchlist_data` not `wl_data`) (.rules compliance)
    - Prioritize code correctness and clarity over speed/efficiency (.rules compliance)
    - _Requirements: 2.1, 3.1, 4.1, 5.1, 6.1, 7.1_

  - [ ] 2.2 Implement enhanced data management with real-time WebSocket updates
    - Use gpui-component's virtualized Table for efficient large dataset handling
    - Leverage built-in Chart component for high-performance K-line rendering
    - Add/remove stock symbols with validation using `?` operator, never `unwrap()`
    - Store panel data using Zed's settings system
    - Display information using gpui-component's enhanced UI elements
    - **Integrate WebSocket service for real-time price updates**
    - **Add real-time data subscription management per panel**
    - **Implement automatic UI updates when WebSocket data arrives**
    - Use `cx.notify()` when state changes affect rendering
    - Handle async operations with `cx.spawn()` and proper error propagation
    - Use `.log_err()` for visibility when ignoring non-critical errors (.rules compliance)
    - Never silently discard errors with `let _ =` on fallible operations (.rules compliance)
    - Use bounds checking with `.get()` instead of direct indexing `[]` (.rules compliance)
    - _Requirements: 2.2, 2.3, 2.5, 8.4, 8.5, 10.4_

  - [ ]* 2.3 Write GPUI property tests for enhanced panel operations
    - Use `TestAppContext` and `cx.background_executor().timer()` for timing
    - Test gpui-component Table and Chart integration
    - **Property 2: Watchlist Data Management**
    - **Property 3: Watchlist Item Removal**
    - **Property 5: Panel Positioning Consistency**
    - **Property 6: Chart Data Rendering** (enhanced with gpui-component Chart)
    - **Validates: Requirements 2.2, 2.3, 3.1, 3.2, 4.1, 6.1**

  - [ ] 2.4 Add enhanced UI interactions using gpui-component controls
    - Use gpui-component Button for consistent styling and behavior
    - Use gpui-component Input for stock symbol entry with validation
    - Handle stock selection using `cx.listener()` pattern with Table row clicks
    - Display empty states with helpful instructions using gpui-component layouts
    - Implement input validation with proper error messages (no panicking) (.rules compliance)
    - Use action system for keyboard shortcuts
    - Use explicit error handling with `match` or `if let Err(...)` for custom logic (.rules compliance)
    - _Requirements: 2.4, 2.6, 10.3, 10.9_

  - [ ]* 2.5 Write GPUI unit tests for enhanced panel UI edge cases
    - Test empty panel displays using `TestAppContext` and gpui-component widgets
    - Test invalid input handling with proper error propagation (.rules compliance)
    - Test bounds checking for virtualized Table operations (.rules compliance)
    - Test Chart component error handling and fallback states
    - Use `cx.background_executor().timer()` instead of `smol::Timer::after()` (AGENTS.md compliance)
    - _Requirements: 2.6, 8.8, 10.3_

- [ ] 3. Checkpoint - Ensure all panels work correctly
  - Run `./script/clippy` to check for lint errors (AGENTS.md compliance)
  - Run `cargo nextest run -p stock_trading` to verify all tests pass (AGENTS.md compliance)
  - Ensure all error handling follows `.rules` patterns (no `unwrap()`, proper `?` usage, `.log_err()` for visibility)
  - Verify all variable names use full words without abbreviations (.rules compliance)
  - Ask the user if questions arise about panel behavior or integration

- [ ] 4. Implement enhanced chart rendering with gpui-component Chart
  - [ ] 4.1 Integrate gpui-component Chart for professional K-line display
    - Use gpui-component's built-in Chart component with candlestick support
    - Configure Chart with proper OHLC data binding and styling
    - Add timeframe selection using gpui-component Button group
    - Implement zoom and pan functionality using Chart's built-in features
    - Use `cx.listener()` for proper event handling with Chart interactions
    - Handle rendering errors gracefully with `.log_err()`, never `unwrap()` (.rules compliance)
    - Use safe indexing with bounds checking for chart data access (.rules compliance)
    - Leverage GPU acceleration for smooth chart rendering performance
    - _Requirements: 3.2, 3.3, 3.4, 8.8_

  - [ ] 4.2 Connect enhanced chart to panel selection using event system
    - Subscribe to panel events using `cx.subscribe()`
    - Update Chart component when stock is selected using `cx.notify()`
    - Handle timeframe changes with proper async data updates
    - Use proper error propagation for data loading failures with `?` operator (.rules compliance)
    - Use variable shadowing in async contexts for clarity (.rules compliance)
    - Implement Chart data validation and error states
    - _Requirements: 3.5, 8.7_

  - [ ]* 4.3 Write GPUI property tests for enhanced chart functionality
    - Use `TestAppContext` for Chart component testing
    - Use `cx.background_executor().timer()` instead of `smol::Timer::after()` (AGENTS.md compliance)
    - Test gpui-component Chart integration and performance
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 6: Chart Data Rendering** (enhanced with gpui-component)
    - **Property 7: Chart Interaction Functionality**
    - **Property 8: Timeframe Support**
    - **Property 9: Timeframe Transition**
    - **Validates: Requirements 3.2, 3.3, 3.4, 3.5**

- [ ] 5. Implement data service entity with strict error handling patterns
  - [ ] 5.1 Create DataService entity with WebSocket integration
    - Implement GPUI entity with proper `Context<Self>` usage
    - Create HTTP-based market data API client using Zed's HTTP client
    - Integrate MockDataService for development and testing
    - Add configuration flag to switch between mock and real data
    - Add support for real-time quotes and historical data
    - **Integrate WebSocketService for real-time data streaming**
    - **Implement automatic fallback from WebSocket to HTTP polling**
    - **Add WebSocket subscription management tied to UI panel visibility**
    - Use `cx.background_spawn()` for network operations
    - Implement proper error propagation with `?` operator, never `unwrap()` (.rules compliance)
    - Use safe indexing with bounds checking for cache operations (.rules compliance)
    - Use full words for variable names (e.g., `historical_data` not `hist_data`) (.rules compliance)
    - _Requirements: 8.1, 8.2, 8.8_

  - [ ] 5.2 Implement caching and real-time data management
    - Add intelligent caching using HashMap with timestamp tracking
    - Implement automatic data refresh during market hours using timers
    - Add memory management and cleanup logic with thresholds
    - Integrate mock data simulation with realistic price movements
    - Add configurable update intervals for real-time simulation
    - Support switching between mock and live data sources
    - **Implement WebSocket message caching and deduplication**
    - **Add real-time data validation and quality checks**
    - **Implement automatic WebSocket reconnection with exponential backoff**
    - Use `cx.spawn()` for periodic cleanup tasks
    - Handle cache errors with `.log_err()` for visibility, never `let _ =` (.rules compliance)
    - Use explicit error handling with `match` or `if let Err(...)` for custom logic (.rules compliance)
    - _Requirements: 8.1, 8.3, 8.5, 8.9_

  - [ ]* 5.3 Write GPUI property tests for data management
    - Use `TestAppContext` and mock HTTP responses
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 23: Data Caching Efficiency**
    - **Property 24: Stale Data Refresh**
    - **Property 25: Memory Management**
    - **Validates: Requirements 8.1, 8.3, 8.5**

- [ ] 6.5 Implement WebSocket real-time data streaming
  - [ ] 6.5.1 Create WebSocket connection management
    - Implement WebSocketService entity with tokio-tungstenite integration
    - Add connection state management (connecting, connected, disconnected, error)
    - Implement automatic reconnection with exponential backoff strategy
    - Add connection health monitoring with heartbeat/ping-pong
    - Support multiple WebSocket endpoints with failover capability
    - Use proper async patterns with `cx.background_spawn()` (.rules compliance)
    - Implement comprehensive error handling with `?` operator (.rules compliance)
    - _Requirements: 8.2, 10.1, 10.2_

  - [ ] 6.5.2 Add real-time subscription management
    - Implement symbol-based subscription system
    - Add message type filtering (quotes, trades, order book updates)
    - Create subscription lifecycle management (subscribe/unsubscribe)
    - Implement subscription persistence across reconnections
    - Add subscription rate limiting and throttling
    - Use bounds checking for subscription management (.rules compliance)
    - Never use `unwrap()` for subscription operations (.rules compliance)
    - _Requirements: 8.1, 8.3, 10.2_

  - [ ] 6.5.3 Implement real-time message processing
    - Add WebSocket message parsing and validation
    - Implement message routing to appropriate panels
    - Add message deduplication and ordering
    - Create real-time data quality checks
    - Implement message buffering for high-frequency updates
    - Use `.log_err()` for message processing errors (.rules compliance)
    - Add explicit error handling for malformed messages (.rules compliance)
    - _Requirements: 8.4, 8.7, 10.3_

  - [ ]* 6.5.4 Write WebSocket integration tests
    - Test WebSocket connection and reconnection scenarios
    - Validate subscription management and message routing
    - Test error handling for network failures and malformed data
    - Verify real-time data flow from WebSocket to UI panels
    - Test mock WebSocket service simulation accuracy
    - Use `TestAppContext` and `cx.background_executor().timer()` (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 24: Stale Data Refresh**
    - **Property 29: Comprehensive Error Handling**
    - **Property 30: Rate Limiting Management**
    - **Validates: Requirements 8.2, 8.3, 10.1, 10.2**
- [ ] 7. Checkpoint - Ensure core functionality and WebSocket integration work together
  - Run `./script/clippy` to check for lint errors and coding standard violations (AGENTS.md compliance)
  - Run `cargo nextest run -p stock_trading` to verify all tests pass (AGENTS.md compliance)
  - Verify all error handling follows `.rules` patterns (no `unwrap()`, proper `?` usage, `.log_err()` for visibility)
  - Check that variable names use full words without abbreviations (.rules compliance)
  - Test WebSocket connection and real-time data flow
  - Verify mock WebSocket service provides realistic data simulation
  - Ask the user if questions arise about data flow or error handling

- [ ] 8. Implement comprehensive error handling following Zed patterns
  - [ ] 8.1 Add network error handling with proper async patterns
    - Handle connection failures with offline status display
    - Implement API rate limiting with exponential backoff using `cx.spawn()`
    - Add graceful degradation for network issues with fallback UI
    - Use `?` operator for error propagation and `.log_err()` for visibility (.rules compliance)
    - Never silently discard errors with `let _ =` on fallible operations (.rules compliance)
    - Use explicit error handling with `match` or `if let Err(...)` for custom logic (.rules compliance)
    - _Requirements: 10.1, 10.2, 10.4, 10.9_

  - [ ] 8.2 Add input validation and error recovery
    - Validate stock symbols and order parameters without using `unwrap()` or panicking (.rules compliance)
    - Handle parsing errors with fallback to cached data using safe error patterns
    - Use proper error propagation in async contexts returning `anyhow::Result`
    - Ensure errors reach UI layer for user feedback
    - Use bounds checking for all indexing operations to prevent panics (.rules compliance)
    - _Requirements: 10.3, 10.5, 10.8_

  - [ ]* 8.3 Write GPUI property tests for error handling
    - Test error scenarios using `TestAppContext` and mock failures
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 29: Comprehensive Error Handling**
    - **Property 30: Rate Limiting Management**
    - **Validates: Requirements 10.1, 10.2, 10.3, 10.4, 10.5**

- [ ] 9. Implement action system integration following Zed patterns
  - [ ] 9.1 Add trading actions using Zed's action system
    - Define actions using `actions!` macro and `#[derive(Action)]`
    - Integrate with Zed's existing action dispatch system
    - Add keyboard shortcuts for panel toggles and trading operations
    - Use proper action handlers with `cx.listener()` pattern
    - Use full words for action names (e.g., `ToggleWatchlistPanel` not `ToggleWL`) (.rules compliance)
    - _Requirements: 1.1, 1.2, 1.3_

  - [ ]* 9.2 Write GPUI unit tests for action integration
    - Test action dispatch using `TestAppContext`
    - Test keyboard shortcuts and panel opening
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - _Requirements: 1.1, 1.3_

- [ ] 10. Implement settings and configuration with proper error handling
  - [ ] 10.1 Create trading system settings integration
    - Add StockTradingSettings struct with JSON schema
    - Integrate with Zed's existing settings system
    - Add configuration UI for refresh rates and API endpoints
    - Use proper error handling for settings validation (no `unwrap()`) (.rules compliance)
    - _Requirements: 9.1, 9.4_

  - [ ] 10.2 Add panel persistence and theme integration
    - Save panel positions and sizes between sessions
    - Update colors when themes change
    - Validate settings input with user feedback using proper error patterns
    - Use `.log_err()` for visibility when ignoring non-critical settings errors (.rules compliance)
    - _Requirements: 9.2, 9.3, 9.5_

  - [ ]* 10.3 Write property tests for settings management
    - **Property 26: Settings Persistence**
    - **Property 27: Theme Integration**
    - **Property 28: Settings Validation**
    - **Validates: Requirements 9.2, 9.3, 9.5**

- [ ] 11. Implement panel system integration with comprehensive testing
  - [ ] 11.1 Add advanced panel management features
    - Implement flexible panel docking to all supported positions
    - Add proportional layout maintenance during resize
    - Implement panel state restoration after close/reopen
    - Use safe indexing and bounds checking for panel operations (.rules compliance)
    - _Requirements: 7.1, 7.2, 7.3_

  - [ ] 11.2 Add multi-panel navigation and persistence
    - Implement tab-based navigation for multiple panels
    - Persist all panel configurations between sessions
    - Ensure smooth integration with Zed's workspace system
    - Handle panel lifecycle errors gracefully with proper error propagation (.rules compliance)
    - _Requirements: 7.4, 7.5_

  - [ ]* 11.3 Write property tests for panel system integration
    - **Property 18: Panel Docking Flexibility**
    - **Property 19: Layout Proportionality**
    - **Property 20: Panel State Restoration**
    - **Property 21: Data Persistence**
    - **Property 22: Multi-Panel Navigation**
    - **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.5**

- [ ] 12. Integration and final wiring using GPUI entity patterns
  - [ ] 12.1 Wire all entities together in TradingManager
    - Create central TradingManager entity as coordinator
    - Connect all panel entities through event subscription using `cx.subscribe()`
    - Implement cross-panel communication using `EventEmitter` traits
    - Use `WeakEntity` references to avoid circular dependencies
    - Handle entity lifecycle properly with subscription management
    - Use proper error handling throughout integration (no `unwrap()`, use `?`) (.rules compliance)
    - _Requirements: All requirements integration_

  - [ ] 12.2 Add trading system initialization to Zed Lite main.rs
    - Modify Zed Lite's main.rs to initialize trading system entities
    - Register all panels with the workspace using proper entity creation
    - Add trading actions to the application action registry
    - Follow Zed's initialization patterns and error handling
    - Use `stock_trading::init(cx)` pattern similar to other Zed components
    - _Requirements: System integration_

  - [ ]* 12.3 Write GPUI integration tests for complete system
    - Test end-to-end workflows using `TestAppContext`
    - Test multi-entity coordination and event flow
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - _Requirements: Complete system validation_

- [ ] 13. API接口设计和WebSocket实时数据集成
  - [ ] 13.1 设计标准化的金融数据API接口
    - 定义统一的API trait用于市场数据获取
    - 设计RESTful API接口规范（获取股票信息、历史数据、实时报价）
    - **定义WebSocket接口规范用于实时数据推送**
    - **创建WebSocket消息协议和数据格式标准**
    - **设计WebSocket订阅管理和心跳机制**
    - 创建API响应数据结构和错误处理模式
    - 支持多种数据源（Alpha Vantage, Yahoo Finance, IEX Cloud, Polygon.io等）
    - 使用full words命名所有API相关结构 (.rules compliance)
    - 实现proper error propagation with `?` operator (.rules compliance)
    - _Requirements: 8.1, 8.2, 10.1_

  - [ ] 13.2 实现API抽象层和WebSocket适配器模式
    - 创建DataProvider trait定义统一接口
    - 实现MockDataProvider用于开发测试
    - **实现MockWebSocketProvider用于实时数据模拟**
    - 设计RealDataProvider框架用于接入真实API
    - **设计RealWebSocketProvider用于真实WebSocket连接**
    - 添加API配置管理（API密钥、端点URL、WebSocket URL、限流设置）
    - 实现API响应缓存和错误重试机制
    - **实现WebSocket重连机制和消息缓冲**
    - 支持多个数据源的fallback机制
    - 使用proper async patterns with `cx.background_spawn()` (.rules compliance)
    - Never use `unwrap()` for API responses (.rules compliance)
    - _Requirements: 8.1, 8.4, 10.1, 10.2_

  - [ ] 13.3 准备生产环境API和WebSocket集成
    - 创建API密钥管理和安全存储机制
    - 实现API限流和配额管理
    - **实现WebSocket连接池和负载均衡**
    - **添加WebSocket消息压缩和优化**
    - 添加API监控和日志记录
    - **添加WebSocket连接监控和性能指标**
    - 设计API数据质量检查和验证
    - **实现实时数据质量监控和异常检测**
    - 实现数据源切换的热更新机制
    - 添加API性能监控和告警
    - Use `.log_err()` for API error visibility (.rules compliance)
    - Implement bounds checking for API response parsing (.rules compliance)
    - _Requirements: 8.4, 10.1, 10.2, 10.4_

  - [ ]* 13.4 编写API和WebSocket集成测试
    - 测试MockDataProvider和RealDataProvider的一致性
    - **测试MockWebSocketProvider和RealWebSocketProvider的一致性**
    - 验证API错误处理和重试机制
    - **验证WebSocket重连和消息处理机制**
    - 测试数据源切换和fallback功能
    - **测试WebSocket订阅管理和消息路由**
    - 验证API限流和配额管理
    - **验证WebSocket连接池和负载均衡**
    - 使用`TestAppContext`进行异步API测试 (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 23: Data Caching Efficiency**
    - **Property 24: Stale Data Refresh** (via WebSocket)
    - **Property 29: Comprehensive Error Handling**
    - **Property 30: Rate Limiting Management**
    - **Validates: Requirements 8.1, 8.2, 8.3, 10.1, 10.2**
- [ ] 14. Final checkpoint - Ensure complete system with WebSocket works
  - Run `./script/clippy` to check for all lint errors and coding standard violations (AGENTS.md compliance)
  - Run `cargo nextest run -p stock_trading` to verify all tests pass (AGENTS.md compliance)
  - Run `cargo doc --workspace --no-deps --open` to verify documentation builds (AGENTS.md compliance)
  - Verify all error handling follows `.rules` patterns throughout the codebase
  - Check that all variable names use full words without abbreviations (.rules compliance)
  - Ensure no `mod.rs` files were created and all files use direct paths (.rules compliance)
  - Test mock data service with realistic financial scenarios
  - **Test WebSocket real-time data streaming and UI updates**
  - **Verify WebSocket reconnection and error handling**
  - **Test mock WebSocket service simulation accuracy**
  - Verify API abstraction layer is ready for production integration
  - **Verify WebSocket abstraction layer supports multiple data sources**
  - Ask the user if questions arise about the complete system integration
  - Run `cargo nextest run -p stock_trading` to verify all tests pass (AGENTS.md compliance)
  - Run `cargo doc --workspace --no-deps --open` to verify documentation builds (AGENTS.md compliance)
  - Verify all error handling follows `.rules` patterns throughout the codebase
  - Check that all variable names use full words without abbreviations (.rules compliance)
  - Ensure no `mod.rs` files were created and all files use direct paths (.rules compliance)
  - Ask the user if questions arise about the complete system integration

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- **Strict adherence to .rules**: Use `?` for error propagation, `.log_err()` for visibility, never `let _ =` on fallible operations
- **File organization**: Prefer fewer, more substantial files; avoid creating many small files unless new logical components (.rules)
- **No mod.rs files**: Use direct file paths like `src/panels.rs` instead of `src/panels/mod.rs` (.rules)
- **Variable naming**: Use full words without abbreviations (e.g., `market_data` not `mkt_data`) (.rules)
- **Error handling**: Never use `unwrap()` or panic; use bounds checking with `.get()` for indexing operations (.rules)
- **Testing**: Use `cargo nextest run` and GPUI's `TestAppContext` with `cx.background_executor().timer()` (AGENTS.md)
- **Build tools**: Use `./script/clippy` instead of `cargo clippy` for linting (AGENTS.md)
- **Comments**: Only explain "why" for tricky/non-obvious code, no organizational comments (.rules)
- **Async patterns**: Use variable shadowing in async contexts for clarity (.rules)
- **Entity management**: Follow GPUI patterns with proper context usage and lifecycle management
- **License compliance**: Set `publish = false` in Cargo.toml to avoid license errors (.rules)