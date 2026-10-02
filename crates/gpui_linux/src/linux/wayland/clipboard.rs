use std::{
    fs::File,
    io::{ErrorKind, Write},
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
};

use calloop::{LoopHandle, PostAction};
use filedescriptor::Pipe;
use strum::IntoEnumIterator;
use wayland_client::{Connection, protocol::wl_data_offer::WlDataOffer};
use wayland_protocols::wp::primary_selection::zv1::client::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;

use gpui_util::ResultExt as _;
use http_client::Url;
use smallvec::SmallVec;

use crate::linux::{
    WaylandClientStatePtr,
    platform::{PIPE_READ_TIMEOUT, read_fd_with_timeout},
};
use gpui::{ClipboardEntry, ClipboardItem, ExternalPaths, Image, ImageFormat, hash};

/// Text mime types that we'll offer to other programs.
pub(crate) const TEXT_MIME_TYPES: [&str; 3] =
    ["text/plain;charset=utf-8", "UTF8_STRING", "text/plain"];
pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

/// Text mime types that we'll accept from other programs.
pub(crate) const ALLOWED_TEXT_MIME_TYPES: [&str; 2] = ["text/plain;charset=utf-8", "UTF8_STRING"];

#[derive(Default)]
struct ClipboardOwnership {
    is_self_owner: bool,
}

impl ClipboardOwnership {
    fn selection_requested(&mut self) {
        self.is_self_owner = true;
    }

    fn external_offer_received(&mut self) {
        self.is_self_owner = false;
    }
}

fn read_file_or_text<T>(
    read_uri_list: impl FnOnce() -> Option<T>,
    read_text: impl FnOnce() -> Option<T>,
) -> Option<T> {
    read_uri_list().or_else(read_text)
}

pub(crate) struct Clipboard {
    connection: Connection,
    loop_handle: LoopHandle<'static, WaylandClientStatePtr>,
    self_mime: String,

    // Internal clipboard
    contents: Option<ClipboardItem>,
    primary_contents: Option<ClipboardItem>,
    // True when this process last set the clipboard selection. Most Wayland compositors do not
    // send a wl_data_device.selection event back to the client that called set_selection, so we
    // track ownership ourselves to avoid needing current_offer to be populated for same-process
    // cross-window reads.
    ownership: ClipboardOwnership,

    // External clipboard
    cached_read: Option<ClipboardItem>,
    current_offer: Option<DataOffer<WlDataOffer>>,
    cached_primary_read: Option<ClipboardItem>,
    current_primary_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>,
}

pub(crate) trait ReceiveData {
    fn receive_data(&self, mime_type: String, fd: BorrowedFd<'_>);
}

impl ReceiveData for WlDataOffer {
    fn receive_data(&self, mime_type: String, fd: BorrowedFd<'_>) {
        self.receive(mime_type, fd);
    }
}

impl ReceiveData for ZwpPrimarySelectionOfferV1 {
    fn receive_data(&self, mime_type: String, fd: BorrowedFd<'_>) {
        self.receive(mime_type, fd);
    }
}

#[derive(Clone, Debug)]
/// Wrapper for `WlDataOffer` and `ZwpPrimarySelectionOfferV1`, used to help track mime types.
pub(crate) struct DataOffer<T: ReceiveData> {
    pub inner: T,
    mime_types: Vec<String>,
}

impl<T: ReceiveData> DataOffer<T> {
    pub fn new(offer: T) -> Self {
        Self {
            inner: offer,
            mime_types: Vec::new(),
        }
    }

    pub fn add_mime_type(&mut self, mime_type: String) {
        self.mime_types.push(mime_type)
    }

    fn has_mime_type(&self, mime_type: &str) -> bool {
        self.mime_types.iter().any(|t| t == mime_type)
    }

