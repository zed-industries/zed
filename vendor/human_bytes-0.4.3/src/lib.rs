//! ## A Rust crate & cli to convert bytes into human-readable values.

//! It can return either KiB/MiB/GiB/TiB or KB/MB/GB/TB by disabling the `si-units` feature.
//!
//! > 1 KiB = 1024 B, 1 KB = 1000 B
//!
//! It supports from 0 bytes to several yottabytes (I cannot tell how many because I have to use `u128`s
//! to fit a single YB)
//!
//! For more info, check out the [README.md](https://sr.ht/~f9/human_bytes)

#[cfg(test)]
mod tests;

#[cfg(not(feature = "si-units"))]
// Just be future-proof
const SUFFIX: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

#[cfg(feature = "si-units")]
// Just be future-proof
const SUFFIX: [&str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

#[cfg(not(feature = "si-units"))]
const UNIT: f64 = 1000.0;

#[cfg(feature = "si-units")]
const UNIT: f64 = 1024.0;

/// Converts bytes to human-readable values
pub fn human_bytes<T: Into<f64>>(bytes: T) -> String {
    let size = bytes.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / UNIT.log10();

    #[cfg(feature = "fast")]
    {
        let mut buffer = ryu::Buffer::new();
        let result = buffer
            // Source for this hack: https://stackoverflow.com/a/28656825
            .format((UNIT.powf(base - base.floor()) * 10.0).round() / 10.0)
            .trim_end_matches(".0");

        // Add suffix
        [result, SUFFIX[base.floor() as usize]].join(" ")
    }

    #[cfg(not(feature = "fast"))]
    {
        let result = format!("{:.1}", UNIT.powf(base - base.floor()),)
            .trim_end_matches(".0")
            .to_owned();

        // Add suffix
        [&result, SUFFIX[base.floor() as usize]].join(" ")
    }
}
