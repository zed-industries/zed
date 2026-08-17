use std::mem;
use std::mem::MaybeUninit;

use crate::well_known_types;

/// `MaybeUninit::write_slice` is not stable.
pub(crate) fn maybe_uninit_write_slice<'a, T>(
    this: &'a mut [MaybeUninit<T>],
    src: &[T],
) -> &'a mut [T]
where
    T: Copy,
{
    // SAFETY: copy-paste from rust stdlib.

    let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };

    this.copy_from_slice(uninit_src);

    unsafe { &mut *(this as *mut [MaybeUninit<T>] as *mut [T]) }
}

/// `MaybeUninit::array_assume_init` is not stable.
#[inline]
pub(crate) unsafe fn maybe_ununit_array_assume_init<T, const N: usize>(
    array: [MaybeUninit<T>; N],
) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    (&array as *const _ as *const [T; N]).read()
}

// bool <-> BoolValue

impl From<well_known_types::wrappers::BoolValue> for bool {
    fn from(inner: well_known_types::wrappers::BoolValue) -> Self {
        inner.value
    }
}

impl From<bool> for well_known_types::wrappers::BoolValue {
    fn from(inner: bool) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// Vec<u8> <-> BytesValue

impl From<well_known_types::wrappers::BytesValue> for Vec<u8> {
    fn from(inner: well_known_types::wrappers::BytesValue) -> Self {
        inner.value
    }
}

impl From<Vec<u8>> for well_known_types::wrappers::BytesValue {
    fn from(inner: Vec<u8>) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// f64 <-> DoubleValue

impl From<well_known_types::wrappers::DoubleValue> for f64 {
    fn from(inner: well_known_types::wrappers::DoubleValue) -> Self {
        inner.value
    }
}

impl From<f64> for well_known_types::wrappers::DoubleValue {
    fn from(inner: f64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// f32 <-> FloatValue

impl From<well_known_types::wrappers::FloatValue> for f32 {
    fn from(inner: well_known_types::wrappers::FloatValue) -> Self {
        inner.value
    }
}

impl From<f32> for well_known_types::wrappers::FloatValue {
    fn from(inner: f32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// i32 <-> Int32Value

impl From<well_known_types::wrappers::Int32Value> for i32 {
    fn from(inner: well_known_types::wrappers::Int32Value) -> Self {
        inner.value
    }
}

impl From<i32> for well_known_types::wrappers::Int32Value {
    fn from(inner: i32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// i64 <-> Int64Value

impl From<well_known_types::wrappers::Int64Value> for i64 {
    fn from(inner: well_known_types::wrappers::Int64Value) -> Self {
        inner.value
    }
}

impl From<i64> for well_known_types::wrappers::Int64Value {
    fn from(inner: i64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// u32 <-> UInt32Value

impl From<well_known_types::wrappers::UInt32Value> for u32 {
    fn from(inner: well_known_types::wrappers::UInt32Value) -> Self {
        inner.value
    }
}

impl From<u32> for well_known_types::wrappers::UInt32Value {
    fn from(inner: u32) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// u64 <-> UInt64Value

impl From<well_known_types::wrappers::UInt64Value> for u64 {
    fn from(inner: well_known_types::wrappers::UInt64Value) -> Self {
        inner.value
    }
}

impl From<u64> for well_known_types::wrappers::UInt64Value {
    fn from(inner: u64) -> Self {
        let mut value = Self::new();
        value.value = inner;
        value
    }
}

// () <-> Empty

impl From<well_known_types::empty::Empty> for () {
    fn from(_inner: well_known_types::empty::Empty) -> Self {}
}

impl From<()> for well_known_types::empty::Empty {
    fn from(_inner: ()) -> Self {
        Self::new()
    }
}
