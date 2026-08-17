#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct XYZW<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Columns2<V> {
    pub x_axis: V,
    pub y_axis: V,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Columns3<V> {
    pub x_axis: V,
    pub y_axis: V,
    pub z_axis: V,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Columns4<V> {
    pub x_axis: V,
    pub y_axis: V,
    pub z_axis: V,
    pub w_axis: V,
}

/// The `XYZF32A16` is used for the `Vec3A` type, that is a 16 btye aligned `XYZ<f32>` type.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C, align(16)))]
pub struct XYZF32A16 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<XYZW<f32>> for XYZF32A16 {
    #[inline(always)]
    fn from(v: XYZW<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<XYZ<f32>> for XYZF32A16 {
    #[inline(always)]
    fn from(v: XYZ<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<XYZF32A16> for XYZ<f32> {
    #[inline(always)]
    fn from(v: XYZF32A16) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<XYZF32A16> for XY<f32> {
    #[inline(always)]
    fn from(v: XYZF32A16) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C, align(16))]
pub(crate) struct Align16<T>(pub T);

impl<T> Align16<T> {
    #[allow(dead_code)]
    pub fn as_ptr(&self) -> *const T {
        &self.0
    }

    #[allow(dead_code)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.0
    }
}

#[test]
fn test_align16() {
    use core::{mem, ptr};
    let mut a = Align16::<f32>(1.0);
    assert_eq!(mem::align_of_val(&a), 16);
    unsafe {
        assert_eq!(ptr::read(a.as_ptr()).to_bits(), f32::to_bits(1.0));
        ptr::write(a.as_mut_ptr(), -1.0);
    }
    assert_eq!(a.0.to_bits(), f32::to_bits(-1.0));
}
