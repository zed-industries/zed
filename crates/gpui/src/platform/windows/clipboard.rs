use std::sync::LazyLock;

use anyhow::Result;
use collections::FxHashMap;
use itertools::Itertools;
use util::ResultExt;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::{
        DataExchange::{
            CloseClipboard, CountClipboardFormats, EmptyClipboard, EnumClipboardFormats,
            GetClipboardData, GetClipboardFormatNameW, GetPriorityClipboardFormat,
            IsClipboardFormatAvailable, OpenClipboard, RegisterClipboardFormatW, SetClipboardData,
        },
        Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE},
        Ole::CF_UNICODETEXT,
    },
};
use windows_core::PCWSTR;

use crate::{hash, ClipboardEntry, ClipboardItem, ClipboardString, Image, ImageFormat};

// Clipboard formats
static CLIPBOARD_HASH_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GPUI internal text hash")));
static CLIPBOARD_METADATA_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GPUI internal metadata")));
static CLIPBOARD_PNG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("PNG")));

// Helper format list
static AVAILABLE_FORMATS: LazyLock<[u32; 2]> =
    LazyLock::new(|| [CF_UNICODETEXT.0 as u32, *CLIPBOARD_PNG_FORMAT]);
static AVAILABLE_IMAGE_FORMATS: LazyLock<[u32; 1]> = LazyLock::new(|| [*CLIPBOARD_PNG_FORMAT]);

// Helper struct
static FORMATS_MAP: LazyLock<FxHashMap<u32, ClipboardFormatType>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(CF_UNICODETEXT.0 as u32, ClipboardFormatType::Text);
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ClipboardFormatType::Image);
    formats_map
});

#[derive(Debug, Clone, Copy)]
enum ClipboardFormatType {
    Text,
    Image,
}

fn register_clipboard_format(format: PCWSTR) -> u32 {
    let ret = unsafe { RegisterClipboardFormatW(format) };
    if ret == 0 {
        panic!(
            "Error when registering clipboard format: {}",
            std::io::Error::last_os_error()
        );
    }
    ret
}

fn format_to_type(item_format: u32) -> &'static ClipboardFormatType {
    FORMATS_MAP.get(&item_format).unwrap()
}

pub(crate) fn write_to_clipboard(item: ClipboardItem) {
    write_to_clipboard_inner(item).log_err();
    unsafe { CloseClipboard().log_err() };
}

pub(crate) fn read_from_clipboard() -> Option<ClipboardItem> {
    let result = read_from_clipboard_inner().log_err();
    unsafe { CloseClipboard().log_err() };
    result
}

fn write_to_clipboard_inner(item: ClipboardItem) -> Result<()> {
    unsafe {
        OpenClipboard(None)?;
        EmptyClipboard()?;
    }
    match item.entries().first() {
        Some(entry) => match entry {
            ClipboardEntry::String(string) => {
                write_string_to_clipboard(string)?;
            }
            ClipboardEntry::Image(image) => {
                write_image_to_clipboard(image)?;
            }
        },
        None => {
            // Writing an empty list of entries just clears the clipboard.
        }
    }
    Ok(())
}

fn write_string_to_clipboard(item: &ClipboardString) -> Result<()> {
    let encode_wide = item.text.encode_utf16().chain(Some(0)).collect_vec();
    set_data_to_clipboard(&encode_wide, CF_UNICODETEXT.0 as u32)?;

    if let Some(metadata) = item.metadata.as_ref() {
        let hash_result = {
            let hash = ClipboardString::text_hash(&item.text);
            hash.to_ne_bytes()
        };
        let encode_wide =
            unsafe { std::slice::from_raw_parts(hash_result.as_ptr().cast::<u16>(), 4) };
        set_data_to_clipboard(encode_wide, *CLIPBOARD_HASH_FORMAT)?;

        let metadata_wide = metadata.encode_utf16().chain(Some(0)).collect_vec();
        set_data_to_clipboard(&metadata_wide, *CLIPBOARD_METADATA_FORMAT)?;
    }
    Ok(())
}

fn set_data_to_clipboard<T>(data: &[T], format: u32) -> Result<()> {
    unsafe {
        let global = GlobalAlloc(GMEM_MOVEABLE, data.len() * std::mem::size_of::<T>())?;
        let handle = GlobalLock(global);
        // u_memcpy(handle as _, data.as_ptr(), data.len() as _);
        std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
        let _ = GlobalUnlock(global);
        SetClipboardData(format, HANDLE(global.0))?;
    }
    Ok(())
}

