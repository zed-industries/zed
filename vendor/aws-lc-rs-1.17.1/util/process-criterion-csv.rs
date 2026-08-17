#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
clap = { version = "4.0.29", features = ["derive"] }
itertools = "0.10.5"
---
//! Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
//! SPDX-License-Identifier: Apache-2.0 OR ISC
//! To run, you will need to install rust-script:
//! ```
//! $ cargo install rust-script
//! ```
//!
//! After running the benchmarks, you can collect the data into a single CSV:
//! ```
//! $ find ./target/criterion -name "raw.csv" | xargs cat | sort | egrep -v "^group" > bench-aarch64-AL2.csv
//! ```
//!


use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;
use std::ops::{Deref, DerefMut, Div};
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use std::{fs, io};

use clap::Parser;

use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};

struct Stats(Vec<f64>);
struct FinalizedStats(Vec<f64>);

impl Deref for Stats {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for FinalizedStats {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Stats {
    fn new() -> Self {
        Stats(Vec::new())
    }

    fn finalize(&self) -> FinalizedStats {
        let mut data = self.0.clone();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        FinalizedStats(data)
    }
}

impl FinalizedStats {
    fn percentile(&self, percent: f64) -> f64 {
        if percent >= 0.9999999 {
            return self[self.len() - 1];
        } else if percent < 0.0 {
            return self[0];
        }
        self[f64::trunc(percent * self.len() as f64) as usize]
    }

    fn median(&self) -> f64 {
        self.percentile(0.5)
    }

    fn max(&self) -> f64 {
        self[self.len() - 1]
    }

    fn min(&self) -> f64 {
        self[0]
    }
}

#[derive(Parser)]
#[command(about)]
struct Cli {
    /// CSV file to operate on
    csv_file: PathBuf,

    /// Compute the median
    #[arg(long, default_value = "false", required_unless_present_any(["max", "min", "percentile"]))]
    median: bool,

    /// Compute the maximum
    #[arg(long, default_value = "false", required_unless_present_any(["median", "min", "percentile"]))]
    max: bool,

    /// Compute the minimum
    #[arg(long, default_value = "false", required_unless_present_any(["median", "max", "percentile"]))]
    min: bool,

    /// Compute percentile
    #[arg(short, long, required_unless_present_any(["median", "max", "min"]))]
    percentile: Vec<u8>,

    /// Turn debugging information on
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    fn header_line(&self) -> String {
        let mut line = String::new();
        if self.min {
            line = format!("{},aws-lc (min), ring (min), % diff (min)", line);
        }
        if self.median {
            line = format!("{},aws-lc (median), ring (median), % diff (median)", line);
        }
        if self.max {
            line = format!("{},aws-lc (max), ring (max), % diff (max)", line);
        }
        if self.percentile.len() > 0 {
            let mut percentiles = self.percentile.clone();
            percentiles.sort();
            for percentile in percentiles {
                line = format!(
                    "{},aws-lc (P{:02}), ring (P{:02}), % diff (P{:02})",
                    line, percentile, percentile, percentile
                );
            }
        }
        line
    }

