use core::slice;
use std::ffi::c_void;

use cocoa::{
    appkit::{NSPasteboard, NSPasteboardTypePNG, NSPasteboardTypeString, NSPasteboardTypeTIFF},
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSData, NSString, NSUInteger, NSURL},
};
use objc::{msg_send, runtime::Object, sel, sel_impl};
use strum::IntoEnumIterator as _;

use crate::{
    ClipboardEntry, ClipboardItem, ClipboardString, Image, ImageFormat, asset_cache::hash,
    platform::mac::ns_string,
};

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;

pub struct Pasteboard {
    inner: id,
    text_hash_type: id,
    metadata_type: id,
}

impl Pasteboard {
    pub fn general() -> Self {
        unsafe { Self::new(NSPasteboard::generalPasteboard(nil)) }
    }

    pub fn find() -> Self {
        unsafe { Self::new(NSPasteboard::pasteboardWithName(nil, NSPasteboardNameFind)) }
    }

    #[cfg(test)]
    pub fn unique() -> Self {
        unsafe { Self::new(NSPasteboard::pasteboardWithUniqueName(nil)) }
    }

    unsafe fn new(inner: id) -> Self {
        Self {
            inner,
            text_hash_type: unsafe { ns_string("zed-text-hash") },
            metadata_type: unsafe { ns_string("zed-metadata") },
        }
    }

