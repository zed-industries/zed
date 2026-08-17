//! Console Control Protocol
//!
//! The console-control protocols allows modifying the behavior of the default
//! console device. It is supported by TianoCore and widely adopted.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xf42f7782,
    0x012e,
    0x4c12,
    0x99,
    0x56,
    &[0x49, 0xf9, 0x43, 0x04, 0xf7, 0x21],
);

pub type ScreenMode = u32;

pub const SCREEN_TEXT: ScreenMode = 0x00000000;
pub const SCREEN_GRAPHICS: ScreenMode = 0x00000001;
pub const SCREEN_MAX_VALUE: ScreenMode = 0x00000002;

#[repr(C)]
pub struct Protocol {
    pub get_mode: eficall! {fn(
        *mut Protocol,
        *mut ScreenMode,
        *mut crate::base::Boolean,
        *mut crate::base::Boolean,
    ) -> crate::base::Status},
    pub set_mode: eficall! {fn(
        *mut Protocol,
        ScreenMode,
    ) -> crate::base::Status},
    pub lock_std_in: eficall! {fn(
        *mut Protocol,
        *mut crate::base::Char16,
    ) -> crate::base::Status},
}
