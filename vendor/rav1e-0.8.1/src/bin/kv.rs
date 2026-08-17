#![allow(unused_variables)]
use serde::{ser, Serialize, Serializer};
use thiserror::*;

use std::fmt;

struct KVString {
  output: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Error)]
enum Error {
  #[error("unsupported")]
  Unsupported,
}

impl ser::Error for Error {
  #[cold]
  fn custom<T: fmt::Display>(msg: T) -> Error {
    Error::Unsupported
  }
}

/// Serialize a configuration as a Key-Value string.
pub fn to_string<T>(value: &T) -> Result<String, ()>
where
  T: Serialize,
{
  let mut serializer = KVString { output: String::new() };
  value.serialize(&mut serializer).map_err(|_| ())?;
  Ok(serializer.output)
}

impl Serializer for &mut KVString {
  type Ok = ();
  type Error = Error;
  type SerializeSeq = ser::Impossible<(), Self::Error>;
  type SerializeTuple = ser::Impossible<(), Self::Error>;
  type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
  type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
  type SerializeMap = ser::Impossible<(), Self::Error>;
  type SerializeStruct = Self;
  type SerializeStructVariant = ser::Impossible<(), Self::Error>;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
    self.output += if v { "true" } else { "false" };
    Ok(())
  }
  fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
    self.output += &v.to_string();
    Ok(())
  }
  fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
    self.serialize_f64(f64::from(v))
  }

  fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
    self.output += &v.to_string();
    Ok(())
  }
  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    unimplemented!()
  }
  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    self.output += v;
    Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    unimplemented!()
  }
  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    self.output += "None";
    Ok(())
  }
  fn serialize_some<T: ?Sized>(
    self, value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: Serialize,
  {
    value.serialize(self)
  }
  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    self.output += "";
    Ok(())
  }
  fn serialize_unit_struct(
    self, name: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    self.serialize_unit()
  }

  fn serialize_unit_variant(
    self, name: &'static str, variant_index: u32, variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    self.serialize_str(variant)
  }

  fn serialize_newtype_struct<T: ?Sized>(
    self, name: &'static str, value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: Serialize,
  {
    unimplemented!()
  }

  fn serialize_newtype_variant<T: ?Sized>(
    self, name: &'static str, variant_index: u32, variant: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: Serialize,
  {
    unimplemented!()
  }

  fn serialize_seq(
    self, len: Option<usize>,
  ) -> Result<Self::SerializeSeq, Self::Error> {
    unimplemented!()
  }
  fn serialize_tuple(
    self, len: usize,
  ) -> Result<Self::SerializeTuple, Self::Error> {
    unimplemented!()
  }
  fn serialize_tuple_struct(
    self, name: &'static str, len: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
    unimplemented!()
  }
  fn serialize_tuple_variant(
    self, name: &'static str, variant_index: u32, variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    unimplemented!()
  }
  fn serialize_map(
    self, len: Option<usize>,
  ) -> Result<Self::SerializeMap, Self::Error> {
    unimplemented!()
  }
  fn serialize_struct(
    self, name: &'static str, len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
    Ok(self)
  }
  fn serialize_struct_variant(
    self, name: &'static str, variant_index: u32, variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    unimplemented!()
  }
}

impl<'a> ser::SerializeStruct for &mut KVString {
  type Ok = ();
  type Error = Error;

  // Assume a single flat struct for now
  fn serialize_field<T>(
    &mut self, key: &'static str, value: &T,
  ) -> Result<(), Self::Error>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += ", "
    }
    self.output += key;
    self.output += " = ";
    value.serialize(&mut **self)
  }

  fn end(self) -> Result<(), Self::Error> {
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use rav1e::prelude::SpeedSettings;

  #[test]
  fn serialize_speed_settings() {
    for preset in 0..=10 {
      let s = SpeedSettings::from_preset(preset);
      let out = super::to_string(&s).unwrap();
      println!("preset {}: {}", preset, out);
    }
  }
}
