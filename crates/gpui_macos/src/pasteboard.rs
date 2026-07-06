use std::ffi::c_void;
use std::path::PathBuf;

use cocoa::{
    appkit::{
        NSFilenamesPboardType, NSPasteboard, NSPasteboardTypePNG, NSPasteboardTypeString,
        NSPasteboardTypeTIFF,
    },
    base::{id, nil},
    foundation::{NSArray, NSData, NSFastEnumeration},
};
use objc::{
    class, msg_send,
    runtime::{BOOL, Object, YES},
    sel, sel_impl,
};
use smallvec::SmallVec;
use strum::IntoEnumIterator as _;

use crate::{NSStringExt, ns_string};
use gpui::{
    ClipboardEntry, ClipboardItem, ClipboardString, ExternalPaths, Image, ImageFormat, hash,
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
        unsafe {
            // `pasteboardWithUniqueName` returns an autoreleased (+0) pasteboard.
            // Retain it so it outlives any autorelease pool active during the test.
            let inner = NSPasteboard::pasteboardWithUniqueName(nil);
            let _: id = msg_send![inner, retain];
            Self::new(inner)
        }
    }

    unsafe fn new(inner: id) -> Self {
        // `ns_string` returns autoreleased (+0) objects, but these type identifiers
        // are stored for the lifetime of the `Pasteboard`. Retain them so they
        // survive draining of any autorelease pool active when `new` is called.
        let text_hash_type = unsafe { ns_string("zed-text-hash") };
        let metadata_type = unsafe { ns_string("zed-metadata") };
        unsafe {
            let _: id = msg_send![text_hash_type, retain];
            let _: id = msg_send![metadata_type, retain];
        }
        Self {
            inner,
            text_hash_type,
            metadata_type,
        }
    }

    pub fn read(&self) -> Option<ClipboardItem> {
        unsafe {
            // Check for file paths first.
            //
            // The property list is supplied by whatever app last owned the
            // pasteboard, so it may not actually be an array of strings.
            // Messaging it as one when it isn't would raise an Objective-C
            // exception, and unwinding an ObjC exception through these Rust
            // frames is undefined behavior. Validate the classes before use and
            // skip any entries that don't conform.
            let filenames = NSPasteboard::propertyListForType(self.inner, NSFilenamesPboardType);
            let filenames_is_array = filenames != nil && {
                let is_array: BOOL = msg_send![filenames, isKindOfClass: class!(NSArray)];
                is_array == YES
            };
            if filenames_is_array && NSArray::count(filenames) > 0 {
                let mut paths = SmallVec::new();
                for file in filenames.iter() {
                    let is_string: BOOL = msg_send![file, isKindOfClass: class!(NSString)];
                    if is_string != YES {
                        continue;
                    }
                    let path = NSStringExt::to_str(&file).to_owned();
                    paths.push(PathBuf::from(path));
                }
                if !paths.is_empty() {
                    let mut entries = vec![ClipboardEntry::ExternalPaths(ExternalPaths(paths))];

                    // Also include the string representation so text editors can
                    // paste the path as text.
                    if let Some(string_item) = self.read_string_from_pasteboard() {
                        entries.push(string_item);
                    }

                    return Some(ClipboardItem { entries });
                }
            }

            // Next, check for a plain string.
            if let Some(string_entry) = self.read_string_from_pasteboard() {
                return Some(ClipboardItem {
                    entries: vec![string_entry],
                });
            }

            // Finally, try the various supported image types.
            for format in ImageFormat::iter() {
                if let Some(item) = self.read_image(format) {
                    return Some(item);
                }
            }
        }

        None
    }

    fn read_image(&self, format: ImageFormat) -> Option<ClipboardItem> {
        let ut_type: UTType = format.into();

        unsafe {
            let types: id = self.inner.types();
            if msg_send![types, containsObject: ut_type.inner()] {
                self.with_data_for_type(ut_type.inner_mut(), |bytes| {
                    let id = hash(&bytes);
                    let bytes = bytes.to_vec();

                    ClipboardItem {
                        entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
                    }
                })
            } else {
                None
            }
        }
    }

    unsafe fn read_string_from_pasteboard(&self) -> Option<ClipboardEntry> {
        unsafe {
            let pasteboard_types: id = self.inner.types();
            let string_type: id = ns_string("public.utf8-plain-text");

            if !msg_send![pasteboard_types, containsObject: string_type] {
                return None;
            }

            self.with_data_for_type(string_type, |text_bytes| {
                let text = String::from_utf8_lossy(text_bytes).into_owned();
                let metadata = self.read_metadata(&text);

                ClipboardEntry::String(ClipboardString { text, metadata })
            })
        }
    }

    /// Reads the metadata stored alongside a string entry, returning it only
    /// when the stored hash matches `text` and the metadata is valid UTF-8.
    unsafe fn read_metadata(&self, text: &str) -> Option<String> {
        let hash = unsafe {
            self.with_data_for_type(self.text_hash_type, |hash_bytes| {
                let hash_bytes = hash_bytes.try_into().ok()?;
                Some(u64::from_be_bytes(hash_bytes))
            })
        }??;

        if hash != ClipboardString::text_hash(text) {
            return None;
        }

        unsafe {
            self.with_data_for_type(self.metadata_type, |metadata| {
                String::from_utf8(metadata.to_vec()).ok()
            })
        }?
    }

    /// # Safety
    ///
    /// `kind` must be a valid pasteboard type identifier `NSString`. (`self.inner`
    /// is already guaranteed to be a valid `NSPasteboard` by `Pasteboard::new`'s
    /// contract.)
    unsafe fn with_data_for_type<R>(&self, kind: id, f: impl FnOnce(&[u8]) -> R) -> Option<R> {
        unsafe {
            let data = self.inner.dataForType(kind);
            if data == nil {
                None
            } else {
                Some(with_nsdata_bytes(data, f))
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
}

unsafe fn with_nsdata_bytes<R>(data: id, f: impl FnOnce(&[u8]) -> R) -> R {
    unsafe {
        let bytes = data.bytes();
        if bytes.is_null() {
            // https://developer.apple.com/documentation/foundation/nsdata/1410616-bytes?language=objc
            // "If the length of the NSData object is 0, this property returns nil."
            debug_assert_eq!(data.length(), 0);
            f(&[])
        } else {
            f(std::slice::from_raw_parts(
                bytes as *const u8,
                data.length() as usize,
            ))
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
            ImageFormat::Pnm => Self::pnm(),
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

    pub fn pnm() -> Self {
        //https://en.wikipedia.org/w/index.php?title=Netpbm&oldid=1336679433 under Uniform Type Identifier
        Self(unsafe { ns_string("public.pbm") })
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
    use cocoa::{
        appkit::{NSFilenamesPboardType, NSPasteboard, NSPasteboardTypeString},
        base::{id, nil},
        foundation::{NSArray, NSData},
    };
    use std::ffi::c_void;

    use gpui::{ClipboardEntry, ClipboardItem, ClipboardString, ImageFormat};

    use super::*;

    unsafe fn simulate_external_file_copy(pasteboard: &Pasteboard, paths: &[&str]) {
        unsafe {
            let ns_paths: Vec<id> = paths.iter().map(|p| ns_string(p)).collect();
            let ns_array = NSArray::arrayWithObjects(nil, &ns_paths);

            let mut types = vec![NSFilenamesPboardType];
            types.push(NSPasteboardTypeString);

            let types_array = NSArray::arrayWithObjects(nil, &types);
            pasteboard.inner.declareTypes_owner(types_array, nil);

            pasteboard
                .inner
                .setPropertyList_forType(ns_array, NSFilenamesPboardType);

            let joined = paths.join("\n");
            let bytes = NSData::dataWithBytes_length_(
                nil,
                joined.as_ptr() as *const c_void,
                joined.len() as u64,
            );
            pasteboard
                .inner
                .setData_forType(bytes, NSPasteboardTypeString);
        }
    }

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

    #[test]
    fn test_read_external_path() {
        let pasteboard = Pasteboard::unique();

        unsafe {
            simulate_external_file_copy(&pasteboard, &["/test.txt"]);
        }

        let item = pasteboard.read().expect("should read clipboard item");

        // Test both ExternalPaths and String entries exist
        assert_eq!(item.entries.len(), 2);

        // Test first entry is ExternalPaths
        match &item.entries[0] {
            ClipboardEntry::ExternalPaths(ep) => {
                assert_eq!(ep.paths(), &[PathBuf::from("/test.txt")]);
            }
            other => panic!("expected ExternalPaths, got {:?}", other),
        }

        // Test second entry is String
        match &item.entries[1] {
            ClipboardEntry::String(s) => {
                assert_eq!(s.text(), "/test.txt");
            }
            other => panic!("expected String, got {:?}", other),
        }
    }

    #[test]
    fn test_read_external_paths_with_spaces() {
        let pasteboard = Pasteboard::unique();
        let paths = ["/some file with spaces.txt"];

        unsafe {
            simulate_external_file_copy(&pasteboard, &paths);
        }

        let item = pasteboard.read().expect("should read clipboard item");

        match &item.entries[0] {
            ClipboardEntry::ExternalPaths(ep) => {
                assert_eq!(ep.paths(), &[PathBuf::from("/some file with spaces.txt")]);
            }
            other => panic!("expected ExternalPaths, got {:?}", other),
        }
    }

    #[test]
    fn test_read_multiple_external_paths() {
        let pasteboard = Pasteboard::unique();
        let paths = ["/file.txt", "/image.png"];

        unsafe {
            simulate_external_file_copy(&pasteboard, &paths);
        }

        let item = pasteboard.read().expect("should read clipboard item");
        assert_eq!(item.entries.len(), 2);

        // Test both ExternalPaths and String entries exist
        match &item.entries[0] {
            ClipboardEntry::ExternalPaths(ep) => {
                assert_eq!(
                    ep.paths(),
                    &[PathBuf::from("/file.txt"), PathBuf::from("/image.png"),]
                );
            }
            other => panic!("expected ExternalPaths, got {:?}", other),
        }

        match &item.entries[1] {
            ClipboardEntry::String(s) => {
                assert_eq!(s.text(), "/file.txt\n/image.png");
                assert_eq!(s.metadata, None);
            }
            other => panic!("expected String, got {:?}", other),
        }
    }

    #[test]
    fn test_read_image() {
        let pasteboard = Pasteboard::unique();

        // Smallest valid PNG: 1x1 transparent pixel
        let png_bytes: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x62, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xE5, 0x27, 0xDE, 0xFC, 0x00, 0x00,
            0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        unsafe {
            let ns_png_type = NSPasteboardTypePNG;
            let types_array = NSArray::arrayWithObjects(nil, &[ns_png_type]);
            pasteboard.inner.declareTypes_owner(types_array, nil);

            let data = NSData::dataWithBytes_length_(
                nil,
                png_bytes.as_ptr() as *const c_void,
                png_bytes.len() as u64,
            );
            pasteboard.inner.setData_forType(data, ns_png_type);
        }

        let item = pasteboard.read().expect("should read PNG image");

        // Test Image entry exists
        assert_eq!(item.entries.len(), 1);
        match &item.entries[0] {
            ClipboardEntry::Image(img) => {
                assert_eq!(img.format, ImageFormat::Png);
                assert_eq!(img.bytes, png_bytes);
            }
            other => panic!("expected Image, got {:?}", other),
        }
    }
}
