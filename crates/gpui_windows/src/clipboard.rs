use std::sync::LazyLock;

use anyhow::Result;
use collections::FxHashMap;
use itertools::Itertools;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::{
        DataExchange::{
            CloseClipboard, CountClipboardFormats, EmptyClipboard, EnumClipboardFormats,
            GetClipboardData, GetClipboardFormatNameW, IsClipboardFormatAvailable, OpenClipboard,
            RegisterClipboardFormatW, SetClipboardData,
        },
        Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock},
        Ole::{CF_DIB, CF_HDROP, CF_UNICODETEXT},
    },
    UI::Shell::{DragQueryFileW, HDROP},
};
use windows_core::PCWSTR;

use gpui::{
    ClipboardEntry, ClipboardItem, ClipboardString, ExternalPaths, Image, ImageFormat, hash,
};

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;

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
static CLIPBOARD_HTML_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("HTML Format")));

// Helper maps and sets
static FORMATS_MAP: LazyLock<FxHashMap<u32, ClipboardFormatType>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(CF_UNICODETEXT.0 as u32, ClipboardFormatType::Text);
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT, ClipboardFormatType::Image);
    formats_map.insert(CF_DIB.0 as u32, ClipboardFormatType::Image);
    formats_map.insert(CF_HDROP.0 as u32, ClipboardFormatType::Files);
    formats_map.insert(*CLIPBOARD_HTML_FORMAT, ClipboardFormatType::Text);
    formats_map
});
static IMAGE_FORMATS_MAP: LazyLock<FxHashMap<u32, ImageFormat>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ImageFormat::Png);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT, ImageFormat::Gif);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT, ImageFormat::Jpeg);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT, ImageFormat::Svg);
    formats_map
});

#[derive(Debug, Clone, Copy)]
enum ClipboardFormatType {
    Text,
    Image,
    Files,
}

pub(crate) fn write_to_clipboard(item: ClipboardItem) {
    with_clipboard(|| write_to_clipboard_inner(item));
}

pub(crate) fn read_from_clipboard() -> Option<ClipboardItem> {
    with_clipboard(|| {
        with_best_match_format(|item_format| match format_to_type(item_format) {
            ClipboardFormatType::Text => {
                if item_format == CF_UNICODETEXT.0 as u32 {
                    read_string_from_clipboard()
                } else if item_format == *CLIPBOARD_HTML_FORMAT {
                    read_html_string_from_clipboard()
                } else {
                    None
                }
            }
            ClipboardFormatType::Image => read_image_from_clipboard(item_format),
            ClipboardFormatType::Files => read_files_from_clipboard(),
        })
    })
    .flatten()
}

pub(crate) fn with_file_names<F>(hdrop: HDROP, mut f: F)
where
    F: FnMut(String),
{
    let file_count = unsafe { DragQueryFileW(hdrop, DRAGDROP_GET_FILES_COUNT, None) };
    for file_index in 0..file_count {
        let filename_length = unsafe { DragQueryFileW(hdrop, file_index, None) } as usize;
        let mut buffer = vec![0u16; filename_length + 1];
        let ret = unsafe { DragQueryFileW(hdrop, file_index, Some(buffer.as_mut_slice())) };
        if ret == 0 {
            log::error!("unable to read file name of dragged file");
            continue;
        }
        match String::from_utf16(&buffer[0..filename_length]) {
            Ok(file_name) => f(file_name),
            Err(e) => {
                log::error!("dragged file name is not UTF-16: {}", e)
            }
        }
    }
}

fn with_clipboard<F, T>(f: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    match unsafe { OpenClipboard(None) } {
        Ok(()) => {
            let result = f();
            if let Err(e) = unsafe { CloseClipboard() } {
                log::error!("Failed to close clipboard: {e}",);
            }
            Some(result)
        }
        Err(e) => {
            log::error!("Failed to open clipboard: {e}",);
            None
        }
    }
}

