use std::{
    fs::File,
    io::{ErrorKind, Write},
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
    sync::{Arc, Weak},
};

use calloop::{LoopHandle, PostAction};
use filedescriptor::Pipe;
use strum::IntoEnumIterator;
use wayland_client::{Connection, protocol::wl_data_offer::WlDataOffer};
use wayland_protocols::wp::primary_selection::zv1::client::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;

use crate::linux::platform::{PIPE_READ_TIMEOUT, read_fd_with_timeout};
use gpui::{
    ClipboardEntry, ClipboardItem, ClipboardReadMode, ClipboardRevision, ClipboardSnapshot, Image,
    ImageFormat, hash,
};
use uuid::Uuid;

/// Text mime types that we'll offer to other programs.
pub(crate) const TEXT_MIME_TYPES: [&str; 3] =
    ["text/plain;charset=utf-8", "UTF8_STRING", "text/plain"];
pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

/// Text mime types that we'll accept from other programs.
pub(crate) const ALLOWED_TEXT_MIME_TYPES: [&str; 2] = ["text/plain;charset=utf-8", "UTF8_STRING"];

pub(crate) struct Clipboard {
    connection: Connection,
    loop_handle: LoopHandle<'static, ()>,
    keyboard_focus: Option<ClipboardFocusToken>,
    focus_sync_pending: bool,
    external_selection_ready: bool,
    revision: Option<ClipboardRevision>,
    primary_revision: Option<ClipboardRevision>,
    sources: Vec<Weak<ClipboardSource>>,
    primary_sources: Vec<Weak<ClipboardSource>>,

    // Internal clipboard
    contents: Option<Arc<ClipboardSource>>,
    primary_contents: Option<Arc<ClipboardSource>>,

    // External clipboard
    cached_read: Option<ClipboardItem>,
    current_offer: Option<DataOffer<WlDataOffer>>,
    cached_primary_read: Option<ClipboardItem>,
    current_primary_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>,
}

#[derive(Clone)]
pub(crate) struct ClipboardFocusToken(Arc<()>);

#[derive(Clone, Debug)]
pub(crate) struct ClipboardSource {
    pub revision: ClipboardRevision,
    pub mime_type: String,
    item: ClipboardItem,
}

impl ClipboardSource {
    fn new(item: ClipboardItem) -> Arc<Self> {
        Arc::new(Self {
            revision: ClipboardRevision::default(),
            mime_type: format!("application/x-gpui-clipboard-{}", Uuid::new_v4()),
            item,
        })
    }

    fn for_offer<T: ReceiveData>(
        sources: &mut Vec<Weak<Self>>,
        offer: Option<&DataOffer<T>>,
    ) -> Option<Arc<Self>> {
        let mut offered = false;
        let mut latest = None;
        sources.retain(|source| {
            let Some(source) = source.upgrade() else {
                return false;
            };
            offered |= offer.is_some_and(|offer| offer.has_mime_type(&source.mime_type));
            latest = Some(source);
            true
        });
        if offered { latest } else { None }
    }
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
    pub fn new(connection: Connection, loop_handle: LoopHandle<'static, ()>) -> Self {
        Self {
            connection,
            loop_handle,
            keyboard_focus: None,
            focus_sync_pending: false,
            external_selection_ready: false,
            revision: None,
            primary_revision: None,
            sources: Vec::new(),
            primary_sources: Vec::new(),

            contents: None,
            primary_contents: None,

            cached_read: None,
            current_offer: None,
            cached_primary_read: None,
            current_primary_offer: None,
        }
    }

    pub fn set(&mut self, item: ClipboardItem) -> Arc<ClipboardSource> {
        let source = ClipboardSource::new(item);
        self.sources.retain(|source| source.strong_count() > 0);
        self.sources.push(Arc::downgrade(&source));
        self.revision = Some(source.revision.clone());
        self.contents = Some(source.clone());
        self.cached_read = None;
        source
    }

