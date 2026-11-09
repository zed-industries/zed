use std::ffi::CStr;
use std::path::PathBuf;

use objc2::{Message, rc::Retained};
#[expect(
    deprecated,
    reason = "Preserve the existing file-copy pasteboard format"
)]
use objc2_app_kit::NSFilenamesPboardType;
use objc2_app_kit::{
    NSPasteboard, NSPasteboardNameFind, NSPasteboardTypePNG, NSPasteboardTypeString,
    NSPasteboardTypeTIFF,
};
use objc2_foundation::{NSArray, NSData, NSString};
use smallvec::SmallVec;
use strum::IntoEnumIterator as _;
#[cfg(test)]
use uuid::Uuid;

use gpui::{
    ClipboardEntry, ClipboardItem, ClipboardString, ExternalPaths, Image, ImageFormat, hash,
};

pub struct Pasteboard {
    inner: Retained<NSPasteboard>,
    text_hash_type: Retained<NSString>,
    metadata_type: Retained<NSString>,
}

impl Pasteboard {
    pub fn general() -> Self {
        Self::new(NSPasteboard::generalPasteboard())
    }

    pub fn find() -> Self {
        Self::new(NSPasteboard::pasteboardWithName(unsafe {
            NSPasteboardNameFind
        }))
    }

    #[cfg(test)]
    pub fn unique() -> Self {
        let name = NSString::from_str(&format!("zed-test-pasteboard-{}", Uuid::new_v4()));
        Self::new(NSPasteboard::pasteboardWithName(&name))
    }

    fn new(inner: Retained<NSPasteboard>) -> Self {
        // These constructors return autoreleased objects, but a Pasteboard can
        // outlive the autorelease pool in which it was created.
        Self {
            inner,
            text_hash_type: NSString::from_str("zed-text-hash"),
            metadata_type: NSString::from_str("zed-metadata"),
        }
    }

    pub fn read(&self) -> Option<ClipboardItem> {
        unsafe {
            // Check for file paths first
            #[expect(
                deprecated,
                reason = "Preserve the existing file-copy pasteboard format"
            )]
            let filenames = self.inner.propertyListForType(NSFilenamesPboardType);
            if let Some(filenames) = filenames
                .as_ref()
                .and_then(|filenames| filenames.downcast_ref::<NSArray>())
            {
                let mut paths = SmallVec::new();
                for file in filenames {
                    if let Some(file) = file.downcast_ref::<NSString>() {
                        let path = file.UTF8String();
                        if path.is_null() {
                            continue;
                        }
                        let path = CStr::from_ptr(path).to_string_lossy().into_owned();
                        paths.push(PathBuf::from(path));
                    }
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
        let ut_type = UTType::from(format);

        let types = self.inner.types()?;
        if types.containsObject(ut_type.inner()) {
            self.data_for_type(ut_type.inner()).map(|bytes| {
                let id = hash(&bytes);

                ClipboardItem {
                    entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
                }
            })
        } else {
            None
        }
    }

    fn read_string_from_pasteboard(&self) -> Option<ClipboardEntry> {
        let pasteboard_types = self.inner.types()?;
        let string_type = NSString::from_str("public.utf8-plain-text");

        if !pasteboard_types.containsObject(&string_type) {
            return None;
        }

        let text_bytes = self.data_for_type(&string_type)?;

        let text = String::from_utf8_lossy(&text_bytes).to_string();
        let metadata = self
            .data_for_type(&self.text_hash_type)
            .and_then(|hash_bytes| {
                let hash_bytes = hash_bytes.as_slice().try_into().ok()?;
                let hash = u64::from_be_bytes(hash_bytes);
                let metadata = self.data_for_type(&self.metadata_type)?;

                if hash == ClipboardString::text_hash(&text) {
                    String::from_utf8(metadata).ok()
                } else {
                    None
                }
            });

        Some(ClipboardEntry::String(ClipboardString { text, metadata }))
    }

    fn data_for_type(&self, kind: &NSString) -> Option<Vec<u8>> {
        self.inner.dataForType(kind).map(|data| data.to_vec())
    }

    pub fn write(&self, item: ClipboardItem) {
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

    fn write_plaintext(&self, string: &ClipboardString) {
        unsafe {
            self.inner.clearContents();

            let text_bytes = NSData::with_bytes(string.text.as_bytes());
            self.inner
                .setData_forType(Some(&text_bytes), NSPasteboardTypeString);

            if let Some(metadata) = string.metadata.as_ref() {
                let hash_bytes = ClipboardString::text_hash(&string.text).to_be_bytes();
                let hash_bytes = NSData::with_bytes(&hash_bytes);
                self.inner
                    .setData_forType(Some(&hash_bytes), &self.text_hash_type);

                let metadata_bytes = NSData::with_bytes(metadata.as_bytes());
                self.inner
                    .setData_forType(Some(&metadata_bytes), &self.metadata_type);
            }
        }
    }

    fn write_image(&self, image: &Image) {
        self.inner.clearContents();

        let bytes = NSData::with_bytes(&image.bytes);

        self.inner
            .setData_forType(Some(&bytes), UTType::from(image.format).inner());
    }
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
pub struct UTType(Retained<NSString>);

impl UTType {
    pub fn png() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/png
        Self(unsafe { NSPasteboardTypePNG }.retain()) // This is a rare case where there's a built-in NSPasteboardType
    }

    pub fn jpeg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/jpeg
        Self(NSString::from_str("public.jpeg"))
    }

    pub fn gif() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/gif
        Self(NSString::from_str("com.compuserve.gif"))
    }

