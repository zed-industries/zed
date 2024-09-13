use anyhow::Result;
use itertools::Itertools;
use util::ResultExt;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    Globalization::u_memcpy,
    System::{
        DataExchange::{
            CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard,
            RegisterClipboardFormatW, SetClipboardData,
        },
        Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE},
        Ole::CF_UNICODETEXT,
    },
};
use windows_core::PCWSTR;

use crate::{hash, ClipboardEntry, ClipboardItem, ClipboardString, Image, ImageFormat};

const CLIPBOARD_HASH_FORMAT: PCWSTR = windows::core::w!("GPUI internal text hash");
const CLIPBOARD_METADATA_FORMAT: PCWSTR = windows::core::w!("GPUI internal metadata");
const CLIPBOARD_PNG_FORMAT: PCWSTR = windows::core::w!("PNG");

pub(crate) struct ClipboardFormatStore {
    pub(crate) text: u32,
    pub(crate) hash: u32,
    pub(crate) metadata: u32,
    pub(crate) png: u32,
}

impl ClipboardFormatStore {
    pub(crate) fn new() -> Result<Self> {
        let text = CF_UNICODETEXT.0 as u32;
        let hash = register_clipboard_format(CLIPBOARD_HASH_FORMAT)?;
        let metadata = register_clipboard_format(CLIPBOARD_PNG_FORMAT)?;
        let png = register_clipboard_format(CLIPBOARD_PNG_FORMAT)?;
        Ok(Self {
            text,
            hash,
            metadata,
            png,
        })
    }
}

fn register_clipboard_format(format: PCWSTR) -> Result<u32> {
    let ret = unsafe { RegisterClipboardFormatW(format) };
    if ret == 0 {
        anyhow::bail!(
            "Error when registering clipboard format: {}",
            std::io::Error::last_os_error()
        )
    } else {
        Ok(ret)
    }
}

pub(crate) fn write_to_clipboard(item: ClipboardItem, format_store: &ClipboardFormatStore) {
    write_to_clipboard_inner(item, format_store).log_err();
    unsafe { CloseClipboard().log_err() };
}

pub(crate) fn read_from_clipboard(format_store: &ClipboardFormatStore) -> Option<ClipboardItem> {
    let result = read_from_clipboard_inner(format_store).log_err();
    unsafe { CloseClipboard().log_err() };
    result
}

fn write_to_clipboard_inner(
    item: ClipboardItem,
    format_store: &ClipboardFormatStore,
) -> Result<()> {
    unsafe {
        OpenClipboard(None)?;
        EmptyClipboard()?;
    }
    for entry in item.entries() {
        match entry {
            ClipboardEntry::String(string) => {
                write_string_to_clipboard(string, format_store)?;
            }
            ClipboardEntry::Image(image) => {
                write_image_to_clipboard(image, format_store)?;
            }
        }
    }
    Ok(())
}

fn write_string_to_clipboard(
    item: &ClipboardString,
    format_store: &ClipboardFormatStore,
) -> Result<()> {
    let encode_wide = item.text.encode_utf16().chain(Some(0)).collect_vec();
    set_data_to_clipboard(&encode_wide, CF_UNICODETEXT.0 as u32)?;

    if let Some(metadata) = item.metadata.as_ref() {
        let hash_result = {
            let hash = ClipboardString::text_hash(&item.text);
            hash.to_ne_bytes()
        };
        let encode_wide =
            unsafe { std::slice::from_raw_parts(hash_result.as_ptr().cast::<u16>(), 4) };
        set_data_to_clipboard(encode_wide, format_store.hash)?;

        let metadata_wide = metadata.encode_utf16().chain(Some(0)).collect_vec();
        set_data_to_clipboard(&metadata_wide, format_store.metadata)?;
    }
    Ok(())
}

fn set_data_to_clipboard(data: &[u16], format: u32) -> Result<()> {
    unsafe {
        let global = GlobalAlloc(GMEM_MOVEABLE, data.len() * 2)?;
        let handle = GlobalLock(global);
        u_memcpy(handle as _, data.as_ptr(), data.len() as _);
        let _ = GlobalUnlock(global);
        SetClipboardData(format, HANDLE(global.0))?;
    }
    Ok(())
}

fn write_image_to_clipboard(item: &Image, format_store: &ClipboardFormatStore) -> Result<()> {
    if item.format != ImageFormat::Png {
        anyhow::bail!("Unsupported format: {:?}", item.format);
    }
    unsafe {
        let data = item.bytes();
        let global = GlobalAlloc(GMEM_MOVEABLE, data.len())?;
        let handle = GlobalLock(global);
        std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
        let _ = GlobalUnlock(global);
        SetClipboardData(format_store.png, HANDLE(global.0))?;
    }
    Ok(())
}

fn read_from_clipboard_inner(format_store: &ClipboardFormatStore) -> Result<ClipboardItem> {
    unsafe {
        OpenClipboard(None)?;
        let mut entries = Vec::new();
        if let Some(string) = read_string_from_clipboard(format_store).log_err() {
            entries.push(string);
        }
        if let Some(image) = read_image_from_clipboard(format_store).log_err() {
            entries.push(image);
        }
        if entries.is_empty() {
            anyhow::bail!("No content in clipboard");
        } else {
            Ok(ClipboardItem { entries })
        }
    }
}

fn read_string_from_clipboard(format_store: &ClipboardFormatStore) -> Result<ClipboardEntry> {
    let text = unsafe {
        let handle = GetClipboardData(CF_UNICODETEXT.0 as u32)?;
        let text = PCWSTR(handle.0 as *const u16);
        String::from_utf16_lossy(text.as_wide())
    };
    let Some(hash) = read_hash_from_clipboard(format_store) else {
        return Ok(ClipboardEntry::String(ClipboardString::new(text)));
    };
    let Some(metadata) = read_metadata_from_clipboard(format_store) else {
        return Ok(ClipboardEntry::String(ClipboardString::new(text)));
    };
    if hash == ClipboardString::text_hash(&text) {
        Ok(ClipboardEntry::String(ClipboardString {
            text,
            metadata: Some(metadata),
        }))
    } else {
        Ok(ClipboardEntry::String(ClipboardString::new(text)))
    }
}

fn read_hash_from_clipboard(format_store: &ClipboardFormatStore) -> Option<u64> {
    unsafe {
        let handle = GetClipboardData(format_store.hash).log_err()?;
        let raw_ptr = handle.0 as *const u16;
        let hash_bytes: [u8; 8] = std::slice::from_raw_parts(raw_ptr.cast::<u8>(), 8)
            .to_vec()
            .try_into()
            .log_err()?;
        Some(u64::from_ne_bytes(hash_bytes))
    }
}

fn read_metadata_from_clipboard(format_store: &ClipboardFormatStore) -> Option<String> {
    unsafe {
        let handle = GetClipboardData(format_store.metadata).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        Some(String::from_utf16_lossy(text.as_wide()))
    }
}

fn read_image_from_clipboard(format_store: &ClipboardFormatStore) -> Result<ClipboardEntry> {
    unsafe {
        let global = HGLOBAL(GetClipboardData(format_store.png)?.0);
        let image_ptr = GlobalLock(global);
        let iamge_size = GlobalSize(global);
        let bytes = std::slice::from_raw_parts(image_ptr as *mut u8 as _, iamge_size).to_vec();
        let _ = GlobalUnlock(global);
        let id = hash(&bytes);
        Ok(ClipboardEntry::Image(Image {
            format: ImageFormat::Png,
            bytes,
            id,
        }))
    }
}
