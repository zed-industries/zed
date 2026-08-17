#[cfg(not(feature = "std"))]
pub use libm::{floorf, roundf, truncf};

#[cfg(feature = "std")]
#[inline]
pub fn floorf(x: f32) -> f32 {
    x.floor()
}

#[cfg(feature = "std")]
#[inline]
pub fn roundf(x: f32) -> f32 {
    x.round()
}

#[cfg(feature = "std")]
#[inline]
pub fn truncf(x: f32) -> f32 {
    x.trunc()
}