    fn data_line(&self, aws_stats: &FinalizedStats, ring_stats: &FinalizedStats) -> String {
        let mut line = String::new();
        if self.min {
            let (aws, ring, rel) = compute(&aws_stats, &ring_stats, &FinalizedStats::min);
            line = format!("{},{:.2},{:.2},{:.2}", line, aws, ring, rel);
        }
        if self.median {
            let (aws, ring, rel) = compute(&aws_stats, &ring_stats, &FinalizedStats::median);
            line = format!("{},{:.2},{:.2},{:.2}", line, aws, ring, rel);
        }
        if self.max {
            let (aws, ring, rel) = compute(&aws_stats, &ring_stats, &FinalizedStats::max);
            line = format!("{},{:.2},{:.2},{:.2}", line, aws, ring, rel);
        }
        if self.percentile.len() > 0 {
            let mut percentiles = self.percentile.clone();
            percentiles.sort();
            for percentile in percentiles {
                let percentile = percentile as f64 / 100.0;
                let (aws, ring, rel) = compute(&aws_stats, &ring_stats, &|s: &FinalizedStats| {
                    s.percentile(percentile)
                });
                line = format!("{},{:.2},{:.2},{:.2}", line, aws, ring, rel);
            }
        }
        line
    }
}

fn insert_result(test: &str, avg: f64, results: &mut HashMap<String, Stats>) {
    if !results.contains_key(test) {
        results.insert(test.to_string(), Stats::new());
    }
    let results_vec = results.get_mut(test).unwrap();
    results_vec.push(avg);
}

fn compute<F>(aws_stats: &FinalizedStats, ring_stats: &FinalizedStats, comp: &F) -> (f64, f64, f64)
where
    F: Fn(&FinalizedStats) -> f64,
{
    let aws = comp(aws_stats);
    let ring = comp(ring_stats);
    let relative_percentage = 100.0 * (1.0 - aws / ring);
    (aws, ring, relative_percentage)
}

fn numerical_string_compare(a: &str, b: &str) -> Ordering {
    let mut number_compare = false;
    let mut a_num = 0u32;
    let mut b_num = 0u32;

    for pair in a.chars().into_iter().zip_longest(b.chars().into_iter()) {
        match pair {
            Both(ac, bc) => {
                if ac.is_digit(10) {
                    if bc.is_digit(10) {
                        if ac.cmp(&bc).is_eq() {
                            continue;
                        }
                        a_num *= 10;
                        a_num += ac.to_digit(10).unwrap();
                        b_num *= 10;
                        b_num += bc.to_digit(10).unwrap();
                        number_compare = true;
                    } else if number_compare {
                        return Ordering::Greater;
                    } else {
                        return ac.cmp(&bc);
                    }
                } else if bc.is_digit(10) {
                    return if number_compare {
                        Ordering::Less
                    } else {
                        ac.cmp(&bc)
                    };
                } else if number_compare {
                    return a_num.cmp(&b_num);
                } else {
                    let result = ac.cmp(&bc);
                    if !result.is_eq() {
                        return result;
                    }
                }
            }
            Left(_ac) => {
                return Ordering::Greater;
            }
            Right(_bc) => {
                return Ordering::Less;
            }
        }
    }
    return if number_compare {
        a_num.cmp(&b_num)
    } else {
        Ordering::Equal
    };
}

/// Tests can be run from the command line:
///    $ rust-script --test ./util/process-criterion-csv.rs
#[test]
fn test_numerical_string_compare() {
    let h123 = "Hello256-123-bytes";
    let h987 = "Hello256-987-bytes";
    let h1234 = "Hello256-1234-bytes";
    let h9876 = "Hello256-9876-bytes";
    assert_eq!(Ordering::Equal, numerical_string_compare(h123, h123));
    assert_eq!(Ordering::Less, numerical_string_compare(h123, h987));
    assert_eq!(Ordering::Greater, numerical_string_compare(h987, h123));
    assert_eq!(Ordering::Less, numerical_string_compare(h123, h1234));
    assert_eq!(Ordering::Greater, numerical_string_compare(h1234, h123));
    assert_eq!(Ordering::Less, numerical_string_compare(h123, h9876));
    assert_eq!(Ordering::Greater, numerical_string_compare(h9876, h123));

    assert_eq!(Ordering::Equal, numerical_string_compare(h987, h987));
    assert_eq!(Ordering::Less, numerical_string_compare(h987, h1234));
    assert_eq!(Ordering::Greater, numerical_string_compare(h1234, h987));
    assert_eq!(Ordering::Less, numerical_string_compare(h987, h9876));
    assert_eq!(Ordering::Greater, numerical_string_compare(h9876, h987));

    assert_eq!(Ordering::Equal, numerical_string_compare(h1234, h1234));
    assert_eq!(Ordering::Less, numerical_string_compare(h1234, h9876));
    assert_eq!(Ordering::Greater, numerical_string_compare(h9876, h1234));

    assert_eq!(Ordering::Equal, numerical_string_compare(h9876, h9876));
}

#[test]
fn test_numerical_end_string_compare() {
    let h123 = "Hello256-123";
    let h987 = "Hello256-987";
    assert_eq!(Ordering::Less, numerical_string_compare(h123, h987));
    assert_eq!(Ordering::Greater, numerical_string_compare(h987, h123));
    assert_eq!(Ordering::Equal, numerical_string_compare(h123, h123));
    assert_eq!(Ordering::Equal, numerical_string_compare(h987, h987));
}

fn main() {
    let cli = Cli::parse();

    let mut ring_results: HashMap<String, Stats> = HashMap::new();
    let mut aws_results: HashMap<String, Stats> = HashMap::new();

    let contents = fs::read_to_string(&cli.csv_file)
        .expect(&format!("Unable to open file: '{:?}'", &cli.csv_file));

    for line in contents.lines() {
        if line.starts_with("group") {
            continue;
        }
        let components: Vec<&str> = line.split(",").collect();
        assert_eq!(8, components.len());
        let test = components[0].trim();
        let lib = components[1].trim();
        let time = f64::from_str(components[5]).expect(&format!("Unable to parse time: {}", line));
        let iter = u32::from_str(components[7])
            .expect(&format!("Unable to parse iteration count: {}", line));
        let avg = time.div(iter as f64);
        match lib {
            "AWS-LC" => insert_result(test, avg, &mut aws_results),
            "Ring" => insert_result(test, avg, &mut ring_results),
            _ => panic!("Unrecognized library: {}", lib),
        }
    }

    let mut test_keys: Vec<&String> = aws_results.keys().collect();
    test_keys.sort_by(|a, b| numerical_string_compare(a, b));
    let mut handle = io::stdout().lock();
    writeln!(handle, "Test{}", cli.header_line()).unwrap();

    for test in test_keys {
        let aws_stats = aws_results.get(test.as_str()).unwrap().finalize();
        let ring_stats = ring_results.get(test.as_str()).unwrap().finalize();
        write!(handle, "{}", test).unwrap();
        write!(handle, "{}", cli.data_line(&aws_stats, &ring_stats)).unwrap();
        writeln!(handle, "").unwrap();
    }
    drop(handle);
}
