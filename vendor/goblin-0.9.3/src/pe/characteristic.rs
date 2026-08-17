//! Constants for flags that indicate attributes of the object or image file. These flags are used in the
//! [`goblin::pe::header::CoffHeader::characteristics`](crate::pe::header::CoffHeader::characteristics) field.

/*
type characteristic =
    | IMAGE_FILE_RELOCS_STRIPPED
    | IMAGE_FILE_EXECUTABLE_IMAGE
    | IMAGE_FILE_LINE_NUMS_STRIPPED
    | IMAGE_FILE_LOCAL_SYMS_STRIPPED
    | IMAGE_FILE_AGGRESSIVE_WS_TRIM
    | IMAGE_FILE_LARGE_ADDRESS_AWARE
    | RESERVED
    | IMAGE_FILE_BYTES_REVERSED_LO
    | IMAGE_FILE_32BIT_MACHINE
    | IMAGE_FILE_DEBUG_STRIPPED
    | IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP
    | IMAGE_FILE_NET_RUN_FROM_SWAP
    | IMAGE_FILE_SYSTEM
    | IMAGE_FILE_DLL
    | IMAGE_FILE_UP_SYSTEM_ONLY
    | IMAGE_FILE_BYTES_REVERSED_HI
    | UNKNOWN of int

let get_characteristic =
  function
  | 0x0001 -> IMAGE_FILE_RELOCS_STRIPPED
  | 0x0002 -> IMAGE_FILE_EXECUTABLE_IMAGE
  | 0x0004 -> IMAGE_FILE_LINE_NUMS_STRIPPED
  | 0x0008 -> IMAGE_FILE_LOCAL_SYMS_STRIPPED
  | 0x0010 -> IMAGE_FILE_AGGRESSIVE_WS_TRIM
  | 0x0020 -> IMAGE_FILE_LARGE_ADDRESS_AWARE
  | 0x0040 -> RESERVED
  | 0x0080 -> IMAGE_FILE_BYTES_REVERSED_LO
  | 0x0100 -> IMAGE_FILE_32BIT_MACHINE
  | 0x0200 -> IMAGE_FILE_DEBUG_STRIPPED
  | 0x0400 -> IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP
  | 0x0800 -> IMAGE_FILE_NET_RUN_FROM_SWAP
  | 0x1000 -> IMAGE_FILE_SYSTEM
  | 0x2000 -> IMAGE_FILE_DLL
  | 0x4000 -> IMAGE_FILE_UP_SYSTEM_ONLY
  | 0x8000 -> IMAGE_FILE_BYTES_REVERSED_HI
  | x -> UNKNOWN x

let characteristic_to_string =
  function
  | IMAGE_FILE_RELOCS_STRIPPED -> "IMAGE_FILE_RELOCS_STRIPPED"
  | IMAGE_FILE_EXECUTABLE_IMAGE -> "IMAGE_FILE_EXECUTABLE_IMAGE"
  | IMAGE_FILE_LINE_NUMS_STRIPPED -> "IMAGE_FILE_LINE_NUMS_STRIPPED"
  | IMAGE_FILE_LOCAL_SYMS_STRIPPED -> "IMAGE_FILE_LOCAL_SYMS_STRIPPED"
  | IMAGE_FILE_AGGRESSIVE_WS_TRIM -> "IMAGE_FILE_AGGRESSIVE_WS_TRIM"
  | IMAGE_FILE_LARGE_ADDRESS_AWARE -> "IMAGE_FILE_LARGE_ADDRESS_AWARE"
  | RESERVED -> "RESERVED"
  | IMAGE_FILE_BYTES_REVERSED_LO -> "IMAGE_FILE_BYTES_REVERSED_LO"
  | IMAGE_FILE_32BIT_MACHINE -> "IMAGE_FILE_32BIT_MACHINE"
  | IMAGE_FILE_DEBUG_STRIPPED -> "IMAGE_FILE_DEBUG_STRIPPED"
  | IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP -> "IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP"
  | IMAGE_FILE_NET_RUN_FROM_SWAP -> "IMAGE_FILE_NET_RUN_FROM_SWAP"
  | IMAGE_FILE_SYSTEM -> "IMAGE_FILE_SYSTEM"
  | IMAGE_FILE_DLL -> "IMAGE_FILE_DLL"
  | IMAGE_FILE_UP_SYSTEM_ONLY -> "IMAGE_FILE_UP_SYSTEM_ONLY"
  | IMAGE_FILE_BYTES_REVERSED_HI -> "IMAGE_FILE_BYTES_REVERSED_HI"
  | UNKNOWN x -> Printf.sprintf "UNKNOWN_CHARACTERISTIC 0x%x" x

let is_dll characteristics =
  let characteristic = characteristic_to_int IMAGE_FILE_DLL in
  characteristics land characteristic = characteristic

let has characteristic characteristics =
  let characteristic = characteristic_to_int characteristic in
  characteristics land characteristic = characteristic

(* TODO: this is a mad hack *)
let show_type characteristics =
  if (has IMAGE_FILE_DLL characteristics) then "DLL"
  else if (has IMAGE_FILE_EXECUTABLE_IMAGE characteristics) then "EXE"
  else "MANY"                   (* print all *)
 */

/// Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not
/// contain base relocations and must therefore be loaded at its preferred base address. If the base address
/// is not available, the loader reports an error. The default behavior of the linker is to strip base relocations
/// from executable (EXE) files.
pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;

/// Image only. This indicates that the image file is valid and can be run.
/// If this flag is not set, it indicates a linker error.
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;

/// COFF line numbers have been removed. This flag is deprecated and should be zero.
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004;

/// COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008;

/// Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010;

/// Application can handle > 2-GB addresses.
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;

/// This flag is reserved for future use.
pub const RESERVED: u16 = 0x0040;

/// Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory.
/// This flag is deprecated and should be zero.
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080;

/// Machine is based on a 32-bit-word architecture.
pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;

/// Debugging information is removed from the image file.
pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200;

/// If the image is on removable media, fully load it and copy it to the swap file.
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;

/// If the image is on network media, fully load it and copy it to the swap file.
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800;

/// The image file is a system file, not a user program.
pub const IMAGE_FILE_SYSTEM: u16 = 0x1000;

/// The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
pub const IMAGE_FILE_DLL: u16 = 0x2000;

/// The file should be run only on a uniprocessor machine.
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000;

/// Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000;

/// Checks whether the characteristics value indicates that the file is a DLL (dynamically-linked library).
pub fn is_dll(characteristics: u16) -> bool {
    characteristics & IMAGE_FILE_DLL == IMAGE_FILE_DLL
}

/// Checks whether the characteristics value indicates that the file is an executable.
pub fn is_exe(characteristics: u16) -> bool {
    characteristics & IMAGE_FILE_EXECUTABLE_IMAGE == IMAGE_FILE_EXECUTABLE_IMAGE
}