fn register_clipboard_format(format: PCWSTR) -> u32 {
    let ret = unsafe { RegisterClipboardFormatW(format) };
    if ret == 0 {
        panic!(
            "Error when registering clipboard format: {}",
            std::io::Error::last_os_error()
        );
    }
    log::debug!(
        "Registered clipboard format {} as {}",
        unsafe { format.display() },
        ret
    );
    ret
}

#[inline]
fn format_to_type(item_format: u32) -> &'static ClipboardFormatType {
    FORMATS_MAP.get(&item_format).unwrap()
}

// Currently, we only write the first item.
fn write_to_clipboard_inner(item: ClipboardItem) -> Result<()> {
    unsafe {
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
            ClipboardEntry::ExternalPaths(_) => {}
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
        let global = GlobalAlloc(GMEM_MOVEABLE, std::mem::size_of_val(data))?;
        let handle = GlobalLock(global);
        std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
        let _ = GlobalUnlock(global);
        SetClipboardData(format, Some(HANDLE(global.0)))?;
    }
    Ok(())
}

// Here writing PNG to the clipboard to better support other apps. For more info, please ref to
// the PR.
fn write_image_to_clipboard(item: &Image) -> Result<()> {
    match item.format {
        ImageFormat::Svg => set_data_to_clipboard(item.bytes(), *CLIPBOARD_SVG_FORMAT)?,
        ImageFormat::Gif => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_GIF_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Gif)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        ImageFormat::Png => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_PNG_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Png)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        ImageFormat::Jpeg => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_JPG_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Jpeg)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        other => {
            log::warn!(
                "Clipboard unsupported image format: {:?}, convert to PNG instead.",
                item.format
            );
            let png_bytes = convert_image_to_png_format(item.bytes(), other)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
    }
    Ok(())
}

fn convert_image_to_png_format(bytes: &[u8], image_format: ImageFormat) -> Result<Vec<u8>> {
    let image =
        image::load_from_memory_with_format(bytes, gpui_image_format_to_image(image_format))?;
    let mut output_buf = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut output_buf),
        image::ImageFormat::Png,
    )?;
    Ok(output_buf)
}

// Here, we enumerate all formats on the clipboard and find the first one that we can process.
// The reason we don't use `GetPriorityClipboardFormat` is that it sometimes returns the
// wrong format.
// For instance, when copying a JPEG image from  Microsoft Word, there may be several formats
// on the clipboard: Jpeg, Png, Svg.
// If we use `GetPriorityClipboardFormat`, it will return Svg, which is not what we want.
fn with_best_match_format<F>(f: F) -> Option<ClipboardItem>
where
    F: Fn(u32) -> Option<ClipboardEntry>,
{
    let mut text = None;
    let mut image = None;
    let mut files = None;
    let count = unsafe { CountClipboardFormats() };
    let mut clipboard_format = 0;
    for _ in 0..count {
        clipboard_format = unsafe { EnumClipboardFormats(clipboard_format) };
        let Some(item_format) = FORMATS_MAP.get(&clipboard_format) else {
            continue;
        };
        let bucket = match item_format {
            ClipboardFormatType::Text if text.is_none() => &mut text,
            ClipboardFormatType::Image if image.is_none() => &mut image,
            ClipboardFormatType::Files if files.is_none() => &mut files,
            _ => continue,
        };
        if let Some(entry) = f(clipboard_format) {
            *bucket = Some(entry);
        }
    }

    if let Some(entry) = [image, files, text].into_iter().flatten().next() {
        return Some(ClipboardItem {
            entries: vec![entry],
        });
    }

    // log the formats that we don't support yet.
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

fn read_string_from_clipboard() -> Option<ClipboardEntry> {
    let text = with_clipboard_data(CF_UNICODETEXT.0 as u32, |data_ptr, _| {
        let pcwstr = PCWSTR(data_ptr as *const u16);
        String::from_utf16_lossy(unsafe { pcwstr.as_wide() })
    })?;
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

fn read_html_string_from_clipboard() -> Option<ClipboardEntry> {
    // CF_HTML is UTF-8 encoded (unlike CF_UNICODETEXT which is UTF-16)
    let text = with_clipboard_data(*CLIPBOARD_HTML_FORMAT, |data_ptr, size| {
        let bytes = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) };
        // Trim any trailing null bytes
        let bytes = match bytes.iter().position(|&b| b == 0) {
            Some(pos) => &bytes[..pos],
            None => bytes,
        };
        let html_str = std::str::from_utf8(bytes).ok()?;
        Some(extract_plain_text_from_cf_html(html_str))
    })??;
    if text.is_empty() {
        return None;
    }
    Some(ClipboardEntry::String(ClipboardString::new(text)))
}

