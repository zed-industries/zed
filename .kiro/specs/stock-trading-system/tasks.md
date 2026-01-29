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

- [ ] 1. Set up stock trading crate structure with gpui-component integration
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

- [ ]* 1.1 Write GPUI property test for data structure validation using gpui-component
  - Use `TestAppContext` and `cx.background_executor().timer()` instead of `smol::Timer::after()` (AGENTS.md compliance)
  - Test gpui-component integration with Root component
  - Follow .rules: never use `unwrap()`, use `?` for error propagation
  - **Property 12: Order Validation**
  - **Validates: Requirements 5.2**

- [ ] 2. Implement all trading panels using gpui-component widgets
  - [ ] 2.1 Create panels.rs with enhanced UI components from gpui-component
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

  - [ ] 2.2 Implement enhanced data management with gpui-component features
    - Use gpui-component's virtualized Table for efficient large dataset handling
    - Leverage built-in Chart component for high-performance K-line rendering
    - Add/remove stock symbols with validation using `?` operator, never `unwrap()`
    - Store panel data using Zed's settings system
    - Display information using gpui-component's enhanced UI elements
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
  - [ ] 5.1 Create DataService entity for market data fetching
    - Implement GPUI entity with proper `Context<Self>` usage
    - Create HTTP-based market data API client using Zed's HTTP client
    - Add support for real-time quotes and historical data
    - Use `cx.background_spawn()` for network operations
    - Implement proper error propagation with `?` operator, never `unwrap()` (.rules compliance)
    - Use safe indexing with bounds checking for cache operations (.rules compliance)
    - Use full words for variable names (e.g., `historical_data` not `hist_data`) (.rules compliance)
    - _Requirements: 8.1, 8.2, 8.8_

  - [ ] 5.2 Implement caching and data management with memory cleanup
    - Add intelligent caching using HashMap with timestamp tracking
    - Implement automatic data refresh during market hours using timers
    - Add memory management and cleanup logic with thresholds
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

- [ ] 6. Checkpoint - Ensure core functionality works together
  - Run `./script/clippy` to check for lint errors and coding standard violations (AGENTS.md compliance)
  - Run `cargo nextest run -p stock_trading` to verify all tests pass (AGENTS.md compliance)
  - Verify all error handling follows `.rules` patterns (no `unwrap()`, proper `?` usage, `.log_err()` for visibility)
  - Check that variable names use full words without abbreviations (.rules compliance)
  - Ask the user if questions arise about data flow or error handling

- [ ] 7. Implement comprehensive error handling following Zed patterns
  - [ ] 7.1 Add network error handling with proper async patterns
    - Handle connection failures with offline status display
    - Implement API rate limiting with exponential backoff using `cx.spawn()`
    - Add graceful degradation for network issues with fallback UI
    - Use `?` operator for error propagation and `.log_err()` for visibility (.rules compliance)
    - Never silently discard errors with `let _ =` on fallible operations (.rules compliance)
    - Use explicit error handling with `match` or `if let Err(...)` for custom logic (.rules compliance)
    - _Requirements: 10.1, 10.2, 10.4, 10.9_

  - [ ] 7.2 Add input validation and error recovery
    - Validate stock symbols and order parameters without using `unwrap()` or panicking (.rules compliance)
    - Handle parsing errors with fallback to cached data using safe error patterns
    - Use proper error propagation in async contexts returning `anyhow::Result`
    - Ensure errors reach UI layer for user feedback
    - Use bounds checking for all indexing operations to prevent panics (.rules compliance)
    - _Requirements: 10.3, 10.5, 10.8_

  - [ ]* 7.3 Write GPUI property tests for error handling
    - Test error scenarios using `TestAppContext` and mock failures
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - **Property 29: Comprehensive Error Handling**
    - **Property 30: Rate Limiting Management**
    - **Validates: Requirements 10.1, 10.2, 10.3, 10.4, 10.5**

