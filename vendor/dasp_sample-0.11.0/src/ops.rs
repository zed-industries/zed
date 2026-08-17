pub mod f32 {
    #[allow(unused_imports)]
    use core;

    #[cfg(not(feature = "std"))]
    pub fn sqrt(x: f32) -> f32 {
        unsafe { core::intrinsics::sqrtf32(x) }
    }
    #[cfg(feature = "std")]
    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }
}

pub mod f64 {
    #[allow(unused_imports)]
    use core;

    #[cfg(not(feature = "std"))]
    pub fn sqrt(x: f64) -> f64 {
        unsafe { core::intrinsics::sqrtf64(x) }
    }
    #[cfg(feature = "std")]
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }
}
