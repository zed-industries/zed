use std::io::{Read, Write};
use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};

use anyhow::{bail, Result};
use filedescriptor::{FileDescriptor, Pipe};
use wayland_client::protocol::wl_data_offer::{self, WlDataOffer};
use wayland_client::protocol::wl_data_source::WlDataSource;

use super::WaylandClient;
use crate::ClipboardItem;

pub const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";

pub(crate) struct Clipboard {
    offer: Option<WlDataOffer>,
    has_text: bool,
    source: Option<WlDataSource>,
    source_content: Option<String>,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            offer: None,
            has_text: false,
            source: None,
            source_content: None,
        }
    }

    pub fn receive_offer(&mut self, offer: WlDataOffer) {
        self.offer = Some(offer);
        self.has_text = false;
    }

    pub fn receive_mime_type(&mut self, mime: &str) {
        self.has_text = self.has_text || mime == TEXT_MIME_TYPE;
    }

    pub fn send_source(&mut self, mime_type: &str, fd: OwnedFd) {
        if let Some(ref mut source_content) = self.source_content {
            let mut file = FileDescriptor::new(fd);
            let _ = file.write(source_content.as_bytes());
        }
    }

    pub fn read(&self, client: &WaylandClient) -> Option<String> {
        let Some(ref offer) = self.offer else {
            return None;
        };
        let read_pipe = setup_offer_read(offer.clone()).unwrap();

        client.flush();

        match read_pipe_with_timeout(read_pipe) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }

    pub fn write(&mut self, source: WlDataSource, item: ClipboardItem) {
        self.source = Some(source);
        self.source_content = Some(item.text);
    }
}

fn setup_offer_read(offer: wl_data_offer::WlDataOffer) -> anyhow::Result<FileDescriptor> {
    let pipe = Pipe::new().map_err(anyhow::Error::msg)?;
    offer.receive(TEXT_MIME_TYPE.to_string(), unsafe {
        BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
    });
    Ok(pipe.read)
}

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