    pub fn webp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/webp
        Self(NSString::from_str("org.webmproject.webp"))
    }

    pub fn bmp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/bmp
        Self(NSString::from_str("com.microsoft.bmp"))
    }

    pub fn svg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/svg
        Self(NSString::from_str("public.svg-image"))
    }

    pub fn ico() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/ico
        Self(NSString::from_str("com.microsoft.ico"))
    }

    pub fn tiff() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/tiff
        Self(unsafe { NSPasteboardTypeTIFF }.retain()) // This is a rare case where there's a built-in NSPasteboardType
    }

    pub fn pnm() -> Self {
        //https://en.wikipedia.org/w/index.php?title=Netpbm&oldid=1336679433 under Uniform Type Identifier
        Self(NSString::from_str("public.pbm"))
    }

    fn inner(&self) -> &NSString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use gpui::{ClipboardEntry, ClipboardItem, ClipboardString, ImageFormat};
    use objc2::rc::autoreleasepool;

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
            let bytes = NSData::with_bytes(text_from_other_app.as_bytes());
            pasteboard
                .inner
                .setData_forType(Some(&bytes), NSPasteboardTypeString);
        }
        assert_eq!(
            pasteboard.read(),
            Some(ClipboardItem::new_string(text_from_other_app.to_string()))
        );

        for text in ["", "héllo\0世界🦀"] {
            let item = ClipboardItem::new_string(text.to_string());
            pasteboard.write(item.clone());
            assert_eq!(pasteboard.read(), Some(item));
        }
    }

    #[test]
    fn test_custom_types_survive_creation_autorelease_pool() {
        let pasteboard = autoreleasepool(|_| Pasteboard::unique());

        assert_eq!(pasteboard.text_hash_type.to_string(), "zed-text-hash");
        assert_eq!(pasteboard.metadata_type.to_string(), "zed-metadata");

        let item = ClipboardItem {
            entries: vec![ClipboardEntry::String(
                ClipboardString::new("retained pasteboard".to_string())
                    .with_json_metadata(vec![1, 2]),
            )],
        };
        autoreleasepool(|_| pasteboard.write(item.clone()));
        assert_eq!(autoreleasepool(|_| pasteboard.read()), Some(item));
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
            let types_array = NSArray::from_slice(&[ns_png_type]);
            pasteboard.inner.declareTypes_owner(&types_array, None);

            let data = NSData::with_bytes(png_bytes);
            pasteboard.inner.setData_forType(Some(&data), ns_png_type);
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

    unsafe fn simulate_external_file_copy(pasteboard: &Pasteboard, paths: &[&str]) {
        unsafe {
            let ns_paths = paths
                .iter()
                .map(|path| NSString::from_str(path))
                .collect::<Vec<_>>();
            let ns_array = NSArray::from_retained_slice(&ns_paths);

            #[expect(
                deprecated,
                reason = "Preserve the existing file-copy pasteboard format"
            )]
            let types_array = NSArray::from_slice(&[NSFilenamesPboardType, NSPasteboardTypeString]);
            pasteboard.inner.declareTypes_owner(&types_array, None);

            #[expect(
                deprecated,
                reason = "Preserve the existing file-copy pasteboard format"
            )]
            pasteboard
                .inner
                .setPropertyList_forType(&ns_array, NSFilenamesPboardType);

            let joined = paths.join("\n");
            let bytes = NSData::with_bytes(joined.as_bytes());
            pasteboard
                .inner
                .setData_forType(Some(&bytes), NSPasteboardTypeString);
        }
    }
}