    pub fn set_primary(&mut self, item: ClipboardItem) -> Arc<ClipboardSource> {
        let source = ClipboardSource::new(item);
        self.primary_sources
            .retain(|source| source.strong_count() > 0);
        self.primary_sources.push(Arc::downgrade(&source));
        self.primary_revision = Some(source.revision.clone());
        self.primary_contents = Some(source.clone());
        self.cached_primary_read = None;
        source
    }

    pub fn set_offer(&mut self, data_offer: Option<DataOffer<WlDataOffer>>) {
        self.contents = ClipboardSource::for_offer(&mut self.sources, data_offer.as_ref());
        self.revision = Some(
            self.contents
                .as_ref()
                .map(|source| source.revision.clone())
                .unwrap_or_default(),
        );
        self.cached_read = None;
        self.current_offer = data_offer;
    }

    pub fn set_primary_offer(&mut self, data_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>) {
        self.primary_contents =
            ClipboardSource::for_offer(&mut self.primary_sources, data_offer.as_ref());
        self.primary_revision = Some(
            self.primary_contents
                .as_ref()
                .map(|source| source.revision.clone())
                .unwrap_or_default(),
        );
        self.cached_primary_read = None;
        self.current_primary_offer = data_offer;
    }

    pub fn keyboard_focus_gained(&mut self) -> Option<ClipboardFocusToken> {
        self.external_selection_ready = false;
        self.keyboard_focus = Some(ClipboardFocusToken(Arc::new(())));
        self.start_focus_sync()
    }

    pub fn keyboard_focus_synced(
        &mut self,
        token: &ClipboardFocusToken,
    ) -> Option<ClipboardFocusToken> {
        self.focus_sync_pending = false;
        if self
            .keyboard_focus
            .as_ref()
            .is_some_and(|focus| Arc::ptr_eq(&focus.0, &token.0))
        {
            self.external_selection_ready = true;
            None
        } else {
            self.start_focus_sync()
        }
    }

    pub fn keyboard_focus_lost(&mut self) {
        self.keyboard_focus = None;
        self.external_selection_ready = false;
    }

    pub fn cancel(&mut self, source: &ClipboardSource) {
        self.sources.retain(|current| {
            current
                .upgrade()
                .is_some_and(|current| current.revision != source.revision)
        });
        if self
            .contents
            .as_ref()
            .is_some_and(|current| current.revision == source.revision)
        {
            self.contents = None;
            self.current_offer = None;
            self.cached_read = None;
            self.revision = None;
        }
    }

    pub fn cancel_primary(&mut self, source: &ClipboardSource) {
        self.primary_sources.retain(|current| {
            current
                .upgrade()
                .is_some_and(|current| current.revision != source.revision)
        });
        if self
            .primary_contents
            .as_ref()
            .is_some_and(|current| current.revision == source.revision)
        {
            self.primary_contents = None;
            self.current_primary_offer = None;
            self.cached_primary_read = None;
            self.primary_revision = None;
        }
    }

    pub fn send(&self, source: &ClipboardSource, fd: OwnedFd) {
        if let Some(text) = source.item.text() {
            self.send_bytes(fd, text.into_bytes());
        }
    }

    pub fn read_snapshot(&mut self, mode: ClipboardReadMode) -> ClipboardSnapshot {
        if !self.external_selection_ready && self.contents.is_none() {
            return ClipboardSnapshot::default();
        }
        ClipboardSnapshot {
            item: (mode == ClipboardReadMode::Contents)
                .then(|| self.read())
                .flatten(),
            revision: self.revision.clone(),
        }
    }

    pub fn read_primary_snapshot(&mut self, mode: ClipboardReadMode) -> ClipboardSnapshot {
        if !self.external_selection_ready && self.primary_contents.is_none() {
            return ClipboardSnapshot::default();
        }
        ClipboardSnapshot {
            item: (mode == ClipboardReadMode::Contents)
                .then(|| self.read_primary())
                .flatten(),
            revision: self.primary_revision.clone(),
        }
    }

