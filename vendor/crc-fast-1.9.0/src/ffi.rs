// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! FFI bindings for the Rust library
//!
//! This module provides a C-compatible interface for the Rust library, allowing
//! C programs to use the library's functionality.

#![cfg(all(
    feature = "ffi",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]

use crate::CrcAlgorithm;
use crate::CrcParams;
use crate::{get_calculator_target, Digest};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;
use std::sync::{Mutex, OnceLock};

static STRING_CACHE: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();

/// Error codes for FFI operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrcFastError {
    /// Operation completed successfully
    Success = 0,
    /// Lock was poisoned (thread panicked while holding lock)
    LockPoisoned = 1,
    /// Null pointer was passed where non-null required
    NullPointer = 2,
    /// Invalid key count for CRC parameters
    InvalidKeyCount = 3,
    /// Unsupported CRC width (must be 32 or 64)
    UnsupportedWidth = 4,
    /// Invalid UTF-8 string
    InvalidUtf8 = 5,
    /// File I/O error
    IoError = 6,
    /// Internal string conversion error
    StringConversionError = 7,
}

impl CrcFastError {
    /// Returns a static string describing the error
    fn message(&self) -> &'static str {
        match self {
            CrcFastError::Success => "Operation completed successfully",
            CrcFastError::LockPoisoned => "Lock was poisoned (thread panicked while holding lock)",
            CrcFastError::NullPointer => "Null pointer was passed where non-null required",
            CrcFastError::InvalidKeyCount => "Invalid key count for CRC parameters",
            CrcFastError::UnsupportedWidth => "Unsupported CRC width (must be 32 or 64)",
            CrcFastError::InvalidUtf8 => "Invalid UTF-8 string",
            CrcFastError::IoError => "File I/O error",
            CrcFastError::StringConversionError => "Internal string conversion error",
        }
    }
}

// Thread-local storage for the last error that occurred
thread_local! {
    static LAST_ERROR: std::cell::Cell<CrcFastError> = const { std::cell::Cell::new(CrcFastError::Success) };
}

/// Sets the thread-local last error
fn set_last_error(error: CrcFastError) {
    LAST_ERROR.with(|e| e.set(error));
}

/// Clears the thread-local last error (sets it to Success)
fn clear_last_error() {
    LAST_ERROR.with(|e| e.set(CrcFastError::Success));
}

// Global storage for stable key pointers to ensure they remain valid across FFI boundary
static STABLE_KEY_STORAGE: OnceLock<Mutex<HashMap<u64, Box<[u64]>>>> = OnceLock::new();

/// Creates a stable pointer to the keys for FFI usage.
/// The keys are stored in global memory to ensure the pointer remains valid.
/// Returns (pointer, count) on success, or (null, 0) on error.
/// Sets CrcFastError::LockPoisoned on lock failure.
fn create_stable_key_pointer(keys: &crate::CrcKeysStorage) -> (*const u64, u32) {
    let storage = STABLE_KEY_STORAGE.get_or_init(|| Mutex::new(HashMap::new()));

    // Create a unique hash for this key set to avoid duplicates
    let key_hash = match keys {
        crate::CrcKeysStorage::KeysFold256(keys) => {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            keys.hash(&mut hasher);
            hasher.finish()
        }
        crate::CrcKeysStorage::KeysFutureTest(keys) => {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            keys.hash(&mut hasher);
            hasher.finish()
        }
    };

    let mut storage_map = match storage.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_last_error(CrcFastError::LockPoisoned);
            return (std::ptr::null(), 0);
        }
    };

    // Check if we already have this key set stored
    if let Some(stored_keys) = storage_map.get(&key_hash) {
        return (stored_keys.as_ptr(), stored_keys.len() as u32);
    }

    // Store the keys in stable memory
    let key_vec: Vec<u64> = match keys {
        crate::CrcKeysStorage::KeysFold256(keys) => keys.to_vec(),
        crate::CrcKeysStorage::KeysFutureTest(keys) => keys.to_vec(),
    };

    let boxed_keys = key_vec.into_boxed_slice();
    let count = boxed_keys.len() as u32;

    storage_map.insert(key_hash, boxed_keys);

    let ptr = storage_map.get(&key_hash).expect("just inserted").as_ptr();

    (ptr, count)
}

