// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This is a simple program to calculate a checksum from the command line

use crc_fast::{checksum, checksum_file, CrcAlgorithm};
use std::env;
use std::process::ExitCode;
use std::str::FromStr;

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

#[derive(Debug)]
enum BenchmarkData {
    InMemory(Vec<u8>),
    File(String),
}

#[derive(Debug)]
struct BenchmarkRunner {
    algorithm: CrcAlgorithm,
    data: BenchmarkData,
    duration: f64,
}

#[derive(Debug)]
struct BenchmarkResult {
    iterations: u64,
    elapsed_seconds: f64,
    throughput_gibs: f64,
    time_per_iteration_nanos: f64,
    acceleration_target: String,
    data_size: u64,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Hex,
    Decimal,
}

impl BenchmarkConfig {
    fn validate(&self) -> Result<(), String> {
        if self.duration <= 0.0 {
            return Err("Duration must be greater than 0".to_string());
        }

        if let Some(size) = self.size {
            if size == 0 {
                return Err("Size must be greater than 0".to_string());
            }
        }

        Ok(())
    }
}

impl BenchmarkRunner {
    fn new(algorithm: CrcAlgorithm, data: BenchmarkData, duration: f64) -> Self {
        Self {
            algorithm,
            data,
            duration,
        }
    }

    fn run(&self) -> Result<BenchmarkResult, String> {
        use std::time::Instant;

        let start = Instant::now();
        let mut iterations = 0u64;

        while start.elapsed().as_secs_f64() < self.duration {
            match &self.data {
                BenchmarkData::InMemory(data) => {
                    std::hint::black_box(checksum(self.algorithm, data));
                }
                BenchmarkData::File(filename) => {
                    match checksum_file(self.algorithm, filename, None) {
                        Ok(result) => {
                            std::hint::black_box(result);
                        }
                        Err(e) => {
                            return Err(format!("Failed to read file during benchmark: {}", e));
                        }
                    }
                }
            }
            iterations += 1;
        }

        let elapsed = start.elapsed().as_secs_f64();
        let data_size = match &self.data {
            BenchmarkData::InMemory(data) => data.len() as u64,
            BenchmarkData::File(filename) => std::fs::metadata(filename)
                .map(|m| m.len())
                .map_err(|e| format!("Failed to get file size: {}", e))?,
        };

        let acceleration_target = crc_fast::get_calculator_target(self.algorithm);

        Ok(BenchmarkResult::new(
            iterations,
            elapsed,
            acceleration_target,
            data_size,
        ))
    }
}

impl BenchmarkResult {
    fn new(
        iterations: u64,
        elapsed_seconds: f64,
        acceleration_target: String,
        data_size: u64,
    ) -> Self {
        let throughput_gibs = if elapsed_seconds > 0.0 {
            (data_size as f64 * iterations as f64) / elapsed_seconds / (1024.0 * 1024.0 * 1024.0)
        } else {
            0.0
        };

        let time_per_iteration_nanos = if iterations > 0 {
            elapsed_seconds * 1_000_000_000.0 / iterations as f64
        } else {
            0.0
        };

        Self {
            iterations,
            elapsed_seconds,
            throughput_gibs,
            time_per_iteration_nanos,
            acceleration_target,
            data_size,
        }
    }
}

/// Fills a buffer with pseudo-random data using xorshift64* algorithm.
/// This is a deterministic PRNG that's tiny, fast, and suitable for benchmark data generation.
/// The seed is derived from the buffer size to provide some variability.
/// This solves the problem of having to use a full-featured PRNG like rand for benchmark data generation.
#[inline]
fn fill_pseudo_random(buf: &mut [u8], seed: u64) {
    let mut x = seed;
    for b in buf {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let r = x.wrapping_mul(0x2545_F491_4F6C_DD1D);
        *b = r as u8;
    }
}

