pub type IObjectArray = *mut ::core::ffi::c_void;
pub type IObjectCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub type DEVICE_SCALE_FACTOR = i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = 0i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = 100i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = 120i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = 125i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = 140i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = 150i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = 160i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = 175i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = 180i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = 200i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = 225i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = 250i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = 300i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = 350i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = 400i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = 450i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = 500i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub type PERCEIVED = i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = -3i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = -3i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = -2i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = -1i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = 0i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = 1i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = 2i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = 3i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = 4i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = 5i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = 6i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = 7i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = 8i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = 9i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = 10i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const PERCEIVED_TYPE_LAST: PERCEIVED = 10i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub type SHCOLSTATE = i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = 2i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = 3i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = 15i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = 16i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_SLOW: SHCOLSTATE = 32i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = 64i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = 128i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = 256i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = 512i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = 1024i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = 2048i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = 65536i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = 131072i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = 262144i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = 4096i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = 8192i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = 16384i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = 61440i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub type STRRET_TYPE = i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_WSTR: STRRET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_OFFSET: STRRET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub const STRRET_CSTR: STRRET_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct COMDLG_FILTERSPEC {
    pub pszName: ::windows_sys::core::PCWSTR,
    pub pszSpec: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
impl ::core::marker::Copy for SHELLDETAILS {}
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl ::core::marker::Copy for SHITEMID {}
impl ::core::clone::Clone for SHITEMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl ::core::marker::Copy for STRRET {}
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`*"]
pub union STRRET_0 {
    pub pOleStr: ::windows_sys::core::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
impl ::core::marker::Copy for STRRET_0 {}
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
