pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = RDPSRAPI_APP_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTENDEE_DISCONNECT_REASON(pub i32);
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(1i32);
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = RDPENCOMAPI_ATTENDEE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANNEL_ACCESS_ENUM(pub i32);
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(0i32);
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANNEL_FLAGS(pub i32);
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = CHANNEL_FLAGS(4i32);
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = CHANNEL_FLAGS(1i32);
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = CHANNEL_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANNEL_PRIORITY(pub i32);
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = CHANNEL_PRIORITY(2i32);
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = CHANNEL_PRIORITY(0i32);
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = CHANNEL_PRIORITY(1i32);
pub const CONST_ATTENDEE_ID_DEFAULT: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
pub const CONST_ATTENDEE_ID_EVERYONE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
pub const CONST_ATTENDEE_ID_HOST: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(0i32);
pub const CONST_CONN_INTERVAL: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(50i32);
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(1024i32);
pub const CONST_MAX_CHANNEL_NAME_LEN: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(8i32);
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(409600i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CTRL_LEVEL(pub i32);
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(3i32);
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = CTRL_LEVEL(1i32);
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = CTRL_LEVEL(4i32);
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = CTRL_LEVEL(2i32);
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplication, IRDPSRAPIApplication_Vtbl, 0x41e7a09d_eb7a_436e_935d_780ca2628324);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplication {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplication, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplication {
    pub unsafe fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Windows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShared)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Windows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Shared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShared: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplication_Impl: super::Com::IDispatch_Impl {
    fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIApplication_Vtbl {
    pub const fn new<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Windows<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplication_Impl::Windows(this) {
                    Ok(ok__) => {
                        pwindowlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplication_Impl::Id(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shared<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplication_Impl::Shared(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShared<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIApplication_Impl::SetShared(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplication_Impl::Name(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Flags<Identity: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplication_Impl::Flags(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Windows: Windows::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Shared: Shared::<Identity, OFFSET>,
            SetShared: SetShared::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplication as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIApplication {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplicationFilter, IRDPSRAPIApplicationFilter_Vtbl, 0xd20f10ca_6637_4f06_b1d5_277ea7e5160d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplicationFilter {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplicationFilter, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationFilter {
    pub unsafe fn Applications(&self) -> windows_core::Result<IRDPSRAPIApplicationList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Applications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Windows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), newval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Applications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Windows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplicationFilter_Impl: super::Com::IDispatch_Impl {
    fn Applications(&self) -> windows_core::Result<IRDPSRAPIApplicationList>;
    fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIApplicationFilter_Vtbl {
    pub const fn new<Identity: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Applications<Identity: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplications: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplicationFilter_Impl::Applications(this) {
                    Ok(ok__) => {
                        papplications.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Windows<Identity: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindows: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplicationFilter_Impl::Windows(this) {
                    Ok(ok__) => {
                        pwindows.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplicationFilter_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIApplicationFilter_Impl::SetEnabled(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Applications: Applications::<Identity, OFFSET>,
            Windows: Windows::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationFilter as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIApplicationFilter {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplicationList, IRDPSRAPIApplicationList_Vtbl, 0xd4b4aeb3_22dc_4837_b3b6_42ea2517849a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplicationList {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplicationList, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationList {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIApplicationList_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIApplication>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIApplicationList_Vtbl {
    pub const fn new<Identity: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplicationList_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, papplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIApplicationList_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                    Ok(ok__) => {
                        papplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationList as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIApplicationList {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendee, IRDPSRAPIAttendee_Vtbl, 0xec0671b3_1b78_4b80_a464_9132247543e3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendee {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendee, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendee {
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoteName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ControlLevel(&self) -> windows_core::Result<CTRL_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetControlLevel)(windows_core::Interface::as_raw(self), pnewval).ok() }
    }
    pub unsafe fn Invitation(&self) -> windows_core::Result<IRDPSRAPIInvitation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invitation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TerminateConnection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TerminateConnection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConnectivityInfo(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectivityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CTRL_LEVEL) -> windows_core::HRESULT,
    pub SetControlLevel: unsafe extern "system" fn(*mut core::ffi::c_void, CTRL_LEVEL) -> windows_core::HRESULT,
    pub Invitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminateConnection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ConnectivityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendee_Impl: super::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn RemoteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ControlLevel(&self) -> windows_core::Result<CTRL_LEVEL>;
    fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> windows_core::Result<()>;
    fn Invitation(&self) -> windows_core::Result<IRDPSRAPIInvitation>;
    fn TerminateConnection(&self) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn ConnectivityInfo(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIAttendee_Vtbl {
    pub const fn new<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::Id(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoteName<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::RemoteName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ControlLevel<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut CTRL_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::ControlLevel(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetControlLevel<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: CTRL_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAttendee_Impl::SetControlLevel(this, core::mem::transmute_copy(&pnewval)).into()
            }
        }
        unsafe extern "system" fn Invitation<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::Invitation(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TerminateConnection<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAttendee_Impl::TerminateConnection(this).into()
            }
        }
        unsafe extern "system" fn Flags<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::Flags(this) {
                    Ok(ok__) => {
                        plflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Identity: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendee_Impl::ConnectivityInfo(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            RemoteName: RemoteName::<Identity, OFFSET>,
            ControlLevel: ControlLevel::<Identity, OFFSET>,
            SetControlLevel: SetControlLevel::<Identity, OFFSET>,
            Invitation: Invitation::<Identity, OFFSET>,
            TerminateConnection: TerminateConnection::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            ConnectivityInfo: ConnectivityInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendee as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIAttendee {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendeeDisconnectInfo, IRDPSRAPIAttendeeDisconnectInfo_Vtbl, 0xc187689f_447c_44a1_9c14_fffbb3b7ec17);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendeeDisconnectInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendeeDisconnectInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeDisconnectInfo {
    pub unsafe fn Attendee(&self) -> windows_core::Result<IRDPSRAPIAttendee> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attendee)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Reason(&self) -> windows_core::Result<ATTENDEE_DISCONNECT_REASON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Code(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Code)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Attendee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ATTENDEE_DISCONNECT_REASON) -> windows_core::HRESULT,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendeeDisconnectInfo_Impl: super::Com::IDispatch_Impl {
    fn Attendee(&self) -> windows_core::Result<IRDPSRAPIAttendee>;
    fn Reason(&self) -> windows_core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub const fn new<Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Attendee<Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendeeDisconnectInfo_Impl::Attendee(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reason<Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendeeDisconnectInfo_Impl::Reason(this) {
                    Ok(ok__) => {
                        preason.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Code<Identity: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendeeDisconnectInfo_Impl::Code(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Attendee: Attendee::<Identity, OFFSET>,
            Reason: Reason::<Identity, OFFSET>,
            Code: Code::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeDisconnectInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIAttendeeDisconnectInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendeeManager, IRDPSRAPIAttendeeManager_Vtbl, 0xba3a37e8_33da_4749_8da0_07fa34da7944);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendeeManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendeeManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, id: i32) -> windows_core::Result<IRDPSRAPIAttendee> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIAttendeeManager_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, id: i32) -> windows_core::Result<IRDPSRAPIAttendee>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIAttendeeManager_Vtbl {
    pub const fn new<Identity: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendeeManager_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAttendeeManager_Impl::get_Item(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIAttendeeManager {}
windows_core::imp::define_interface!(IRDPSRAPIAudioStream, IRDPSRAPIAudioStream_Vtbl, 0xe3e30ef9_89c6_4541_ba3b_19336ac6d31c);
windows_core::imp::interface_hierarchy!(IRDPSRAPIAudioStream, windows_core::IUnknown);
impl IRDPSRAPIAudioStream {
    pub unsafe fn Initialize(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppbdata as _, pcbdata as _, ptimestamp as _).ok() }
    }
    pub unsafe fn FreeBuffer(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAudioStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u64) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPSRAPIAudioStream_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<i64>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::Result<()>;
    fn FreeBuffer(&self) -> windows_core::Result<()>;
}
impl IRDPSRAPIAudioStream_Vtbl {
    pub const fn new<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIAudioStream_Impl::Initialize(this) {
                    Ok(ok__) => {
                        pnperiodinhundrednsintervals.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAudioStream_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAudioStream_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAudioStream_Impl::GetBuffer(this, core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&ptimestamp)).into()
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIAudioStream_Impl::FreeBuffer(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIAudioStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPIAudioStream {}
windows_core::imp::define_interface!(IRDPSRAPIClipboardUseEvents, IRDPSRAPIClipboardUseEvents_Vtbl, 0xd559f59a_7a27_4138_8763_247ce5f659a8);
windows_core::imp::interface_hierarchy!(IRDPSRAPIClipboardUseEvents, windows_core::IUnknown);
impl IRDPSRAPIClipboardUseEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnPasteFromClipboard<P1>(&self, clipboardformat: u32, pattendee: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnPasteFromClipboard)(windows_core::Interface::as_raw(self), clipboardformat, pattendee.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIClipboardUseEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnPasteFromClipboard: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnPasteFromClipboard: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIClipboardUseEvents_Impl: windows_core::IUnknownImpl {
    fn OnPasteFromClipboard(&self, clipboardformat: u32, pattendee: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIClipboardUseEvents_Vtbl {
    pub const fn new<Identity: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPasteFromClipboard<Identity: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipboardformat: u32, pattendee: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIClipboardUseEvents_Impl::OnPasteFromClipboard(this, core::mem::transmute_copy(&clipboardformat), core::mem::transmute_copy(&pattendee)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPasteFromClipboard: OnPasteFromClipboard::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIClipboardUseEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRDPSRAPIClipboardUseEvents {}
windows_core::imp::define_interface!(IRDPSRAPIDebug, IRDPSRAPIDebug_Vtbl, 0xaa1e42b5_496d_4ca4_a690_348dcb2ec4ad);
windows_core::imp::interface_hierarchy!(IRDPSRAPIDebug, windows_core::IUnknown);
impl IRDPSRAPIDebug {
    pub unsafe fn SetCLXCmdLine(&self, clxcmdline: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCLXCmdLine)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clxcmdline)).ok() }
    }
    pub unsafe fn CLXCmdLine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CLXCmdLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCLXCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CLXCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPSRAPIDebug_Impl: windows_core::IUnknownImpl {
    fn SetCLXCmdLine(&self, clxcmdline: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CLXCmdLine(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IRDPSRAPIDebug_Vtbl {
    pub const fn new<Identity: IRDPSRAPIDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCLXCmdLine<Identity: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clxcmdline: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIDebug_Impl::SetCLXCmdLine(this, core::mem::transmute(&clxcmdline)).into()
            }
        }
        unsafe extern "system" fn CLXCmdLine<Identity: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclxcmdline: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIDebug_Impl::CLXCmdLine(this) {
                    Ok(ok__) => {
                        pclxcmdline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCLXCmdLine: SetCLXCmdLine::<Identity, OFFSET>,
            CLXCmdLine: CLXCmdLine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPIDebug {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIFrameBuffer, IRDPSRAPIFrameBuffer_Vtbl, 0x3d67e7d2_b27b_448e_81b3_c6110ed8b4be);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIFrameBuffer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIFrameBuffer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIFrameBuffer {
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Bpp(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bpp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameBufferBits)(windows_core::Interface::as_raw(self), x, y, width, heigth, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIFrameBuffer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Bpp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFrameBufferBits: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIFrameBuffer_Impl: super::Com::IDispatch_Impl {
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Bpp(&self) -> windows_core::Result<i32>;
    fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIFrameBuffer_Vtbl {
    pub const fn new<Identity: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Width<Identity: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIFrameBuffer_Impl::Width(this) {
                    Ok(ok__) => {
                        plwidth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Height<Identity: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIFrameBuffer_Impl::Height(this) {
                    Ok(ok__) => {
                        plheight.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Bpp<Identity: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbpp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIFrameBuffer_Impl::Bpp(this) {
                    Ok(ok__) => {
                        plbpp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Identity: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIFrameBuffer_Impl::GetFrameBufferBits(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&heigth)) {
                    Ok(ok__) => {
                        ppbits.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Bpp: Bpp::<Identity, OFFSET>,
            GetFrameBufferBits: GetFrameBufferBits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIFrameBuffer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIFrameBuffer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIInvitation, IRDPSRAPIInvitation_Vtbl, 0x4fac1d43_fc51_45bb_b1b4_2b53aa562fa3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIInvitation {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIInvitation, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitation {
    pub unsafe fn ConnectionString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GroupName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Password(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AttendeeLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttendeeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttendeeLimit(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttendeeLimit)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Revoked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Revoked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRevoked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRevoked)(windows_core::Interface::as_raw(self), newval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttendeeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttendeeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Revoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIInvitation_Impl: super::Com::IDispatch_Impl {
    fn ConnectionString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GroupName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Password(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AttendeeLimit(&self) -> windows_core::Result<i32>;
    fn SetAttendeeLimit(&self, newval: i32) -> windows_core::Result<()>;
    fn Revoked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRevoked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIInvitation_Vtbl {
    pub const fn new<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionString<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitation_Impl::ConnectionString(this) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GroupName<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitation_Impl::GroupName(this) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Password<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitation_Impl::Password(this) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttendeeLimit<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitation_Impl::AttendeeLimit(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIInvitation_Impl::SetAttendeeLimit(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Revoked<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitation_Impl::Revoked(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRevoked<Identity: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIInvitation_Impl::SetRevoked(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, OFFSET>,
            GroupName: GroupName::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
            AttendeeLimit: AttendeeLimit::<Identity, OFFSET>,
            SetAttendeeLimit: SetAttendeeLimit::<Identity, OFFSET>,
            Revoked: Revoked::<Identity, OFFSET>,
            SetRevoked: SetRevoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitation as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIInvitation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIInvitationManager, IRDPSRAPIInvitationManager_Vtbl, 0x4722b049_92c3_4c2d_8a65_f7348f644dcf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIInvitationManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIInvitationManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, item: &super::Variant::VARIANT) -> windows_core::Result<IRDPSRAPIInvitation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(item), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateInvitation(&self, bstrauthstring: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, attendeelimit: i32) -> windows_core::Result<IRDPSRAPIInvitation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInvitation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrauthstring), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(bstrpassword), attendeelimit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitationManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CreateInvitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIInvitationManager_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: &super::Variant::VARIANT) -> windows_core::Result<IRDPSRAPIInvitation>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn CreateInvitation(&self, bstrauthstring: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, attendeelimit: i32) -> windows_core::Result<IRDPSRAPIInvitation>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIInvitationManager_Vtbl {
    pub const fn new<Identity: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitationManager_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: super::Variant::VARIANT, ppinvitation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitationManager_Impl::get_Item(this, core::mem::transmute(&item)) {
                    Ok(ok__) => {
                        ppinvitation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitationManager_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInvitation<Identity: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthstring: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, attendeelimit: i32, ppinvitation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIInvitationManager_Impl::CreateInvitation(this, core::mem::transmute(&bstrauthstring), core::mem::transmute(&bstrgroupname), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&attendeelimit)) {
                    Ok(ok__) => {
                        ppinvitation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            CreateInvitation: CreateInvitation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitationManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIInvitationManager {}
windows_core::imp::define_interface!(IRDPSRAPIPerfCounterLogger, IRDPSRAPIPerfCounterLogger_Vtbl, 0x071c2533_0fa4_4e8f_ae83_9c10b4305ab5);
windows_core::imp::interface_hierarchy!(IRDPSRAPIPerfCounterLogger, windows_core::IUnknown);
impl IRDPSRAPIPerfCounterLogger {
    pub unsafe fn LogValue(&self, lvalue: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LogValue)(windows_core::Interface::as_raw(self), lvalue).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LogValue: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
pub trait IRDPSRAPIPerfCounterLogger_Impl: windows_core::IUnknownImpl {
    fn LogValue(&self, lvalue: i64) -> windows_core::Result<()>;
}
impl IRDPSRAPIPerfCounterLogger_Vtbl {
    pub const fn new<Identity: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LogValue<Identity: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvalue: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIPerfCounterLogger_Impl::LogValue(this, core::mem::transmute_copy(&lvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LogValue: LogValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLogger as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPIPerfCounterLogger {}
windows_core::imp::define_interface!(IRDPSRAPIPerfCounterLoggingManager, IRDPSRAPIPerfCounterLoggingManager_Vtbl, 0x9a512c86_ac6e_4a8e_b1a4_fcef363f6e64);
windows_core::imp::interface_hierarchy!(IRDPSRAPIPerfCounterLoggingManager, windows_core::IUnknown);
impl IRDPSRAPIPerfCounterLoggingManager {
    pub unsafe fn CreateLogger(&self, bstrcountername: &windows_core::BSTR) -> windows_core::Result<IRDPSRAPIPerfCounterLogger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLogger)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcountername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateLogger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPSRAPIPerfCounterLoggingManager_Impl: windows_core::IUnknownImpl {
    fn CreateLogger(&self, bstrcountername: &windows_core::BSTR) -> windows_core::Result<IRDPSRAPIPerfCounterLogger>;
}
impl IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub const fn new<Identity: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLogger<Identity: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcountername: *mut core::ffi::c_void, pplogger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIPerfCounterLoggingManager_Impl::CreateLogger(this, core::mem::transmute(&bstrcountername)) {
                    Ok(ok__) => {
                        pplogger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateLogger: CreateLogger::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLoggingManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPIPerfCounterLoggingManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISessionProperties, IRDPSRAPISessionProperties_Vtbl, 0x339b24f2_9bc0_4f16_9aac_f165433d13d4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISessionProperties {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISessionProperties, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISessionProperties {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Property(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_Property(&self, propertyname: &windows_core::BSTR, newval: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), core::mem::transmute_copy(newval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISessionProperties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Property: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_Property: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISessionProperties_Impl: super::Com::IDispatch_Impl {
    fn get_Property(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn put_Property(&self, propertyname: &windows_core::BSTR, newval: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPISessionProperties_Vtbl {
    pub const fn new<Identity: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Property<Identity: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISessionProperties_Impl::get_Property(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Property<Identity: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, newval: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISessionProperties_Impl::put_Property(this, core::mem::transmute(&propertyname), core::mem::transmute(&newval)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Property: get_Property::<Identity, OFFSET>,
            put_Property: put_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISessionProperties as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPISessionProperties {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISharingSession, IRDPSRAPISharingSession_Vtbl, 0xeeb20886_e470_4cf6_842b_2739c0ec5cfb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISharingSession {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISharingSession, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorDepth)(windows_core::Interface::as_raw(self), colordepth).ok() }
    }
    pub unsafe fn ColorDepth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ColorDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attendees)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invitations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VirtualChannelManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ConnectToClient(&self, bstrconnectionstring: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectToClient)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnectionstring)).ok() }
    }
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesktopSharedRect)(windows_core::Interface::as_raw(self), left, top, right, bottom).ok() }
    }
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesktopSharedRect)(windows_core::Interface::as_raw(self), pleft as _, ptop as _, pright as _, pbottom as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ColorDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attendees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Invitations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VirtualChannelManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectToClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesktopSharedRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetDesktopSharedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISharingSession_Impl: super::Com::IDispatch_Impl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetColorDepth(&self, colordepth: i32) -> windows_core::Result<()>;
    fn ColorDepth(&self) -> windows_core::Result<i32>;
    fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties>;
    fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn ConnectToClient(&self, bstrconnectionstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::Result<()>;
    fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPISharingSession_Vtbl {
    pub const fn new<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn SetColorDepth<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colordepth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::SetColorDepth(this, core::mem::transmute_copy(&colordepth)).into()
            }
        }
        unsafe extern "system" fn ColorDepth<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolordepth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::ColorDepth(this) {
                    Ok(ok__) => {
                        pcolordepth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Attendees<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::Attendees(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invitations<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::Invitations(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::ApplicationFilter(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession_Impl::VirtualChannelManager(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn ConnectToClient<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::ConnectToClient(this, core::mem::transmute(&bstrconnectionstring)).into()
            }
        }
        unsafe extern "system" fn SetDesktopSharedRect<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::SetDesktopSharedRect(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn GetDesktopSharedRect<Identity: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession_Impl::GetDesktopSharedRect(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetColorDepth: SetColorDepth::<Identity, OFFSET>,
            ColorDepth: ColorDepth::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Attendees: Attendees::<Identity, OFFSET>,
            Invitations: Invitations::<Identity, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            ConnectToClient: ConnectToClient::<Identity, OFFSET>,
            SetDesktopSharedRect: SetDesktopSharedRect::<Identity, OFFSET>,
            GetDesktopSharedRect: GetDesktopSharedRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPISharingSession {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISharingSession2, IRDPSRAPISharingSession2_Vtbl, 0xfee4ee57_e3e8_4205_8fb0_8fd1d0675c21);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISharingSession2 {
    type Target = IRDPSRAPISharingSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISharingSession2, windows_core::IUnknown, super::Com::IDispatch, IRDPSRAPISharingSession);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession2 {
    pub unsafe fn ConnectUsingTransportStream<P0>(&self, pstream: P0, bstrgroup: &windows_core::BSTR, bstrauthenticatedattendeename: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectUsingTransportStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), core::mem::transmute_copy(bstrgroup), core::mem::transmute_copy(bstrauthenticatedattendeename)).ok() }
    }
    pub unsafe fn FrameBuffer(&self) -> windows_core::Result<IRDPSRAPIFrameBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FrameBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendControlLevelChangeResponse<P0>(&self, pattendee: P0, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPIAttendee>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendControlLevelChangeResponse)(windows_core::Interface::as_raw(self), pattendee.param().abi(), requestedlevel, reasoncode).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession2_Vtbl {
    pub base__: IRDPSRAPISharingSession_Vtbl,
    pub ConnectUsingTransportStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendControlLevelChangeResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CTRL_LEVEL, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPISharingSession2_Impl: IRDPSRAPISharingSession_Impl {
    fn ConnectUsingTransportStream(&self, pstream: windows_core::Ref<IRDPSRAPITransportStream>, bstrgroup: &windows_core::BSTR, bstrauthenticatedattendeename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FrameBuffer(&self) -> windows_core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(&self, pattendee: windows_core::Ref<IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPISharingSession2_Vtbl {
    pub const fn new<Identity: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectUsingTransportStream<Identity: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, bstrgroup: *mut core::ffi::c_void, bstrauthenticatedattendeename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession2_Impl::ConnectUsingTransportStream(this, core::mem::transmute_copy(&pstream), core::mem::transmute(&bstrgroup), core::mem::transmute(&bstrauthenticatedattendeename)).into()
            }
        }
        unsafe extern "system" fn FrameBuffer<Identity: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPISharingSession2_Impl::FrameBuffer(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Identity: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattendee: *mut core::ffi::c_void, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPISharingSession2_Impl::SendControlLevelChangeResponse(this, core::mem::transmute_copy(&pattendee), core::mem::transmute_copy(&requestedlevel), core::mem::transmute_copy(&reasoncode)).into()
            }
        }
        Self {
            base__: IRDPSRAPISharingSession_Vtbl::new::<Identity, OFFSET>(),
            ConnectUsingTransportStream: ConnectUsingTransportStream::<Identity, OFFSET>,
            FrameBuffer: FrameBuffer::<Identity, OFFSET>,
            SendControlLevelChangeResponse: SendControlLevelChangeResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRDPSRAPISharingSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPISharingSession2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPITcpConnectionInfo, IRDPSRAPITcpConnectionInfo_Vtbl, 0xf74049a4_3d06_4028_8193_0a8c29bc2452);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPITcpConnectionInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPITcpConnectionInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPITcpConnectionInfo {
    pub unsafe fn Protocol(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalIP(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalIP)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PeerPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeerPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PeerIP(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeerIP)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITcpConnectionInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LocalIP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PeerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PeerIP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPITcpConnectionInfo_Impl: super::Com::IDispatch_Impl {
    fn Protocol(&self) -> windows_core::Result<i32>;
    fn LocalPort(&self) -> windows_core::Result<i32>;
    fn LocalIP(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PeerPort(&self) -> windows_core::Result<i32>;
    fn PeerIP(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPITcpConnectionInfo_Vtbl {
    pub const fn new<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Protocol<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprotocol: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITcpConnectionInfo_Impl::Protocol(this) {
                    Ok(ok__) => {
                        plprotocol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalPort<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITcpConnectionInfo_Impl::LocalPort(this) {
                    Ok(ok__) => {
                        plport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalIP<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsrlocalip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITcpConnectionInfo_Impl::LocalIP(this) {
                    Ok(ok__) => {
                        pbsrlocalip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeerPort<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITcpConnectionInfo_Impl::PeerPort(this) {
                    Ok(ok__) => {
                        plport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeerIP<Identity: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITcpConnectionInfo_Impl::PeerIP(this) {
                    Ok(ok__) => {
                        pbstrip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, OFFSET>,
            LocalPort: LocalPort::<Identity, OFFSET>,
            LocalIP: LocalIP::<Identity, OFFSET>,
            PeerPort: PeerPort::<Identity, OFFSET>,
            PeerIP: PeerIP::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITcpConnectionInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPITcpConnectionInfo {}
windows_core::imp::define_interface!(IRDPSRAPITransportStream, IRDPSRAPITransportStream_Vtbl, 0x36cfa065_43bb_4ef7_aed7_9b88a5053036);
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStream, windows_core::IUnknown);
impl IRDPSRAPITransportStream {
    pub unsafe fn AllocBuffer(&self, maxpayload: i32) -> windows_core::Result<IRDPSRAPITransportStreamBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocBuffer)(windows_core::Interface::as_raw(self), maxpayload, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FreeBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
    pub unsafe fn WriteBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
    pub unsafe fn ReadBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReadBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
    pub unsafe fn Open<P0>(&self, pcallbacks: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pcallbacks.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPSRAPITransportStream_Impl: windows_core::IUnknownImpl {
    fn AllocBuffer(&self, maxpayload: i32) -> windows_core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(&self, pbuffer: windows_core::Ref<IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn WriteBuffer(&self, pbuffer: windows_core::Ref<IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn ReadBuffer(&self, pbuffer: windows_core::Ref<IRDPSRAPITransportStreamBuffer>) -> windows_core::Result<()>;
    fn Open(&self, pcallbacks: windows_core::Ref<IRDPSRAPITransportStreamEvents>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IRDPSRAPITransportStream_Vtbl {
    pub const fn new<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllocBuffer<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxpayload: i32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStream_Impl::AllocBuffer(this, core::mem::transmute_copy(&maxpayload)) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStream_Impl::FreeBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn WriteBuffer<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStream_Impl::WriteBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn ReadBuffer<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStream_Impl::ReadBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbacks: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStream_Impl::Open(this, core::mem::transmute_copy(&pcallbacks)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStream_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocBuffer: AllocBuffer::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPITransportStream {}
windows_core::imp::define_interface!(IRDPSRAPITransportStreamBuffer, IRDPSRAPITransportStreamBuffer_Vtbl, 0x81c80290_5085_44b0_b460_f865c39cb4a9);
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStreamBuffer, windows_core::IUnknown);
impl IRDPSRAPITransportStreamBuffer {
    pub unsafe fn Storage(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Storage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StorageSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StorageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PayloadSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PayloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPayloadSize(&self, lval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPayloadSize)(windows_core::Interface::as_raw(self), lval).ok() }
    }
    pub unsafe fn PayloadOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PayloadOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPayloadOffset(&self, lretval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPayloadOffset)(windows_core::Interface::as_raw(self), lretval).ok() }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn Context(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Storage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub StorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PayloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPayloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PayloadOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPayloadOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPSRAPITransportStreamBuffer_Impl: windows_core::IUnknownImpl {
    fn Storage(&self) -> windows_core::Result<*mut u8>;
    fn StorageSize(&self) -> windows_core::Result<i32>;
    fn PayloadSize(&self) -> windows_core::Result<i32>;
    fn SetPayloadSize(&self, lval: i32) -> windows_core::Result<()>;
    fn PayloadOffset(&self) -> windows_core::Result<i32>;
    fn SetPayloadOffset(&self, lretval: i32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, lflags: i32) -> windows_core::Result<()>;
    fn Context(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetContext(&self, pcontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IRDPSRAPITransportStreamBuffer_Vtbl {
    pub const fn new<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Storage<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbstorage: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::Storage(this) {
                    Ok(ok__) => {
                        ppbstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StorageSize<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxstore: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::StorageSize(this) {
                    Ok(ok__) => {
                        plmaxstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PayloadSize<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::PayloadSize(this) {
                    Ok(ok__) => {
                        plretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPayloadSize<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamBuffer_Impl::SetPayloadSize(this, core::mem::transmute_copy(&lval)).into()
            }
        }
        unsafe extern "system" fn PayloadOffset<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::PayloadOffset(this) {
                    Ok(ok__) => {
                        plretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lretval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamBuffer_Impl::SetPayloadOffset(this, core::mem::transmute_copy(&lretval)).into()
            }
        }
        unsafe extern "system" fn Flags<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::Flags(this) {
                    Ok(ok__) => {
                        plflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamBuffer_Impl::SetFlags(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Context<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPITransportStreamBuffer_Impl::Context(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContext<Identity: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamBuffer_Impl::SetContext(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Storage: Storage::<Identity, OFFSET>,
            StorageSize: StorageSize::<Identity, OFFSET>,
            PayloadSize: PayloadSize::<Identity, OFFSET>,
            SetPayloadSize: SetPayloadSize::<Identity, OFFSET>,
            PayloadOffset: PayloadOffset::<Identity, OFFSET>,
            SetPayloadOffset: SetPayloadOffset::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPITransportStreamBuffer {}
windows_core::imp::define_interface!(IRDPSRAPITransportStreamEvents, IRDPSRAPITransportStreamEvents_Vtbl, 0xea81c254_f5af_4e40_982e_3e63bb595276);
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStreamEvents, windows_core::IUnknown);
impl IRDPSRAPITransportStreamEvents {
    pub unsafe fn OnWriteCompleted<P0>(&self, pbuffer: P0)
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnWriteCompleted)(windows_core::Interface::as_raw(self), pbuffer.param().abi()) }
    }
    pub unsafe fn OnReadCompleted<P0>(&self, pbuffer: P0)
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadCompleted)(windows_core::Interface::as_raw(self), pbuffer.param().abi()) }
    }
    pub unsafe fn OnStreamClosed(&self, hrreason: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).OnStreamClosed)(windows_core::Interface::as_raw(self), hrreason) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnWriteCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnReadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnStreamClosed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IRDPSRAPITransportStreamEvents_Impl: windows_core::IUnknownImpl {
    fn OnWriteCompleted(&self, pbuffer: windows_core::Ref<IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(&self, pbuffer: windows_core::Ref<IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(&self, hrreason: windows_core::HRESULT);
}
impl IRDPSRAPITransportStreamEvents_Vtbl {
    pub const fn new<Identity: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnWriteCompleted<Identity: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamEvents_Impl::OnWriteCompleted(this, core::mem::transmute_copy(&pbuffer))
            }
        }
        unsafe extern "system" fn OnReadCompleted<Identity: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamEvents_Impl::OnReadCompleted(this, core::mem::transmute_copy(&pbuffer))
            }
        }
        unsafe extern "system" fn OnStreamClosed<Identity: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPITransportStreamEvents_Impl::OnStreamClosed(this, core::mem::transmute_copy(&hrreason))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWriteCompleted: OnWriteCompleted::<Identity, OFFSET>,
            OnReadCompleted: OnReadCompleted::<Identity, OFFSET>,
            OnStreamClosed: OnStreamClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPSRAPITransportStreamEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIViewer, IRDPSRAPIViewer_Vtbl, 0xc6bfcd38_8ce9_404d_8ae8_f31d00c65cb5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIViewer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIViewer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIViewer {
    pub unsafe fn Connect(&self, bstrconnectionstring: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnectionstring), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrpassword)).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attendees)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invitations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VirtualChannelManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSmartSizing(&self, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSmartSizing)(windows_core::Interface::as_raw(self), vbsmartsizing).ok() }
    }
    pub unsafe fn SmartSizing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmartSizing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestControl)(windows_core::Interface::as_raw(self), ctrllevel).ok() }
    }
    pub unsafe fn SetDisconnectedText(&self, bstrdisconnectedtext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisconnectedText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdisconnectedtext)).ok() }
    }
    pub unsafe fn DisconnectedText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisconnectedText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestColorDepthChange(&self, bpp: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestColorDepthChange)(windows_core::Interface::as_raw(self), bpp).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StartReverseConnectListener(&self, bstrconnectionstring: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartReverseConnectListener)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnectionstring), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIViewer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attendees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Invitations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VirtualChannelManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSmartSizing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SmartSizing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RequestControl: unsafe extern "system" fn(*mut core::ffi::c_void, CTRL_LEVEL) -> windows_core::HRESULT,
    pub SetDisconnectedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestColorDepthChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartReverseConnectListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIViewer_Impl: super::Com::IDispatch_Impl {
    fn Connect(&self, bstrconnectionstring: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager>;
    fn SetSmartSizing(&self, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SmartSizing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> windows_core::Result<()>;
    fn SetDisconnectedText(&self, bstrdisconnectedtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisconnectedText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestColorDepthChange(&self, bpp: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties>;
    fn StartReverseConnectListener(&self, bstrconnectionstring: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIViewer_Vtbl {
    pub const fn new<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::Connect(this, core::mem::transmute(&bstrconnectionstring), core::mem::transmute(&bstrname), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Attendees<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::Attendees(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invitations<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::Invitations(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::ApplicationFilter(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::VirtualChannelManager(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmartSizing<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbsmartsizing: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::SetSmartSizing(this, core::mem::transmute_copy(&vbsmartsizing)).into()
            }
        }
        unsafe extern "system" fn SmartSizing<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbsmartsizing: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::SmartSizing(this) {
                    Ok(ok__) => {
                        pvbsmartsizing.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestControl<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::RequestControl(this, core::mem::transmute_copy(&ctrllevel)).into()
            }
        }
        unsafe extern "system" fn SetDisconnectedText<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdisconnectedtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::SetDisconnectedText(this, core::mem::transmute(&bstrdisconnectedtext)).into()
            }
        }
        unsafe extern "system" fn DisconnectedText<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisconnectedtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::DisconnectedText(this) {
                    Ok(ok__) => {
                        pbstrdisconnectedtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpp: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIViewer_Impl::RequestColorDepthChange(this, core::mem::transmute_copy(&bpp)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Identity: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectionstring: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, pbstrreverseconnectstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIViewer_Impl::StartReverseConnectListener(this, core::mem::transmute(&bstrconnectionstring), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)) {
                    Ok(ok__) => {
                        pbstrreverseconnectstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Attendees: Attendees::<Identity, OFFSET>,
            Invitations: Invitations::<Identity, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, OFFSET>,
            SetSmartSizing: SetSmartSizing::<Identity, OFFSET>,
            SmartSizing: SmartSizing::<Identity, OFFSET>,
            RequestControl: RequestControl::<Identity, OFFSET>,
            SetDisconnectedText: SetDisconnectedText::<Identity, OFFSET>,
            DisconnectedText: DisconnectedText::<Identity, OFFSET>,
            RequestColorDepthChange: RequestColorDepthChange::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            StartReverseConnectListener: StartReverseConnectListener::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIViewer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIViewer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIVirtualChannel, IRDPSRAPIVirtualChannel_Vtbl, 0x05e12f95_28b3_4c9a_8780_d0248574a1e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIVirtualChannel {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIVirtualChannel, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannel {
    pub unsafe fn SendData(&self, bstrdata: &windows_core::BSTR, lattendeeid: i32, channelsendflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdata), lattendeeid, channelsendflags).ok() }
    }
    pub unsafe fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAccess)(windows_core::Interface::as_raw(self), lattendeeid, accesstype).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<CHANNEL_PRIORITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SendData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub SetAccess: unsafe extern "system" fn(*mut core::ffi::c_void, i32, CHANNEL_ACCESS_ENUM) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CHANNEL_PRIORITY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIVirtualChannel_Impl: super::Com::IDispatch_Impl {
    fn SendData(&self, bstrdata: &windows_core::BSTR, lattendeeid: i32, channelsendflags: u32) -> windows_core::Result<()>;
    fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn Priority(&self) -> windows_core::Result<CHANNEL_PRIORITY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIVirtualChannel_Vtbl {
    pub const fn new<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendData<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, lattendeeid: i32, channelsendflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIVirtualChannel_Impl::SendData(this, core::mem::transmute(&bstrdata), core::mem::transmute_copy(&lattendeeid), core::mem::transmute_copy(&channelsendflags)).into()
            }
        }
        unsafe extern "system" fn SetAccess<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIVirtualChannel_Impl::SetAccess(this, core::mem::transmute_copy(&lattendeeid), core::mem::transmute_copy(&accesstype)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannel_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Flags<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannel_Impl::Flags(this) {
                    Ok(ok__) => {
                        plflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Priority<Identity: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannel_Impl::Priority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SendData: SendData::<Identity, OFFSET>,
            SetAccess: SetAccess::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannel as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIVirtualChannel {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIVirtualChannelManager, IRDPSRAPIVirtualChannelManager_Vtbl, 0x0d11c661_5d0d_4ee4_89df_2166ae1fdfed);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIVirtualChannelManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIVirtualChannelManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, item: &super::Variant::VARIANT) -> windows_core::Result<IRDPSRAPIVirtualChannel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(item), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateVirtualChannel(&self, bstrchannelname: &windows_core::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> windows_core::Result<IRDPSRAPIVirtualChannel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrchannelname), priority, channelflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannelManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CHANNEL_PRIORITY, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIVirtualChannelManager_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: &super::Variant::VARIANT) -> windows_core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(&self, bstrchannelname: &windows_core::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> windows_core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIVirtualChannelManager_Vtbl {
    pub const fn new<Identity: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannelManager_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: super::Variant::VARIANT, pchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannelManager_Impl::get_Item(this, core::mem::transmute(&item)) {
                    Ok(ok__) => {
                        pchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrchannelname: *mut core::ffi::c_void, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIVirtualChannelManager_Impl::CreateVirtualChannel(this, core::mem::transmute(&bstrchannelname), core::mem::transmute_copy(&priority), core::mem::transmute_copy(&channelflags)) {
                    Ok(ok__) => {
                        ppchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannelManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIVirtualChannelManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIWindow, IRDPSRAPIWindow_Vtbl, 0xbeafe0f9_c77b_4933_ba9f_a24cddcc27cf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIWindow {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIWindow, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindow {
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<IRDPSRAPIApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShared)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Show(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShared: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIWindow_Impl: super::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<i32>;
    fn Application(&self) -> windows_core::Result<IRDPSRAPIApplication>;
    fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShared(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Show(&self) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIWindow_Vtbl {
    pub const fn new<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindow_Impl::Id(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Application<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindow_Impl::Application(this) {
                    Ok(ok__) => {
                        papplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shared<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindow_Impl::Shared(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShared<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIWindow_Impl::SetShared(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindow_Impl::Name(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Show<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPSRAPIWindow_Impl::Show(this).into()
            }
        }
        unsafe extern "system" fn Flags<Identity: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindow_Impl::Flags(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Shared: Shared::<Identity, OFFSET>,
            SetShared: SetShared::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIWindow as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIWindow {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIWindowList, IRDPSRAPIWindowList_Vtbl, 0x8a05ce44_715a_4116_a189_a118f30a07bd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIWindowList {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIWindowList, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowList {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIWindow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindowList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRDPSRAPIWindowList_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIWindow>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRDPSRAPIWindowList_Vtbl {
    pub const fn new<Identity: IRDPSRAPIWindowList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindowList_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, pwindow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRDPSRAPIWindowList_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                    Ok(ok__) => {
                        pwindow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), _NewEnum: _NewEnum::<Identity, OFFSET>, get_Item: get_Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPSRAPIWindowList as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRDPSRAPIWindowList {}
windows_core::imp::define_interface!(IRDPViewerInputSink, IRDPViewerInputSink_Vtbl, 0xbb590853_a6c5_4a7b_8dd4_76b69eea12d5);
windows_core::imp::interface_hierarchy!(IRDPViewerInputSink, windows_core::IUnknown);
impl IRDPViewerInputSink {
    pub unsafe fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMouseButtonEvent)(windows_core::Interface::as_raw(self), buttontype, vbbuttondown, xpos, ypos).ok() }
    }
    pub unsafe fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMouseMoveEvent)(windows_core::Interface::as_raw(self), xpos, ypos).ok() }
    }
    pub unsafe fn SendMouseWheelEvent(&self, wheelrotation: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMouseWheelEvent)(windows_core::Interface::as_raw(self), wheelrotation).ok() }
    }
    pub unsafe fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendKeyboardEvent)(windows_core::Interface::as_raw(self), codetype, keycode, vbkeyup, vbrepeat, vbextended).ok() }
    }
    pub unsafe fn SendSyncEvent(&self, syncflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendSyncEvent)(windows_core::Interface::as_raw(self), syncflags).ok() }
    }
    pub unsafe fn BeginTouchFrame(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginTouchFrame)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTouchInput)(windows_core::Interface::as_raw(self), contactid, event, x, y).ok() }
    }
    pub unsafe fn EndTouchFrame(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndTouchFrame)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPViewerInputSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendMouseButtonEvent: unsafe extern "system" fn(*mut core::ffi::c_void, RDPSRAPI_MOUSE_BUTTON_TYPE, super::super::Foundation::VARIANT_BOOL, u32, u32) -> windows_core::HRESULT,
    pub SendMouseMoveEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SendMouseWheelEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SendKeyboardEvent: unsafe extern "system" fn(*mut core::ffi::c_void, RDPSRAPI_KBD_CODE_TYPE, u16, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SendSyncEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BeginTouchFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTouchInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32, i32) -> windows_core::HRESULT,
    pub EndTouchFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRDPViewerInputSink_Impl: windows_core::IUnknownImpl {
    fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> windows_core::Result<()>;
    fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> windows_core::Result<()>;
    fn SendMouseWheelEvent(&self, wheelrotation: u16) -> windows_core::Result<()>;
    fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SendSyncEvent(&self, syncflags: u32) -> windows_core::Result<()>;
    fn BeginTouchFrame(&self) -> windows_core::Result<()>;
    fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::Result<()>;
    fn EndTouchFrame(&self) -> windows_core::Result<()>;
}
impl IRDPViewerInputSink_Vtbl {
    pub const fn new<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendMouseButtonEvent<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: super::super::Foundation::VARIANT_BOOL, xpos: u32, ypos: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::SendMouseButtonEvent(this, core::mem::transmute_copy(&buttontype), core::mem::transmute_copy(&vbbuttondown), core::mem::transmute_copy(&xpos), core::mem::transmute_copy(&ypos)).into()
            }
        }
        unsafe extern "system" fn SendMouseMoveEvent<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpos: u32, ypos: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::SendMouseMoveEvent(this, core::mem::transmute_copy(&xpos), core::mem::transmute_copy(&ypos)).into()
            }
        }
        unsafe extern "system" fn SendMouseWheelEvent<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wheelrotation: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::SendMouseWheelEvent(this, core::mem::transmute_copy(&wheelrotation)).into()
            }
        }
        unsafe extern "system" fn SendKeyboardEvent<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: super::super::Foundation::VARIANT_BOOL, vbrepeat: super::super::Foundation::VARIANT_BOOL, vbextended: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::SendKeyboardEvent(this, core::mem::transmute_copy(&codetype), core::mem::transmute_copy(&keycode), core::mem::transmute_copy(&vbkeyup), core::mem::transmute_copy(&vbrepeat), core::mem::transmute_copy(&vbextended)).into()
            }
        }
        unsafe extern "system" fn SendSyncEvent<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::SendSyncEvent(this, core::mem::transmute_copy(&syncflags)).into()
            }
        }
        unsafe extern "system" fn BeginTouchFrame<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::BeginTouchFrame(this).into()
            }
        }
        unsafe extern "system" fn AddTouchInput<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::AddTouchInput(this, core::mem::transmute_copy(&contactid), core::mem::transmute_copy(&event), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn EndTouchFrame<Identity: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRDPViewerInputSink_Impl::EndTouchFrame(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMouseButtonEvent: SendMouseButtonEvent::<Identity, OFFSET>,
            SendMouseMoveEvent: SendMouseMoveEvent::<Identity, OFFSET>,
            SendMouseWheelEvent: SendMouseWheelEvent::<Identity, OFFSET>,
            SendKeyboardEvent: SendKeyboardEvent::<Identity, OFFSET>,
            SendSyncEvent: SendSyncEvent::<Identity, OFFSET>,
            BeginTouchFrame: BeginTouchFrame::<Identity, OFFSET>,
            AddTouchInput: AddTouchInput::<Identity, OFFSET>,
            EndTouchFrame: EndTouchFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRDPViewerInputSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRDPViewerInputSink {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPENCOMAPI_CONSTANTS(pub i32);
pub const RDPSRAPIApplication: windows_core::GUID = windows_core::GUID::from_u128(0xc116a484_4b25_4b9f_8a54_b934b06e57fa);
pub const RDPSRAPIApplicationFilter: windows_core::GUID = windows_core::GUID::from_u128(0xe35ace89_c7e8_427e_a4f9_b9da072826bd);
pub const RDPSRAPIApplicationList: windows_core::GUID = windows_core::GUID::from_u128(0x9e31c815_7433_4876_97fb_ed59fe2baa22);
pub const RDPSRAPIAttendee: windows_core::GUID = windows_core::GUID::from_u128(0x74f93bb5_755f_488e_8a29_2390108aef55);
pub const RDPSRAPIAttendeeDisconnectInfo: windows_core::GUID = windows_core::GUID::from_u128(0xb47d7250_5bdb_405d_b487_caad9c56f4f8);
pub const RDPSRAPIAttendeeManager: windows_core::GUID = windows_core::GUID::from_u128(0xd7b13a01_f7d4_42a6_8595_12fc8c24e851);
pub const RDPSRAPIFrameBuffer: windows_core::GUID = windows_core::GUID::from_u128(0xa4f66bcc_538e_4101_951d_30847adb5101);
pub const RDPSRAPIInvitation: windows_core::GUID = windows_core::GUID::from_u128(0x49174dc6_0731_4b5e_8ee1_83a63d3868fa);
pub const RDPSRAPIInvitationManager: windows_core::GUID = windows_core::GUID::from_u128(0x53d9c9db_75ab_4271_948a_4c4eb36a8f2b);
pub const RDPSRAPISessionProperties: windows_core::GUID = windows_core::GUID::from_u128(0xdd7594ff_ea2a_4c06_8fdf_132de48b6510);
pub const RDPSRAPITcpConnectionInfo: windows_core::GUID = windows_core::GUID::from_u128(0xbe49db3f_ebb6_4278_8ce0_d5455833eaee);
pub const RDPSRAPIWindow: windows_core::GUID = windows_core::GUID::from_u128(0x03cf46db_ce45_4d36_86ed_ed28b74398bf);
pub const RDPSRAPIWindowList: windows_core::GUID = windows_core::GUID::from_u128(0x9c21e2b8_5dd4_42cc_81ba_1c099852e6fa);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPSRAPI_APP_FLAGS(pub i32);
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPSRAPI_KBD_CODE_TYPE(pub i32);
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(pub i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(4i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(8i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(2i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(0i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(pub i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(3i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(4i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDPSRAPI_WND_FLAGS(pub i32);
pub const RDPSession: windows_core::GUID = windows_core::GUID::from_u128(0x9b78f0e6_3e05_4a5b_b2e8_e743a8956b65);
pub const RDPTransportStreamBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x8d4a1c69_f17f_4549_a699_761c6e6b5c0a);
pub const RDPTransportStreamEvents: windows_core::GUID = windows_core::GUID::from_u128(0x31e3ab20_5350_483f_9dc6_6748665efdeb);
pub const RDPViewer: windows_core::GUID = windows_core::GUID::from_u128(0x32be5ed2_5c86_480f_a914_0ff8885a1b3f);
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = RDPSRAPI_WND_FLAGS(1i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_IRDPSessionEvents, _IRDPSessionEvents_Vtbl, 0x98a97042_6698_40e9_8efd_b3200990004b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _IRDPSessionEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_IRDPSessionEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _IRDPSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _IRDPSessionEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _IRDPSessionEvents_Vtbl {
    pub const fn new<Identity: _IRDPSessionEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IRDPSessionEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _IRDPSessionEvents {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