/// A handle to the Digest object
#[repr(C)]
pub struct CrcFastDigestHandle(*mut Digest);

/// The supported CRC algorithms
#[repr(C)]
#[derive(Clone, Copy)]
pub enum CrcFastAlgorithm {
    // CrcCustom works with any supported widths (16, 32, 64)
    CrcCustom,
    Crc16Arc,
    Crc16Cdma2000,
    Crc16Cms,
    Crc16Dds110,
    Crc16DectR,
    Crc16DectX,
    Crc16Dnp,
    Crc16En13757,
    Crc16Genibus,
    Crc16Gsm,
    Crc16Ibm3740,
    Crc16IbmSdlc,
    Crc16IsoIec144433A,
    Crc16Kermit,
    Crc16Lj1200,
    Crc16M17,
    Crc16MaximDow,
    Crc16Mcrf4xx,
    Crc16Modbus,
    Crc16Nrsc5,
    Crc16OpensafetyA,
    Crc16OpensafetyB,
    Crc16Profibus,
    Crc16Riello,
    Crc16SpiFujitsu,
    Crc16T10Dif,
    Crc16Teledisk,
    Crc16Tms37157,
    Crc16Umts,
    Crc16Usb,
    Crc16Xmodem,
    Crc32Aixm,
    Crc32Autosar,
    Crc32Base91D,
    Crc32Bzip2,
    Crc32CdRomEdc,
    Crc32Cksum,
    Crc32Custom,
    Crc32Iscsi,
    Crc32IsoHdlc,
    Crc32Jamcrc,
    Crc32Mef,
    Crc32Mpeg2,
    Crc32Xfer,
    Crc64Custom,
    Crc64Ecma182,
    Crc64GoIso,
    Crc64Ms,
    Crc64Nvme,
    Crc64Redis,
    Crc64We,
    Crc64Xz,
}