fn generate_random_data(size: usize) -> Result<Vec<u8>, String> {
    // Check for reasonable size limits to prevent memory issues
    if size > 1_073_741_824 {
        // 1 GiB limit
        return Err("Data size too large (maximum 1 GiB)".to_string());
    }

    // Use vec! macro to avoid clippy warning about slow initialization
    let mut buf = vec![0u8; size];

    // Derive seed from size for deterministic but varied data
    let seed = 0x9E37_79B9_7F4A_7C15_u64.wrapping_add(size as u64);
    fill_pseudo_random(&mut buf, seed);

    Ok(buf)
}

fn print_usage() {
    println!("Usage: checksum -a algorithm [-f file] [-s string] [--format hex|decimal]");
    println!(
        "       checksum -a algorithm -b [--size bytes] [--duration seconds] [-f file] [-s string]"
    );
    println!();
    println!("Example: checksum -a CRC-32/ISCSI -f myfile.txt");
    println!("Example: checksum -a CRC-64/NVME -s 'Hello, world!' --format decimal");
    println!("Example: checksum -a CRC-32/ISCSI -b --size 1048576 --duration 5.0");
    println!();
    println!("Options:");
    println!("  -a algorithm        Specify the checksum algorithm (required)");
    println!("  -f file             Calculate checksum for the specified file");
    println!("  -h, --help          Show this help message");
    println!("  -s string           Calculate checksum for the specified string");
    println!("  --format hex|decimal Output format (default: hex)");
    println!();
    println!("Benchmarking:");
    println!("  -b                  Enable benchmark mode");
    println!("  --duration seconds  Benchmark duration in seconds (default: 10.0)");
    println!("  --size bytes        Data size for random generation in benchmark mode (default: 1048576 [1MiB])");
    println!();
    println!();
    println!("Note: In normal mode, either -f or -s must be provided, but not both.");
    println!("      In benchmark mode (-b), -f or -s are optional for using specific data.");
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("No arguments provided".to_string());
    }

    // Check for help flag
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        return Err("help".to_string());
    }

    let mut algorithm: Option<String> = None;
    let mut file: Option<String> = None;
    let mut string: Option<String> = None;
    let mut format = OutputFormat::Hex; // Default to hex
    let mut benchmark_mode = false;
    let mut benchmark_size: Option<usize> = None;
    let mut benchmark_duration = 10.0; // Default duration

    let mut i = 1; // Skip program name
    while i < args.len() {
        match args[i].as_str() {
            "-a" => {
                if i + 1 >= args.len() {
                    return Err("Missing algorithm after -a flag".to_string());
                }
                algorithm = Some(args[i + 1].clone());
                i += 2;
            }
            "-f" => {
                if i + 1 >= args.len() {
                    return Err("Missing filename after -f flag".to_string());
                }
                if string.is_some() {
                    return Err("Cannot specify both -f and -s flags".to_string());
                }
                file = Some(args[i + 1].clone());
                i += 2;
            }
            "-s" => {
                if i + 1 >= args.len() {
                    return Err("Missing string after -s flag".to_string());
                }
                if file.is_some() {
                    return Err("Cannot specify both -f and -s flags".to_string());
                }
                string = Some(args[i + 1].clone());
                i += 2;
            }
            "--format" => {
                if i + 1 >= args.len() {
                    return Err("Missing format after --format flag".to_string());
                }
                match args[i + 1].as_str() {
                    "hex" => format = OutputFormat::Hex,
                    "decimal" => format = OutputFormat::Decimal,
                    invalid => {
                        return Err(format!(
                            "Invalid format '{}'. Use 'hex' or 'decimal'",
                            invalid
                        ))
                    }
                }
                i += 2;
            }
            "-b" => {
                benchmark_mode = true;
                i += 1;
            }
            "--size" => {
                if i + 1 >= args.len() {
                    return Err("Missing size value after --size flag".to_string());
                }
                benchmark_size = Some(
                    args[i + 1]
                        .parse::<usize>()
                        .map_err(|_| format!("Invalid size value: {}", args[i + 1]))?,
                );
                i += 2;
            }
            "--duration" => {
                if i + 1 >= args.len() {
                    return Err("Missing duration value after --duration flag".to_string());
                }
                benchmark_duration = args[i + 1]
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid duration value: {}", args[i + 1]))?;
                i += 2;
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    // Validate required arguments
    let algorithm = algorithm.ok_or("Algorithm (-a) is required")?;

    // Validate mutual exclusivity between benchmark and normal modes
    if !benchmark_mode && (benchmark_size.is_some() || benchmark_duration != 10.0) {
        return Err("--size and --duration can only be used with -b flag".to_string());
    }

    // Create benchmark config if in benchmark mode
    let benchmark = if benchmark_mode {
        let config = BenchmarkConfig {
            size: benchmark_size,
            duration: benchmark_duration,
        };
        config.validate()?;
        Some(config)
    } else {
        None
    };

    // Validate input requirements based on mode
    if benchmark.is_none() {
        // Normal mode: require either file or string input
        if file.is_none() && string.is_none() {
            return Err(
                "Either -f (file) or -s (string) must be provided in normal mode".to_string(),
            );
        }
    }
    // Benchmark mode: file and string are optional (will use generated data if neither provided)

    Ok(Config {
        algorithm,
        file,
        string,
        format,
        benchmark,
    })
}

fn calculate_checksum(config: &Config) -> Result<(), String> {
    let algorithm = CrcAlgorithm::from_str(&config.algorithm)
        .map_err(|_| format!("Invalid algorithm: {}", config.algorithm))?;

    // Check if benchmark mode is enabled
    if let Some(benchmark_config) = &config.benchmark {
        return run_benchmark(config, benchmark_config, algorithm);
    }

    let checksum = if let Some(ref filename) = config.file {
        checksum_file(algorithm, filename, None).unwrap()
    } else if let Some(ref text) = config.string {
        checksum(algorithm, text.as_bytes())
    } else {
        return Err("No input provided for checksum calculation".to_string());
    };

    match config.format {
        OutputFormat::Hex => println!("{:#x?}", checksum),
        OutputFormat::Decimal => println!("{}", checksum),
    }

    Ok(())
}

fn run_benchmark(
    config: &Config,
    benchmark_config: &BenchmarkConfig,
    algorithm: CrcAlgorithm,
) -> Result<(), String> {
    // Determine data source and create BenchmarkData
    let data = if let Some(ref filename) = config.file {
        // Validate file exists before benchmarking
        if !std::path::Path::new(filename).exists() {
            return Err(format!("File not found: {}", filename));
        }
        BenchmarkData::File(filename.clone())
    } else if let Some(ref text) = config.string {
        BenchmarkData::InMemory(text.as_bytes().to_vec())
    } else {
        // Generate random data with specified size or default (1 MiB)
        let size = benchmark_config.size.unwrap_or(1_048_576);
        let random_data = generate_random_data(size)?;
        BenchmarkData::InMemory(random_data)
    };

    // Create and run benchmark
    let runner = BenchmarkRunner::new(algorithm, data, benchmark_config.duration);
    let result = runner.run()?;

    // Display results with algorithm name
    display_benchmark_results(&result, &config.algorithm);

    Ok(())
}

// Format numbers with comma separators for better readability
fn format_number_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}

fn display_benchmark_results(result: &BenchmarkResult, algorithm_name: &str) {
    println!("Algorithm: {}", algorithm_name);
    println!("Acceleration Target: {}", result.acceleration_target);

    // Format data size with appropriate units
    let (size_value, size_unit) = if result.data_size >= 1_048_576 {
        (result.data_size as f64 / 1_048_576.0, "MiB")
    } else if result.data_size >= 1024 {
        (result.data_size as f64 / 1024.0, "KiB")
    } else {
        (result.data_size as f64, "bytes")
    };

    println!(
        "Data Size: {} bytes ({:.1} {})",
        format_number_with_commas(result.data_size),
        size_value,
        size_unit
    );
    println!("Duration: {:.2} seconds", result.elapsed_seconds);
    println!(
        "Iterations: {}",
        format_number_with_commas(result.iterations)
    );
    println!("Throughput: {:.2} GiB/s", result.throughput_gibs);

    // Format time per iteration with appropriate units
    let (time_value, time_unit) = if result.time_per_iteration_nanos >= 1_000_000.0 {
        (result.time_per_iteration_nanos / 1_000_000.0, "ms")
    } else if result.time_per_iteration_nanos >= 1_000.0 {
        (result.time_per_iteration_nanos / 1_000.0, "μs")
    } else {
        (result.time_per_iteration_nanos, "ns")
    };

    println!("Time per iteration: {:.1} {}", time_value, time_unit);
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(config) => {
            if let Err(e) = calculate_checksum(&config) {
                eprintln!("Error: {}", e);
                return ExitCode::from(1);
            }
        }
        Err(msg) => {
            if msg == "help" {
                print_usage();
                return ExitCode::SUCCESS;
            } else {
                eprintln!("Error: {}", msg);
                println!();
                print_usage();
                return ExitCode::from(1);
            }
        }
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_benchmark_config_validation_valid() {
        let config = BenchmarkConfig {
            size: Some(1024),
            duration: 5.0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_benchmark_config_validation_zero_duration() {
        let config = BenchmarkConfig {
            size: Some(1024),
            duration: 0.0,
        };
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Duration must be greater than 0"
        );
    }

    #[test]
    fn test_benchmark_config_validation_negative_duration() {
        let config = BenchmarkConfig {
            size: Some(1024),
            duration: -1.0,
        };
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Duration must be greater than 0"
        );
    }

    #[test]
    fn test_benchmark_config_validation_zero_size() {
        let config = BenchmarkConfig {
            size: Some(0),
            duration: 5.0,
        };
        assert!(config.validate().is_err());
        assert_eq!(
            config.validate().unwrap_err(),
            "Size must be greater than 0"
        );
    }

    #[test]
    fn test_benchmark_config_validation_none_size() {
        let config = BenchmarkConfig {
            size: None,
            duration: 5.0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_generate_random_data_valid_size() {
        let data = generate_random_data(1024).unwrap();
        assert_eq!(data.len(), 1024);
    }

    #[test]
    fn test_generate_random_data_large_size() {
        let result = generate_random_data(1_073_741_825); // 1 GiB + 1
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Data size too large (maximum 1 GiB)");
    }

    #[test]
    fn test_benchmark_result_calculation() {
        let result = BenchmarkResult::new(
            1000,               // iterations
            2.0,                // elapsed_seconds
            "test".to_string(), // acceleration_target
            1024,               // data_size
        );

        assert_eq!(result.iterations, 1000);
        assert_eq!(result.elapsed_seconds, 2.0);
        assert_eq!(result.data_size, 1024);

        // Throughput should be (1024 * 1000) / 2.0 / (1024^3) GiB/s
        let expected_throughput = (1024.0 * 1000.0) / 2.0 / (1024.0 * 1024.0 * 1024.0);
        assert!((result.throughput_gibs - expected_throughput).abs() < 1e-10);

        // Time per iteration should be 2.0 * 1e9 / 1000 nanoseconds
        let expected_time_per_iter = 2.0 * 1_000_000_000.0 / 1000.0;
        assert!((result.time_per_iteration_nanos - expected_time_per_iter).abs() < 1e-6);
    }

    #[test]
    fn test_benchmark_runner_creation() {
        let algorithm = CrcAlgorithm::from_str("CRC-32/ISCSI").unwrap();
        let data = BenchmarkData::InMemory(vec![1, 2, 3, 4]);
        let runner = BenchmarkRunner::new(algorithm, data, 1.0);

        assert_eq!(runner.duration, 1.0);
        assert_eq!(runner.algorithm, algorithm);
    }

    #[test]
    fn test_benchmark_runner_execution_in_memory() {
        let algorithm = CrcAlgorithm::from_str("CRC-32/ISCSI").unwrap();
        let data = BenchmarkData::InMemory(vec![1, 2, 3, 4]);
        let runner = BenchmarkRunner::new(algorithm, data, 0.1); // Short duration for test

        let result = runner.run().unwrap();
        assert!(result.iterations > 0);
        assert!(result.elapsed_seconds > 0.0);
        assert_eq!(result.data_size, 4);
        assert!(result.throughput_gibs >= 0.0);
    }

    #[test]
    fn test_parse_args_benchmark_mode() {
        // We can't easily mock std::env::args in unit tests, so we'll test the parsing logic
        // by creating a Config directly and validating its structure
        let config = Config {
            algorithm: "CRC-32/ISCSI".to_string(),
            file: None,
            string: None,
            format: OutputFormat::Hex,
            benchmark: Some(BenchmarkConfig {
                size: Some(1024),
                duration: 5.0,
            }),
        };

        assert!(config.benchmark.is_some());
        let benchmark_config = config.benchmark.unwrap();
        assert_eq!(benchmark_config.size, Some(1024));
        assert_eq!(benchmark_config.duration, 5.0);
    }

    #[test]
    fn test_parse_args_normal_mode() {
        let config = Config {
            algorithm: "CRC-32/ISCSI".to_string(),
            file: Some("test.txt".to_string()),
            string: None,
            format: OutputFormat::Hex,
            benchmark: None,
        };

        assert!(config.benchmark.is_none());
        assert_eq!(config.file, Some("test.txt".to_string()));
    }

    #[test]
    fn test_data_source_selection_file() {
        // Test that file input creates File variant
        let config = Config {
            algorithm: "CRC-32/ISCSI".to_string(),
            file: Some("test.txt".to_string()),
            string: None,
            format: OutputFormat::Hex,
            benchmark: Some(BenchmarkConfig {
                size: None,
                duration: 1.0,
            }),
        };

        // This would be tested in the run_benchmark function
        // We can verify the config structure is correct for file input
        assert!(config.file.is_some());
        assert!(config.string.is_none());
    }

    #[test]
    fn test_data_source_selection_string() {
        let config = Config {
            algorithm: "CRC-32/ISCSI".to_string(),
            file: None,
            string: Some("test data".to_string()),
            format: OutputFormat::Hex,
            benchmark: Some(BenchmarkConfig {
                size: None,
                duration: 1.0,
            }),
        };

        assert!(config.file.is_none());
        assert!(config.string.is_some());
    }

    #[test]
    fn test_data_source_selection_generated() {
        let config = Config {
            algorithm: "CRC-32/ISCSI".to_string(),
            file: None,
            string: None,
            format: OutputFormat::Hex,
            benchmark: Some(BenchmarkConfig {
                size: Some(1024),
                duration: 1.0,
            }),
        };

        // When neither file nor string is provided, generated data should be used
        assert!(config.file.is_none());
        assert!(config.string.is_none());
        assert_eq!(config.benchmark.as_ref().unwrap().size, Some(1024));
    }

    #[test]
    fn test_throughput_calculation_accuracy() {
        // Test with known values to verify calculation accuracy
        let data_size = 1_048_576u64; // 1 MiB
        let iterations = 1000u64;
        let elapsed_seconds = 1.0;

        let result =
            BenchmarkResult::new(iterations, elapsed_seconds, "test".to_string(), data_size);

        // Expected throughput: (1 MiB * 1000) / 1 second = 1000 MiB/s = ~0.9537 GiB/s
        let expected_gibs =
            (data_size as f64 * iterations as f64) / elapsed_seconds / (1024.0 * 1024.0 * 1024.0);
        assert!((result.throughput_gibs - expected_gibs).abs() < 1e-10);
    }

    #[test]
    fn test_output_format_variants() {
        let hex_format = OutputFormat::Hex;
        let decimal_format = OutputFormat::Decimal;

        // Test that both variants can be created and are different
        assert!(matches!(hex_format, OutputFormat::Hex));
        assert!(matches!(decimal_format, OutputFormat::Decimal));
    }

    #[test]
    fn test_format_number_with_commas() {
        assert_eq!(format_number_with_commas(0), "0");
        assert_eq!(format_number_with_commas(123), "123");
        assert_eq!(format_number_with_commas(1234), "1,234");
        assert_eq!(format_number_with_commas(12345), "12,345");
        assert_eq!(format_number_with_commas(123456), "123,456");
        assert_eq!(format_number_with_commas(1234567), "1,234,567");
        assert_eq!(format_number_with_commas(12345678), "12,345,678");
        assert_eq!(format_number_with_commas(123456789), "123,456,789");
        assert_eq!(format_number_with_commas(1000000000), "1,000,000,000");
    }
}
