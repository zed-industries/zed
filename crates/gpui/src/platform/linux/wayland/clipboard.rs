use std::sync::Arc;
use std::io::Read;
use std::os::fd::{AsRawFd, BorrowedFd};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::cell::RefCell;

use anyhow::{Result, bail};
use parking_lot::Mutex;
use filedescriptor::{Pipe, FileDescriptor};
use wayland_client::protocol::wl_data_offer::{self, WlDataOffer};

const TEXT_MIME_TYPE: &str = "text/plain;charset=utf-8";

pub(crate) struct Clipboard {
    offer: Option<WlDataOffer>,
    has_text: bool,
}

pub(crate) struct ClipboardRead {
    offer: WlDataOffer,
    did_start: RefCell<bool>,
    result: Arc<Mutex<Option<Result<String>>>>,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            offer: None,
            has_text: false,
        }
    }

    pub fn receive_offer(self: &mut Self, offer: WlDataOffer) {
        self.offer = Some(offer);
        self.has_text = false;
    }

    pub fn receive_mime_type(self: &mut Self, mime: &str) {
        self.has_text = self.has_text || mime == TEXT_MIME_TYPE;
    }

    pub fn read(self: &Self) -> Result<ClipboardRead> {
        let Some(ref offer) = self.offer else { bail!("no clipboard data available"); };
        Ok(ClipboardRead::new(offer.clone()))
    }

    pub fn read_sync(self: &Self) -> Option<String> {
        let Some(ref offer) = self.offer else { return None; };
        let read_pipe = setup_offer_read(offer.clone()).unwrap();

        match read_pipe_with_timeout(read_pipe) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

impl ClipboardRead {
    pub fn new(offer: WlDataOffer) -> Self {
        Self {
            offer,
            did_start: RefCell::new(false),
            result: Arc::new(Mutex::new(None))
        }
    }
}

impl Future for ClipboardRead {
    type Output = Result<String, &'static str>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut did_start = self.did_start.borrow_mut();
        if !*did_start {
            *did_start = true;

            let read_pipe = setup_offer_read(self.offer.clone()).unwrap();
            let result_cell = self.result.clone();
            let waker = cx.waker().clone();

            std::thread::spawn(move || {
                match read_pipe_with_timeout(read_pipe) {
                    Ok(result) => {
                        let mut result_cell = result_cell.lock();
                        *result_cell = Some(Ok(result));
                    }
                    Err(e) => {
                        log::error!("while reading clipboard: {}", e);
                        let mut result_cell = result_cell.lock();
                        *result_cell = Some(Err(e));
                    }
                };
                waker.wake();
            });
        }

        match &mut *self.result.lock() {
            None => Poll::Pending,
            Some(result) => match result {
                Ok(result) => Poll::Ready(Ok(result.clone())),
                Err(e) => Poll::Ready(Err("failed to read clipboard")),
            },
        }
    }
}

pub(crate) fn setup_offer_read(offer: wl_data_offer::WlDataOffer) -> anyhow::Result<FileDescriptor> {
    let pipe = Pipe::new().map_err(anyhow::Error::msg)?;
    offer.receive(TEXT_MIME_TYPE.to_string(), unsafe { BorrowedFd::borrow_raw(pipe.write.as_raw_fd()) });
    Ok(pipe.read)
}

pub(crate) fn read_pipe_with_timeout(mut file: FileDescriptor) -> Result<String> {
    let mut result = Vec::new();

    file.set_non_blocking(true)?;
    let mut pfd = libc::pollfd {
        fd: file.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    let mut buf = [0u8; 8192];

    loop {
        if unsafe { libc::poll(&mut pfd, 1, 3000) == 1 } {
            match file.read(&mut buf) {
                Ok(size) if size == 0 => {
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