// Convert from FFI enum to internal enum
#[allow(deprecated)]
impl From<CrcFastAlgorithm> for CrcAlgorithm {
    fn from(value: CrcFastAlgorithm) -> Self {
        match value {
            CrcFastAlgorithm::Crc16Arc => CrcAlgorithm::Crc16Arc,
            CrcFastAlgorithm::Crc16Cdma2000 => CrcAlgorithm::Crc16Cdma2000,
            CrcFastAlgorithm::Crc16Cms => CrcAlgorithm::Crc16Cms,
            CrcFastAlgorithm::Crc16Dds110 => CrcAlgorithm::Crc16Dds110,
            CrcFastAlgorithm::Crc16DectR => CrcAlgorithm::Crc16DectR,
            CrcFastAlgorithm::Crc16DectX => CrcAlgorithm::Crc16DectX,
            CrcFastAlgorithm::Crc16Dnp => CrcAlgorithm::Crc16Dnp,
            CrcFastAlgorithm::Crc16En13757 => CrcAlgorithm::Crc16En13757,
            CrcFastAlgorithm::Crc16Genibus => CrcAlgorithm::Crc16Genibus,
            CrcFastAlgorithm::Crc16Gsm => CrcAlgorithm::Crc16Gsm,
            CrcFastAlgorithm::Crc16Ibm3740 => CrcAlgorithm::Crc16Ibm3740,
            CrcFastAlgorithm::Crc16IbmSdlc => CrcAlgorithm::Crc16IbmSdlc,
            CrcFastAlgorithm::Crc16IsoIec144433A => CrcAlgorithm::Crc16IsoIec144433A,
            CrcFastAlgorithm::Crc16Kermit => CrcAlgorithm::Crc16Kermit,
            CrcFastAlgorithm::Crc16Lj1200 => CrcAlgorithm::Crc16Lj1200,
            CrcFastAlgorithm::Crc16M17 => CrcAlgorithm::Crc16M17,
            CrcFastAlgorithm::Crc16MaximDow => CrcAlgorithm::Crc16MaximDow,
            CrcFastAlgorithm::Crc16Mcrf4xx => CrcAlgorithm::Crc16Mcrf4xx,
            CrcFastAlgorithm::Crc16Modbus => CrcAlgorithm::Crc16Modbus,
            CrcFastAlgorithm::Crc16Nrsc5 => CrcAlgorithm::Crc16Nrsc5,
            CrcFastAlgorithm::Crc16OpensafetyA => CrcAlgorithm::Crc16OpensafetyA,
            CrcFastAlgorithm::Crc16OpensafetyB => CrcAlgorithm::Crc16OpensafetyB,
            CrcFastAlgorithm::Crc16Profibus => CrcAlgorithm::Crc16Profibus,
            CrcFastAlgorithm::Crc16Riello => CrcAlgorithm::Crc16Riello,
            CrcFastAlgorithm::Crc16SpiFujitsu => CrcAlgorithm::Crc16SpiFujitsu,
            CrcFastAlgorithm::Crc16T10Dif => CrcAlgorithm::Crc16T10Dif,
            CrcFastAlgorithm::Crc16Teledisk => CrcAlgorithm::Crc16Teledisk,
            CrcFastAlgorithm::Crc16Tms37157 => CrcAlgorithm::Crc16Tms37157,
            CrcFastAlgorithm::Crc16Umts => CrcAlgorithm::Crc16Umts,
            CrcFastAlgorithm::Crc16Usb => CrcAlgorithm::Crc16Usb,
            CrcFastAlgorithm::Crc16Xmodem => CrcAlgorithm::Crc16Xmodem,
            CrcFastAlgorithm::Crc32Aixm => CrcAlgorithm::Crc32Aixm,
            CrcFastAlgorithm::Crc32Autosar => CrcAlgorithm::Crc32Autosar,
            CrcFastAlgorithm::Crc32Base91D => CrcAlgorithm::Crc32Base91D,
            CrcFastAlgorithm::Crc32Bzip2 => CrcAlgorithm::Crc32Bzip2,
            CrcFastAlgorithm::Crc32CdRomEdc => CrcAlgorithm::Crc32CdRomEdc,
            CrcFastAlgorithm::Crc32Cksum => CrcAlgorithm::Crc32Cksum,
            CrcFastAlgorithm::Crc32Custom => CrcAlgorithm::Crc32Custom,
            CrcFastAlgorithm::Crc32Iscsi => CrcAlgorithm::Crc32Iscsi,
            CrcFastAlgorithm::Crc32IsoHdlc => CrcAlgorithm::Crc32IsoHdlc,
            CrcFastAlgorithm::Crc32Jamcrc => CrcAlgorithm::Crc32Jamcrc,
            CrcFastAlgorithm::Crc32Mef => CrcAlgorithm::Crc32Mef,
            CrcFastAlgorithm::Crc32Mpeg2 => CrcAlgorithm::Crc32Mpeg2,
            CrcFastAlgorithm::Crc32Xfer => CrcAlgorithm::Crc32Xfer,
            CrcFastAlgorithm::CrcCustom => CrcAlgorithm::CrcCustom,
            CrcFastAlgorithm::Crc64Custom => CrcAlgorithm::Crc64Custom,
            CrcFastAlgorithm::Crc64Ecma182 => CrcAlgorithm::Crc64Ecma182,
            CrcFastAlgorithm::Crc64GoIso => CrcAlgorithm::Crc64GoIso,
            CrcFastAlgorithm::Crc64Ms => CrcAlgorithm::Crc64Ms,
            CrcFastAlgorithm::Crc64Nvme => CrcAlgorithm::Crc64Nvme,
            CrcFastAlgorithm::Crc64Redis => CrcAlgorithm::Crc64Redis,
            CrcFastAlgorithm::Crc64We => CrcAlgorithm::Crc64We,
            CrcFastAlgorithm::Crc64Xz => CrcAlgorithm::Crc64Xz,
        }
    }
}

