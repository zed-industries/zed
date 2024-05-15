use std::{
    io::Write,
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
};

use filedescriptor::{FileDescriptor, Pipe};
use smallvec::SmallVec;
use wayland_backend::client::ObjectId;
use wayland_client::{
    protocol::{wl_data_offer::WlDataOffer, wl_data_source::WlDataSource},
    Connection, Proxy,
};

use crate::platform::linux::platform::read_fd;

pub(crate) const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";
pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

pub(crate) struct Clipboard {
    connection: Connection,
    self_mime: String,
    contents: Option<String>,
    // External clipboard contents
    cached_read: Option<String>,
    current_offer: Option<DataOffer>,
}

#[derive(Clone)]
pub(crate) struct DataOffer {
    inner: WlDataOffer,
    mime_types: Vec<String>,
}

// At most we have to store three data offers: drag and drop, primary selection and clipboard.
const MAX_DATA_OFFERS: usize = 3;

pub(crate) struct DataOffersMap {
    inner: SmallVec<[(ObjectId, DataOffer); MAX_DATA_OFFERS]>,
}

impl DataOffer {
    pub fn new(offer: WlDataOffer) -> Self {
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
}

impl DataOffersMap {
    pub fn new() -> Self {
        Self {
            inner: SmallVec::new(),
        }
    }

    pub fn insert(&mut self, id: ObjectId, data_offer: DataOffer) {
        if self.inner.len() == MAX_DATA_OFFERS {
            self.inner.remove(0);
        }
        self.inner.push((id, data_offer));
    }

    pub fn get(&self, id: &ObjectId) -> Option<&DataOffer> {
        self.inner
            .iter()
            .find(|(key, _)| key == id)
            .map(|(_, value)| value)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut DataOffer> {
        self.inner
            .iter_mut()
            .find(|(key, _)| key == id)
            .map(|(_, value)| value)
    }
}

impl Clipboard {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            self_mime: format!("pid/{}", std::process::id()),
            contents: None,
            cached_read: None,
            current_offer: None,
        }
    }

    pub fn set_contents(&mut self, text: String) {
        self.contents = Some(text);
    }

    pub fn set_offer(&mut self, data_offer: Option<DataOffer>) {
        self.cached_read = None;
        if let Some(old_offer) = self.current_offer.take() {
            old_offer.inner.destroy();
        }
        self.current_offer = data_offer;
    }

    pub fn self_mime(&self) -> String {
        self.self_mime.clone()
    }

    pub fn handle_send(&self, _mime_type: String, fd: OwnedFd) {
        if let Some(contents) = &self.contents {
            let mut file = FileDescriptor::new(fd);
            if let Err(err) = file.write(contents.as_bytes()) {
                log::error!("error sending clipboard data: {err:?}");
            }
        }
    }

    pub fn handle_read(&mut self) -> Option<String> {
        if let Some(cached_read) = self.cached_read.clone() {
            return Some(cached_read);
        }

        let data_offer = self.current_offer.clone()?;
        if data_offer.has_mime_type(&self.self_mime) {
            return self.contents.clone();
        }
        if !data_offer.has_mime_type(TEXT_MIME_TYPE) {
            return None;
        }

        let pipe = Pipe::new().unwrap();
        data_offer
            .inner
            .receive(TEXT_MIME_TYPE.to_string(), unsafe {
                BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
            });
        let fd = pipe.read;
        drop(pipe.write);

        self.connection.flush().unwrap();

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
        result
    }
}