/// Extracts plain text from CF_HTML clipboard format.
///
/// CF_HTML has a header with byte offsets pointing to the HTML fragment:
/// ```text
/// Version:0.9
/// StartHTML:000000105
/// EndHTML:000000184
/// StartFragment:000000141
/// EndFragment:000000148
/// <html><body><!--StartFragment-->content<!--EndFragment--></body></html>
/// ```
fn extract_plain_text_from_cf_html(cf_html: &str) -> String {
    // Try to extract just the fragment using the header offsets
    let fragment = extract_html_fragment(cf_html).unwrap_or(cf_html);
    strip_html_tags(fragment)
}

fn extract_html_fragment(cf_html: &str) -> Option<&str> {
    let start = cf_html
        .lines()
        .find(|line| line.starts_with("StartFragment:"))?
        .strip_prefix("StartFragment:")?
        .trim()
        .parse::<usize>()
        .ok()?;
    let end = cf_html
        .lines()
        .find(|line| line.starts_with("EndFragment:"))?
        .strip_prefix("EndFragment:")?
        .trim()
        .parse::<usize>()
        .ok()?;
    cf_html.get(start..end)
}

/// Strips HTML tags and decodes common HTML entities to produce plain text.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut chars = html.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => {
                // Check for <br> variants to insert newlines
                let rest: String = chars.clone().take(10).collect();
                let rest_lower = rest.to_ascii_lowercase();
                if rest_lower.starts_with("br")
                    && rest_lower[2..]
                        .starts_with(|c: char| c == '>' || c == '/' || c == ' ')
                {
                    result.push('\n');
                }
                if rest_lower.starts_with("/p>")
                    || rest_lower.starts_with("/div>")
                    || rest_lower.starts_with("/li>")
                    || rest_lower.starts_with("/tr>")
                {
                    result.push('\n');
                }
                in_tag = true;
            }
            '>' if in_tag => {
                in_tag = false;
            }
            '&' if !in_tag => {
                // Decode HTML entities
                let mut entity = String::new();
                for ec in chars.by_ref() {
                    if ec == ';' {
                        break;
                    }
                    entity.push(ec);
                    if entity.len() > 10 {
                        // Not a real entity, push what we have
                        result.push('&');
                        result.push_str(&entity);
                        break;
                    }
                }
                if entity.len() <= 10 {
                    match entity.as_str() {
                        "amp" => result.push('&'),
                        "lt" => result.push('<'),
                        "gt" => result.push('>'),
                        "quot" => result.push('"'),
                        "apos" => result.push('\''),
                        "nbsp" => result.push(' '),
                        s if s.starts_with('#') => {
                            let code = if s.starts_with("#x") || s.starts_with("#X") {
                                u32::from_str_radix(&s[2..], 16).ok()
                            } else {
                                s[1..].parse::<u32>().ok()
                            };
                            if let Some(c) = code.and_then(char::from_u32) {
                                result.push(c);
                            }
                        }
                        _ => {
                            result.push('&');
                            result.push_str(&entity);
                            result.push(';');
                        }
                    }
                }
            }
            _ if !in_tag => {
                result.push(ch);
            }
            _ => {}
        }
    }

    // Collapse multiple consecutive newlines and trim
    let mut cleaned = String::with_capacity(result.len());
    let mut prev_newline = false;
    for ch in result.chars() {
        if ch == '\n' {
            if !prev_newline {
                cleaned.push('\n');
            }
            prev_newline = true;
        } else if ch == '\r' {
            continue;
        } else {
            prev_newline = false;
            cleaned.push(ch);
        }
    }
    cleaned.trim_matches('\n').to_string()
}