/// Gets the last error that occurred in the current thread
/// Returns CrcFastError::Success if no error has occurred
#[no_mangle]
pub extern "C" fn crc_fast_get_last_error() -> CrcFastError {
    LAST_ERROR.with(|e| e.get())
}

/// Clears the last error for the current thread
#[no_mangle]
pub extern "C" fn crc_fast_clear_error() {
    clear_last_error();
}

/// Gets a human-readable error message for the given error code
/// Returns a pointer to a static string (do not free)
#[no_mangle]
pub extern "C" fn crc_fast_error_message(error: CrcFastError) -> *const c_char {
    let message = error.message();
    // These are static strings, so we can safely return them as C strings
    // The strings are guaranteed to be valid UTF-8 and null-terminated
    match std::ffi::CString::new(message) {
        Ok(c_str) => {
            // Leak the string so it remains valid for the lifetime of the program
            // This is safe because error messages are static and small
            Box::leak(Box::new(c_str)).as_ptr()
        }
        Err(_) => std::ptr::null(),
    }
}

/// Custom CRC parameters
#[repr(C)]
pub struct CrcFastParams {
    pub algorithm: CrcFastAlgorithm,
    pub width: u8,
    pub poly: u64,
    pub init: u64,
    pub refin: bool,
    pub refout: bool,
    pub xorout: u64,
    pub check: u64,
    pub key_count: u32,
    pub keys: *const u64,
}

/// Fallible conversion from FFI struct to internal struct
/// Returns None if the parameters are invalid (unsupported key count)
fn try_params_from_ffi(value: &CrcFastParams) -> Option<CrcParams> {
    // Validate key pointer
    if value.keys.is_null() {
        return None;
    }

    // Convert C array back to appropriate CrcKeysStorage
    let keys = unsafe { std::slice::from_raw_parts(value.keys, value.key_count as usize) };

    let storage = match value.key_count {
        23 => match keys.try_into() {
            Ok(arr) => crate::CrcKeysStorage::from_keys_fold_256(arr),
            Err(_) => return None,
        },
        25 => match keys.try_into() {
            Ok(arr) => crate::CrcKeysStorage::from_keys_fold_future_test(arr),
            Err(_) => return None,
        },
        _ => return None, // Unsupported key count
    };

    // For reflected CRC-16, bit-reverse the init value for the SIMD algorithm
    let init_algorithm = if value.width == 16 && value.refin {
        (value.init as u16).reverse_bits() as u64
    } else {
        value.init
    };

    Some(CrcParams {
        algorithm: value.algorithm.into(),
        name: "custom", // C interface doesn't need the name field
        width: value.width,
        poly: value.poly,
        init: value.init,
        init_algorithm,
        refin: value.refin,
        refout: value.refout,
        xorout: value.xorout,
        check: value.check,
        keys: storage,
    })
}

// Convert from FFI struct to internal struct (legacy, may panic)
// For backwards compatibility, but prefer try_params_from_ffi
impl From<CrcFastParams> for CrcParams {
    fn from(value: CrcFastParams) -> Self {
        try_params_from_ffi(&value)
            .expect("Invalid CRC parameters: unsupported key count or null pointer")
    }
}

