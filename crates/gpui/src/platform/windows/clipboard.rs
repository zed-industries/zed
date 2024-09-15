use std::sync::LazyLock;

use anyhow::Result;
use collections::{FxHashMap, FxHashSet};
use itertools::Itertools;
use util::ResultExt;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::{
        DataExchange::{
            CloseClipboard, CountClipboardFormats, EmptyClipboard, EnumClipboardFormats,
            GetClipboardData, GetClipboardFormatNameW, IsClipboardFormatAvailable, OpenClipboard,
            RegisterClipboardFormatW, SetClipboardData,
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
static CLIPBOARD_SVG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("image/svg+xml")));
static CLIPBOARD_GIF_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GIF")));
static CLIPBOARD_PNG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("PNG")));
static CLIPBOARD_JPG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("JFIF")));

// Helper maps and sets
static FORMATS_MAP: LazyLock<FxHashMap<u32, ClipboardFormatType>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(CF_UNICODETEXT.0 as u32, ClipboardFormatType::Text);
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT, ClipboardFormatType::Image);
    formats_map
});
static FORMATS_NUMBER_MAP: LazyLock<FxHashMap<u32, ImageFormat>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ImageFormat::Png);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT, ImageFormat::Gif);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT, ImageFormat::Jpeg);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT, ImageFormat::Svg);
    formats_map
});
static ALL_FORMATS_SET: LazyLock<FxHashSet<u32>> = LazyLock::new(|| {
    let mut formats_map = FxHashSet::default();
    formats_map.insert(CF_UNICODETEXT.0 as u32);
    formats_map.insert(*CLIPBOARD_PNG_FORMAT);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT);
    formats_map
});
static IMAGE_FORMATS_SET: LazyLock<FxHashSet<u32>> = LazyLock::new(|| {
    let mut formats_map = FxHashSet::default();
    formats_map.insert(*CLIPBOARD_PNG_FORMAT);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT);
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

#[inline]
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
        std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
        let _ = GlobalUnlock(global);
        SetClipboardData(format, HANDLE(global.0))?;
    }
    Ok(())
}

fn write_image_to_clipboard(item: &Image) -> Result<()> {
    match item.format {
        ImageFormat::Svg => set_data_to_clipboard(item.bytes(), *CLIPBOARD_SVG_FORMAT)?,
        ImageFormat::Gif => set_data_to_clipboard(item.bytes(), *CLIPBOARD_GIF_FORMAT)?,
        ImageFormat::Png => set_data_to_clipboard(item.bytes(), *CLIPBOARD_PNG_FORMAT)?,
        ImageFormat::Jpeg => set_data_to_clipboard(item.bytes(), *CLIPBOARD_JPG_FORMAT)?,
        _ => anyhow::bail!("Clipboard unsupported image format: {:?}", item.format),
    }
    Ok(())
}

// fn to_png_format(bytes: &[u8]) -> Result<()> {
//     let x = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
//     x.as_bytes().to_vec()
// }

fn read_from_clipboard_inner() -> Result<ClipboardItem> {
    unsafe {
        OpenClipboard(None)?;
        let Some(item_format) = find_best_match_format(&ALL_FORMATS_SET) else {
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
    let Some(format_number) = find_best_match_format(&IMAGE_FORMATS_SET) else {
        return None;
    };
    let Some(image_format) = format_number_to_image_format(format_number) else {
        return None;
    };
    println!("==> image: {:?}", image_format);
    read_image_for_type(format_number, *image_format)
}

// Here we enumerate all formats on clipboard, find the first one we can process.
// The reason we dont use `GetPriorityClipboardFormat` here is that it sometimes return the wrong format.
// Say copy a JPEG image from Word, there are serveral formats in clipboard:
// Jpeg, Png, Svg
// If we use `GetPriorityClipboardFormat` we will get Svg back, which is not what we want.
fn find_best_match_format(formats_set: &LazyLock<FxHashSet<u32>>) -> Option<u32> {
    let count = unsafe { CountClipboardFormats() };
    let mut clipboard_format = 0;
    for _ in 0..count {
        clipboard_format = unsafe { EnumClipboardFormats(clipboard_format) };
        let Some(image_format) = formats_set.get(&clipboard_format) else {
            continue;
        };
        return Some(*image_format);
    }
    // log the formats that we dont support
    {
        clipboard_format = 0;
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
}

#[inline]
fn format_number_to_image_format(format_number: u32) -> Option<&'static ImageFormat> {
    FORMATS_NUMBER_MAP.get(&format_number)
}

fn read_image_for_type(format_number: u32, format: ImageFormat) -> Option<ClipboardEntry> {
    unsafe {
        let global = HGLOBAL(GetClipboardData(format_number).log_err()?.0);
        let image_ptr = GlobalLock(global);
        let iamge_size = GlobalSize(global);
        let bytes = std::slice::from_raw_parts(image_ptr as *mut u8 as _, iamge_size).to_vec();
        let _ = GlobalUnlock(global);
        let id = hash(&bytes);
        Some(ClipboardEntry::Image(Image { format, bytes, id }))
    }
}
