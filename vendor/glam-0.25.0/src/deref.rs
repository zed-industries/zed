#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Cols2<V> {
    pub x_axis: V,
    pub y_axis: V,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Cols3<V> {
    pub x_axis: V,
    pub y_axis: V,
    pub z_axis: V,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Cols4<V> {
    pub x_axis: V,
    pub y_axis: V,
    pub z_axis: V,
    pub w_axis: V,
}
