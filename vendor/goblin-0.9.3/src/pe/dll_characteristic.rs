//! Constants for characteristics of image files. These constants are used in the
//! [`goblin::pe::optional_header::WindowsFields::dll_characteristics`](crate::pe::optional_header::WindowsFields::dll_characteristics)
//! field.
//!
//! The values 0x0001, 0x0002, 0x0004, 0x0008 are reserved for future use and must be zero.

/// Image can handle a high entropy 64-bit virtual address space.
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 0x0020;

/// DLL can be relocated at load time.
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;

/// Code Integrity checks are enforced.
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 0x0080;

/// Image is NX compatible.
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;

/// Isolation aware, but do not isolate the image.
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 0x0200;

/// Does not use structured exception (SE) handling. No SE handler may be called in this image.
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 0x0400;

/// Do not bind the image.
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 0x0800;

/// Image must execute in an AppContainer.
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 0x1000;

/// A WDM driver.
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 0x2000;

/// Image supports Control Flow Guard.
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 0x4000;

/// Terminal Server aware.
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;
