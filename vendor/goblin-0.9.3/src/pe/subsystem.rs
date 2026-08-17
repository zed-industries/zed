//! Constants for subsystems required to run image files. These constants are used in the
//! [`goblin::pe::optional_header::WindowsFields::subsystem`](crate::pe::optional_header::WindowsFields::subsystem)
//! field.

/// An unknown subsystem.
pub const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0;

/// Device drivers and native Windows processes.
pub const IMAGE_SUBSYSTEM_NATIVE: u16 = 1;

/// The Windows graphical user interface (GUI) subsystem.
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;

/// The Windows character subsystem.
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3;

/// The OS/2 character subsystem.
pub const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5;

/// The Posix character subsystem.
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7;

/// Native Win9x driver.
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8;

/// Windows CE.
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9;

/// An Extensible Firmware Interface (EFI) application.
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10;

/// An EFI driver with boot services.
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11;

/// An EFI driver with run-time services.
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12;

/// An EFI ROM image.
pub const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13;

/// XBOX.
pub const IMAGE_SUBSYSTEM_XBOX: u16 = 14;

/// Windows boot application.
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;