    pub fn read(&mut self) -> Option<ClipboardItem> {
        if let Some(source) = &self.contents {
            return Some(source.item.clone());
        }
        if !self.external_selection_ready {
            return None;
        }
        let offer = self.current_offer.as_ref()?;
        if let Some(cached) = self.cached_read.clone() {
            return Some(cached);
        }

        let item = offer
            .read_text(&self.connection)
            .or_else(|| offer.read_image(&self.connection))?;

        self.cached_read = Some(item.clone());
        Some(item)
    }

    pub fn read_primary(&mut self) -> Option<ClipboardItem> {
        if let Some(source) = &self.primary_contents {
            return Some(source.item.clone());
        }
        if !self.external_selection_ready {
            return None;
        }
        let offer = self.current_primary_offer.as_ref()?;
        if let Some(cached) = self.cached_primary_read.clone() {
            return Some(cached);
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

    fn start_focus_sync(&mut self) -> Option<ClipboardFocusToken> {
        if self.focus_sync_pending {
            return None;
        }
        let focus = self.keyboard_focus.clone()?;
        self.focus_sync_pending = true;
        Some(focus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calloop::EventLoop;
    use std::os::unix::net::UnixStream;
    use wayland_client::Proxy;

    #[test]
    fn test_clipboard_source_lifetimes() {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(connection, event_loop.handle());
            let other_revision = if primary {
                clipboard.revision.clone()
            } else {
                clipboard.primary_revision.clone()
            };
            let first = set_test_source(&mut clipboard, primary, "A");
            let second = set_test_source(&mut clipboard, primary, "B");
            offer_test_source(&mut clipboard, primary, &first);
            assert_eq!(test_source_text(&clipboard, primary).as_deref(), Some("B"));
            if primary {
                clipboard.cancel_primary(&first);
            } else {
                clipboard.cancel(&first);
            }
            assert_eq!(test_source_text(&clipboard, primary).as_deref(), Some("B"));
            offer_test_source(&mut clipboard, primary, &first);
            assert_eq!(test_source_text(&clipboard, primary), None);
            offer_test_source(&mut clipboard, primary, &second);
            assert_eq!(test_source_text(&clipboard, primary).as_deref(), Some("B"));
            if primary {
                clipboard.cancel_primary(&second);
            } else {
                clipboard.cancel(&second);
            }
            let third = set_test_source(&mut clipboard, primary, "C");
            offer_test_source(&mut clipboard, primary, &second);
            assert_eq!(test_source_text(&clipboard, primary), None);
            offer_test_source(&mut clipboard, primary, &third);
            assert_eq!(test_source_text(&clipboard, primary).as_deref(), Some("C"));
            assert_eq!(
                if primary {
                    &clipboard.revision
                } else {
                    &clipboard.primary_revision
                },
                &other_revision,
            );
        }
    }

    #[test]
    fn test_clipboard_focus_provenance() {
        for own_clipboard in [false, true] {
            for own_primary in [false, true] {
                let (socket, _server) = UnixStream::pair().expect("socket pair");
                let connection = Connection::from_socket(socket).expect("test connection");
                let event_loop = EventLoop::try_new().expect("event loop");
                let mut clipboard = Clipboard::new(connection, event_loop.handle());
                let token = clipboard.keyboard_focus_gained().expect("focus sync");
                assert!(clipboard.keyboard_focus_synced(&token).is_none());
                let sources = [
                    own_clipboard.then(|| set_test_source(&mut clipboard, false, "clipboard")),
                    own_primary.then(|| set_test_source(&mut clipboard, true, "primary")),
                ];
                let channels = [(false, "clipboard"), (true, "primary")];
                for ((primary, text), source) in channels.into_iter().zip(&sources) {
                    if let Some(source) = source {
                        offer_test_source(&mut clipboard, primary, source);
                    } else {
                        offer_test_external(&mut clipboard, primary, text);
                    }
                }
                let revisions = [
                    clipboard.revision.clone(),
                    clipboard.primary_revision.clone(),
                ];
                for ((primary, text), revision) in channels.into_iter().zip(&revisions) {
                    assert!(revision.is_some());
                    assert_eq!(
                        test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                        (Some(text.to_owned()), revision.clone()),
                    );
                }

                clipboard.keyboard_focus_lost();
                clipboard.keyboard_focus_lost();
                let mut pending_sync = None;
                for entering in [false, true] {
                    if entering {
                        pending_sync = clipboard.keyboard_focus_gained();
                    }
                    for (((primary, text), source), revision) in
                        channels.into_iter().zip(&sources).zip(&revisions)
                    {
                        let expected_text = source.as_ref().map(|_| text.to_owned());
                        let expected_revision = source.as_ref().and(revision.clone());
                        for mode in [ClipboardReadMode::Contents, ClipboardReadMode::RevisionOnly] {
                            assert_eq!(
                                test_snapshot(&mut clipboard, primary, mode),
                                (
                                    (mode == ClipboardReadMode::Contents)
                                        .then(|| expected_text.clone())
                                        .flatten(),
                                    expected_revision.clone(),
                                ),
                            );
                        }
                        let item = if primary {
                            clipboard.read_primary()
                        } else {
                            clipboard.read()
                        };
                        assert_eq!(item.and_then(|item| item.text()), expected_text);
                        if let Some(source) = source {
                            assert_eq!(source.item.text().as_deref(), Some(text));
                        }
                    }
                }

                let token = pending_sync.expect("refocus sync");
                assert!(clipboard.keyboard_focus_synced(&token).is_none());
                for ((primary, text), revision) in channels.into_iter().zip(revisions) {
                    assert_eq!(
                        test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                        (Some(text.to_owned()), revision),
                    );
                }
            }
        }
    }

    #[test]
    fn test_clipboard_focus_selection_events() {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(connection, event_loop.handle());
            let token = clipboard.keyboard_focus_gained().expect("focus sync");
            assert!(clipboard.keyboard_focus_synced(&token).is_none());
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (None, None),
            );
            let other = set_test_source(&mut clipboard, !primary, "other channel");
            offer_test_external(&mut clipboard, primary, "before");
            for offer_before_enter in [false, true] {
                let (_, before) =
                    test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents);
                clipboard.keyboard_focus_lost();
                if offer_before_enter {
                    offer_test_external(&mut clipboard, primary, "after");
                }
                assert_eq!(
                    test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                    (None, None),
                );
                let token = clipboard.keyboard_focus_gained().expect("refocus sync");
                assert_eq!(
                    test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                    (None, None),
                );
                if !offer_before_enter {
                    offer_test_external(&mut clipboard, primary, "after");
                }
                assert_eq!(
                    test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                    (None, None),
                );
                assert!(clipboard.keyboard_focus_synced(&token).is_none());
                let (item, after) =
                    test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents);
                assert_eq!(item.as_deref(), Some("after"));
                assert!(after.is_some());
                assert_ne!(after, before);
            }
            let (_, after) = test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents);

