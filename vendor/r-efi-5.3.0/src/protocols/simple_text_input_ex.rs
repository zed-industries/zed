//! Extended Simple Text Input Protocol
//!
//! The simple-text-input-ex protocol extends the simple-text-input protocol by allowing more
//! details reporting about modifiers, etc.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdd9e7534,
    0x7762,
    0x4698,
    0x8c,
    0x14,
    &[0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
);

pub const SHIFT_STATE_VALID: u32 = 0x80000000u32;
pub const RIGHT_SHIFT_PRESSED: u32 = 0x00000001u32;
pub const LEFT_SHIFT_PRESSED: u32 = 0x00000002u32;
pub const RIGHT_CONTROL_PRESSED: u32 = 0x00000004u32;
pub const LEFT_CONTROL_PRESSED: u32 = 0x00000008u32;
pub const RIGHT_ALT_PRESSED: u32 = 0x00000010u32;
pub const LEFT_ALT_PRESSED: u32 = 0x00000020u32;
pub const RIGHT_LOGO_PRESSED: u32 = 0x00000040u32;
pub const LEFT_LOGO_PRESSED: u32 = 0x00000080u32;
pub const MENU_KEY_PRESSED: u32 = 0x00000100u32;
pub const SYS_REQ_PRESSED: u32 = 0x00000200u32;

pub const TOGGLE_STATE_VALID: u8 = 0x80u8;
pub const KEY_STATE_EXPOSED: u8 = 0x40u8;
pub const SCROLL_LOCK_ACTIVE: u8 = 0x01u8;
pub const NUM_LOCK_ACTIVE: u8 = 0x02u8;
pub const CAPS_LOCK_ACTIVE: u8 = 0x04u8;

pub type KeyToggleState = u8;
pub type KeyNotifyFunction = eficall! {fn(*mut KeyData) -> crate::base::Status};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyState {
    pub key_shift_state: u32,
    pub key_toggle_state: KeyToggleState,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyData {
    pub key: crate::protocols::simple_text_input::InputKey,
    pub key_state: KeyState,
}

pub type ProtocolReset = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
) -> crate::base::Status};

pub type ProtocolReadKeyStrokeEx = eficall! {fn(
    *mut Protocol,
    *mut KeyData,
) -> crate::base::Status};

pub type ProtocolSetState = eficall! {fn(
    *mut Protocol,
    *mut KeyToggleState,
) -> crate::base::Status};

pub type ProtocolRegisterKeyNotify = eficall! {fn(
    *mut Protocol,
    *mut KeyData,
    KeyNotifyFunction,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolUnregisterKeyNotify = eficall! {fn(
    *mut Protocol,
    *mut core::ffi::c_void,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub reset: ProtocolReset,
    pub read_key_stroke_ex: ProtocolReadKeyStrokeEx,
    pub wait_for_key_ex: crate::base::Event,
    pub set_state: ProtocolSetState,
    pub register_key_notify: ProtocolRegisterKeyNotify,
    pub unregister_key_notify: ProtocolUnregisterKeyNotify,
}
