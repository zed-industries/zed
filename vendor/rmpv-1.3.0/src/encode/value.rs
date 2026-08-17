use std::io::Write;

use rmp::encode::{
    write_array_len, write_bin, write_bool, write_ext_meta, write_f32, write_f64, write_map_len,
    write_nil, write_sint, write_str, write_uint,
};

use super::Error;
use crate::{IntPriv, Integer, Utf8String, Value};

/// Encodes and attempts to write the most efficient representation of the given Value.
///
/// # Note
///
/// All instances of `ErrorKind::Interrupted` are handled by this function and the underlying
/// operation is retried.
pub fn write_value<W>(wr: &mut W, val: &Value) -> Result<(), Error>
    where W: Write
{
    match *val {
        Value::Nil => {
            write_nil(wr).map_err(Error::InvalidMarkerWrite)?;
        }
        Value::Boolean(val) => {
            write_bool(wr, val).map_err(Error::InvalidMarkerWrite)?;
        }
        Value::Integer(Integer { n }) => {
            match n {
                IntPriv::PosInt(n) => {
                    write_uint(wr, n)?;
                }
                IntPriv::NegInt(n) => {
                    write_sint(wr, n)?;
                }
            }
        }
        Value::F32(val) => {
            write_f32(wr, val)?;
        }
        Value::F64(val) => {
            write_f64(wr, val)?;
        }
        Value::String(Utf8String { ref s }) => match *s {
            Ok(ref val) => write_str(wr, val)?,
            Err(ref err) => write_bin(wr, &err.0)?,
        },
        Value::Binary(ref val) => {
            write_bin(wr, val)?;
        }
        Value::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value(wr, v)?;
            }
        }
        Value::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for (key, val) in map {
                write_value(wr, key)?;
                write_value(wr, val)?;
            }
        }
        Value::Ext(ty, ref data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_all(data).map_err(Error::InvalidDataWrite)?;
        }
    }

    Ok(())
}