    pub fn read(&self) -> Option<ClipboardItem> {
        // First, see if it's a string.
        unsafe {
            let pasteboard_types: id = self.inner.types();
            let string_type: id = ns_string("public.utf8-plain-text");

            let mut clipboard_str: Option<&[u8]> = None;

            if msg_send![pasteboard_types, containsObject: string_type] {
                let data = self.inner.dataForType(string_type);
                if data == nil {
                    clipboard_str = None;
                } else if data.bytes().is_null() {
                    // https://developer.apple.com/documentation/foundation/nsdata/1410616-bytes?language=objc
                    // "If the length of the NSData object is 0, this property returns nil."
                    clipboard_str = Some(&[]);
                } else {
                    clipboard_str = Some(slice::from_raw_parts(
                        data.bytes() as *mut u8,
                        data.length() as usize,
                    ));
                }
            }

            let file_type: id = ns_string("public.file-url");
            let mut clipboard_url: Option<&[u8]> = None;
            let escape_url;

            if msg_send![pasteboard_types, containsObject:file_type] {
                let data = self.inner.dataForType(file_type);
                if data == nil {
                    clipboard_url = None;
                } else if data.bytes().is_null() {
                    // https://developer.apple.com/documentation/foundation/nsdata/1410616-bytes?language=objc
                    // "If the length of the NSData object is 0, this property returns nil."
                    clipboard_url = Some(&[]);
                } else {
                    // Have to decode the path by creating NSURL
                    let path = NSURL::alloc(nil)
                        .initWithDataRepresentation_relativeToURL_(data, nil)
                        .autorelease()
                        .path(); /* (NSString) or nil */
                    if path == nil {
                        clipboard_url = Some(&[]);
                    } else {
                        let len = msg_send![path, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
                        let bytes = path.UTF8String() as *const u8;
                        let path =
                            str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap_or_default();

                        escape_url = self.shell_escape_str(path);

                        clipboard_url = Some(escape_url.as_bytes());
                    }
                }
            }

            match (clipboard_str, clipboard_url) {
                (Some(clipboard_str), Some(clipboard_url)) => {
                    return Some(self.read_url(clipboard_url, clipboard_str));
                }
                (Some(clipboard_str), _) => {
                    return Some(self.read_string(clipboard_str));
                }
                (_, Some(clipboard_url)) => {
                    return Some(self.read_url(&[], clipboard_url));
                }
                _ => {}
            }

            // If it wasn't a string or url, try the various supported image types.
            for format in ImageFormat::iter() {
                if let Some(item) = self.read_image(format) {
                    return Some(item);
                }
            }
        }

        // If it wasn't a string or a supported image type, give up.
        None
    }

    fn read_image(&self, format: ImageFormat) -> Option<ClipboardItem> {
        let mut ut_type: UTType = format.into();

        unsafe {
            let types: id = self.inner.types();
            if msg_send![types, containsObject: ut_type.inner()] {
                self.data_for_type(ut_type.inner_mut()).map(|bytes| {
                    let bytes = bytes.to_vec();
                    let id = hash(&bytes);

                    ClipboardItem {
                        entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
                    }
                })
            } else {
                None
            }
        }
    }

    fn read_string(&self, text_bytes: &[u8]) -> ClipboardItem {
        unsafe {
            let text = String::from_utf8_lossy(text_bytes).to_string();
            let metadata = self
                .data_for_type(self.text_hash_type)
                .and_then(|hash_bytes| {
                    let hash_bytes = hash_bytes.try_into().ok()?;
                    let hash = u64::from_be_bytes(hash_bytes);
                    let metadata = self.data_for_type(self.metadata_type)?;

                    if hash == ClipboardString::text_hash(&text) {
                        String::from_utf8(metadata.to_vec()).ok()
                    } else {
                        None
                    }
                });

            ClipboardItem {
                entries: vec![ClipboardEntry::String(ClipboardString { text, metadata })],
            }
        }
    }

    fn read_url(&self, url_text_bytes: &[u8], str_text_bytes: &[u8]) -> ClipboardItem {
        unsafe {
            let url_text = String::from_utf8_lossy(url_text_bytes).to_string();
            let str_text = String::from_utf8_lossy(str_text_bytes).to_string();
            let metadata = self
                .data_for_type(self.text_hash_type)
                .and_then(|hash_bytes| {
                    let hash_bytes = hash_bytes.try_into().ok()?;
                    let hash = u64::from_be_bytes(hash_bytes);
                    let metadata = self.data_for_type(self.metadata_type)?;

                    if hash == ClipboardString::text_hash(&url_text) {
                        String::from_utf8(metadata.to_vec()).ok()
                    } else {
                        None
                    }
                });

            ClipboardItem {
                entries: vec![ClipboardEntry::URL {
                    path: ClipboardString {
                        text: url_text,
                        metadata: metadata.clone(),
                    },
                    string: ClipboardString {
                        text: str_text,
                        metadata,
                    },
                }],
            }
        }
    }

    unsafe fn data_for_type(&self, kind: id) -> Option<&[u8]> {
        unsafe {
            let data = self.inner.dataForType(kind);
            if data == nil {
                None
            } else {
                Some(slice::from_raw_parts(
                    data.bytes() as *mut u8,
                    data.length() as usize,
                ))
            }
        }
    }

    pub fn write(&self, item: ClipboardItem) {
        unsafe {
            match item.entries.as_slice() {
                [] => {
                    // Writing an empty list of entries just clears the clipboard.
                    self.inner.clearContents();
                }
                [ClipboardEntry::String(string)] => {
                    self.write_plaintext(string);
                }
                [ClipboardEntry::Image(image)] => {
                    self.write_image(image);
                }
                [ClipboardEntry::ExternalPaths(_)] => {}
                _ => {
                    // Agus NB: We're currently only writing string entries to the clipboard when we have more than one.
                    //
                    // This was the existing behavior before I refactored the outer clipboard code:
                    // https://github.com/zed-industries/zed/blob/65f7412a0265552b06ce122655369d6cc7381dd6/crates/gpui/src/platform/mac/platform.rs#L1060-L1110
                    //
                    // Note how `any_images` is always `false`. We should fix that, but that's orthogonal to the refactor.

                    let mut combined = ClipboardString {
                        text: String::new(),
                        metadata: None,
                    };

                    for entry in item.entries {
                        match entry {
                            ClipboardEntry::String(text) => {
                                combined.text.push_str(&text.text());
                                if combined.metadata.is_none() {
                                    combined.metadata = text.metadata;
                                }
                            }
                            _ => {}
                        }
                    }

                    self.write_plaintext(&combined);
                }
            }
        }
    }

    fn write_plaintext(&self, string: &ClipboardString) {
        unsafe {
            self.inner.clearContents();

            let text_bytes = NSData::dataWithBytes_length_(
                nil,
                string.text.as_ptr() as *const c_void,
                string.text.len() as u64,
            );
            self.inner
                .setData_forType(text_bytes, NSPasteboardTypeString);

            if let Some(metadata) = string.metadata.as_ref() {
                let hash_bytes = ClipboardString::text_hash(&string.text).to_be_bytes();
                let hash_bytes = NSData::dataWithBytes_length_(
                    nil,
                    hash_bytes.as_ptr() as *const c_void,
                    hash_bytes.len() as u64,
                );
                self.inner.setData_forType(hash_bytes, self.text_hash_type);

                let metadata_bytes = NSData::dataWithBytes_length_(
                    nil,
                    metadata.as_ptr() as *const c_void,
                    metadata.len() as u64,
                );
                self.inner
                    .setData_forType(metadata_bytes, self.metadata_type);
            }
        }
    }

    unsafe fn write_image(&self, image: &Image) {
        unsafe {
            self.inner.clearContents();

            let bytes = NSData::dataWithBytes_length_(
                nil,
                image.bytes.as_ptr() as *const c_void,
                image.bytes.len() as u64,
            );

            self.inner
                .setData_forType(bytes, Into::<UTType>::into(image.format).inner_mut());
        }
    }

    /// Escape a path for safe usage in the macOS Terminal / POSIX shell.
    /// This mimics what happens when you copy from Finder and paste into Terminal.
    /// Posix shell quoting rules: https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html#tag_19_02
    fn shell_escape_str(&self, shell_str: &str) -> String {
        let mut escaped = String::with_capacity(shell_str.len());

        for ch in shell_str.chars() {
            match ch {
                // Whitespace
                ' ' => escaped.push_str("\\ "),
                '\t' => escaped.push_str("\\t"),
                '\n' => escaped.push_str("\\n"),
                // Shell metacharacters
                '"' => escaped.push_str("\\\""),
                '\'' => escaped.push_str("\\'"),
                '\\' => escaped.push_str("\\\\"),
                '$' => escaped.push_str("\\$"),
                '`' => escaped.push_str("\\`"),
                '!' => escaped.push_str("\\!"),
                '#' => escaped.push_str("\\#"),
                '&' => escaped.push_str("\\&"),
                '*' => escaped.push_str("\\*"),
                '(' => escaped.push_str("\\("),
                ')' => escaped.push_str("\\)"),
                '[' => escaped.push_str("\\["),
                ']' => escaped.push_str("\\]"),
                '{' => escaped.push_str("\\{"),
                '}' => escaped.push_str("\\}"),
                '|' => escaped.push_str("\\|"),
                ';' => escaped.push_str("\\;"),
                '<' => escaped.push_str("\\<"),
                '>' => escaped.push_str("\\>"),
                '~' => escaped.push_str("\\~"),
                '?' => escaped.push_str("\\?"),
                '=' => escaped.push_str("\\="),
                '%' => escaped.push_str("\\%"),
                ',' => escaped.push_str("\\,"),
                '^' => escaped.push_str("\\^"),
                // Default: pass through unchanged
                _ => escaped.push(ch),
            }
        }
        escaped
    }
}

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnamefind?language=objc)
    pub static NSPasteboardNameFind: id;
}