- [ ] 8. Implement action system integration following Zed patterns
  - [ ] 8.1 Add trading actions using Zed's action system
    - Define actions using `actions!` macro and `#[derive(Action)]`
    - Integrate with Zed's existing action dispatch system
    - Add keyboard shortcuts for panel toggles and trading operations
    - Use proper action handlers with `cx.listener()` pattern
    - Use full words for action names (e.g., `ToggleWatchlistPanel` not `ToggleWL`) (.rules compliance)
    - _Requirements: 1.1, 1.2, 1.3_

  - [ ]* 8.2 Write GPUI unit tests for action integration
    - Test action dispatch using `TestAppContext`
    - Test keyboard shortcuts and panel opening
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - _Requirements: 1.1, 1.3_

- [ ] 9. Implement settings and configuration with proper error handling
  - [ ] 9.1 Create trading system settings integration
    - Add StockTradingSettings struct with JSON schema
    - Integrate with Zed's existing settings system
    - Add configuration UI for refresh rates and API endpoints
    - Use proper error handling for settings validation (no `unwrap()`) (.rules compliance)
    - _Requirements: 9.1, 9.4_

  - [ ] 9.2 Add panel persistence and theme integration
    - Save panel positions and sizes between sessions
    - Update colors when themes change
    - Validate settings input with user feedback using proper error patterns
    - Use `.log_err()` for visibility when ignoring non-critical settings errors (.rules compliance)
    - _Requirements: 9.2, 9.3, 9.5_

  - [ ]* 9.3 Write property tests for settings management
    - **Property 26: Settings Persistence**
    - **Property 27: Theme Integration**
    - **Property 28: Settings Validation**
    - **Validates: Requirements 9.2, 9.3, 9.5**

- [ ] 10. Implement panel system integration with comprehensive testing
  - [ ] 10.1 Add advanced panel management features
    - Implement flexible panel docking to all supported positions
    - Add proportional layout maintenance during resize
    - Implement panel state restoration after close/reopen
    - Use safe indexing and bounds checking for panel operations (.rules compliance)
    - _Requirements: 7.1, 7.2, 7.3_

  - [ ] 10.2 Add multi-panel navigation and persistence
    - Implement tab-based navigation for multiple panels
    - Persist all panel configurations between sessions
    - Ensure smooth integration with Zed's workspace system
    - Handle panel lifecycle errors gracefully with proper error propagation (.rules compliance)
    - _Requirements: 7.4, 7.5_

  - [ ]* 10.3 Write property tests for panel system integration
    - **Property 18: Panel Docking Flexibility**
    - **Property 19: Layout Proportionality**
    - **Property 20: Panel State Restoration**
    - **Property 21: Data Persistence**
    - **Property 22: Multi-Panel Navigation**
    - **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.5**

- [ ] 11. Integration and final wiring using GPUI entity patterns
  - [ ] 11.1 Wire all entities together in TradingManager
    - Create central TradingManager entity as coordinator
    - Connect all panel entities through event subscription using `cx.subscribe()`
    - Implement cross-panel communication using `EventEmitter` traits
    - Use `WeakEntity` references to avoid circular dependencies
    - Handle entity lifecycle properly with subscription management
    - Use proper error handling throughout integration (no `unwrap()`, use `?`) (.rules compliance)
    - _Requirements: All requirements integration_

  - [ ] 11.2 Add trading system initialization to Zed Lite main.rs
    - Modify Zed Lite's main.rs to initialize trading system entities
    - Register all panels with the workspace using proper entity creation
    - Add trading actions to the application action registry
    - Follow Zed's initialization patterns and error handling
    - Use `stock_trading::init(cx)` pattern similar to other Zed components
    - _Requirements: System integration_

  - [ ]* 11.3 Write GPUI integration tests for complete system
    - Test end-to-end workflows using `TestAppContext`
    - Test multi-entity coordination and event flow
    - Use `cx.background_executor().timer()` for test timing (AGENTS.md compliance)
    - Follow .rules: never use `unwrap()`, use `?` for error propagation
    - _Requirements: Complete system validation_

- [ ] 12. Final checkpoint - Ensure complete system works
  - Run `./script/clippy` to check for all lint errors and coding standard violations (AGENTS.md compliance)
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