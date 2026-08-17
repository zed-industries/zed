::windows_targets::link ! ( "ole32.dll""system" #[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"] fn CoGetInterceptor ( iidintercepted : *const ::windows_sys::core::GUID , punkouter : ::windows_sys::core::IUnknown , iid : *const ::windows_sys::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "ole32.dll""system" #[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"] fn CoGetInterceptorFromTypeInfo ( iidintercepted : *const ::windows_sys::core::GUID , punkouter : ::windows_sys::core::IUnknown , typeinfo : super:: ITypeInfo , iid : *const ::windows_sys::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
pub type ICallFrame = *mut ::core::ffi::c_void;
pub type ICallFrameEvents = *mut ::core::ffi::c_void;
pub type ICallFrameWalker = *mut ::core::ffi::c_void;
pub type ICallIndirect = *mut ::core::ffi::c_void;
pub type ICallInterceptor = *mut ::core::ffi::c_void;
pub type ICallUnmarshal = *mut ::core::ffi::c_void;
pub type IInterfaceRelated = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_COPY = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_FREE = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = 16i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = 31i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_NULL = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = 0i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub type CALLFRAME_WALK = i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows_sys::core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::windows_sys::core::IUnknown,
    pub guidTransferSyntax: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