impl From<ImageFormat> for UTType {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Png => Self::png(),
            ImageFormat::Jpeg => Self::jpeg(),
            ImageFormat::Tiff => Self::tiff(),
            ImageFormat::Webp => Self::webp(),
            ImageFormat::Gif => Self::gif(),
            ImageFormat::Bmp => Self::bmp(),
            ImageFormat::Svg => Self::svg(),
            ImageFormat::Ico => Self::ico(),
        }
    }
}

// See https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/
pub struct UTType(id);

impl UTType {
    pub fn png() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/png
        Self(unsafe { NSPasteboardTypePNG }) // This is a rare case where there's a built-in NSPasteboardType
    }

    pub fn jpeg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/jpeg
        Self(unsafe { ns_string("public.jpeg") })
    }

    pub fn gif() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/gif
        Self(unsafe { ns_string("com.compuserve.gif") })
    }

    pub fn webp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/webp
        Self(unsafe { ns_string("org.webmproject.webp") })
    }

    pub fn bmp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/bmp
        Self(unsafe { ns_string("com.microsoft.bmp") })
    }

    pub fn svg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/svg
        Self(unsafe { ns_string("public.svg-image") })
    }

    pub fn ico() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/ico
        Self(unsafe { ns_string("com.microsoft.ico") })
    }

    pub fn tiff() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/tiff
        Self(unsafe { NSPasteboardTypeTIFF }) // This is a rare case where there's a built-in NSPasteboardType
    }

    fn inner(&self) -> *const Object {
        self.0
    }

    pub fn inner_mut(&self) -> *mut Object {
        self.0 as *mut _
    }
}

#[cfg(test)]
mod tests {
    use cocoa::{appkit::NSPasteboardTypeString, foundation::NSData};

    use crate::{ClipboardEntry, ClipboardItem, ClipboardString};

    use super::*;

    #[test]
    fn test_string() {
        let pasteboard = Pasteboard::unique();
        assert_eq!(pasteboard.read(), None);

        let item = ClipboardItem::new_string("1".to_string());
        pasteboard.write(item.clone());
        assert_eq!(pasteboard.read(), Some(item));

        let item = ClipboardItem {
            entries: vec![ClipboardEntry::String(
                ClipboardString::new("2".to_string()).with_json_metadata(vec![3, 4]),
            )],
        };
        pasteboard.write(item.clone());
        assert_eq!(pasteboard.read(), Some(item));

        let text_from_other_app = "text from other app";
        unsafe {
            let bytes = NSData::dataWithBytes_length_(
                nil,
                text_from_other_app.as_ptr() as *const c_void,
                text_from_other_app.len() as u64,
            );
            pasteboard
                .inner
                .setData_forType(bytes, NSPasteboardTypeString);
        }
        assert_eq!(
            pasteboard.read(),
            Some(ClipboardItem::new_string(text_from_other_app.to_string()))
        );
    }
}
