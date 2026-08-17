//! Absolute Pointer Protocol
//!
//! Provides a simple method for accessing absolute pointer devices. This
//! includes devices such as touch screens and digitizers. The Absolute Pointer
//! Protocol allows information about a pointer device to be retrieved. The
//! protocol is attached to the device handle of an absolute pointer device,
//! and can be used for input from the user in the preboot environment.
//!
//! Supported devices may return 1, 2, or 3 axis of information. The Z axis may
//! optionally be used to return pressure data measurements derived from user
//! pen force.
//!
//! All supported devices must support a touch-active status. Supported devices
//! may optionally support a second input button, for example a pen
//! side-button.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x8d59d32b,
    0xc655,
    0x4ae9,
    0x9b,
    0x15,
    &[0xf2, 0x59, 0x04, 0x99, 0x2a, 0x43],
);

pub const SUPPORTS_ALT_ACTIVE: u32 = 0x00000001;
pub const SUPPORTS_PRESSURE_AS_Z: u32 = 0x00000002;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Mode {
    pub absolute_min_x: u64,
    pub absolute_min_y: u64,
    pub absolute_min_z: u64,
    pub absolute_max_x: u64,
    pub absolute_max_y: u64,
    pub absolute_max_z: u64,
    pub attributes: u32,
}

pub const TOUCH_ACTIVE: u32 = 0x00000001;
pub const ALT_ACTIVE: u32 = 0x00000002;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct State {
    pub current_x: u64,
    pub current_y: u64,
    pub current_z: u64,
    pub active_buttons: u32,
}

pub type Reset = eficall! {fn(
    this: *mut Protocol,
    extended_verification: bool,
) -> crate::base::Status};

pub type GetState = eficall! {fn(
    this: *mut Protocol,
    state: *mut State,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub reset: Reset,
    pub get_state: GetState,
    pub wait_for_input: crate::efi::Event,
    pub mode: *mut Mode,
}
