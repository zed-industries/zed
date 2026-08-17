// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This is a simple program to get custom CRC parameters from the command line.

use std::env;
use std::process::ExitCode;

#[derive(Debug)]
struct Config {
    width: Option<u32>,
    polynomial: Option<u64>,
    init: Option<u64>,
    reflected: Option<bool>,
    xorout: Option<u64>,
    check: Option<u64>,
    name: Option<String>,
}

impl Config {
    fn new() -> Self {
        Config {
            width: None,
            polynomial: None,
            init: None,
            reflected: None,
            xorout: None,
            check: None,
            name: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.width.is_some()
            && self.polynomial.is_some()
            && self.init.is_some()
            && self.reflected.is_some()
            && self.xorout.is_some()
            && self.check.is_some()
            && self.name.is_some()
    }
}

fn parse_hex_or_decimal(s: &str) -> Result<u64, String> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).map_err(|_| format!("Invalid hexadecimal value: {s}",))
    } else {
        s.parse::<u64>()
            .map_err(|_| format!("Invalid decimal value: {s}",))
    }
}

fn parse_bool(s: &str) -> Result<bool, String> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(format!("Invalid boolean value: {s} (use true/false)",)),
    }
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut config = Config::new();
    let mut i = 1; // Skip program name

    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -n (name)".to_string());
                }
                config.name = Some(args[i + 1].clone());
                i += 2;
            }
            "-w" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -w (width)".to_string());
                }
                config.width = Some(
                    args[i + 1]
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid width value: {}", args[i + 1]))?,
                );
                i += 2;
            }
            "-p" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -p (polynomial)".to_string());
                }
                config.polynomial = Some(parse_hex_or_decimal(&args[i + 1])?);
                i += 2;
            }
            "-i" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -i (init)".to_string());
                }
                config.init = Some(parse_hex_or_decimal(&args[i + 1])?);
                i += 2;
            }
            "-r" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -r (reflected)".to_string());
                }
                config.reflected = Some(parse_bool(&args[i + 1])?);
                i += 2;
            }
            "-x" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -x (xorout)".to_string());
                }
                config.xorout = Some(parse_hex_or_decimal(&args[i + 1])?);
                i += 2;
            }
            "-c" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -c (check)".to_string());
                }
                config.check = Some(parse_hex_or_decimal(&args[i + 1])?);
                i += 2;
            }
            arg => {
                return Err(format!("Unknown argument: {arg}",));
            }
        }
    }

    Ok(config)
}

fn print_usage() {
    println!("Usage: get-custom-params -n <name> -w <width> -p <polynomial> -i <init> -r <reflected> -x <xorout> -c <check>");
    println!();
    println!("Example: get-custom-params -n CRC-32/ISCSI -w 32 -p 0x1edc6f41 -i 0xFFFFFFFF -r true -x 0xFFFFFFFF -c 0xe3069283");
    println!("Example: get-custom-params -n CRC-64/NVME -w 64 -p 0xad93d23594c93659 -i 0xffffffffffffffff -r true -x 0xffffffffffffffff -c 0xae8b14860a799888");
    println!();
    println!("Arguments:");
    println!("  -n <name>       Name of the CRC algorithm (e.g., CRC-32/ISCSI)");
    println!("  -w <width>      CRC width (number of bits)");
    println!("  -p <polynomial> CRC polynomial (hex or decimal)");
    println!("  -i <init>       Initial value (hex or decimal)");
    println!("  -r <reflected>  Reflected input/output (true/false)");
    println!("  -x <xorout>     XOR output value (hex or decimal)");
    println!("  -c <check>      Check value (hex or decimal)");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
        return ExitCode::from(1);
    }

    let config = match parse_args(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error: {error}",);
            println!();
            print_usage();
            return ExitCode::from(1);
        }
    };

    // Check if all required arguments are provided
    if !config.is_complete() {
        eprintln!("Error: All arguments are required");
        println!();
        print_usage();
        return ExitCode::from(1);
    }

    let static_name: &'static str = Box::leak(config.name.unwrap().into_boxed_str());

    let params = crc_fast::CrcParams::new(
        static_name,
        config.width.unwrap() as u8,
        config.polynomial.unwrap(),
        config.init.unwrap(),
        config.reflected.unwrap(),
        config.xorout.unwrap(),
        config.check.unwrap(),
    );

    println!();
    println!("// Generated CRC parameters for {static_name}",);
    println!(
        "pub const {}: CrcParams = CrcParams {{",
        static_name
            .to_uppercase()
            .replace("-", "_")
            .replace("/", "_")
    );
    println!(
        "    algorithm: CrcAlgorithm::{}Custom,",
        if config.width.unwrap() == 32 {
            "Crc32"
        } else {
            "Crc64"
        }
    );
    println!("    name: \"{static_name}\",",);
    println!("    width: {},", config.width.unwrap());
    println!("    poly: 0x{:x},", config.polynomial.unwrap());
    println!("    init: 0x{:x},", config.init.unwrap());
    println!("    refin: {},", config.reflected.unwrap());
    println!("    refout: {},", config.reflected.unwrap());
    println!("    xorout: 0x{:x},", config.xorout.unwrap());
    println!("    check: 0x{:x},", config.check.unwrap());
    println!("    keys: CrcKeysStorage::from_keys_fold_256([");

    // Print the keys array
    for i in 0..23 {
        let key = params.get_key(i);
        println!("        0x{key:016x},",);
    }

    println!("    ]),");
    println!("}};");
    println!();

    ExitCode::from(0)
}
