//! Implementation of `Ord` and `Eq` for `Value` that works around limitations
//! of f64 by using a `TotalOrdF64` type.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
};

use crate::value::Value;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum OrdValue<'a> {
    Null,
    Int64(i64),
    Float64(TotalOrdF64),
    Boolean(bool),
    String(&'a String),
    Bytes(&'a Vec<u8>),
    Array(&'a Vec<Value>),
    Object(&'a BTreeMap<String, Value>),
}

impl<'a> From<&'a Value> for OrdValue<'a> {
    fn from(v: &'a Value) -> OrdValue<'a> {
        match v {
            Value::Null => OrdValue::Null,
            Value::Int64(x) => OrdValue::Int64(*x),
            Value::Float64(x) => OrdValue::Float64(TotalOrdF64(*x)),
            Value::Boolean(x) => OrdValue::Boolean(*x),
            Value::String(x) => OrdValue::String(x),
            Value::Bytes(x) => OrdValue::Bytes(x),
            Value::Array(x) => OrdValue::Array(x),
            Value::Object(x) => OrdValue::Object(x),
        }
    }
}

#[derive(Clone, Debug)]
struct TotalOrdF64(f64);

impl Ord for TotalOrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl PartialOrd for TotalOrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for TotalOrdF64 {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}
impl Eq for TotalOrdF64 {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        OrdValue::from(self).cmp(&OrdValue::from(other))
    }
}
