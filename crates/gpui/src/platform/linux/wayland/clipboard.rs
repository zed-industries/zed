use std::io::{Read, Write};
use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};

use anyhow::{bail, Result};
use filedescriptor::{FileDescriptor, Pipe};
use wayland_backend::client::ObjectId;
use wayland_client::protocol::wl_data_offer::WlDataOffer;
use wayland_client::protocol::wl_data_source::WlDataSource;
use wayland_client::Proxy;

use super::WaylandClient;
use crate::ClipboardItem;

pub const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";

pub(crate) struct Clipboard {
    offer: Option<DataOffer>,
    offer_has_text: bool,
    offer_is_self: bool,
    source: Option<WlDataSource>,
    source_content: Option<String>,
    self_mime: String,
}

#[derive(Clone)]
pub(crate) struct DataOffer {
    instance: WlDataOffer,
    mime_types: Vec<String>,
}

impl DataOffer {
    pub fn new(offer: WlDataOffer) -> Self {
        Self {
            instance: offer,
            mime_types: vec![],
        }
    }

    pub fn id(&self) -> ObjectId {
        self.instance.id()
    }

    pub fn add_mime_type(&mut self, mime: String) {
        self.mime_types.push(mime);
    }
}

fn contains_mime_type(offer: &Option<DataOffer>, mime_type: &str) -> bool {
    match offer {
        Some(ref offer) => offer.mime_types.iter().any(|m| m == mime_type),
        _ => false,
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            offer: None,
            offer_has_text: false,
            offer_is_self: false,
            source: None,
            source_content: None,
            self_mime: format!("pid/{}", std::process::id()),
        }
    }

    pub fn set_offer(&mut self, offer: Option<DataOffer>) {
        self.offer = offer;

        // If the current offer is a self-offer, we want to keep its content around
        // event if wayland asks us to destroy the object.
        let keep_offer_content = self.offer.is_none() && self.offer_is_self;
        if !keep_offer_content {
            self.offer_has_text = contains_mime_type(&self.offer, TEXT_MIME_TYPE);
            self.offer_is_self = contains_mime_type(&self.offer, &self.self_mime);
        }
    }

    pub fn read(&self, client: &WaylandClient) -> Option<String> {
        if self.offer_is_self {
            return self.source_content.clone();
        }
        let Some(ref offer) = self.offer else {
            return None;
        };

        let read_pipe = match setup_offer_read(&offer.instance) {
            Ok(read_pipe) => read_pipe,
            Err(e) => {
                log::error!("could not read clipboard: {}", e);
                return None;
            }
        };

        client.flush();

        match read_pipe_with_timeout(read_pipe) {
            Ok(result) => Some(result),
            Err(e) => {
                log::error!("could not read clipboard: {}", e);
                None
            }
        }
    }

    pub fn prepare_source(&mut self, source: &WlDataSource) {
        source.offer(TEXT_MIME_TYPE.to_owned());
        source.offer(self.self_mime.clone());
    }

    pub fn set_source(&mut self, source: WlDataSource, item: ClipboardItem) {
        self.source = Some(source);
        self.source_content = Some(item.text);
    }

    pub fn send_source(&mut self, mime_type: &str, fd: OwnedFd) {
        if let Some(ref mut source_content) = self.source_content {
            let mut file = FileDescriptor::new(fd);
            if let Err(e) = file.write(source_content.as_bytes()) {
                log::error!("could not write clipboard: {}", e);
            };
        }
    }
}

fn setup_offer_read(offer: &WlDataOffer) -> anyhow::Result<FileDescriptor> {
    let pipe = Pipe::new().map_err(anyhow::Error::msg)?;
    offer.receive(TEXT_MIME_TYPE.to_string(), unsafe {
        BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
    });
    Ok(pipe.read)
}

// sourced from https://github.com/wez/wezterm/blob/95581d8697f3749f84ccb1402ac94ea6582b227f/window/src/os/wayland/window.rs#L468
fn read_pipe_with_timeout(mut file: FileDescriptor) -> Result<String> {
    let mut result = Vec::new();

    file.set_non_blocking(true)?;

    let mut pfd = libc::pollfd {
        fd: file.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    let mut buf = [0u8; 8192];

    loop {
        if unsafe { libc::poll(&mut pfd, 1, 1000) == 1 } {
            match file.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(size) => {
                    result.extend_from_slice(&buf[..size]);
                }
                Err(e) => bail!("error reading from pipe: {}", e),
            }
        } else {
            bail!("timed out reading from pipe");
        }
    }

    let result = String::from_utf8(result)?;

    // Normalize the text to unix line endings, otherwise
    // copying from eg: firefox inserts a lot of blank
    // lines, and that is super annoying.
    let result = result.replace("\r\n", "\n");

    Ok(result)
}
