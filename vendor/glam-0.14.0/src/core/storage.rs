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
pub struct Vector2x2<V2> {
    pub x_axis: V2,
    pub y_axis: V2,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Vector3x3<V3> {
    pub x_axis: V3,
    pub y_axis: V3,
    pub z_axis: V3,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Vector4x4<V4> {
    pub x_axis: V4,
    pub y_axis: V4,
    pub z_axis: V4,
    pub w_axis: V4,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(align(16))]
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