            clipboard.keyboard_focus_lost();
            if primary {
                clipboard.set_primary_offer(None);
            } else {
                clipboard.set_offer(None);
            }
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (None, None),
            );
            let token = clipboard
                .keyboard_focus_gained()
                .expect("empty selection sync");
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::RevisionOnly),
                (None, None),
            );
            assert!(clipboard.keyboard_focus_synced(&token).is_none());
            let (item, empty) = test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents);
            assert_eq!(item, None);
            assert!(empty.is_some());
            assert_ne!(empty, after);

            let first = set_test_source(&mut clipboard, primary, "first");
            let second = set_test_source(&mut clipboard, primary, "second");
            clipboard.keyboard_focus_lost();
            if primary {
                clipboard.cancel_primary(&first);
            } else {
                clipboard.cancel(&first);
            }
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (Some("second".to_owned()), Some(second.revision.clone())),
            );
            if primary {
                clipboard.cancel_primary(&second);
            } else {
                clipboard.cancel(&second);
            }
            let token = clipboard
                .keyboard_focus_gained()
                .expect("cancelled source sync");
            assert!(clipboard.keyboard_focus_synced(&token).is_none());
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (None, None),
            );
            assert_eq!(first.item.text().as_deref(), Some("first"));
            assert_eq!(second.item.text().as_deref(), Some("second"));
            assert_eq!(
                test_snapshot(&mut clipboard, !primary, ClipboardReadMode::Contents),
                (
                    Some("other channel".to_owned()),
                    Some(other.revision.clone())
                ),
            );
        }
    }

    #[test]
    fn test_clipboard_focus_sync_ignores_stale_barriers() {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(connection, event_loop.handle());
            offer_test_external(&mut clipboard, primary, "retained");
            let first = clipboard.keyboard_focus_gained().expect("first focus sync");
            clipboard.keyboard_focus_lost();
            assert!(clipboard.keyboard_focus_synced(&first).is_none());
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (None, None),
            );

            let second = clipboard
                .keyboard_focus_gained()
                .expect("second focus sync");
            for _ in 0..3 {
                clipboard.keyboard_focus_lost();
                assert!(clipboard.keyboard_focus_gained().is_none());
            }
            let latest = clipboard
                .keyboard_focus_synced(&second)
                .expect("latest focus sync");
            assert_eq!(
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents),
                (None, None),
            );
            assert!(clipboard.keyboard_focus_synced(&latest).is_none());
            let (item, revision) =
                test_snapshot(&mut clipboard, primary, ClipboardReadMode::Contents);
            assert_eq!(item.as_deref(), Some("retained"));
            assert!(revision.is_some());
        }
    }

    fn set_test_source(
        clipboard: &mut Clipboard,
        primary: bool,
        text: &str,
    ) -> Arc<ClipboardSource> {
        let item = ClipboardItem::new_string(text.to_owned());
        if primary {
            clipboard.set_primary(item)
        } else {
            clipboard.set(item)
        }
    }

    fn offer_test_source(clipboard: &mut Clipboard, primary: bool, source: &ClipboardSource) {
        let backend = clipboard.connection.backend().downgrade();
        if primary {
            let mut offer = DataOffer::new(ZwpPrimarySelectionOfferV1::inert(backend));
            offer.add_mime_type(source.mime_type.clone());
            clipboard.set_primary_offer(Some(offer));
        } else {
            let mut offer = DataOffer::new(WlDataOffer::inert(backend));
            offer.add_mime_type(source.mime_type.clone());
            clipboard.set_offer(Some(offer));
        }
    }

    fn offer_test_external(clipboard: &mut Clipboard, primary: bool, text: &str) {
        let backend = clipboard.connection.backend().downgrade();
        let item = ClipboardItem::new_string(text.to_owned());
        if primary {
            let mut offer = DataOffer::new(ZwpPrimarySelectionOfferV1::inert(backend));
            offer.add_mime_type("text/plain;charset=utf-8".to_owned());
            clipboard.set_primary_offer(Some(offer));
            clipboard.cached_primary_read = Some(item);
        } else {
            let mut offer = DataOffer::new(WlDataOffer::inert(backend));
            offer.add_mime_type("text/plain;charset=utf-8".to_owned());
            clipboard.set_offer(Some(offer));
            clipboard.cached_read = Some(item);
        }
    }

    fn test_snapshot(
        clipboard: &mut Clipboard,
        primary: bool,
        mode: ClipboardReadMode,
    ) -> (Option<String>, Option<ClipboardRevision>) {
        let snapshot = if primary {
            clipboard.read_primary_snapshot(mode)
        } else {
            clipboard.read_snapshot(mode)
        };
        (
            snapshot.item.and_then(|item| item.text()),
            snapshot.revision,
        )
    }

    fn test_source_text(clipboard: &Clipboard, primary: bool) -> Option<String> {
        if primary {
            clipboard.primary_contents.as_ref()
        } else {
            clipboard.contents.as_ref()
        }
        .and_then(|source| source.item.text())
    }
}