fn read_hash_from_clipboard() -> Option<u64> {
    if unsafe { IsClipboardFormatAvailable(*CLIPBOARD_HASH_FORMAT).is_err() } {
        return None;
    }
    with_clipboard_data(*CLIPBOARD_HASH_FORMAT, |data_ptr, size| {
        if size < 8 {
            return None;
        }
        let hash_bytes: [u8; 8] = unsafe {
            std::slice::from_raw_parts(data_ptr.cast::<u8>(), 8)
                .try_into()
                .ok()
        }?;
        Some(u64::from_ne_bytes(hash_bytes))
    })?
}

fn read_metadata_from_clipboard() -> Option<String> {
    unsafe { IsClipboardFormatAvailable(*CLIPBOARD_METADATA_FORMAT).ok()? };
    with_clipboard_data(*CLIPBOARD_METADATA_FORMAT, |data_ptr, _size| {
        let pcwstr = PCWSTR(data_ptr as *const u16);
        String::from_utf16_lossy(unsafe { pcwstr.as_wide() })
    })
}

fn read_image_from_clipboard(format: u32) -> Option<ClipboardEntry> {
    // Handle CF_DIB format specially - it's raw bitmap data that needs conversion
    if format == CF_DIB.0 as u32 {
        return read_image_for_type(format, ImageFormat::Bmp, Some(convert_dib_to_bmp));
    }
    let image_format = format_number_to_image_format(format)?;
    read_image_for_type::<fn(&[u8]) -> Option<Vec<u8>>>(format, *image_format, None)
}

/// Convert DIB data to BMP file format.
/// DIB is essentially BMP without a file header, so we just need to add the 14-byte BITMAPFILEHEADER.
fn convert_dib_to_bmp(dib_data: &[u8]) -> Option<Vec<u8>> {
    if dib_data.len() < 40 {
        return None;
    }

    let file_size = 14 + dib_data.len() as u32;
    // Calculate pixel data offset
    let header_size = u32::from_le_bytes(dib_data[0..4].try_into().ok()?);
    let bit_count = u16::from_le_bytes(dib_data[14..16].try_into().ok()?);
    let compression = u32::from_le_bytes(dib_data[16..20].try_into().ok()?);

    // Calculate color table size
    let color_table_size = if bit_count <= 8 {
        let colors_used = u32::from_le_bytes(dib_data[32..36].try_into().ok()?);
        let num_colors = if colors_used == 0 {
            1u32 << bit_count
        } else {
            colors_used
        };
        num_colors * 4
    } else if compression == 3 {
        12 // BI_BITFIELDS
    } else {
        0
    };

    let pixel_data_offset = 14 + header_size + color_table_size;

    // Build BITMAPFILEHEADER (14 bytes)
    let mut bmp_data = Vec::with_capacity(file_size as usize);
    bmp_data.extend_from_slice(b"BM"); // Signature
    bmp_data.extend_from_slice(&file_size.to_le_bytes()); // File size
    bmp_data.extend_from_slice(&[0u8; 4]); // Reserved
    bmp_data.extend_from_slice(&pixel_data_offset.to_le_bytes()); // Pixel data offset
    bmp_data.extend_from_slice(dib_data); // DIB data

    Some(bmp_data)
}

#[inline]
fn format_number_to_image_format(format_number: u32) -> Option<&'static ImageFormat> {
    IMAGE_FORMATS_MAP.get(&format_number)
}

fn read_image_for_type<F>(
    format_number: u32,
    format: ImageFormat,
    convert: Option<F>,
) -> Option<ClipboardEntry>
where
    F: FnOnce(&[u8]) -> Option<Vec<u8>>,
{
    let (bytes, id) = with_clipboard_data(format_number, |data_ptr, size| {
        let raw_bytes = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) };
        let bytes = match convert {
            Some(converter) => converter(raw_bytes)?,
            None => raw_bytes.to_vec(),
        };
        let id = hash(&bytes);
        Some((bytes, id))
    })??;
    Some(ClipboardEntry::Image(Image { format, bytes, id }))
}

