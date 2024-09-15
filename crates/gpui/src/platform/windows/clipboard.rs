use anyhow::Result;
use collections::FxHashMap;
use itertools::Itertools;
use util::ResultExt;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    Globalization::u_memcpy,
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

const CLIPBOARD_HASH_FORMAT: PCWSTR = windows::core::w!("GPUI internal text hash");
const CLIPBOARD_METADATA_FORMAT: PCWSTR = windows::core::w!("GPUI internal metadata");
const CLIPBOARD_PNG_FORMAT: PCWSTR = windows::core::w!("PNG");

pub(crate) struct ClipboardFormatStore {
    pub(crate) text: u32,
    pub(crate) hash: u32,
    pub(crate) metadata: u32,
    pub(crate) png: u32,
    formats_map: FxHashMap<u32, ClipboardFormatType>,
}

#[derive(Debug, Clone, Copy)]
enum ClipboardFormatType {
    Text,
    Image,
}

impl ClipboardFormatStore {
    pub(crate) fn new() -> Result<Self> {
        let text = CF_UNICODETEXT.0 as u32;
        let hash = register_clipboard_format(CLIPBOARD_HASH_FORMAT)?;
        let metadata = register_clipboard_format(CLIPBOARD_PNG_FORMAT)?;
        let png = register_clipboard_format(CLIPBOARD_PNG_FORMAT)?;
        let mut formats_map = FxHashMap::default();
        formats_map.insert(text, ClipboardFormatType::Text);
        formats_map.insert(png, ClipboardFormatType::Image);

        Ok(Self {
            text,
            hash,
            metadata,
            png,
            formats_map,
        })
    }

    fn available_formats(&self) -> [u32; 2] {
        [self.text, self.png]
    }

    fn format_to_type(&self, format: u32) -> &ClipboardFormatType {
        self.formats_map.get(&format).unwrap()
    }

    fn image_formats(&self) -> [u32; 1] {
        [self.png]
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
        anyhow::bail!("Clipboard unsupported image format: {:?}", item.format);
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
        let Some(item_format) = check_available_formats(&format_store.available_formats()) else {
            anyhow::bail!("No available content in clipboard");
        };

        let item_type = format_store.format_to_type(item_format);
        match item_type {
            ClipboardFormatType::Text => {
                if let Some(string) = read_string_from_clipboard(format_store) {
                    entries.push(string);
                }
            }
            ClipboardFormatType::Image => {
                if let Some(image) = read_image_from_clipboard(format_store) {
                    entries.push(image);
                }
            }
        }
        debug_assert!(!entries.is_empty());
        Ok(ClipboardItem { entries })
    }
}

fn read_string_from_clipboard(format_store: &ClipboardFormatStore) -> Option<ClipboardEntry> {
    let text = unsafe {
        let handle = GetClipboardData(CF_UNICODETEXT.0 as u32).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        String::from_utf16_lossy(text.as_wide())
    };
    let Some(hash) = read_hash_from_clipboard(format_store) else {
        return Some(ClipboardEntry::String(ClipboardString::new(text)));
    };
    let Some(metadata) = read_metadata_from_clipboard(format_store) else {
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

fn read_hash_from_clipboard(format_store: &ClipboardFormatStore) -> Option<u64> {
    unsafe {
        if IsClipboardFormatAvailable(format_store.hash).is_err() {
            return None;
        }
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
        if IsClipboardFormatAvailable(format_store.metadata).is_err() {
            return None;
        }
        let handle = GetClipboardData(format_store.metadata).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        Some(String::from_utf16_lossy(text.as_wide()))
    }
}

fn read_image_from_clipboard(format_store: &ClipboardFormatStore) -> Option<ClipboardEntry> {
    unsafe {
        let Some(image_format) = check_available_formats(&format_store.image_formats()) else {
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
                log::info!(
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
