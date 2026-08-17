pub type IMILBitmapEffect = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectConnections = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectConnectionsInfo = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectConnector = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectConnectorInfo = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectEvents = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectFactory = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectGroup = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectGroupImpl = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectImpl = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectInputConnector = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectInteriorInputConnector = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectInteriorOutputConnector = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectOutputConnector = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectOutputConnectorImpl = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectPrimitive = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectPrimitiveImpl = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectRenderContext = *mut ::core::ffi::c_void;
pub type IMILBitmapEffectRenderContextImpl = *mut ::core::ffi::c_void;
pub type IMILBitmapEffects = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectBevel: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectDropShadow: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectEmboss: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Wpf\"`*"]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
