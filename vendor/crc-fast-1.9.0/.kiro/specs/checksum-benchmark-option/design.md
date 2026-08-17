# Design Document

## Overview

This design extends the existing `bin/checksum.rs` tool with benchmark functionality through a new `-b` flag. The benchmark mode will measure CRC performance using either user-provided data (files/strings) or randomly generated data, reporting throughput in GiB/s along with the acceleration target used.

The design maintains backward compatibility while adding a clean benchmark interface that leverages existing patterns from the `benches/benchmark.rs` implementation.

## Architecture

### Command Line Interface

The tool will extend the existing argument parsing to support:
- `-b`: Enable benchmark mode
- `--size <bytes>`: Specify data size for random generation (when no file/string provided)
- `--duration <seconds>`: Benchmark duration as floating-point seconds (default: 10.0)
- Existing `-a <algorithm>`: CRC algorithm (required in benchmark mode)
- Existing `-f <file>` or `-s <string>`: Optional data source for benchmarking

### Data Flow

```
User Input → Argument Parsing → Mode Detection → Benchmark Execution → Results Display
                                      ↓
                              [Normal Checksum Mode]
                                      ↓
                              [Existing Functionality]
```

In benchmark mode:
1. Parse and validate benchmark parameters
2. Determine data source (file, string, or generated)
3. For string/generated data: Load/generate test data once; For file data: use file path directly
4. Run benchmark loop for specified duration using appropriate checksum function
5. Calculate and display results

## Components and Interfaces

### Enhanced Config Structure

```rust
#[derive(Debug)]
struct Config {
    algorithm: String,
    file: Option<String>,
    string: Option<String>,
    format: OutputFormat,
    benchmark: Option<BenchmarkConfig>,
}

#[derive(Debug)]
struct BenchmarkConfig {
    size: Option<usize>,
    duration: f64,
}
```

### Benchmark Execution Module

```rust
enum BenchmarkData {
    InMemory(Vec<u8>),
    File(String),
}

struct BenchmarkRunner {
    algorithm: CrcAlgorithm,
    data: BenchmarkData,
    duration: f64,
}

impl BenchmarkRunner {
    fn new(algorithm: CrcAlgorithm, data: BenchmarkData, duration: f64) -> Self
    fn run(&self) -> BenchmarkResult
}

struct BenchmarkResult {
    iterations: u64,
    elapsed_seconds: f64,
    throughput_gibs: f64,
    time_per_iteration_nanos: f64,
    acceleration_target: String,
    data_size: u64,
}
```

### Data Generation

The benchmark will reuse the random data generation pattern from `benches/benchmark.rs`:

```rust
fn generate_random_data(size: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut buf = vec![0u8; size];
    rng.fill_bytes(&mut buf);
    buf
}
```

## Data Models

### Input Data Sources

1. **File Input**: Use `checksum_file()` function to benchmark the entire file I/O and checksum stack
2. **String Input**: Use string bytes directly with in-memory `checksum()` function
3. **Generated Data**: Create random data of specified size using `rand::RngCore::fill_bytes()` and use in-memory `checksum()` function

### Benchmark Metrics

- **Iterations**: Number of checksum calculations performed
- **Elapsed Time**: Actual benchmark duration in seconds
- **Throughput**: Calculated as `(data_size * iterations) / elapsed_time / (1024^3)` GiB/s
- **Acceleration Target**: Result from `crc_fast::get_calculator_target(algorithm)`

## Error Handling

### Validation Errors

- Invalid algorithm names (reuse existing validation)
- Invalid size parameters (non-positive values)
- Invalid duration parameters (non-positive values)
- File read errors (reuse existing error handling)

### Runtime Errors

- Memory allocation failures for large data sizes
- Timer precision issues (fallback to alternative timing methods)

### Error Messages

All errors will follow the existing pattern of displaying the error message followed by usage information.

## Testing Strategy

### Unit Tests

- Argument parsing validation for benchmark flags
- BenchmarkConfig creation and validation
- Data generation with various sizes
- Throughput calculation accuracy

### Integration Tests

- End-to-end benchmark execution with different algorithms
- File and string input handling in benchmark mode
- Error handling for invalid parameters
- Backward compatibility verification

### Performance Validation

- Verify benchmark results are reasonable (within expected ranges)
- Compare with existing `benches/benchmark.rs` results for consistency
- Test with various data sizes to ensure linear scaling

## Implementation Notes

### Timing Mechanism

Use `std::time::Instant` for high-precision timing, with different approaches for different data sources:

```rust
let start = std::time::Instant::now();
let mut iterations = 0u64;

while start.elapsed().as_secs_f64() < duration {
    match &self.data {
        BenchmarkData::InMemory(data) => {
            std::hint::black_box(checksum(algorithm, data));
        }
        BenchmarkData::File(filename) => {
            std::hint::black_box(checksum_file(algorithm, filename, None).unwrap());
        }
    }
    iterations += 1;
}

let elapsed = start.elapsed().as_secs_f64();
```

### Memory Considerations

- Pre-allocate test data once before benchmark loop
- Use `std::hint::black_box()` to prevent compiler optimizations
- Consider memory alignment for optimal performance (optional enhancement)

### Output Format

```
Algorithm: CRC-32/ISCSI
Acceleration Target: aarch64-neon-sha3
Data Size: 1,048,576 bytes (1.0 MiB)
Duration: 10.00 seconds
Iterations: 12,345
Throughput: 45.67 GiB/s
Time per iteration: 810.2 μs
```

### Default Values

- **Size**: 1,048,576 bytes (1 MiB)
- **Duration**: 10.0 seconds
- **Algorithm**: Must be specified via `-a` flag (no default)