use std::env;
use std::path::{Path, PathBuf};

// configuration defaults
const DEFAULT_PRECISION: &str = "100";
const DEFAULT_ROUNDING_MODE: &str = "HalfEven";
const FMT_EXPONENTIAL_LOWER_THRESHOLD: &str = "5";
const FMT_EXPONENTIAL_UPPER_THRESHOLD: &str = "15";
const FMT_MAX_INTEGER_PADDING: &str = "1000";

const SERDE_MAX_SCALE: &str = "150000";


fn main() {
    let ac = autocfg::new();
    ac.emit_rustc_version(1, 70);

    // abs_diff
    ac.emit_rustc_version(1, 60);

    // slice::fill
    ac.emit_rustc_version(1, 50);

    // Option::zip
    ac.emit_rustc_version(1, 46);

    // Remove this comment if enabled proptests
    // ::PROPERTY-TESTS:: autocfg::emit("property_tests");

    let outdir: PathBuf = std::env::var_os("OUT_DIR").unwrap().into();
    write_default_precision_file(&outdir);
    write_default_rounding_mode(&outdir);
    write_exponential_format_threshold_file(&outdir);
    write_max_serde_parsing_scale_limit(&outdir);
}

/// Loads the environment variable string or default
macro_rules! load_env {
    ($env:ident, $name:literal, $default:ident) => {{
        println!("cargo:rerun-if-env-changed={}", $name);
        $env::var($name).unwrap_or_else(|_| $default.to_owned())
    }};
}

/// Create default_precision.rs, containing definition of constant DEFAULT_PRECISION loaded in src/lib.rs
fn write_default_precision_file(outdir: &Path) {
    let env_var = load_env!(env, "RUST_BIGDECIMAL_DEFAULT_PRECISION", DEFAULT_PRECISION);
    let rust_file_path = outdir.join("default_precision.rs");

    let default_prec: u32 = env_var
        .parse::<std::num::NonZeroU32>()
        .expect("$RUST_BIGDECIMAL_DEFAULT_PRECISION must be an integer > 0")
        .into();

    let rust_file_contents = format!("const DEFAULT_PRECISION: u64 = {};", default_prec);

    std::fs::write(rust_file_path, rust_file_contents).unwrap();
}

/// Create default_rounding_mode.rs, using value of RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE environment variable
fn write_default_rounding_mode(outdir: &Path) {
    let rounding_mode_name = load_env!(env, "RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE", DEFAULT_ROUNDING_MODE);

    let rust_file_path = outdir.join("default_rounding_mode.rs");
    let rust_file_contents = format!("const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::{};", rounding_mode_name);

    std::fs::write(rust_file_path, rust_file_contents).unwrap();
}

/// Create write_default_rounding_mode.rs, containing definition of constant EXPONENTIAL_FORMAT_THRESHOLD loaded in src/impl_fmt.rs
fn write_exponential_format_threshold_file(outdir: &Path) {
    let low_value = load_env!(env, "RUST_BIGDECIMAL_FMT_EXPONENTIAL_LOWER_THRESHOLD", FMT_EXPONENTIAL_LOWER_THRESHOLD);
    let high_value = load_env!(env, "RUST_BIGDECIMAL_FMT_EXPONENTIAL_UPPER_THRESHOLD", FMT_EXPONENTIAL_UPPER_THRESHOLD);
    let max_padding = load_env!(env, "RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING", FMT_MAX_INTEGER_PADDING);

    let low_value: u32 = low_value
        .parse::<std::num::NonZeroU32>()
        .expect("$RUST_BIGDECIMAL_FMT_EXPONENTIAL_LOWER_THRESHOLD must be an integer > 0")
        .into();

    let high_value: u32 = high_value
        .parse::<u32>()
        .expect("$RUST_BIGDECIMAL_FMT_EXPONENTIAL_UPPER_THRESHOLD must be valid u32");

    let max_padding: u32 = max_padding
        .parse::<u32>()
        .expect("$RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING must be valid u32");

    let rust_file_path = outdir.join("exponential_format_threshold.rs");

    let rust_file_contents = [
        format!("const EXPONENTIAL_FORMAT_LEADING_ZERO_THRESHOLD: usize = {};", low_value),
        format!("const EXPONENTIAL_FORMAT_TRAILING_ZERO_THRESHOLD: usize = {};", high_value),
        format!("const FMT_MAX_INTEGER_PADDING: usize = {};", max_padding),
    ];

    std::fs::write(rust_file_path, rust_file_contents.join("\n")).unwrap();
}


/// Create write_default_rounding_mode.rs, containing definition of constant EXPONENTIAL_FORMAT_THRESHOLD loaded in src/impl_fmt.rs
fn write_max_serde_parsing_scale_limit(outdir: &Path) {
    let scale_limit = load_env!(env, "RUST_BIGDECIMAL_SERDE_SCALE_LIMIT", SERDE_MAX_SCALE);

    let scale_limit: u32 = scale_limit
        .parse::<u32>()
        .or_else(|e| if scale_limit.to_lowercase() == "none" { Ok(0) } else { Err(e) })
        .expect("$RUST_BIGDECIMAL_SERDE_SCALE_LIMIT must be an integer");

    let rust_file_path = outdir.join("serde_scale_limit.rs");

    let rust_file_contents = [
        format!("const SERDE_SCALE_LIMIT: i64 = {};", scale_limit),
    ];

    std::fs::write(rust_file_path, rust_file_contents.join("\n")).unwrap();
}
