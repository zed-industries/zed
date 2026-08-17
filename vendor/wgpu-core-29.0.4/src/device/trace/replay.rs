use alloc::borrow::Cow;

use super::Data;

pub trait DataLoader {
    fn load<'loader, 'data>(&'loader self, data: &'data Data) -> Cow<'data, [u8]>;
    fn load_utf8<'loader, 'data>(&'loader self, data: &'data Data) -> Cow<'data, str>;
}

#[cfg(feature = "std")]
pub struct DiskTraceLoader<'a>(&'a std::path::Path);

#[cfg(feature = "std")]
impl<'a> DiskTraceLoader<'a> {
    pub fn new(path: &'a std::path::Path) -> DiskTraceLoader<'a> {
        DiskTraceLoader(path)
    }
}

impl DataLoader for DiskTraceLoader<'_> {
    fn load<'loader, 'data>(&'loader self, data: &'data Data) -> Cow<'data, [u8]> {
        match data {
            Data::File(file) => {
                Cow::from(std::fs::read(self.0.join(file)).expect("Failed to read data file"))
            }
            Data::String(_, s) => Cow::from(s.as_bytes()),
            Data::Binary(_, b) => Cow::from(b),
        }
    }

    /// Load UTF-8 string data
    ///
    /// # Panics
    ///
    /// If the data kind is not a string format or the data is not valid UTF-8
    fn load_utf8<'loader, 'data>(&'loader self, data: &'data Data) -> Cow<'data, str> {
        match data {
            Data::File(file) => Cow::from(
                std::fs::read_to_string(self.0.join(file)).expect("Failed to read data file"),
            ),
            Data::String(kind, s) => {
                assert!(kind.is_string(), "{kind:?} cannot be loaded as a string");
                Cow::from(s)
            }
            Data::Binary(kind, b) => {
                assert!(kind.is_string(), "{kind:?} cannot be loaded as a string");
                Cow::from(core::str::from_utf8(b).unwrap())
            }
        }
    }
}
