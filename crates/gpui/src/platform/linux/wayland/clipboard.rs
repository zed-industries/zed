use std::{
    io::Write,
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
};

use filedescriptor::{FileDescriptor, Pipe};
use wayland_client::{
    protocol::{wl_data_offer::WlDataOffer, wl_data_source::WlDataSource},
    Connection,
};

use crate::platform::linux::platform::read_fd;

pub(crate) const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";
pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

pub(crate) struct Clipboard {
    pending_write: Option<String>,
    cached_read: Option<String>,
    pending_read: Option<WlDataOffer>,
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        if let Some(pending_read) = &self.pending_read {
            pending_read.destroy();
        }
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            pending_write: None,
            cached_read: None,
            pending_read: None,
        }
    }

    pub fn set_pending_write(&mut self, text: String) {
        self.pending_write = Some(text);
    }

    pub fn set_pending_read(&mut self, data_offer: Option<WlDataOffer>) {
        self.cached_read = None;
        self.pending_read = data_offer;
    }

    pub fn handle_send(&self, _mime_type: String, fd: OwnedFd) {
        if let Some(pending_write) = &self.pending_write {
            let mut file = FileDescriptor::new(fd);
            if let Err(err) = file.write(pending_write.as_bytes()) {
                log::error!("error sending clipboard data: {err:?}");
            }
        }
    }

    pub fn handle_read(&mut self) -> Option<String> {
        if let Some(cached_read) = self.cached_read.clone() {
            return Some(cached_read);
        }

        let data_offer = self.pending_read.take()?;
        let pipe = Pipe::new().unwrap();
        data_offer.receive(TEXT_MIME_TYPE.to_string(), unsafe {
            BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
        });
        let fd = pipe.read;
        drop(pipe.write);

        let result = match unsafe { read_fd(fd) } {
            Ok(v) => {
                self.cached_read = Some(v.clone());
                Some(v)
            }
            Err(err) => {
                log::error!("error reading clipboard pipe: {err:?}");
                None
            }
        };

        println!("{result:?}");
        data_offer.destroy();
        result
    }
}
