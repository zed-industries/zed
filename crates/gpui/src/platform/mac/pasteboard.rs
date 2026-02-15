use core::slice;
use std::ffi::c_void;

use cocoa::{
    appkit::{NSPasteboard, NSPasteboardTypePNG, NSPasteboardTypeString, NSPasteboardTypeTIFF},
    base::{id, nil},
    foundation::NSData,
};
use objc::{msg_send, runtime::Object, sel, sel_impl};
use strum::IntoEnumIterator as _;

use crate::{
    ClipboardEntry, ClipboardItem, ClipboardString, Image, ImageFormat, asset_cache::hash,
    platform::mac::ns_string,
};

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

            if msg_send![pasteboard_types, containsObject: string_type] {
                let data = self.inner.dataForType(string_type);
                if data != nil && !data.bytes().is_null() {
                    let bytes =
                        slice::from_raw_parts(data.bytes() as *mut u8, data.length() as usize);
                    return Some(self.read_string(bytes));
                }
            }

            let html_type: id = ns_string("public.html");
            if msg_send![pasteboard_types, containsObject: html_type] {
                let data = self.inner.dataForType(html_type);
                if data != nil && !data.bytes().is_null() {
                    let bytes =
                        slice::from_raw_parts(data.bytes() as *mut u8, data.length() as usize);
                    let html = String::from_utf8_lossy(bytes).to_string();
                    return Some(ClipboardItem {
                        entries: vec![ClipboardEntry::Html(html)],
                    });
                }
            }

            // If it wasn't a string, try the various supported image types.
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
            self.inner.clearContents();
            for entry in item.entries {
                match entry {
                    ClipboardEntry::String(string) => {
                        self.write_plaintext_inner(&string);
                    }
                    ClipboardEntry::Html(html) => {
                        self.write_html_inner(&html);
                    }
                    ClipboardEntry::Image(image) => {
                        self.write_image_inner(&image);
                    }
                    ClipboardEntry::ExternalPaths(_) => {}
                }
            }
        }
    }

    fn write_plaintext_inner(&self, string: &ClipboardString) {
        unsafe {
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

    fn write_html_inner(&self, html: &str) {
        unsafe {
            let html_bytes = NSData::dataWithBytes_length_(
                nil,
                html.as_ptr() as *const c_void,
                html.len() as u64,
            );
            self.inner
                .setData_forType(html_bytes, ns_string("public.html"));
        }
    }

    unsafe fn write_image_inner(&self, image: &Image) {
        unsafe {
            let bytes = NSData::dataWithBytes_length_(
                nil,
                image.bytes.as_ptr() as *const c_void,
                image.bytes.len() as u64,
            );

            self.inner
                .setData_forType(bytes, Into::<UTType>::into(image.format).inner_mut());
        }
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
