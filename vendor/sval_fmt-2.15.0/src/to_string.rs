use alloc::string::String;

/**
Format a value into a string.

This method will use a default format that's like Rust's `Debug`.
*/
pub fn stream_to_string(v: impl sval::Value) -> String {
    let mut out = String::new();
    crate::stream_to_write(&mut out, v).expect("infallible write");
    out
}
