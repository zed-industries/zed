use std::fmt;
use std::marker::PhantomData;
use std::mem;

use crate::reflect::runtime_types::RuntimeTypeEnumOrUnknown;
use crate::reflect::EnumDescriptor;
use crate::reflect::ProtobufValue;
use crate::Enum;
use crate::EnumFull;

/// Protobuf enums with possibly unknown values are preserved in this struct.
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(transparent)]
// This should be <E: ProtobufEnum> when it no longer prevents using const fns.
pub struct EnumOrUnknown<E> {
    value: i32,
    _marker: PhantomData<E>,
}

// Move into <E: ProtobufEnum> when no longer:
// > trait bounds other than `Sized` on const fn parameters are unstable.
impl<E> EnumOrUnknown<E> {
    /// Construct from any `i32` value.
    ///
    /// Note passed value is not required to be a valid enum value.
    pub const fn from_i32(value: i32) -> EnumOrUnknown<E> {
        EnumOrUnknown {
            value,
            _marker: PhantomData,
        }
    }
}

impl<E: Enum> EnumOrUnknown<E> {
    /// Construct from typed enum
    pub fn new(e: E) -> EnumOrUnknown<E> {
        EnumOrUnknown::from_i32(e.value())
    }

    /// Get contained `i32` value of enum
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get `i32` value as typed enum. Return `None` is value is unknown.
    pub fn enum_value(&self) -> Result<E, i32> {
        E::from_i32(self.value).ok_or(self.value)
    }

    /// Get contained enum, panic if value is unknown.
    pub fn unwrap(&self) -> E {
        self.enum_value().unwrap()
    }

    /// Get `i32` value as typed enum.
    /// Return default enum value (first value) if value is unknown.
    pub fn enum_value_or_default(&self) -> E {
        self.enum_value().unwrap_or_default()
    }

    /// Get `i32` value as typed enum.
    /// Return given enum value if value is unknown.
    pub fn enum_value_or(&self, map_unknown: E) -> E {
        self.enum_value().unwrap_or(map_unknown)
    }

    pub(crate) fn cast_to_values(enums: &[EnumOrUnknown<E>]) -> &[i32] {
        assert_eq!(mem::size_of::<EnumOrUnknown<E>>(), mem::size_of::<i32>());
        // SAFETY: `EnumOrUnknown` is `repr(C)`.
        unsafe { std::slice::from_raw_parts(enums.as_ptr() as *const i32, enums.len()) }
    }
}

impl<E: EnumFull> EnumOrUnknown<E> {
    /// Get enum descriptor by type.
    pub fn enum_descriptor() -> EnumDescriptor {
        E::enum_descriptor()
    }
}

impl<E: Enum> From<E> for EnumOrUnknown<E> {
    fn from(e: E) -> Self {
        EnumOrUnknown::new(e)
    }
}

impl<E: Enum> Default for EnumOrUnknown<E> {
    fn default() -> EnumOrUnknown<E> {
        EnumOrUnknown::new(E::default())
    }
}

impl<E: Enum + fmt::Debug> fmt::Debug for EnumOrUnknown<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.enum_value() {
            Ok(e) => fmt::Debug::fmt(&e, f),
            Err(e) => fmt::Debug::fmt(&e, f),
        }
    }
}

impl<E: EnumFull> ProtobufValue for EnumOrUnknown<E> {
    type RuntimeType = RuntimeTypeEnumOrUnknown<E>;
}
