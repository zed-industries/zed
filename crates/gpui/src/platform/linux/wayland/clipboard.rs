use std::{
    io::Write,
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
};

use filedescriptor::{FileDescriptor, Pipe};
use wayland_client::{protocol::wl_data_offer::WlDataOffer, Connection};
use wayland_protocols::wp::primary_selection::zv1::client::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;

use crate::platform::linux::platform::read_fd;

pub(crate) const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";
pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

pub(crate) const ALLOWED_TEXT_MIME_TYPES: [&str; 2] = ["text/plain;charset=utf-8", "UTF8_STRING"];

pub(crate) struct Clipboard {
    connection: Connection,
    self_mime: String,

    // Internal clipboard
    contents: Option<String>,
    primary_contents: Option<String>,

    // External clipboard
    cached_read: Option<String>,
    current_offer: Option<DataOffer<WlDataOffer>>,
    cached_primary_read: Option<String>,
    current_primary_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>,
}

#[derive(Clone, Debug)]
/// Wrapper for `WlDataOffer` and `ZwpPrimarySelectionOfferV1`, used to help track offered mime types.
pub(crate) struct DataOffer<T> {
    pub inner: T,
    mime_types: Vec<String>,
}

impl<T> DataOffer<T> {
    pub fn new(offer: T) -> Self {
        Self {
            inner: offer,
            mime_types: Vec::new(),
        }
    }

    pub fn add_mime_type(&mut self, mime_type: String) {
        self.mime_types.push(mime_type)
    }

    pub fn has_mime_type(&self, mime_type: &str) -> bool {
        self.mime_types.iter().any(|t| t == mime_type)
    }

    pub fn find_text_mime_type(&self) -> Option<String> {
        for offered_mime_type in &self.mime_types {
            if let Some(offer_text_mime_type) = ALLOWED_TEXT_MIME_TYPES
                .into_iter()
                .find(|text_mime_type| text_mime_type == offered_mime_type)
            {
                return Some(offer_text_mime_type.to_owned());
            }
        }
        None
    }
}

impl Clipboard {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            self_mime: format!("pid/{}", std::process::id()),

            contents: None,
            primary_contents: None,

            cached_read: None,
            current_offer: None,
            cached_primary_read: None,
            current_primary_offer: None,
        }
    }

    pub fn set(&mut self, text: String) {
        self.contents = Some(text);
    }

    pub fn set_primary(&mut self, text: String) {
        self.primary_contents = Some(text);
    }

    pub fn set_offer(&mut self, data_offer: Option<DataOffer<WlDataOffer>>) {
        self.cached_read = None;
        self.current_offer = data_offer;
    }

    pub fn set_primary_offer(&mut self, data_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>) {
        self.cached_primary_read = None;
        self.current_primary_offer = data_offer;
    }

    pub fn self_mime(&self) -> String {
        self.self_mime.clone()
    }

    pub fn send(&self, _mime_type: String, fd: OwnedFd) {
        if let Some(contents) = &self.contents {
            let mut file = FileDescriptor::new(fd);
            if let Err(err) = file.write(contents.as_bytes()) {
                log::error!("error sending clipboard data: {err:?}");
            }
        }
    }

    pub fn send_primary(&self, _mime_type: String, fd: OwnedFd) {
        if let Some(primary_contents) = &self.primary_contents {
            let mut file = FileDescriptor::new(fd);
            if let Err(err) = file.write(primary_contents.as_bytes()) {
                log::error!("error sending clipboard data: {err:?}");
            }
        }
    }

    pub fn read(&mut self) -> Option<String> {
        let offer = self.current_offer.clone()?;
        if let Some(cached) = self.cached_read.clone() {
            return Some(cached);
        }

        if offer.has_mime_type(&self.self_mime) {
            return self.contents.clone();
        }

        let mime_type = offer.find_text_mime_type()?;
        let pipe = Pipe::new().unwrap();
        offer.inner.receive(mime_type, unsafe {
            BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
        });
        let fd = pipe.read;
        drop(pipe.write);

        self.connection.flush().unwrap();

        match unsafe { read_fd(fd) } {
            Ok(v) => {
                self.cached_read = Some(v.clone());
                Some(v)
            }
            Err(err) => {
                log::error!("error reading clipboard pipe: {err:?}");
                None
            }
        }
    }

    pub fn read_primary(&mut self) -> Option<String> {
        let offer = self.current_primary_offer.clone()?;
        if let Some(cached) = self.cached_primary_read.clone() {
            return Some(cached);
        }

        if offer.has_mime_type(&self.self_mime) {
            return self.primary_contents.clone();
        }

        let mime_type = offer.find_text_mime_type()?;
        let pipe = Pipe::new().unwrap();
        offer.inner.receive(mime_type, unsafe {
            BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
        });
        let fd = pipe.read;
        drop(pipe.write);

        self.connection.flush().unwrap();

        match unsafe { read_fd(fd) } {
            Ok(v) => {
                self.cached_primary_read = Some(v.clone());
                Some(v)
            }
            Err(err) => {
                log::error!("error reading clipboard pipe: {err:?}");
                None
            }
        }
    }
}
