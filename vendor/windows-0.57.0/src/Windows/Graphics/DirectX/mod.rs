#[cfg(feature = "Graphics_DirectX_Direct3D11")]
pub mod Direct3D11;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DirectXAlphaMode(pub i32);
impl DirectXAlphaMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Premultiplied: Self = Self(1i32);
    pub const Straight: Self = Self(2i32);
    pub const Ignore: Self = Self(3i32);
}
impl windows_core::TypeKind for DirectXAlphaMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DirectXAlphaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DirectXAlphaMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DirectXAlphaMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXAlphaMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DirectXColorSpace(pub i32);
impl DirectXColorSpace {
    pub const RgbFullG22NoneP709: Self = Self(0i32);
    pub const RgbFullG10NoneP709: Self = Self(1i32);
    pub const RgbStudioG22NoneP709: Self = Self(2i32);
    pub const RgbStudioG22NoneP2020: Self = Self(3i32);
    pub const Reserved: Self = Self(4i32);
    pub const YccFullG22NoneP709X601: Self = Self(5i32);
    pub const YccStudioG22LeftP601: Self = Self(6i32);
    pub const YccFullG22LeftP601: Self = Self(7i32);
    pub const YccStudioG22LeftP709: Self = Self(8i32);
    pub const YccFullG22LeftP709: Self = Self(9i32);
    pub const YccStudioG22LeftP2020: Self = Self(10i32);
    pub const YccFullG22LeftP2020: Self = Self(11i32);
    pub const RgbFullG2084NoneP2020: Self = Self(12i32);
    pub const YccStudioG2084LeftP2020: Self = Self(13i32);
    pub const RgbStudioG2084NoneP2020: Self = Self(14i32);
    pub const YccStudioG22TopLeftP2020: Self = Self(15i32);
    pub const YccStudioG2084TopLeftP2020: Self = Self(16i32);
    pub const RgbFullG22NoneP2020: Self = Self(17i32);
    pub const YccStudioGHlgTopLeftP2020: Self = Self(18i32);
    pub const YccFullGHlgTopLeftP2020: Self = Self(19i32);
    pub const RgbStudioG24NoneP709: Self = Self(20i32);
    pub const RgbStudioG24NoneP2020: Self = Self(21i32);
    pub const YccStudioG24LeftP709: Self = Self(22i32);
    pub const YccStudioG24LeftP2020: Self = Self(23i32);
    pub const YccStudioG24TopLeftP2020: Self = Self(24i32);
}
impl windows_core::TypeKind for DirectXColorSpace {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DirectXColorSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DirectXColorSpace").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DirectXColorSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXColorSpace;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DirectXPixelFormat(pub i32);
impl DirectXPixelFormat {
    pub const Unknown: Self = Self(0i32);
    pub const R32G32B32A32Typeless: Self = Self(1i32);
    pub const R32G32B32A32Float: Self = Self(2i32);
    pub const R32G32B32A32UInt: Self = Self(3i32);
    pub const R32G32B32A32Int: Self = Self(4i32);
    pub const R32G32B32Typeless: Self = Self(5i32);
    pub const R32G32B32Float: Self = Self(6i32);
    pub const R32G32B32UInt: Self = Self(7i32);
    pub const R32G32B32Int: Self = Self(8i32);
    pub const R16G16B16A16Typeless: Self = Self(9i32);
    pub const R16G16B16A16Float: Self = Self(10i32);
    pub const R16G16B16A16UIntNormalized: Self = Self(11i32);
    pub const R16G16B16A16UInt: Self = Self(12i32);
    pub const R16G16B16A16IntNormalized: Self = Self(13i32);
    pub const R16G16B16A16Int: Self = Self(14i32);
    pub const R32G32Typeless: Self = Self(15i32);
    pub const R32G32Float: Self = Self(16i32);
    pub const R32G32UInt: Self = Self(17i32);
    pub const R32G32Int: Self = Self(18i32);
    pub const R32G8X24Typeless: Self = Self(19i32);
    pub const D32FloatS8X24UInt: Self = Self(20i32);
    pub const R32FloatX8X24Typeless: Self = Self(21i32);
    pub const X32TypelessG8X24UInt: Self = Self(22i32);
    pub const R10G10B10A2Typeless: Self = Self(23i32);
    pub const R10G10B10A2UIntNormalized: Self = Self(24i32);
    pub const R10G10B10A2UInt: Self = Self(25i32);
    pub const R11G11B10Float: Self = Self(26i32);
    pub const R8G8B8A8Typeless: Self = Self(27i32);
    pub const R8G8B8A8UIntNormalized: Self = Self(28i32);
    pub const R8G8B8A8UIntNormalizedSrgb: Self = Self(29i32);
    pub const R8G8B8A8UInt: Self = Self(30i32);
    pub const R8G8B8A8IntNormalized: Self = Self(31i32);
    pub const R8G8B8A8Int: Self = Self(32i32);
    pub const R16G16Typeless: Self = Self(33i32);
    pub const R16G16Float: Self = Self(34i32);
    pub const R16G16UIntNormalized: Self = Self(35i32);
    pub const R16G16UInt: Self = Self(36i32);
    pub const R16G16IntNormalized: Self = Self(37i32);
    pub const R16G16Int: Self = Self(38i32);
    pub const R32Typeless: Self = Self(39i32);
    pub const D32Float: Self = Self(40i32);
    pub const R32Float: Self = Self(41i32);
    pub const R32UInt: Self = Self(42i32);
    pub const R32Int: Self = Self(43i32);
    pub const R24G8Typeless: Self = Self(44i32);
    pub const D24UIntNormalizedS8UInt: Self = Self(45i32);
    pub const R24UIntNormalizedX8Typeless: Self = Self(46i32);
    pub const X24TypelessG8UInt: Self = Self(47i32);
    pub const R8G8Typeless: Self = Self(48i32);
    pub const R8G8UIntNormalized: Self = Self(49i32);
    pub const R8G8UInt: Self = Self(50i32);
    pub const R8G8IntNormalized: Self = Self(51i32);
    pub const R8G8Int: Self = Self(52i32);
    pub const R16Typeless: Self = Self(53i32);
    pub const R16Float: Self = Self(54i32);
    pub const D16UIntNormalized: Self = Self(55i32);
    pub const R16UIntNormalized: Self = Self(56i32);
    pub const R16UInt: Self = Self(57i32);
    pub const R16IntNormalized: Self = Self(58i32);
    pub const R16Int: Self = Self(59i32);
    pub const R8Typeless: Self = Self(60i32);
    pub const R8UIntNormalized: Self = Self(61i32);
    pub const R8UInt: Self = Self(62i32);
    pub const R8IntNormalized: Self = Self(63i32);
    pub const R8Int: Self = Self(64i32);
    pub const A8UIntNormalized: Self = Self(65i32);
    pub const R1UIntNormalized: Self = Self(66i32);
    pub const R9G9B9E5SharedExponent: Self = Self(67i32);
    pub const R8G8B8G8UIntNormalized: Self = Self(68i32);
    pub const G8R8G8B8UIntNormalized: Self = Self(69i32);
    pub const BC1Typeless: Self = Self(70i32);
    pub const BC1UIntNormalized: Self = Self(71i32);
    pub const BC1UIntNormalizedSrgb: Self = Self(72i32);
    pub const BC2Typeless: Self = Self(73i32);
    pub const BC2UIntNormalized: Self = Self(74i32);
    pub const BC2UIntNormalizedSrgb: Self = Self(75i32);
    pub const BC3Typeless: Self = Self(76i32);
    pub const BC3UIntNormalized: Self = Self(77i32);
    pub const BC3UIntNormalizedSrgb: Self = Self(78i32);
    pub const BC4Typeless: Self = Self(79i32);
    pub const BC4UIntNormalized: Self = Self(80i32);
    pub const BC4IntNormalized: Self = Self(81i32);
    pub const BC5Typeless: Self = Self(82i32);
    pub const BC5UIntNormalized: Self = Self(83i32);
    pub const BC5IntNormalized: Self = Self(84i32);
    pub const B5G6R5UIntNormalized: Self = Self(85i32);
    pub const B5G5R5A1UIntNormalized: Self = Self(86i32);
    pub const B8G8R8A8UIntNormalized: Self = Self(87i32);
    pub const B8G8R8X8UIntNormalized: Self = Self(88i32);
    pub const R10G10B10XRBiasA2UIntNormalized: Self = Self(89i32);
    pub const B8G8R8A8Typeless: Self = Self(90i32);
    pub const B8G8R8A8UIntNormalizedSrgb: Self = Self(91i32);
    pub const B8G8R8X8Typeless: Self = Self(92i32);
    pub const B8G8R8X8UIntNormalizedSrgb: Self = Self(93i32);
    pub const BC6HTypeless: Self = Self(94i32);
    pub const BC6H16UnsignedFloat: Self = Self(95i32);
    pub const BC6H16Float: Self = Self(96i32);
    pub const BC7Typeless: Self = Self(97i32);
    pub const BC7UIntNormalized: Self = Self(98i32);
    pub const BC7UIntNormalizedSrgb: Self = Self(99i32);
    pub const Ayuv: Self = Self(100i32);
    pub const Y410: Self = Self(101i32);
    pub const Y416: Self = Self(102i32);
    pub const NV12: Self = Self(103i32);
    pub const P010: Self = Self(104i32);
    pub const P016: Self = Self(105i32);
    pub const Opaque420: Self = Self(106i32);
    pub const Yuy2: Self = Self(107i32);
    pub const Y210: Self = Self(108i32);
    pub const Y216: Self = Self(109i32);
    pub const NV11: Self = Self(110i32);
    pub const AI44: Self = Self(111i32);
    pub const IA44: Self = Self(112i32);
    pub const P8: Self = Self(113i32);
    pub const A8P8: Self = Self(114i32);
    pub const B4G4R4A4UIntNormalized: Self = Self(115i32);
    pub const P208: Self = Self(130i32);
    pub const V208: Self = Self(131i32);
    pub const V408: Self = Self(132i32);
    pub const SamplerFeedbackMinMipOpaque: Self = Self(189i32);
    pub const SamplerFeedbackMipRegionUsedOpaque: Self = Self(190i32);
}
impl windows_core::TypeKind for DirectXPixelFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DirectXPixelFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DirectXPixelFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DirectXPixelFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DirectXPrimitiveTopology(pub i32);
impl DirectXPrimitiveTopology {
    pub const Undefined: Self = Self(0i32);
    pub const PointList: Self = Self(1i32);
    pub const LineList: Self = Self(2i32);
    pub const LineStrip: Self = Self(3i32);
    pub const TriangleList: Self = Self(4i32);
    pub const TriangleStrip: Self = Self(5i32);
}
impl windows_core::TypeKind for DirectXPrimitiveTopology {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DirectXPrimitiveTopology {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DirectXPrimitiveTopology").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DirectXPrimitiveTopology {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXPrimitiveTopology;i4)");
}
