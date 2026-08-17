//! Tools to unpack a [`Value`](rmpv::Value) to something we can use.
//!
//! Conversion is fallible, so [`try_unpack`](self::TryUnpack::try_unpack)
//! returns the [`Value`](rmpv::Value) if it is not of the correct type.
//!
//! ### Usage
//!
//! ```
//! use rmpv::Value;
//! use nvim_rs::rpc::unpack::TryUnpack;
//!
//! let v = Value::from("hoodle");
//! let s:String = v.try_unpack().unwrap();
//!
//! assert_eq!(String::from("hoodle"), s);
//! ```
use rmpv::Value;

/// Trait to allow seamless conversion from a [`Value`](rmpv::Value) to the type
/// it contains. In particular, this should never panic.
///
/// This is basically a specialized variant of `TryInto<V> for Value`.
pub trait TryUnpack<V> {
  /// Returns the value contained in `self`.
  ///
  /// # Errors
  ///
  /// Returns `Err(self)` if `self` does not contain a value of type `V`.
  fn try_unpack(self) -> Result<V, Value>;
}

/// This is needed because the blanket impl `TryFrom<Value> for Value` uses
/// `Error = !`.
impl TryUnpack<Value> for Value {
  fn try_unpack(self) -> Result<Value, Value> {
    Ok(self)
  }
}

impl TryUnpack<()> for Value {
  fn try_unpack(self) -> Result<(), Value> {
    if self.is_nil() {
      Ok(())
    } else {
      Err(self)
    }
  }
}

// TODO: Replace this when rmpv implements `TryFrom<Value> for String`.
impl TryUnpack<String> for Value {
  fn try_unpack(self) -> Result<String, Value> {
    match self {
      Value::String(s) if s.is_str() => {
        Ok(s.into_str().expect("This was valid UTF8"))
      }
      val => Err(val),
    }
  }
}

impl TryUnpack<(i64, i64)> for Value {
  fn try_unpack(self) -> Result<(i64, i64), Value> {
    if let Value::Array(ref v) = self {
      if v.len() == 2 {
        let mut vi = v.iter().map(Value::as_i64);
        if let (Some(Some(i)), Some(Some(j))) = (vi.next(), vi.next()) {
          return Ok((i, j));
        }
      }
    }
    Err(self)
  }
}

/// The bound `Value: From<T>` is necessary so we can recover the values if one
/// of the elements could not be unpacked. In practice, though, we only
/// implement `TryUnpack<T>` in those cases anyways.
impl<T> TryUnpack<Vec<T>> for Value
where
  Value: TryUnpack<T> + From<T>,
{
  fn try_unpack(self) -> Result<Vec<T>, Value> {
    match self {
      Value::Array(v) => {
        let mut newvec = vec![];
        let mut vi = v.into_iter();

        while let Some(ele) = vi.next() {
          match ele.try_unpack() {
            Ok(t) => newvec.push(t),
            Err(ele) => {
              let mut restorevec: Vec<Value> =
                newvec.into_iter().map(Value::from).collect();
              restorevec.push(ele);
              restorevec.extend(vi);
              return Err(Value::Array(restorevec));
            }
          }
        }
        Ok(newvec)
      }
      val => Err(val),
    }
  }
}

macro_rules! impl_try_unpack_tryfrom {
  ($t: ty) => {
    impl TryUnpack<$t> for Value {
      fn try_unpack(self) -> Result<$t, Value> {
        use std::convert::TryInto;
        self.try_into()
      }
    }
  };
}

impl_try_unpack_tryfrom!(i64);
impl_try_unpack_tryfrom!(bool);
impl_try_unpack_tryfrom!(Vec<(Value, Value)>);