// Convert from internal struct to FFI struct
#[allow(deprecated)]
impl From<CrcParams> for CrcFastParams {
    fn from(params: CrcParams) -> Self {
        // Create stable key pointer for FFI usage
        let (keys_ptr, key_count) = create_stable_key_pointer(&params.keys);

        CrcFastParams {
            algorithm: match params.algorithm {
                CrcAlgorithm::Crc16Arc => CrcFastAlgorithm::Crc16Arc,
                CrcAlgorithm::Crc16Cdma2000 => CrcFastAlgorithm::Crc16Cdma2000,
                CrcAlgorithm::Crc16Cms => CrcFastAlgorithm::Crc16Cms,
                CrcAlgorithm::Crc16Dds110 => CrcFastAlgorithm::Crc16Dds110,
                CrcAlgorithm::Crc16DectR => CrcFastAlgorithm::Crc16DectR,
                CrcAlgorithm::Crc16DectX => CrcFastAlgorithm::Crc16DectX,
                CrcAlgorithm::Crc16Dnp => CrcFastAlgorithm::Crc16Dnp,
                CrcAlgorithm::Crc16En13757 => CrcFastAlgorithm::Crc16En13757,
                CrcAlgorithm::Crc16Genibus => CrcFastAlgorithm::Crc16Genibus,
                CrcAlgorithm::Crc16Gsm => CrcFastAlgorithm::Crc16Gsm,
                CrcAlgorithm::Crc16Ibm3740 => CrcFastAlgorithm::Crc16Ibm3740,
                CrcAlgorithm::Crc16IbmSdlc => CrcFastAlgorithm::Crc16IbmSdlc,
                CrcAlgorithm::Crc16IsoIec144433A => CrcFastAlgorithm::Crc16IsoIec144433A,
                CrcAlgorithm::Crc16Kermit => CrcFastAlgorithm::Crc16Kermit,
                CrcAlgorithm::Crc16Lj1200 => CrcFastAlgorithm::Crc16Lj1200,
                CrcAlgorithm::Crc16M17 => CrcFastAlgorithm::Crc16M17,
                CrcAlgorithm::Crc16MaximDow => CrcFastAlgorithm::Crc16MaximDow,
                CrcAlgorithm::Crc16Mcrf4xx => CrcFastAlgorithm::Crc16Mcrf4xx,
                CrcAlgorithm::Crc16Modbus => CrcFastAlgorithm::Crc16Modbus,
                CrcAlgorithm::Crc16Nrsc5 => CrcFastAlgorithm::Crc16Nrsc5,
                CrcAlgorithm::Crc16OpensafetyA => CrcFastAlgorithm::Crc16OpensafetyA,
                CrcAlgorithm::Crc16OpensafetyB => CrcFastAlgorithm::Crc16OpensafetyB,
                CrcAlgorithm::Crc16Profibus => CrcFastAlgorithm::Crc16Profibus,
                CrcAlgorithm::Crc16Riello => CrcFastAlgorithm::Crc16Riello,
                CrcAlgorithm::Crc16SpiFujitsu => CrcFastAlgorithm::Crc16SpiFujitsu,
                CrcAlgorithm::Crc16T10Dif => CrcFastAlgorithm::Crc16T10Dif,
                CrcAlgorithm::Crc16Teledisk => CrcFastAlgorithm::Crc16Teledisk,
                CrcAlgorithm::Crc16Tms37157 => CrcFastAlgorithm::Crc16Tms37157,
                CrcAlgorithm::Crc16Umts => CrcFastAlgorithm::Crc16Umts,
                CrcAlgorithm::Crc16Usb => CrcFastAlgorithm::Crc16Usb,
                CrcAlgorithm::Crc16Xmodem => CrcFastAlgorithm::Crc16Xmodem,
                CrcAlgorithm::Crc32Aixm => CrcFastAlgorithm::Crc32Aixm,
                CrcAlgorithm::Crc32Autosar => CrcFastAlgorithm::Crc32Autosar,
                CrcAlgorithm::Crc32Base91D => CrcFastAlgorithm::Crc32Base91D,
                CrcAlgorithm::Crc32Bzip2 => CrcFastAlgorithm::Crc32Bzip2,
                CrcAlgorithm::Crc32CdRomEdc => CrcFastAlgorithm::Crc32CdRomEdc,
                CrcAlgorithm::Crc32Cksum => CrcFastAlgorithm::Crc32Cksum,
                CrcAlgorithm::Crc32Custom => CrcFastAlgorithm::Crc32Custom,
                CrcAlgorithm::Crc32Iscsi => CrcFastAlgorithm::Crc32Iscsi,
                CrcAlgorithm::Crc32IsoHdlc => CrcFastAlgorithm::Crc32IsoHdlc,
                CrcAlgorithm::Crc32Jamcrc => CrcFastAlgorithm::Crc32Jamcrc,
                CrcAlgorithm::Crc32Mef => CrcFastAlgorithm::Crc32Mef,
                CrcAlgorithm::Crc32Mpeg2 => CrcFastAlgorithm::Crc32Mpeg2,
                CrcAlgorithm::Crc32Xfer => CrcFastAlgorithm::Crc32Xfer,
                CrcAlgorithm::CrcCustom => CrcFastAlgorithm::CrcCustom,
                CrcAlgorithm::Crc64Custom => CrcFastAlgorithm::Crc64Custom,
                CrcAlgorithm::Crc64Ecma182 => CrcFastAlgorithm::Crc64Ecma182,
                CrcAlgorithm::Crc64GoIso => CrcFastAlgorithm::Crc64GoIso,
                CrcAlgorithm::Crc64Ms => CrcFastAlgorithm::Crc64Ms,
                CrcAlgorithm::Crc64Nvme => CrcFastAlgorithm::Crc64Nvme,
                CrcAlgorithm::Crc64Redis => CrcFastAlgorithm::Crc64Redis,
                CrcAlgorithm::Crc64We => CrcFastAlgorithm::Crc64We,
                CrcAlgorithm::Crc64Xz => CrcFastAlgorithm::Crc64Xz,
            },
            width: params.width,
            poly: params.poly,
            init: params.init,
            refin: params.refin,
            refout: params.refout,
            xorout: params.xorout,
            check: params.check,
            key_count,
            keys: keys_ptr,
        }
    }
}

