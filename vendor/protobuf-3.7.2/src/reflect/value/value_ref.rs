use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem;

use crate::reflect::value::value_box::ReflectValueBox;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::MessageRef;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectEq;
use crate::reflect::ReflectEqMode;
use crate::reflect::RuntimeType;
use crate::MessageDyn;

/// A reference to a value
#[derive(Debug, Clone)]
pub enum ReflectValueRef<'a> {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(&'a str),
    /// `bytes`
    Bytes(&'a [u8]),
    /// `enum`
    Enum(
        EnumDescriptor,
        /// Enum value.
        ///
        /// Note when `allow_alias` option is enabled, more than one enum variant
        /// may have the same value.
        i32,
    ),
    /// `message`
    Message(MessageRef<'a>),
}

impl<'a> fmt::Display for ReflectValueRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReflectValueRef::U32(v) => write!(f, "{}", v),
            ReflectValueRef::U64(v) => write!(f, "{}", v),
            ReflectValueRef::I32(v) => write!(f, "{}", v),
            ReflectValueRef::I64(v) => write!(f, "{}", v),
            ReflectValueRef::F32(v) => write!(f, "{}", v),
            ReflectValueRef::F64(v) => write!(f, "{}", v),
            ReflectValueRef::Bool(v) => write!(f, "{}", v),
            ReflectValueRef::String(v) => write!(f, "{}", v),
            // TODO: better display
            ReflectValueRef::Bytes(v) => write!(f, "{:?}", v),
            ReflectValueRef::Enum(descriptor, value) => match descriptor.value_by_number(*value) {
                Some(v) => write!(f, "{}", v.name()),
                None => write!(f, "{}", value),
            },
            ReflectValueRef::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl<'a> ReflectValueRef<'a> {
    /// Get type of this value.
    pub fn get_type(&self) -> RuntimeType {
        match self {
            ReflectValueRef::U32(..) => RuntimeType::U32,
            ReflectValueRef::U64(..) => RuntimeType::U64,
            ReflectValueRef::I32(..) => RuntimeType::I32,
            ReflectValueRef::I64(..) => RuntimeType::I64,
            ReflectValueRef::F32(..) => RuntimeType::F32,
            ReflectValueRef::F64(..) => RuntimeType::F64,
            ReflectValueRef::Bool(..) => RuntimeType::Bool,
            ReflectValueRef::String(..) => RuntimeType::String,
            ReflectValueRef::Bytes(..) => RuntimeType::VecU8,
            ReflectValueRef::Enum(d, ..) => RuntimeType::Enum(d.clone()),
            ReflectValueRef::Message(m) => RuntimeType::Message(m.descriptor_dyn()),
        }
    }

    /// Value is "non-zero"?
    pub(crate) fn is_non_zero(&self) -> bool {
        match self {
            ReflectValueRef::U32(v) => *v != 0,
            ReflectValueRef::U64(v) => *v != 0,
            ReflectValueRef::I32(v) => *v != 0,
            ReflectValueRef::I64(v) => *v != 0,
            ReflectValueRef::F32(v) => *v != 0.,
            ReflectValueRef::F64(v) => *v != 0.,
            ReflectValueRef::Bool(v) => *v,
            ReflectValueRef::String(v) => !v.is_empty(),
            ReflectValueRef::Bytes(v) => !v.is_empty(),
            ReflectValueRef::Enum(_d, v) => *v != 0,
            ReflectValueRef::Message(_) => true,
        }
    }

    pub(crate) fn is_initialized(&self) -> bool {
        if let ReflectValueRef::Message(m) = self {
            m.is_initialized_dyn()
        } else {
            true
        }
    }

    /// Take `i32` value.
    pub fn to_i32(&self) -> Option<i32> {
        match *self {
            ReflectValueRef::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `i64` value.
    pub fn to_i64(&self) -> Option<i64> {
        match *self {
            ReflectValueRef::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u32` value.
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            ReflectValueRef::U32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u64` value.
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            ReflectValueRef::U64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f32` value.
    pub fn to_f32(&self) -> Option<f32> {
        match *self {
            ReflectValueRef::F32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f64` value.
    pub fn to_f64(&self) -> Option<f64> {
        match *self {
            ReflectValueRef::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `bool` value.
    pub fn to_bool(&self) -> Option<bool> {
        match *self {
            ReflectValueRef::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Take `str` value.
    pub fn to_str(&self) -> Option<&str> {
        match *self {
            ReflectValueRef::String(v) => Some(v),
            _ => None,
        }
    }

    /// Take `[u8]` value.
    pub fn to_bytes(&self) -> Option<&[u8]> {
        match *self {
            ReflectValueRef::Bytes(v) => Some(v),
            _ => None,
        }
    }

    /// Take enum value.
    pub fn to_enum_value(&self) -> Option<i32> {
        match *self {
            ReflectValueRef::Enum(_, v) => Some(v),
            _ => None,
        }
    }

    /// Take message value.
    pub fn to_message(&self) -> Option<MessageRef<'a>> {
        match self {
            ReflectValueRef::Message(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Clone to a box
    pub fn to_box(&self) -> ReflectValueBox {
        match self {
            ReflectValueRef::U32(v) => ReflectValueBox::U32(*v),
            ReflectValueRef::U64(v) => ReflectValueBox::U64(*v),
            ReflectValueRef::I32(v) => ReflectValueBox::I32(*v),
            ReflectValueRef::I64(v) => ReflectValueBox::I64(*v),
            ReflectValueRef::F32(v) => ReflectValueBox::F32(*v),
            ReflectValueRef::F64(v) => ReflectValueBox::F64(*v),
            ReflectValueRef::Bool(v) => ReflectValueBox::Bool(*v),
            ReflectValueRef::String(v) => ReflectValueBox::String((*v).to_owned()),
            ReflectValueRef::Bytes(v) => ReflectValueBox::Bytes((*v).to_owned()),
            ReflectValueRef::Enum(d, v) => ReflectValueBox::Enum(d.clone(), *v),
            ReflectValueRef::Message(v) => ReflectValueBox::Message(v.clone_box()),
        }
    }

    /// Convert a value to arbitrary value.
    pub fn downcast_clone<V: ProtobufValue>(&self) -> Result<V, Self> {
        self.to_box().downcast().map_err(|_| self.clone())
    }
}

pub enum ReflectValueMut<'a> {
    Message(&'a mut dyn MessageDyn),
}

impl<'a> ReflectEq for ReflectValueRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        use crate::reflect::value::value_ref::ReflectValueRef::*;
        match (self, that) {
            (U32(a), U32(b)) => a == b,
            (U64(a), U64(b)) => a == b,
            (I32(a), I32(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (F32(a), F32(b)) => {
                if a.is_nan() || b.is_nan() {
                    a.is_nan() == b.is_nan() && mode.nan_equal
                } else {
                    a == b
                }
            }
            (F64(a), F64(b)) => {
                if a.is_nan() || b.is_nan() {
                    a.is_nan() == b.is_nan() && mode.nan_equal
                } else {
                    a == b
                }
            }
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (Enum(ad, a), Enum(bd, b)) => ad == bd && a == b,
            (Message(a), Message(b)) => a.reflect_eq(b, mode),
            _ => false,
        }
    }
}

impl<'a> PartialEq for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        use self::ReflectValueRef::*;
        match (self, other) {
            (U32(a), U32(b)) => a == b,
            (U64(a), U64(b)) => a == b,
            (I32(a), I32(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            // should probably NaN == NaN here
            (F32(a), F32(b)) => a == b,
            (F64(a), F64(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (Enum(da, a), Enum(db, b)) => da == db && a == b,
            (Message(a), Message(b)) => {
                MessageDescriptor::reflect_eq_maybe_unrelated(&**a, &**b, &ReflectEqMode::default())
            }
            _ => false,
        }
    }
}

impl<'a> PartialEq<ReflectValueRef<'a>> for ReflectValueBox {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        self.as_value_ref() == *other
    }
}

// Panics if contained type is not hashable
impl<'a> Hash for ReflectValueRef<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use self::ReflectValueRef::*;
        Hash::hash(&mem::discriminant(self), state);
        match self {
            U32(v) => Hash::hash(&v, state),
            U64(v) => Hash::hash(&v, state),
            I32(v) => Hash::hash(&v, state),
            I64(v) => Hash::hash(&v, state),
            Bool(v) => Hash::hash(&v, state),
            String(v) => Hash::hash(&v, state),
            Bytes(v) => Hash::hash(&v, state),
            Enum(_d, v) => Hash::hash(v, state),
            F32(_) | F64(_) | Message(_) => panic!("not hashable: {:?}", self),
        }
    }
}

impl<'a> From<EnumValueDescriptor> for ReflectValueRef<'a> {
    fn from(v: EnumValueDescriptor) -> Self {
        let number = v.value();
        ReflectValueRef::Enum(v.enum_descriptor, number)
    }
}

impl From<u32> for ReflectValueRef<'_> {
    fn from(v: u32) -> Self {
        ReflectValueRef::U32(v)
    }
}

impl From<i32> for ReflectValueRef<'_> {
    fn from(v: i32) -> Self {
        ReflectValueRef::I32(v)
    }
}

impl From<u64> for ReflectValueRef<'_> {
    fn from(v: u64) -> Self {
        ReflectValueRef::U64(v)
    }
}

impl From<i64> for ReflectValueRef<'_> {
    fn from(v: i64) -> Self {
        ReflectValueRef::I64(v)
    }
}

impl From<f32> for ReflectValueRef<'_> {
    fn from(v: f32) -> Self {
        ReflectValueRef::F32(v)
    }
}

impl From<f64> for ReflectValueRef<'_> {
    fn from(v: f64) -> Self {
        ReflectValueRef::F64(v)
    }
}

impl From<bool> for ReflectValueRef<'_> {
    fn from(v: bool) -> Self {
        ReflectValueRef::Bool(v)
    }
}

impl<'a> From<&'a str> for ReflectValueRef<'a> {
    fn from(v: &'a str) -> Self {
        ReflectValueRef::String(v)
    }
}

impl<'a> From<&'a [u8]> for ReflectValueRef<'a> {
    fn from(v: &'a [u8]) -> Self {
        ReflectValueRef::Bytes(v)
    }
}
