#![allow(clippy::useless_conversion)]

use crate::fs::MetadataExt;
use std::{fs, io};

#[derive(Debug, Clone)]
pub(crate) struct ImplMetadataExt {
    file_attributes: u32,
    creation_time: u64,
    last_access_time: u64,
    last_write_time: u64,
    file_size: u64,
    volume_serial_number: Option<u32>,
    number_of_links: Option<u32>,
    file_index: Option<u64>,
}

impl ImplMetadataExt {
    /// Constructs a new instance of `Self` from the given [`std::fs::File`]
    /// and [`std::fs::Metadata`].
    #[inline]
    #[allow(unused_variables)]
    pub(crate) fn from(file: &fs::File, std: &fs::Metadata) -> io::Result<Self> {
        let (mut volume_serial_number, mut number_of_links, mut file_index) = (None, None, None);

        #[cfg(windows_by_handle)]
        {
            use std::os::windows::fs::MetadataExt;
            if let Some(some) = std.volume_serial_number() {
                volume_serial_number = Some(some);
            }
            if let Some(some) = std.number_of_links() {
                number_of_links = Some(some);
            }
            if let Some(some) = std.file_index() {
                file_index = Some(some);
            }
        }

        #[cfg(not(windows_by_handle))]
        if volume_serial_number.is_none() || number_of_links.is_none() || file_index.is_none() {
            let fileinfo = winx::winapi_util::file::information(file)?;
            if volume_serial_number.is_none() {
                let t64: u64 = fileinfo.volume_serial_number();
                let t32: u32 = t64.try_into().unwrap();
                volume_serial_number = Some(t32);
            }
            if number_of_links.is_none() {
                let t64: u64 = fileinfo.number_of_links();
                let t32: u32 = t64.try_into().unwrap();
                number_of_links = Some(t32);
            }
            if file_index.is_none() {
                file_index = Some(fileinfo.file_index());
            }
        }

        Ok(Self::from_parts(
            std,
            volume_serial_number,
            number_of_links,
            file_index,
        ))
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::Metadata`].
    ///
    /// As with the comments in [`std::fs::Metadata::volume_serial_number`] and
    /// nearby functions, some fields of the resulting metadata will be `None`.
    ///
    /// [`std::fs::Metadata::volume_serial_number`]: https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.volume_serial_number
    #[inline]
    #[allow(unused_mut)]
    pub(crate) fn from_just_metadata(std: &fs::Metadata) -> Self {
        let (mut volume_serial_number, mut number_of_links, mut file_index) = (None, None, None);

        #[cfg(windows_by_handle)]
        {
            use std::os::windows::fs::MetadataExt;
            if let Some(some) = std.volume_serial_number() {
                volume_serial_number = Some(some);
            }
            if let Some(some) = std.number_of_links() {
                number_of_links = Some(some);
            }
            if let Some(some) = std.file_index() {
                file_index = Some(some);
            }
        }

        Self::from_parts(std, volume_serial_number, number_of_links, file_index)
    }

    #[inline]
    fn from_parts(
        std: &fs::Metadata,
        volume_serial_number: Option<u32>,
        number_of_links: Option<u32>,
        file_index: Option<u64>,
    ) -> Self {
        use std::os::windows::fs::MetadataExt;
        Self {
            file_attributes: std.file_attributes(),
            creation_time: std.creation_time(),
            last_access_time: std.last_access_time(),
            last_write_time: std.last_write_time(),
            file_size: std.file_size(),
            volume_serial_number,
            number_of_links,
            file_index,
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same
    /// device.
    pub(crate) fn is_same_file(&self, other: &Self) -> bool {
        // From [MSDN]:
        // The identifier (low and high parts) and the volume serial number
        // uniquely identify a file on a single computer. To determine whether
        // two open handles represent the same file, combine the identifier
        // and the volume serial number for each file and compare them.
        // [MSDN]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information
        let self_vsn = self
            .volume_serial_number
            .expect("could extract volume serial number of `self`");
        let other_vsn = other
            .volume_serial_number
            .expect("could extract volume serial number of `other`");
        let self_file_index = self.file_index.expect("could extract file index `self`");
        let other_file_index = other.file_index.expect("could extract file index `other`");
        self_vsn == other_vsn && self_file_index == other_file_index
    }

    /// `MetadataExt` requires nightly to be implemented, but we sometimes
    /// just need the file attributes.
    #[inline]
    pub(crate) fn file_attributes(&self) -> u32 {
        self.file_attributes
    }
}

impl MetadataExt for ImplMetadataExt {
    fn file_attributes(&self) -> u32 {
        self.file_attributes
    }

    #[inline]
    fn creation_time(&self) -> u64 {
        self.creation_time
    }

    #[inline]
    fn last_access_time(&self) -> u64 {
        self.last_access_time
    }

    #[inline]
    fn last_write_time(&self) -> u64 {
        self.last_write_time
    }

    #[inline]
    fn file_size(&self) -> u64 {
        self.file_size
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn volume_serial_number(&self) -> Option<u32> {
        self.volume_serial_number
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn number_of_links(&self) -> Option<u32> {
        self.number_of_links
    }

    #[inline]
    #[cfg(windows_by_handle)]
    fn file_index(&self) -> Option<u64> {
        self.file_index
    }
}

#[doc(hidden)]
impl crate::fs::_WindowsByHandle for crate::fs::Metadata {
    #[inline]
    fn file_attributes(&self) -> u32 {
        self.ext.file_attributes
    }

    #[inline]
    fn volume_serial_number(&self) -> Option<u32> {
        self.ext.volume_serial_number
    }

    #[inline]
    fn number_of_links(&self) -> Option<u32> {
        self.ext.number_of_links
    }

    #[inline]
    fn file_index(&self) -> Option<u64> {
        self.ext.file_index
    }
}
