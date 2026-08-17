#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix3x2 {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl windows_core::TypeKind for Matrix3x2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix3x2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix4x4 {
    pub M11: f32,
    pub M12: f32,
    pub M13: f32,
    pub M14: f32,
    pub M21: f32,
    pub M22: f32,
    pub M23: f32,
    pub M24: f32,
    pub M31: f32,
    pub M32: f32,
    pub M33: f32,
    pub M34: f32,
    pub M41: f32,
    pub M42: f32,
    pub M43: f32,
    pub M44: f32,
}
impl windows_core::TypeKind for Matrix4x4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix4x4 {
    const SIGNATURE :windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice ( b"struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4)" ) ;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2 {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for Vector2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector2;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl windows_core::TypeKind for Vector3 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector4 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl windows_core::TypeKind for Vector4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4)",
    );
}
