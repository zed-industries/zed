use crate::{Error, JsonStr};

use alloc::{boxed::Box, string::String};

/**
Stream a value as JSON into a string.

This method will fail if the value contains complex values as keys.
*/
pub fn stream_to_string(v: impl sval::Value) -> Result<String, Error> {
    let mut out = String::new();
    crate::stream_to_fmt_write(&mut out, v)?;

    Ok(out)
}

/**
Stream a value as JSON into a `JsonStr`.

This method will fail if the value contains complex values as keys.
*/
pub fn stream_to_json_str(v: impl sval::Value) -> Result<Box<JsonStr>, Error> {
    Ok(JsonStr::boxed(stream_to_string(v)?))
}