fn write_image_to_clipboard(item: &Image) -> Result<()> {
    if item.format != ImageFormat::Png {
        anyhow::bail!("Clipboard unsupported image format: {:?}", item.format);
    }
    set_data_to_clipboard(item.bytes(), *CLIPBOARD_PNG_FORMAT)?;
    // unsafe {
    //     let data = item.bytes();
    //     let global = GlobalAlloc(GMEM_MOVEABLE, data.len())?;
    //     let handle = GlobalLock(global);
    //     std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
    //     let _ = GlobalUnlock(global);
    //     SetClipboardData(*CLIPBOARD_PNG_FORMAT, HANDLE(global.0))?;
    // }
    Ok(())
}

fn read_from_clipboard_inner() -> Result<ClipboardItem> {
    unsafe {
        OpenClipboard(None)?;
        let Some(item_format) = check_available_formats(&*AVAILABLE_FORMATS) else {
            anyhow::bail!("No available content in clipboard");
        };

        let item_type = format_to_type(item_format);
        let entries = match item_type {
            ClipboardFormatType::Text => {
                if let Some(string) = read_string_from_clipboard() {
                    vec![string]
                } else {
                    vec![]
                }
            }
            ClipboardFormatType::Image => {
                if let Some(image) = read_image_from_clipboard() {
                    vec![image]
                } else {
                    vec![]
                }
            }
        };
        debug_assert!(!entries.is_empty());
        Ok(ClipboardItem { entries })
    }
}

fn read_string_from_clipboard() -> Option<ClipboardEntry> {
    let text = unsafe {
        let handle = GetClipboardData(CF_UNICODETEXT.0 as u32).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        String::from_utf16_lossy(text.as_wide())
    };
    let Some(hash) = read_hash_from_clipboard() else {
        return Some(ClipboardEntry::String(ClipboardString::new(text)));
    };
    let Some(metadata) = read_metadata_from_clipboard() else {
        return Some(ClipboardEntry::String(ClipboardString::new(text)));
    };
    if hash == ClipboardString::text_hash(&text) {
        Some(ClipboardEntry::String(ClipboardString {
            text,
            metadata: Some(metadata),
        }))
    } else {
        Some(ClipboardEntry::String(ClipboardString::new(text)))
    }
}

fn read_hash_from_clipboard() -> Option<u64> {
    unsafe {
        if IsClipboardFormatAvailable(*CLIPBOARD_HASH_FORMAT).is_err() {
            return None;
        }
        let handle = GetClipboardData(*CLIPBOARD_HASH_FORMAT).log_err()?;
        let raw_ptr = handle.0 as *const u16;
        let hash_bytes: [u8; 8] = std::slice::from_raw_parts(raw_ptr.cast::<u8>(), 8)
            .to_vec()
            .try_into()
            .log_err()?;
        Some(u64::from_ne_bytes(hash_bytes))
    }
}

fn read_metadata_from_clipboard() -> Option<String> {
    unsafe {
        if IsClipboardFormatAvailable(*CLIPBOARD_METADATA_FORMAT).is_err() {
            return None;
        }
        let handle = GetClipboardData(*CLIPBOARD_METADATA_FORMAT).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        Some(String::from_utf16_lossy(text.as_wide()))
    }
}

fn read_image_from_clipboard() -> Option<ClipboardEntry> {
    unsafe {
        let Some(image_format) = check_available_formats(&*AVAILABLE_IMAGE_FORMATS) else {
            return None;
        };
        let global = HGLOBAL(GetClipboardData(image_format).log_err()?.0);
        let image_ptr = GlobalLock(global);
        let iamge_size = GlobalSize(global);
        let bytes = std::slice::from_raw_parts(image_ptr as *mut u8 as _, iamge_size).to_vec();
        let _ = GlobalUnlock(global);
        let id = hash(&bytes);
        Some(ClipboardEntry::Image(Image {
            format: ImageFormat::Png,
            bytes,
            id,
        }))
    }
}

fn check_available_formats(formats: &[u32]) -> Option<u32> {
    let ret = unsafe { GetPriorityClipboardFormat(formats) };
    if ret <= 0 {
        if ret == -1 {
            let count = unsafe { CountClipboardFormats() };
            let mut clipboard_format = 0;
            for _ in 0..count {
                clipboard_format = unsafe { EnumClipboardFormats(clipboard_format) };
                let mut buffer = [0u16; 64];
                unsafe { GetClipboardFormatNameW(clipboard_format, &mut buffer) };
                let format_name = String::from_utf16_lossy(&buffer);
                log::warn!(
                    "Try to paste with unsupported clipboard format: {}, {}.",
                    clipboard_format,
                    format_name
                );
            }
        }
        None
    } else {
        Some(ret as u32)
    }
}
