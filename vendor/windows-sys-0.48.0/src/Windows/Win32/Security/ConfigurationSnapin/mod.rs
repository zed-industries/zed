pub type ISceSvcAttachmentData = *mut ::core::ffi::c_void;
pub type ISceSvcAttachmentPersistInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("CCF_SCESVC_ATTACHMENT");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT_DATA: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("CCF_SCESVC_ATTACHMENT_DATA");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_ROOT_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\SeCEdit");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceAnalysisServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceEventLog: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceTemplateServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceAnalysisServices: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceEventLog: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceTemplateServices: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceAnalysisServices: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceEventLog: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceTemplateServices: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type SCESVC_INFO_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type SCE_LOG_ERR_LEVEL = u32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_INFO {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_LINE {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::core::ffi::c_void,
    pub pfQueryInfo: PFSCE_QUERY_INFO,
    pub pfSetInfo: PFSCE_SET_INFO,
    pub pfFreeInfo: PFSCE_FREE_INFO,
    pub pfLogInfo: PFSCE_LOG_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCESVC_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_INFO {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_LINE {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_FREE_INFO = ::core::option::Option<unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_LOG_INFO = ::core::option::Option<unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32>;