/// Creates a new Digest to compute CRC checksums using algorithm
#[no_mangle]
pub extern "C" fn crc_fast_digest_new(algorithm: CrcFastAlgorithm) -> *mut CrcFastDigestHandle {
    clear_last_error();
    let digest = Box::new(Digest::new(algorithm.into()));
    let handle = Box::new(CrcFastDigestHandle(Box::into_raw(digest)));
    Box::into_raw(handle)
}

/// Creates a new Digest with a custom initial state
#[no_mangle]
pub extern "C" fn crc_fast_digest_new_with_init_state(
    algorithm: CrcFastAlgorithm,
    init_state: u64,
) -> *mut CrcFastDigestHandle {
    clear_last_error();
    let digest = Box::new(Digest::new_with_init_state(algorithm.into(), init_state));
    let handle = Box::new(CrcFastDigestHandle(Box::into_raw(digest)));
    Box::into_raw(handle)
}

/// Creates a new Digest to compute CRC checksums using custom parameters
/// Returns NULL if parameters are invalid (invalid key count or null pointer)
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_digest_new_with_params(
    params: CrcFastParams,
) -> *mut CrcFastDigestHandle {
    clear_last_error();
    match try_params_from_ffi(&params) {
        Some(crc_params) => {
            let digest = Box::new(Digest::new_with_params(crc_params));
            let handle = Box::new(CrcFastDigestHandle(Box::into_raw(digest)));
            Box::into_raw(handle)
        }
        None => {
            // Set appropriate error based on the failure
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            std::ptr::null_mut()
        }
    }
}

/// Updates the Digest with data
#[no_mangle]
pub extern "C" fn crc_fast_digest_update(
    handle: *mut CrcFastDigestHandle,
    data: *const c_char,
    len: usize,
) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        digest.update(bytes);
    }
}

/// Calculates the CRC checksum for data that's been written to the Digest
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_finalize(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.finalize()
    }
}

/// Free the Digest resources without finalizing
#[no_mangle]
pub extern "C" fn crc_fast_digest_free(handle: *mut CrcFastDigestHandle) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let handle = Box::from_raw(handle);
        let _ = Box::from_raw(handle.0); // This drops the digest
    }
}

/// Reset the Digest state
#[no_mangle]
pub extern "C" fn crc_fast_digest_reset(handle: *mut CrcFastDigestHandle) {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        digest.reset();
    }
}

/// Finalize and reset the Digest in one operation
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_finalize_reset(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &mut *(*handle).0;

        digest.finalize_reset()
    }
}