fn read_files_from_clipboard() -> Option<ClipboardEntry> {
    let filenames = with_clipboard_data(CF_HDROP.0 as u32, |data_ptr, _size| {
        let hdrop = HDROP(data_ptr);
        let mut filenames = Vec::new();
        with_file_names(hdrop, |file_name| {
            filenames.push(std::path::PathBuf::from(file_name));
        });
        filenames
    })?;
    Some(ClipboardEntry::ExternalPaths(ExternalPaths(
        filenames.into(),
    )))
}

fn with_clipboard_data<F, R>(format: u32, f: F) -> Option<R>
where
    F: FnOnce(*mut std::ffi::c_void, usize) -> R,
{
    let global = HGLOBAL(unsafe { GetClipboardData(format).ok() }?.0);
    let size = unsafe { GlobalSize(global) };
    let data_ptr = unsafe { GlobalLock(global) };
    let result = f(data_ptr, size);
    unsafe { GlobalUnlock(global).ok() };
    Some(result)
}

fn gpui_image_format_to_image(value: ImageFormat) -> image::ImageFormat {
    match value {
        ImageFormat::Png => image::ImageFormat::Png,
        ImageFormat::Jpeg => image::ImageFormat::Jpeg,
        ImageFormat::Webp => image::ImageFormat::WebP,
        ImageFormat::Gif => image::ImageFormat::Gif,
        // TODO: ImageFormat::Svg
        ImageFormat::Bmp => image::ImageFormat::Bmp,
        ImageFormat::Tiff => image::ImageFormat::Tiff,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_html_fragment() {
        let cf_html = "Version:0.9\r\nStartHTML:0000000105\r\nEndHTML:0000000177\r\nStartFragment:0000000137\r\nEndFragment:0000000146\r\n<html><body><!--StartFragment-->Hello Zed<!--EndFragment--></body></html>";
        let fragment = extract_html_fragment(cf_html).unwrap();
        assert_eq!(fragment, "Hello Zed");
    }

    #[test]
    fn test_extract_html_fragment_missing_header() {
        let html = "<html><body>Hello</body></html>";
        assert!(extract_html_fragment(html).is_none());
    }

    #[test]
    fn test_strip_html_tags_plain_text() {
        assert_eq!(strip_html_tags("Hello world"), "Hello world");
    }

    #[test]
    fn test_strip_html_tags_simple() {
        assert_eq!(strip_html_tags("<p>Hello <b>world</b></p>"), "Hello world");
    }

    #[test]
    fn test_strip_html_tags_br() {
        assert_eq!(strip_html_tags("line1<br>line2"), "line1\nline2");
        assert_eq!(strip_html_tags("line1<br/>line2"), "line1\nline2");
        assert_eq!(strip_html_tags("line1<BR>line2"), "line1\nline2");
    }

    #[test]
    fn test_strip_html_tags_block_elements() {
        assert_eq!(
            strip_html_tags("<div>first</div><div>second</div>"),
            "first\nsecond"
        );
        assert_eq!(strip_html_tags("<p>para1</p><p>para2</p>"), "para1\npara2");
    }

    #[test]
    fn test_strip_html_tags_entities() {
        assert_eq!(strip_html_tags("&amp; &lt; &gt; &quot;"), "& < > \"");
        assert_eq!(strip_html_tags("&nbsp;"), " ");
        assert_eq!(strip_html_tags("&#65;"), "A");
        assert_eq!(strip_html_tags("&#x41;"), "A");
    }

    #[test]
    fn test_strip_html_tags_collapses_newlines() {
        assert_eq!(
            strip_html_tags("<p>a</p>\n\n<p>b</p>"),
            "a\nb"
        );
    }

    #[test]
    fn test_extract_plain_text_from_cf_html_full() {
        let cf_html = "Version:0.9\r\nStartHTML:0000000105\r\nEndHTML:0000000192\r\nStartFragment:0000000137\r\nEndFragment:0000000161\r\n<html><body><!--StartFragment--><p>Hello</p><p>World</p><!--EndFragment--></body></html>";
        let text = extract_plain_text_from_cf_html(cf_html);
        assert_eq!(text, "Hello\nWorld");
    }
}
