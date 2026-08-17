# Implementation Plan

- [x] 1. Extend command line argument parsing for benchmark options
  - Add `-b` flag to enable benchmark mode in the argument parser
  - Add `--size` parameter for specifying random data size
  - Add `--duration` parameter for benchmark duration (floating-point seconds)
  - Update the `Config` struct to include optional `BenchmarkConfig`
  - Update usage/help text to include new benchmark options
  - _Requirements: 1.1, 2.1, 2.2, 2.3, 2.4_

- [x] 2. Implement benchmark data structures and validation
  - Create `BenchmarkConfig` struct with size and duration fields
  - Create `BenchmarkData` enum to handle in-memory vs file data sources
  - Create `BenchmarkRunner` struct with algorithm, data, and duration
  - Create `BenchmarkResult` struct with all metrics including time per iteration
  - Add validation logic for benchmark parameters (positive values)
  - _Requirements: 2.5, 3.4_

- [x] 3. Implement benchmark execution logic
  - Create benchmark runner with timing loop using `std::time::Instant`
  - Implement separate execution paths for in-memory data vs file data
  - Use `std::hint::black_box()` to prevent compiler optimizations
  - Calculate throughput in GiB/s and time per iteration with appropriate units
  - Integrate `get_calculator_target()` for acceleration target reporting
  - _Requirements: 1.2, 1.3, 1.4, 1.5_

- [x] 4. Implement data source handling
  - Add random data generation function using `rand::RngCore::fill_bytes()`
  - Implement logic to determine data source (file, string, or generated)
  - Handle file size detection for throughput calculations
  - Create `BenchmarkData` instances based on user input
  - _Requirements: 3.1, 3.2, 3.3_

- [x] 5. Integrate benchmark mode into main application flow
  - Modify main function to detect benchmark mode and route accordingly
  - Ensure mutual exclusivity validation between benchmark and normal modes
  - Add benchmark result formatting and display
  - Update error handling to include benchmark-specific errors
  - Maintain backward compatibility with existing functionality
  - _Requirements: 3.4, 3.5_

- [x] 6. Add comprehensive testing for benchmark functionality
  - Write unit tests for argument parsing with benchmark flags
  - Test benchmark parameter validation (invalid sizes, durations)
  - Test data source selection logic (file vs string vs generated)
  - Test benchmark execution with different algorithms
  - Verify throughput calculation accuracy
  - Test error handling for invalid benchmark configurations
  - _Requirements: All requirements_