/// Combine two Digest checksums
#[no_mangle]
pub extern "C" fn crc_fast_digest_combine(
    handle1: *mut CrcFastDigestHandle,
    handle2: *mut CrcFastDigestHandle,
) {
    if handle1.is_null() || handle2.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return;
    }

    clear_last_error();
    unsafe {
        let digest1 = &mut *(*handle1).0;
        let digest2 = &*(*handle2).0;
        digest1.combine(digest2);
    }
}

/// Gets the amount of data processed by the Digest so far
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_get_amount(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.get_amount()
    }
}

/// Gets the current state of the Digest
/// Returns 0 on error (e.g. null handle)
#[no_mangle]
pub extern "C" fn crc_fast_digest_get_state(handle: *mut CrcFastDigestHandle) -> u64 {
    if handle.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        let digest = &*(*handle).0;
        digest.get_state()
    }
}

/// Helper method to calculate a CRC checksum directly for a string using algorithm
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_checksum(
    algorithm: CrcFastAlgorithm,
    data: *const c_char,
    len: usize,
) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::checksum(algorithm.into(), bytes)
    }
}

/// Helper method to calculate a CRC checksum directly for data using custom parameters
/// Returns 0 if parameters are invalid or data is null
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_with_params(
    params: CrcFastParams,
    data: *const c_char,
    len: usize,
) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    match try_params_from_ffi(&params) {
        Some(crc_params) => {
            clear_last_error();
            unsafe {
                #[allow(clippy::unnecessary_cast)]
                let bytes = slice::from_raw_parts(data as *const u8, len);
                crate::checksum_with_params(crc_params, bytes)
            }
        }
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Helper method to just calculate a CRC checksum directly for a file using algorithm
/// Returns 0 if path is null or file I/O fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_file(
    algorithm: CrcFastAlgorithm,
    path_ptr: *const u8,
    path_len: usize,
) -> u64 {
    if path_ptr.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    unsafe {
        match crate::checksum_file(
            algorithm.into(),
            &convert_to_string(path_ptr, path_len),
            None,
        ) {
            Ok(result) => {
                clear_last_error();
                result
            }
            Err(_) => {
                set_last_error(CrcFastError::IoError);
                0
            }
        }
    }
}

