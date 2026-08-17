#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Plane {
    pub Normal: windows_numerics::Vector3,
    pub D: f32,
}
impl windows_core::TypeKind for Plane {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Plane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quaternion {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl windows_core::TypeKind for Quaternion {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Quaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rational {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl windows_core::TypeKind for Rational {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Rational {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Rational;u4;u4)");
}