    fn read_bytes(&self, connection: &Connection, mime_type: &str) -> Option<Vec<u8>> {
        let pipe = Pipe::new().unwrap();
        self.inner.receive_data(mime_type.to_string(), unsafe {
            BorrowedFd::borrow_raw(pipe.write.as_raw_fd())
        });
        let fd = pipe.read;
        drop(pipe.write);

        connection.flush().unwrap();

        match read_fd_with_timeout(fd, PIPE_READ_TIMEOUT) {
            Ok(bytes) => Some(bytes),
            Err(err) => {
                log::error!("error reading clipboard pipe: {err:?}");
                None
            }
        }
    }

    fn read_text(&self, connection: &Connection) -> Option<ClipboardItem> {
        let mime_type = self.mime_types.iter().find(|&mime_type| {
            ALLOWED_TEXT_MIME_TYPES
                .iter()
                .any(|&allowed| allowed == mime_type)
        })?;
        let bytes = self.read_bytes(connection, mime_type)?;
        let text_content = match String::from_utf8(bytes) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to convert clipboard content to UTF-8: {}", e);
                return None;
            }
        };

        // Normalize the text to unix line endings, otherwise
        // copying from eg: firefox inserts a lot of blank
        // lines, and that is super annoying.
        let result = text_content.replace("\r\n", "\n");
        Some(ClipboardItem::new_string(result))
    }

    fn read_uri_list(&self, connection: &Connection) -> Option<ClipboardItem> {
        if !self.has_mime_type(FILE_LIST_MIME_TYPE) {
            return None;
        }
        let bytes = self.read_bytes(connection, FILE_LIST_MIME_TYPE)?;
        let text = String::from_utf8(bytes).ok()?;
        let paths: SmallVec<[_; 2]> = text
            .lines()
            .map(|line| line.trim_end_matches('\r'))
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .filter_map(|line| Url::parse(line).log_err())
            .filter_map(|url| {
                url.to_file_path()
                    .map_err(|_| log::error!("Failed to convert {url:?} into a file path"))
                    .ok()
            })
            .collect();
        if paths.is_empty() {
            return None;
        }
        Some(ClipboardItem {
            entries: vec![ClipboardEntry::ExternalPaths(ExternalPaths(paths))],
        })
    }

    fn read_image(&self, connection: &Connection) -> Option<ClipboardItem> {
        for format in ImageFormat::iter() {
            let mime_type = format.mime_type();
            if !self.has_mime_type(mime_type) {
                continue;
            }

            if let Some(bytes) = self.read_bytes(connection, mime_type) {
                let id = hash(&bytes);
                return Some(ClipboardItem {
                    entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
                });
            }
        }
        None
    }
}

impl Clipboard {
    pub fn new(
        connection: Connection,
        loop_handle: LoopHandle<'static, WaylandClientStatePtr>,
    ) -> Self {
        Self {
            connection,
            loop_handle,
            self_mime: format!("pid/{}", std::process::id()),

            contents: None,
            primary_contents: None,
            ownership: ClipboardOwnership::default(),

            cached_read: None,
            current_offer: None,
            cached_primary_read: None,
            current_primary_offer: None,
        }
    }

    pub fn set(&mut self, item: ClipboardItem) {
        self.contents = Some(item);
    }

    pub fn selection_requested(&mut self) {
        self.ownership.selection_requested();
    }

    pub fn set_primary(&mut self, item: ClipboardItem) {
        self.primary_contents = Some(item);
    }

    pub fn set_offer(&mut self, data_offer: Option<DataOffer<WlDataOffer>>) {
        self.cached_read = None;
        self.current_offer = data_offer;
        self.ownership.external_offer_received();
    }

    pub fn set_primary_offer(&mut self, data_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>) {
        self.cached_primary_read = None;
        self.current_primary_offer = data_offer;
    }

    pub fn self_mime(&self) -> String {
        self.self_mime.clone()
    }