/// Helper method to calculate a CRC checksum directly for a file using custom parameters
/// Returns 0 if parameters are invalid, path is null, or file I/O fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_file_with_params(
    params: CrcFastParams,
    path_ptr: *const u8,
    path_len: usize,
) -> u64 {
    if path_ptr.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }

    match try_params_from_ffi(&params) {
        Some(crc_params) => unsafe {
            match crate::checksum_file_with_params(
                crc_params,
                &convert_to_string(path_ptr, path_len),
                None,
            ) {
                Ok(result) => {
                    clear_last_error();
                    result
                }
                Err(_) => {
                    set_last_error(CrcFastError::IoError);
                    0
                }
            }
        },
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Combine two CRC checksums using algorithm
#[no_mangle]
pub extern "C" fn crc_fast_checksum_combine(
    algorithm: CrcFastAlgorithm,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    clear_last_error();
    crate::checksum_combine(algorithm.into(), checksum1, checksum2, checksum2_len)
}

/// Combine two CRC checksums using custom parameters
/// Returns 0 if parameters are invalid
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_checksum_combine_with_params(
    params: CrcFastParams,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    match try_params_from_ffi(&params) {
        Some(crc_params) => {
            clear_last_error();
            crate::checksum_combine_with_params(crc_params, checksum1, checksum2, checksum2_len)
        }
        None => {
            if params.keys.is_null() {
                set_last_error(CrcFastError::NullPointer);
            } else {
                set_last_error(CrcFastError::InvalidKeyCount);
            }
            0
        }
    }
}

/// Returns the custom CRC parameters for a given set of Rocksoft CRC parameters
/// If width is not 32 or 64, sets error to UnsupportedWidth
#[no_mangle]
pub extern "C" fn crc_fast_get_custom_params(
    name_ptr: *const c_char,
    width: u8,
    poly: u64,
    init: u64,
    reflected: bool,
    xorout: u64,
    check: u64,
) -> CrcFastParams {
    // Validate width
    if width != 32 && width != 64 {
        set_last_error(CrcFastError::UnsupportedWidth);
    } else {
        clear_last_error();
    }

    let name = if name_ptr.is_null() {
        "custom"
    } else {
        unsafe {
            match CStr::from_ptr(name_ptr).to_str() {
                Ok(s) => s,
                Err(_) => {
                    set_last_error(CrcFastError::InvalidUtf8);
                    "custom"
                }
            }
        }
    };

    // Get the custom params from the library
    let params = CrcParams::new(
        get_or_leak_string(name), // ✅ Use cached leak
        width,
        poly,
        init,
        reflected,
        xorout,
        check,
    );

    // Create stable key pointer for FFI usage
    let (keys_ptr, key_count) = create_stable_key_pointer(&params.keys);

    // Convert to FFI struct - use CrcCustom for all widths since CrcParams::new now uses it
    CrcFastParams {
        algorithm: CrcFastAlgorithm::CrcCustom,
        width: params.width,
        poly: params.poly,
        init: params.init,
        refin: params.refin,
        refout: params.refout,
        xorout: params.xorout,
        check: params.check,
        key_count,
        keys: keys_ptr,
    }
}

/// Gets the target build properties (CPU architecture and fine-tuning parameters) for this algorithm
/// Returns NULL if string conversion fails
/// Call crc_fast_get_last_error() to get the specific error code
#[no_mangle]
pub extern "C" fn crc_fast_get_calculator_target(algorithm: CrcFastAlgorithm) -> *const c_char {
    let target = get_calculator_target(algorithm.into());

    match std::ffi::CString::new(target) {
        Ok(s) => {
            clear_last_error();
            s.into_raw()
        }
        Err(_) => {
            set_last_error(CrcFastError::StringConversionError);
            std::ptr::null_mut()
        }
    }
}

/// Gets the version of this library
/// Returns a pointer to "unknown" if version string is invalid
#[no_mangle]
pub extern "C" fn crc_fast_get_version() -> *const c_char {
    const VERSION: &CStr =
        match CStr::from_bytes_with_nul(concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes()) {
            Ok(version) => version,
            // Fallback to "unknown" if version string is malformed
            Err(_) => c"unknown",
        };

    VERSION.as_ptr()
}

/// Calculates the CRC-32/ISCSI checksum (commonly called "crc32c" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc32_iscsi(data: *const c_char, len: usize) -> u32 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc32_iscsi(bytes)
    }
}

/// Calculates the CRC-32/ISO-HDLC checksum (commonly called "crc32" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc32_iso_hdlc(data: *const c_char, len: usize) -> u32 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc32_iso_hdlc(bytes)
    }
}

/// Calculates the CRC-64/NVME checksum.
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
///
/// Returns 0 on error (e.g. null data pointer)
#[no_mangle]
pub extern "C" fn crc_fast_crc64_nvme(data: *const c_char, len: usize) -> u64 {
    if data.is_null() {
        set_last_error(CrcFastError::NullPointer);
        return 0;
    }
    clear_last_error();
    unsafe {
        #[allow(clippy::unnecessary_cast)]
        let bytes = slice::from_raw_parts(data as *const u8, len);
        crate::crc64_nvme(bytes)
    }
}

unsafe fn convert_to_string(data: *const u8, len: usize) -> String {
    if data.is_null() {
        return String::new();
    }

    // Safely construct string slice from raw parts
    match std::str::from_utf8(slice::from_raw_parts(data, len)) {
        Ok(s) => s.to_string(),
        Err(_) => String::new(), // Return empty string for invalid UTF-8
    }
}

fn get_or_leak_string(s: &str) -> &'static str {
    let cache = STRING_CACHE.get_or_init(|| Mutex::new(HashSet::new()));
    let mut cache = cache.lock().unwrap();

    // Check if we already have this string
    if let Some(&cached) = cache.get(s) {
        return cached;
    }

    // Leak it and cache the result
    let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
    cache.insert(leaked);
    leaked
}
