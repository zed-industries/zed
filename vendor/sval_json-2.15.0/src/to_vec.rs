use crate::Error;

use alloc::vec::Vec;

/**
Stream a value as JSON into a byte buffer.

This method will fail if the value contains complex values as keys.
*/
pub fn stream_to_vec(v: impl sval::Value) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    crate::stream_to_io_write(&mut out, v)?;

    Ok(out)
}