    pub fn send(&self, mime_type: String, fd: OwnedFd) {
        let Some(contents) = self.contents.as_ref() else {
            return;
        };
        if mime_type == FILE_LIST_MIME_TYPE {
            for entry in contents.entries() {
                if let ClipboardEntry::ExternalPaths(paths) = entry {
                    let uri_list = paths
                        .paths()
                        .iter()
                        .filter_map(|path| Url::from_file_path(path).ok())
                        .map(|url| url.to_string())
                        .collect::<Vec<_>>()
                        .join("\r\n");
                    self.send_bytes(fd, uri_list.into_bytes());
                    return;
                }
            }
        } else if let Some(text) = contents.text() {
            self.send_bytes(fd, text.as_bytes().to_owned());
        }
    }

    pub fn send_primary(&self, _mime_type: String, fd: OwnedFd) {
        if let Some(text) = self
            .primary_contents
            .as_ref()
            .and_then(|contents| contents.text())
        {
            self.send_bytes(fd, text.as_bytes().to_owned());
        }
    }

    pub fn read(&mut self) -> Option<ClipboardItem> {
        // When we are the clipboard owner, return our contents directly. Most Wayland compositors
        // do not send a wl_data_device.selection event back to the client that called
        // set_selection, so current_offer is not updated on self-write.
        if self.ownership.is_self_owner {
            return self.contents.clone();
        }

        let offer = self.current_offer.as_ref()?;
        if let Some(cached) = self.cached_read.clone() {
            return Some(cached);
        }

        if offer.has_mime_type(&self.self_mime) {
            return self.contents.clone();
        }

        let item = read_file_or_text(
            || offer.read_uri_list(&self.connection),
            || offer.read_text(&self.connection),
        )
        .or_else(|| offer.read_image(&self.connection))?;

        self.cached_read = Some(item.clone());
        Some(item)
    }

    pub fn read_primary(&mut self) -> Option<ClipboardItem> {
        let offer = self.current_primary_offer.as_ref()?;
        if let Some(cached) = self.cached_primary_read.clone() {
            return Some(cached);
        }

        if offer.has_mime_type(&self.self_mime) {
            return self.primary_contents.clone();
        }

        let item = offer
            .read_text(&self.connection)
            .or_else(|| offer.read_image(&self.connection))?;

        self.cached_primary_read = Some(item.clone());
        Some(item)
    }

    pub fn send_bytes(&self, fd: OwnedFd, bytes: Vec<u8>) {
        let mut written = 0;
        self.loop_handle
            .insert_source(
                calloop::generic::Generic::new(
                    File::from(fd),
                    calloop::Interest::WRITE,
                    calloop::Mode::Level,
                ),
                move |_, file, _| {
                    let file = unsafe { file.get_mut() };
                    loop {
                        match file.write(&bytes[written..]) {
                            Ok(n) if written + n == bytes.len() => {
                                written += n;
                                break Ok(PostAction::Remove);
                            }
                            Ok(n) => written += n,
                            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                                break Ok(PostAction::Continue);
                            }
                            Err(_) => break Ok(PostAction::Remove),
                        }
                    }
                },
            )
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_request_claims_ownership() {
        let mut ownership = ClipboardOwnership::default();

        assert!(!ownership.is_self_owner);
        ownership.selection_requested();
        assert!(ownership.is_self_owner);
    }

    #[test]
    fn external_offer_clears_selection_ownership() {
        let mut ownership = ClipboardOwnership::default();
        ownership.selection_requested();

        ownership.external_offer_received();

        assert!(!ownership.is_self_owner);
    }

    #[test]
    fn uri_list_takes_precedence_over_text() {
        let result = read_file_or_text(|| Some("uri-list"), || Some("text"));

        assert_eq!(result, Some("uri-list"));
    }

    #[test]
    fn text_is_used_when_uri_list_is_unavailable() {
        let result = read_file_or_text(|| None, || Some("text"));

        assert_eq!(result, Some("text"));
    }
}
