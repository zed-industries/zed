//! Integration with wasm-wave: string representations of values and types

#[cfg(feature = "component-model")]
mod component;
mod core;

macro_rules! unwrap_val {
    ($val:expr, $case:path, $name:expr) => {
        match $val {
            $case(v) => v,
            _ => panic!("called unwrap_{name} on non-{name} value", name = $name),
        }
    };
}
macro_rules! unwrap_2val {
    ($val:expr, $case:path, $name:expr) => {
        match $val {
            $case(n, v) => (n, v),
            _ => panic!("called unwrap_{name} on non-{name} value", name = $name),
        }
    };
}
pub(crate) use unwrap_2val;
pub(crate) use unwrap_val;

#[inline]
pub(crate) fn canonicalize_nan32(val: f32) -> f32 {
    if val.is_nan() { f32::NAN } else { val }
}

#[inline]
pub(crate) fn canonicalize_nan64(val: f64) -> f64 {
    if val.is_nan() { f64::NAN } else { val }
}
