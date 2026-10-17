use std::{
    cell::OnceCell,
    fs::File,
    io::{ErrorKind, Write},
    os::fd::{AsFd, BorrowedFd, OwnedFd},
    rc::Rc,
    sync::{Arc, Weak},
    time::Duration,
};

use calloop::{LoopHandle, PostAction};
use filedescriptor::{FileDescriptor, Pipe};
use futures::{AsyncReadExt, FutureExt, StreamExt, stream::FuturesOrdered};
use strum::IntoEnumIterator;
use wayland_client::{Connection, protocol::wl_data_offer::WlDataOffer};
use wayland_protocols::wp::primary_selection::zv1::client::zwp_primary_selection_offer_v1::ZwpPrimarySelectionOfferV1;

use crate::linux::platform::{PIPE_READ_TIMEOUT, read_fd_with_timeout};
use gpui::{
    BackgroundExecutor, ClipboardEntry, ClipboardItem, ClipboardReadError, ClipboardRevision,
    ClipboardSnapshot, ClipboardSnapshotRead, ClipboardSnapshotResolver, ForegroundExecutor, Image,
    ImageFormat, WeakClipboardSnapshotRead, hash,
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
    focus_sync: Option<ClipboardFocusToken>,
    selection_readiness: SelectionReadiness,
    primary_readiness: SelectionReadiness,
    snapshot_reader: SnapshotReader,
    revision: Option<ClipboardRevision>,
    primary_revision: Option<ClipboardRevision>,
    sources: Vec<Weak<ClipboardSource>>,
    primary_sources: Vec<Weak<ClipboardSource>>,

    // Internal clipboard
    contents: Option<Arc<ClipboardSource>>,
    primary_contents: Option<Arc<ClipboardSource>>,

    // External clipboard
    cached_read: ClipboardCache,
    current_offer: Option<DataOffer<WlDataOffer>>,
    cached_primary_read: ClipboardCache,
    current_primary_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>,
}

#[derive(Clone)]
pub(crate) struct ClipboardFocusToken(Arc<()>);

#[derive(Default)]
#[must_use]
pub(crate) struct ClipboardCompletions(Vec<Box<dyn FnOnce()>>);

impl ClipboardCompletions {
    pub fn resolve(self) {
        for complete in self.0 {
            complete();
        }
    }

    fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

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

    fn receive_bytes(
        &self,
        connection: &Connection,
        mime_type: &str,
    ) -> anyhow::Result<FileDescriptor> {
        let pipe = Pipe::new()?;
        self.inner
            .receive_data(mime_type.to_owned(), pipe.write.as_fd());
        drop(pipe.write);
        match connection.flush() {
            Ok(()) => {}
            Err(wayland_client::backend::WaylandError::Io(error))
                if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => return Err(error.into()),
        }
        Ok(pipe.read)
    }

    fn read_bytes(&self, connection: &Connection, mime_type: &str) -> Option<Vec<u8>> {
        let result = self
            .receive_bytes(connection, mime_type)
            .and_then(|fd| read_fd_with_timeout(fd, PIPE_READ_TIMEOUT));
        match result {
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
    pub fn new(
        connection: Connection,
        loop_handle: LoopHandle<'static, ()>,
        background_executor: BackgroundExecutor,
        foreground_executor: ForegroundExecutor,
    ) -> Self {
        Self {
            connection,
            loop_handle,
            snapshot_reader: SnapshotReader {
                background_executor,
                foreground_executor,
                timeout: PIPE_READ_TIMEOUT,
                #[cfg(test)]
                progress: Arc::default(),
            },
            keyboard_focus: None,
            focus_sync: None,
            selection_readiness: SelectionReadiness::default(),
            primary_readiness: SelectionReadiness::default(),
            revision: Some(ClipboardRevision::default()),
            primary_revision: Some(ClipboardRevision::default()),
            sources: Vec::new(),
            primary_sources: Vec::new(),

            contents: None,
            primary_contents: None,

            cached_read: Rc::default(),
            current_offer: None,
            cached_primary_read: Rc::default(),
            current_primary_offer: None,
        }
    }

    pub fn set(&mut self, item: ClipboardItem) -> Arc<ClipboardSource> {
        let source = ClipboardSource::new(item);
        self.sources.retain(|source| source.strong_count() > 0);
        self.sources.push(Arc::downgrade(&source));
        self.revision = Some(source.revision.clone());
        self.contents = Some(source.clone());
        self.selection_readiness.contents = None;
        self.cached_read = Rc::default();
        source
    }

    pub fn set_primary(&mut self, item: ClipboardItem) -> Arc<ClipboardSource> {
        let source = ClipboardSource::new(item);
        self.primary_sources
            .retain(|source| source.strong_count() > 0);
        self.primary_sources.push(Arc::downgrade(&source));
        self.primary_revision = Some(source.revision.clone());
        self.primary_contents = Some(source.clone());
        self.primary_readiness.contents = None;
        self.cached_primary_read = Rc::default();
        source
    }

    pub fn set_offer(
        &mut self,
        data_offer: Option<DataOffer<WlDataOffer>>,
    ) -> ClipboardCompletions {
        self.contents = ClipboardSource::for_offer(&mut self.sources, data_offer.as_ref());
        self.revision = Some(
            self.contents
                .as_ref()
                .map(|source| source.revision.clone())
                .unwrap_or_default(),
        );
        self.cached_read = Rc::default();
        self.current_offer = data_offer;
        self.selection_readiness.contents = None;
        let value = self.selection_value();
        self.selection_readiness.authority =
            if self.keyboard_focus.is_some() || self.current_offer.is_some() {
                SelectionAuthority::Ready
            } else {
                SelectionAuthority::Unknown
            };
        self.selection_readiness
            .complete(&value, &self.connection, &self.snapshot_reader)
    }

    pub fn set_primary_offer(
        &mut self,
        data_offer: Option<DataOffer<ZwpPrimarySelectionOfferV1>>,
    ) -> ClipboardCompletions {
        self.primary_contents =
            ClipboardSource::for_offer(&mut self.primary_sources, data_offer.as_ref());
        self.primary_revision = Some(
            self.primary_contents
                .as_ref()
                .map(|source| source.revision.clone())
                .unwrap_or_default(),
        );
        self.cached_primary_read = Rc::default();
        self.current_primary_offer = data_offer;
        self.primary_readiness.contents = None;
        let value = self.primary_value();
        self.primary_readiness.authority =
            if self.keyboard_focus.is_some() || self.current_primary_offer.is_some() {
                SelectionAuthority::Ready
            } else {
                SelectionAuthority::Unknown
            };
        self.primary_readiness
            .complete(&value, &self.connection, &self.snapshot_reader)
    }

    pub fn keyboard_focus_gained(&mut self) -> ClipboardFocusToken {
        let token = ClipboardFocusToken(Arc::new(()));
        self.keyboard_focus = Some(token.clone());
        token
    }

    pub fn start_focus_sync(&mut self) -> Option<ClipboardFocusToken> {
        if self.focus_sync.is_some()
            || (self.selection_readiness.authority != SelectionAuthority::Unknown
                && self.primary_readiness.authority != SelectionAuthority::Unknown)
        {
            return None;
        }
        let token = self.keyboard_focus.clone()?;
        self.focus_sync = Some(token.clone());
        Some(token)
    }

    pub fn keyboard_focus_synced(&mut self, token: &ClipboardFocusToken) -> ClipboardCompletions {
        if !self
            .focus_sync
            .as_ref()
            .is_some_and(|pending| Arc::ptr_eq(&pending.0, &token.0))
        {
            return ClipboardCompletions::default();
        }
        self.focus_sync = None;
        let mut completions = ClipboardCompletions::default();
        if self
            .keyboard_focus
            .as_ref()
            .is_some_and(|focus| Arc::ptr_eq(&focus.0, &token.0))
        {
            let value = self.selection_value();
            completions.extend(self.selection_readiness.confirm(
                &value,
                &self.connection,
                &self.snapshot_reader,
            ));
            let value = self.primary_value();
            completions.extend(self.primary_readiness.confirm(
                &value,
                &self.connection,
                &self.snapshot_reader,
            ));
        }
        completions
    }

    pub fn keyboard_focus_timed_out(
        &mut self,
        token: &ClipboardFocusToken,
    ) -> ClipboardCompletions {
        let mut completions = ClipboardCompletions::default();
        if self
            .keyboard_focus
            .as_ref()
            .is_some_and(|focus| Arc::ptr_eq(&focus.0, &token.0))
        {
            completions.extend(self.selection_readiness.fail());
            completions.extend(self.primary_readiness.fail());
        }
        completions
    }

    pub fn keyboard_focus_lost(&mut self) -> ClipboardCompletions {
        self.keyboard_focus = None;
        let mut completions = self.selection_readiness.fail();
        completions.extend(self.primary_readiness.fail());
        self.selection_readiness.authority = SelectionAuthority::Unknown;
        self.primary_readiness.authority = SelectionAuthority::Unknown;
        completions
    }

    pub fn can_publish(&self, primary: bool) -> bool {
        self.keyboard_focus.is_some()
            && if primary {
                self.primary_readiness.authority == SelectionAuthority::Ready
            } else {
                self.selection_readiness.authority == SelectionAuthority::Ready
            }
    }

    pub fn capture_snapshot(&mut self) -> ClipboardSnapshotRead {
        let value = self.selection_value();
        self.selection_readiness.capture(
            self.keyboard_focus.is_some(),
            &value,
            &self.connection,
            &self.snapshot_reader,
        )
    }

    pub fn capture_primary_snapshot(&mut self) -> ClipboardSnapshotRead {
        let value = self.primary_value();
        self.primary_readiness.capture(
            self.keyboard_focus.is_some(),
            &value,
            &self.connection,
            &self.snapshot_reader,
        )
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
            self.selection_readiness.contents = None;
            self.current_offer = None;
            self.cached_read = Rc::default();
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
            self.primary_readiness.contents = None;
            self.current_primary_offer = None;
            self.cached_primary_read = Rc::default();
            self.primary_revision = None;
        }
    }

    pub fn send(&self, source: &ClipboardSource, fd: OwnedFd) {
        if let Some(text) = source.item.text() {
            self.send_bytes(fd, text.into_bytes());
        }
    }

    pub fn read_snapshot(&mut self) -> ClipboardSnapshot {
        if !self.can_publish(false) && self.contents.is_none() {
            return ClipboardSnapshot::default();
        }
        ClipboardSnapshot {
            item: self.read(),
            revision: self.revision.clone(),
        }
    }

    pub fn read_primary_snapshot(&mut self) -> ClipboardSnapshot {
        if !self.can_publish(true) && self.primary_contents.is_none() {
            return ClipboardSnapshot::default();
        }
        ClipboardSnapshot {
            item: self.read_primary(),
            revision: self.primary_revision.clone(),
        }
    }

    pub fn read(&mut self) -> Option<ClipboardItem> {
        if let Some(source) = &self.contents {
            return Some(source.item.clone());
        }
        if !self.can_publish(false) {
            return None;
        }
        let offer = self.current_offer.as_ref()?;
        if let Some(cached) = self.cached_read.get() {
            return cached.item.clone();
        }

        let item = offer
            .read_text(&self.connection)
            .or_else(|| offer.read_image(&self.connection))?;

        self.cached_read.get_or_init(|| {
            Arc::new(ClipboardSnapshot {
                item: Some(item.clone()),
                revision: self.revision.clone(),
            })
        });
        Some(item)
    }

    pub fn read_primary(&mut self) -> Option<ClipboardItem> {
        if let Some(source) = &self.primary_contents {
            return Some(source.item.clone());
        }
        if !self.can_publish(true) {
            return None;
        }
        let offer = self.current_primary_offer.as_ref()?;
        if let Some(cached) = self.cached_primary_read.get() {
            return cached.item.clone();
        }

        let item = offer
            .read_text(&self.connection)
            .or_else(|| offer.read_image(&self.connection))?;

        self.cached_primary_read.get_or_init(|| {
            Arc::new(ClipboardSnapshot {
                item: Some(item.clone()),
                revision: self.primary_revision.clone(),
            })
        });
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

    fn selection_value(&self) -> SelectionValue<WlDataOffer> {
        SelectionValue {
            source: self.contents.clone(),
            revision: self.revision.clone(),
            offer: self.current_offer.clone(),
            cached: self.cached_read.clone(),
        }
    }

    fn primary_value(&self) -> SelectionValue<ZwpPrimarySelectionOfferV1> {
        SelectionValue {
            source: self.primary_contents.clone(),
            revision: self.primary_revision.clone(),
            offer: self.current_primary_offer.clone(),
            cached: self.cached_primary_read.clone(),
        }
    }
}

type ClipboardCache = Rc<OnceCell<Arc<ClipboardSnapshot>>>;

struct SelectionValue<T: ReceiveData> {
    source: Option<Arc<ClipboardSource>>,
    revision: Option<ClipboardRevision>,
    offer: Option<DataOffer<T>>,
    cached: ClipboardCache,
}

impl<T: ReceiveData> SelectionValue<T> {
    fn capture(&self, connection: &Connection, reader: &SnapshotReader) -> ClipboardSnapshotRead {
        let Some(revision) = self.revision.clone() else {
            return ClipboardSnapshotRead::failed(ClipboardReadError::Unavailable);
        };
        let item = if let Some(source) = &self.source {
            Some(source.item.clone())
        } else if let Some(snapshot) = self.cached.get() {
            let (read, resolver) = ClipboardSnapshotRead::pending();
            resolver.resolve(Ok(snapshot.clone()));
            return read;
        } else if let Some(offer) = &self.offer {
            let read = reader.capture(offer, connection, revision);
            let cached = Rc::downgrade(&self.cached);
            read.on_ready(move |result| {
                if let Ok(snapshot) = result
                    && let Some(cached) = cached.upgrade()
                {
                    cached.get_or_init(|| snapshot);
                }
            });
            return read;
        } else {
            None
        };
        ClipboardSnapshotRead::ready(ClipboardSnapshot {
            item,
            revision: Some(revision),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectionAuthority {
    Unknown,
    Ready,
    Failed,
}

struct SelectionReadiness {
    authority: SelectionAuthority,
    captures: Vec<ClipboardSnapshotResolver>,
    contents: Option<(Option<ClipboardRevision>, WeakClipboardSnapshotRead)>,
}

impl Default for SelectionReadiness {
    fn default() -> Self {
        Self {
            authority: SelectionAuthority::Unknown,
            captures: Vec::new(),
            contents: None,
        }
    }
}

impl SelectionReadiness {
    fn capture<T: ReceiveData>(
        &mut self,
        focused: bool,
        value: &SelectionValue<T>,
        connection: &Connection,
        reader: &SnapshotReader,
    ) -> ClipboardSnapshotRead {
        if (focused && self.authority == SelectionAuthority::Ready)
            || (!focused && value.source.is_some())
        {
            return self.admit(value, connection, reader);
        }
        if !focused || self.authority == SelectionAuthority::Failed {
            return ClipboardSnapshotRead::failed(ClipboardReadError::Unavailable);
        }
        let (read, resolver) = ClipboardSnapshotRead::pending();
        self.captures.push(resolver);
        read
    }

    fn admit<T: ReceiveData>(
        &mut self,
        value: &SelectionValue<T>,
        connection: &Connection,
        reader: &SnapshotReader,
    ) -> ClipboardSnapshotRead {
        if let Some((revision, read)) = &self.contents
            && *revision == value.revision
            && let Some(read) = read.upgrade()
            && !read.is_failed()
        {
            return read;
        }
        let read = value.capture(connection, reader);
        self.contents = Some((value.revision.clone(), read.downgrade()));
        read
    }

    fn confirm<T: ReceiveData>(
        &mut self,
        value: &SelectionValue<T>,
        connection: &Connection,
        reader: &SnapshotReader,
    ) -> ClipboardCompletions {
        self.authority = SelectionAuthority::Ready;
        self.complete(value, connection, reader)
    }

    fn fail(&mut self) -> ClipboardCompletions {
        if self.authority == SelectionAuthority::Unknown {
            self.authority = SelectionAuthority::Failed;
        }
        ClipboardCompletions(
            std::mem::take(&mut self.captures)
                .into_iter()
                .map(|resolver| {
                    Box::new(move || resolver.resolve(Err(ClipboardReadError::Unavailable)))
                        as Box<dyn FnOnce()>
                })
                .collect(),
        )
    }

    fn complete<T: ReceiveData>(
        &mut self,
        value: &SelectionValue<T>,
        connection: &Connection,
        reader: &SnapshotReader,
    ) -> ClipboardCompletions {
        ClipboardCompletions(
            std::mem::take(&mut self.captures)
                .into_iter()
                .filter(|resolver| !resolver.is_canceled())
                .map(|resolver| {
                    let read = self.admit(value, connection, reader);
                    let executor = reader.foreground_executor.clone();
                    Box::new(move || {
                        if read.is_ready() {
                            read.on_ready(move |result| resolver.resolve(result));
                        } else {
                            resolver.resolve_async(&executor, async move { read.wait().await });
                        }
                    }) as Box<dyn FnOnce()>
                })
                .collect(),
        )
    }
}

struct SnapshotReader {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    timeout: Duration,
    #[cfg(test)]
    progress: Arc<parking_lot::Mutex<Option<futures::channel::oneshot::Sender<usize>>>>,
}

impl SnapshotReader {
    fn capture<T: ReceiveData>(
        &self,
        offer: &DataOffer<T>,
        connection: &Connection,
        revision: ClipboardRevision,
    ) -> ClipboardSnapshotRead {
        let (read, resolver) = ClipboardSnapshotRead::pending();
        let candidates = offer
            .mime_types
            .iter()
            .find(|mime_type| ALLOWED_TEXT_MIME_TYPES.contains(&mime_type.as_str()))
            .map(|mime_type| (mime_type.as_str(), None))
            .into_iter()
            .chain(
                ImageFormat::iter()
                    .filter(|format| offer.has_mime_type(format.mime_type()))
                    .map(|format| (format.mime_type(), Some(format))),
            )
            .map(|(mime_type, format)| {
                let file = offer
                    .receive_bytes(connection, mime_type)
                    .and_then(|fd| Ok(File::from(fd.as_fd().try_clone_to_owned()?)))
                    .map_err(|error| {
                        log::error!("error initiating clipboard snapshot: {error}");
                        ClipboardReadError::Unavailable
                    });
                (file, format)
            })
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
                item: None,
                revision: Some(revision),
            })));
            return read;
        }
        let timeout = self.timeout;
        let executor = self.background_executor.clone();
        #[cfg(test)]
        let progress = self.progress.clone();
        let task = self.background_executor.spawn(async move {
            let mut candidates = candidates
                .into_iter()
                .map(|(file, format)| {
                    let executor = &executor;
                    #[cfg(test)]
                    let progress = &progress;
                    async move {
                        read_snapshot_candidate(
                            file?,
                            format,
                            executor,
                            timeout,
                            #[cfg(test)]
                            progress,
                        )
                        .await
                    }
                })
                .collect::<FuturesOrdered<_>>();
            let mut error = ClipboardReadError::Unavailable;
            while let Some(result) = candidates.next().await {
                match result {
                    Ok(item) => {
                        return Ok(Arc::new(ClipboardSnapshot {
                            item: Some(item),
                            revision: Some(revision),
                        }));
                    }
                    Err(failure) => error = failure,
                }
            }
            Err(error)
        });
        resolver.resolve_async(&self.foreground_executor, task);
        read
    }
}

async fn read_snapshot_candidate(
    file: File,
    format: Option<ImageFormat>,
    executor: &BackgroundExecutor,
    timeout: Duration,
    #[cfg(test)] progress: &parking_lot::Mutex<Option<futures::channel::oneshot::Sender<usize>>>,
) -> Result<ClipboardItem, ClipboardReadError> {
    let mut pipe = smol::Async::new(file).map_err(|error| {
        log::error!("error opening clipboard snapshot pipe: {error}");
        ClipboardReadError::Unavailable
    })?;
    let mut bytes = Vec::new();
    let mut chunk = [0; 8192];
    let timer = executor.timer(timeout).fuse();
    futures::pin_mut!(timer);
    loop {
        let result = {
            let read = pipe.read(&mut chunk).fuse();
            futures::pin_mut!(read);
            futures::select_biased! {
                _ = timer => {
                    log::warn!("clipboard snapshot read timed out");
                    return Err(ClipboardReadError::Unavailable);
                }
                result = read => result,
            }
        };
        let count = match result {
            Ok(0) => break,
            Ok(count) => count,
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(error) => {
                log::error!("error reading clipboard snapshot: {error}");
                return Err(ClipboardReadError::Unavailable);
            }
        };
        timer.set(executor.timer(timeout).fuse());
        bytes
            .try_reserve(count)
            .map_err(|_| ClipboardReadError::Unavailable)?;
        bytes.extend_from_slice(&chunk[..count]);
        #[cfg(test)]
        if let Some(sender) = progress.lock().take() {
            sender.send(bytes.len()).ok();
        }
    }
    let item = if let Some(format) = format {
        let id = hash(&bytes);
        ClipboardItem {
            entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
        }
    } else {
        let mut previous = None;
        bytes.reverse();
        bytes.retain(|byte| {
            let retain = *byte != b'\r' || previous != Some(b'\n');
            previous = Some(*byte);
            retain
        });
        bytes.reverse();
        let text = String::from_utf8(bytes).map_err(|_| ClipboardReadError::UnsupportedContent)?;
        ClipboardItem::new_string(text)
    };
    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    use calloop::EventLoop;
    use futures::AsyncWriteExt;
    use gpui::TestAppContext;
    use std::{
        cell::{Cell, RefCell},
        os::unix::net::UnixStream,
        rc::Rc,
    };
    use wayland_client::Proxy;

    #[gpui::test]
    fn test_clipboard_source_lifetimes(cx: &mut TestAppContext) {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(
                connection,
                event_loop.handle(),
                cx.background_executor.clone(),
                cx.foreground_executor.clone(),
            );
            let other_revision = if primary {
                clipboard.revision.clone()
            } else {
                clipboard.primary_revision.clone()
            };
            let first = set_test_source(&mut clipboard, primary, "A");
            let second = set_test_source(&mut clipboard, primary, "B");
            offer_test_source(&mut clipboard, primary, &first);
            assert_eq!(
                test_snapshot(&mut clipboard, primary),
                (Some("B".to_owned()), Some(second.revision.clone())),
            );
            if primary {
                clipboard.cancel_primary(&first);
            } else {
                clipboard.cancel(&first);
            }
            assert_eq!(
                test_snapshot(&mut clipboard, primary),
                (Some("B".to_owned()), Some(second.revision.clone())),
            );
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

    #[gpui::test]
    fn test_clipboard_focus_provenance(cx: &mut TestAppContext) {
        for own_clipboard in [false, true] {
            for own_primary in [false, true] {
                let (socket, _server) = UnixStream::pair().expect("socket pair");
                let connection = Connection::from_socket(socket).expect("test connection");
                let event_loop = EventLoop::try_new().expect("event loop");
                let mut clipboard = Clipboard::new(
                    connection,
                    event_loop.handle(),
                    cx.background_executor.clone(),
                    cx.foreground_executor.clone(),
                );
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
                clipboard.keyboard_focus_gained();
                for ((primary, text), revision) in channels.into_iter().zip(&revisions) {
                    assert!(revision.is_some());
                    assert_eq!(
                        test_snapshot(&mut clipboard, primary),
                        (Some(text.to_owned()), revision.clone()),
                    );
                }

                clipboard.keyboard_focus_lost().resolve();
                clipboard.keyboard_focus_lost().resolve();
                for entering in [false, true] {
                    if entering {
                        clipboard.keyboard_focus_gained();
                    }
                    for (((primary, text), source), revision) in
                        channels.into_iter().zip(&sources).zip(&revisions)
                    {
                        let readable = source.is_some();
                        let expected_text = readable.then(|| text.to_owned());
                        let expected_revision = readable.then(|| revision.clone()).flatten();
                        assert_eq!(
                            test_snapshot(&mut clipboard, primary),
                            (expected_text.clone(), expected_revision),
                        );
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
                let captures = channels.map(|(primary, _)| {
                    let capture = capture_test_snapshot(&mut clipboard, primary);
                    assert!(!capture.is_ready());
                    assert!(!clipboard.can_publish(primary));
                    capture
                });
                let token = clipboard.start_focus_sync().expect("focus sync");
                assert!(clipboard.start_focus_sync().is_none());
                clipboard.keyboard_focus_synced(&token).resolve();
                for (((primary, text), revision), capture) in
                    channels.into_iter().zip(&revisions).zip(captures)
                {
                    assert_eq!(
                        resolved_test_snapshot(&capture),
                        Ok((Some(text.to_owned()), revision.clone()))
                    );
                    assert!(clipboard.can_publish(primary));
                }
            }
        }
    }

    #[gpui::test]
    fn test_clipboard_focus_selection_events(cx: &mut TestAppContext) {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(
                connection,
                event_loop.handle(),
                cx.background_executor.clone(),
                cx.foreground_executor.clone(),
            );
            clipboard.keyboard_focus_gained();
            assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
            let empty_capture = capture_test_snapshot(&mut clipboard, primary);
            assert!(!empty_capture.is_ready());
            let token = clipboard.start_focus_sync().expect("initial empty sync");
            clipboard.keyboard_focus_synced(&token).resolve();
            let (empty_item, empty_revision) =
                resolved_test_snapshot(&empty_capture).expect("authoritative empty selection");
            assert_eq!(empty_item, None);
            assert!(empty_revision.is_some());
            let other = set_test_source(&mut clipboard, !primary, "other channel");
            for offer_before_enter in [false, true] {
                offer_test_external(&mut clipboard, primary, "before");
                let (_, before) = test_snapshot(&mut clipboard, primary);
                clipboard.keyboard_focus_lost().resolve();
                if offer_before_enter {
                    offer_test_external(&mut clipboard, primary, "after");
                }
                assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
                clipboard.keyboard_focus_gained();
                let other_capture = capture_test_snapshot(&mut clipboard, !primary);
                let capture = capture_test_snapshot(&mut clipboard, primary);
                if !offer_before_enter {
                    assert!(!capture.is_ready());
                    assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
                    offer_test_external(&mut clipboard, primary, "after");
                }
                let revision = if primary {
                    clipboard.primary_revision.clone()
                } else {
                    clipboard.revision.clone()
                };
                assert_eq!(
                    resolved_test_snapshot(&capture),
                    Ok((Some("after".to_owned()), revision)),
                );
                assert!(!other_capture.is_ready());
                assert!(clipboard.start_focus_sync().is_some());
                let token = clipboard.focus_sync.clone().expect("other channel sync");
                clipboard.keyboard_focus_synced(&token).resolve();
                let (item, after) = test_snapshot(&mut clipboard, primary);
                assert_eq!(item.as_deref(), Some("after"));
                assert!(after.is_some());
                assert_ne!(after, before);
            }
            let (_, after) = test_snapshot(&mut clipboard, primary);

            clipboard.keyboard_focus_lost().resolve();
            if primary {
                clipboard.set_primary_offer(None).resolve();
            } else {
                clipboard.set_offer(None).resolve();
            }
            assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
            clipboard.keyboard_focus_gained();
            let capture = capture_test_snapshot(&mut clipboard, primary);
            assert!(!capture.is_ready());
            let token = clipboard.start_focus_sync().expect("empty selection sync");
            clipboard.keyboard_focus_synced(&token).resolve();
            let (item, empty) = test_snapshot(&mut clipboard, primary);
            assert_eq!(resolved_test_snapshot(&capture), Ok((None, empty.clone())));
            assert_eq!(item, None);
            assert!(empty.is_some());
            assert_ne!(empty, after);
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let token = clipboard.start_focus_sync().expect("same-client sync");
            clipboard.keyboard_focus_synced(&token).resolve();
            assert_eq!(test_snapshot(&mut clipboard, primary), (None, empty));

            let first = set_test_source(&mut clipboard, primary, "first");
            let second = set_test_source(&mut clipboard, primary, "second");
            clipboard.keyboard_focus_lost().resolve();
            if primary {
                clipboard.cancel_primary(&first);
            } else {
                clipboard.cancel(&first);
            }
            assert_eq!(
                test_snapshot(&mut clipboard, primary),
                (Some("second".to_owned()), Some(second.revision.clone())),
            );
            if primary {
                clipboard.cancel_primary(&second);
            } else {
                clipboard.cancel(&second);
            }
            clipboard.keyboard_focus_gained();
            assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
            assert_eq!(first.item.text().as_deref(), Some("first"));
            assert_eq!(second.item.text().as_deref(), Some("second"));
            assert_eq!(
                test_snapshot(&mut clipboard, !primary),
                (
                    Some("other channel".to_owned()),
                    Some(other.revision.clone())
                ),
            );
        }
    }

    #[gpui::test]
    fn test_clipboard_focus_churn(cx: &mut TestAppContext) {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(
                connection,
                event_loop.handle(),
                cx.background_executor.clone(),
                cx.foreground_executor.clone(),
            );
            clipboard.keyboard_focus_gained();
            let mut previous_revision = None;
            for text in ["first", "second", "third"] {
                clipboard.keyboard_focus_lost().resolve();
                clipboard.keyboard_focus_lost().resolve();
                assert_eq!(test_snapshot(&mut clipboard, primary), (None, None));
                offer_test_external(&mut clipboard, primary, text);
                assert_eq!(test_snapshot(&mut clipboard, primary), (None, None),);
                clipboard.keyboard_focus_gained();
                let (item, revision) = test_snapshot(&mut clipboard, primary);
                assert_eq!(item.as_deref(), Some(text));
                assert!(revision.is_some());
                assert_ne!(revision, previous_revision);
                assert_eq!(test_snapshot(&mut clipboard, !primary), (None, None),);
                previous_revision = revision;
            }
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let old_token = clipboard.start_focus_sync().expect("old focus sync");
            let cancelled = capture_test_snapshot(&mut clipboard, primary);
            assert!(!cancelled.is_ready());
            clipboard.keyboard_focus_lost().resolve();
            assert_eq!(
                resolved_test_snapshot(&cancelled),
                Err(ClipboardReadError::Unavailable)
            );
            let token = clipboard.keyboard_focus_gained();
            let pending = capture_test_snapshot(&mut clipboard, primary);
            assert!(clipboard.start_focus_sync().is_none());
            clipboard.keyboard_focus_synced(&old_token).resolve();
            assert!(!pending.is_ready());
            let sync = clipboard.start_focus_sync().expect("current focus sync");
            clipboard.keyboard_focus_timed_out(&old_token).resolve();
            assert!(!pending.is_ready());
            clipboard.keyboard_focus_timed_out(&token).resolve();
            assert_eq!(
                resolved_test_snapshot(&pending),
                Err(ClipboardReadError::Unavailable)
            );
            let rejected = capture_test_snapshot(&mut clipboard, primary);
            assert_eq!(
                resolved_test_snapshot(&rejected),
                Err(ClipboardReadError::Unavailable)
            );
            clipboard.keyboard_focus_synced(&sync).resolve();
            assert!(clipboard.can_publish(primary));
            let recovered = capture_test_snapshot(&mut clipboard, primary);
            assert_eq!(
                resolved_test_snapshot(&recovered),
                Ok((Some("third".to_owned()), previous_revision.clone()))
            );
            assert_eq!(
                resolved_test_snapshot(&pending),
                Err(ClipboardReadError::Unavailable)
            );
            assert_eq!(
                resolved_test_snapshot(&rejected),
                Err(ClipboardReadError::Unavailable)
            );
            offer_test_external(&mut clipboard, primary, "recovered");
            let recovered = capture_test_snapshot(&mut clipboard, primary);
            assert_eq!(
                resolved_test_snapshot(&recovered)
                    .expect("fresh offer")
                    .0
                    .as_deref(),
                Some("recovered")
            );
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let dropped = capture_test_snapshot(&mut clipboard, primary);
            drop(clipboard);
            assert_eq!(
                resolved_test_snapshot(&dropped),
                Err(ClipboardReadError::Unavailable)
            );
        }
    }

    #[gpui::test]
    fn test_clipboard_capture_completion_order_and_immutability(cx: &mut TestAppContext) {
        for primary in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let event_loop = EventLoop::try_new().expect("event loop");
            let mut clipboard = Clipboard::new(
                connection,
                event_loop.handle(),
                cx.background_executor.clone(),
                cx.foreground_executor.clone(),
            );
            offer_test_external(&mut clipboard, primary, "A");
            clipboard.keyboard_focus_gained();
            let frozen = capture_test_snapshot(&mut clipboard, primary);
            let revision = if primary {
                clipboard.primary_revision.clone()
            } else {
                clipboard.revision.clone()
            };
            let expected = Ok((Some("A".to_owned()), revision));
            assert_eq!(resolved_test_snapshot(&frozen), expected);
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let first = capture_test_snapshot(&mut clipboard, primary);
            let second = capture_test_snapshot(&mut clipboard, primary);
            let order = Rc::new(RefCell::new(Vec::new()));
            for (index, capture) in [&first, &second].into_iter().enumerate() {
                let order = order.clone();
                capture.on_ready(move |result| {
                    order
                        .borrow_mut()
                        .push((index, result.expect("resolved capture").revision.clone()));
                });
            }
            let token = clipboard
                .start_focus_sync()
                .expect("same-client focus sync");
            clipboard.keyboard_focus_synced(&token).resolve();
            assert_eq!(resolved_test_snapshot(&second), expected);
            let revision = expected.clone().expect("original snapshot").1;
            assert_eq!(*order.borrow(), [(0, revision.clone()), (1, revision)]);
            offer_test_external(&mut clipboard, primary, "B");
            assert_eq!(resolved_test_snapshot(&frozen), expected);
            assert_eq!(resolved_test_snapshot(&second), expected);
            let fresh = capture_test_snapshot(&mut clipboard, primary);
            assert_eq!(
                resolved_test_snapshot(&fresh)
                    .expect("new capture")
                    .0
                    .as_deref(),
                Some("B")
            );

            let source = set_test_source(&mut clipboard, primary, "owned");
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let owned = capture_test_snapshot(&mut clipboard, primary);
            assert!(!owned.is_ready());
            if primary {
                clipboard.cancel_primary(&source);
            } else {
                clipboard.cancel(&source);
            }
            let token = clipboard.start_focus_sync().expect("cancelled source sync");
            clipboard.keyboard_focus_synced(&token).resolve();
            assert_eq!(
                resolved_test_snapshot(&owned),
                Err(ClipboardReadError::Unavailable)
            );

            offer_test_external(&mut clipboard, primary, "old");
            clipboard.keyboard_focus_lost().resolve();
            clipboard.keyboard_focus_gained();
            let replacement = capture_test_snapshot(&mut clipboard, primary);
            assert!(!replacement.is_ready());
            let source = set_test_source(&mut clipboard, primary, "replacement");
            offer_test_source(&mut clipboard, primary, &source);
            assert_eq!(
                resolved_test_snapshot(&replacement),
                Ok((
                    Some("replacement".to_owned()),
                    Some(source.revision.clone())
                ))
            );
        }
    }

    #[gpui::test]
    fn test_clipboard_completion_callbacks_can_reenter(cx: &mut TestAppContext) {
        for primary in [false, true] {
            for event in ["sync", "selection", "leave", "timeout"] {
                let (socket, _server) = UnixStream::pair().expect("socket pair");
                let connection = Connection::from_socket(socket).expect("test connection");
                let event_loop = EventLoop::try_new().expect("event loop");
                let mut clipboard = Clipboard::new(
                    connection,
                    event_loop.handle(),
                    cx.background_executor.clone(),
                    cx.foreground_executor.clone(),
                );
                offer_test_external(&mut clipboard, primary, "A");
                clipboard.keyboard_focus_gained();
                clipboard.keyboard_focus_lost().resolve();
                let token = clipboard.keyboard_focus_gained();
                assert!(clipboard.start_focus_sync().is_some());
                let captures = [
                    capture_test_snapshot(&mut clipboard, primary),
                    capture_test_snapshot(&mut clipboard, primary),
                ];
                let clipboard = Rc::new(RefCell::new(clipboard));
                let observed = Rc::new(RefCell::new(Vec::new()));
                for (index, capture) in captures.iter().enumerate() {
                    let clipboard = clipboard.clone();
                    let observed = observed.clone();
                    capture.on_ready(move |result| {
                        let (snapshot, recaptured) = {
                            let mut clipboard = clipboard.borrow_mut();
                            (
                                test_snapshot(&mut clipboard, primary),
                                capture_test_snapshot(&mut clipboard, primary),
                            )
                        };
                        let expected = if event == "sync" || event == "selection" {
                            Ok(snapshot)
                        } else {
                            Err(ClipboardReadError::Unavailable)
                        };
                        assert_eq!(resolved_test_snapshot(&recaptured), expected);
                        observed.borrow_mut().push((
                            index,
                            result.map(|snapshot| {
                                snapshot.item.as_ref().and_then(|item| item.text())
                            }),
                        ));
                    });
                }
                let completions = {
                    let mut clipboard = clipboard.borrow_mut();
                    match event {
                        "sync" => clipboard.keyboard_focus_synced(&token),
                        "selection" if primary => clipboard.set_primary_offer(None),
                        "selection" => clipboard.set_offer(None),
                        "leave" => clipboard.keyboard_focus_lost(),
                        "timeout" => clipboard.keyboard_focus_timed_out(&token),
                        _ => unreachable!(),
                    }
                };
                assert!(observed.borrow().is_empty());
                assert!(captures.iter().all(|capture| !capture.is_ready()));
                completions.resolve();
                let expected = match event {
                    "sync" => Ok(Some("A".to_owned())),
                    "selection" => Ok(None),
                    _ => Err(ClipboardReadError::Unavailable),
                };
                assert_eq!(*observed.borrow(), [(0, expected.clone()), (1, expected)]);
            }
        }
    }

    #[gpui::test]
    async fn test_clipboard_capture_uncached_order(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        for deferred in [false, true] {
            let (socket, _server) = UnixStream::pair().expect("socket pair");
            let connection = Connection::from_socket(socket).expect("test connection");
            let reader = snapshot_test_reader(cx);
            let mut readiness = SelectionReadiness::default();
            if !deferred {
                readiness.authority = SelectionAuthority::Ready;
            }
            let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
            let first_revision = value.revision.clone();
            let first = readiness.capture(true, &mut value, &connection, &reader);
            let duplicate = readiness.capture(true, &mut value, &connection, &reader);
            if deferred {
                assert!(!first.is_ready());
                let completions = readiness.confirm(&mut value, &connection, &reader);
                assert_eq!(offer.requests.get(), 1);
                assert!(!first.is_ready());
                completions.resolve();
            }
            assert_eq!(offer.requests.get(), 1);
            assert!(!first.is_ready());
            assert!(!duplicate.is_ready());
            offer.valid.set(false);
            value.offer = None;
            cx.run_until_parked();
            let responsive = Rc::new(Cell::new(false));
            let foreground = cx.foreground_executor.spawn({
                let responsive = responsive.clone();
                async move {
                    responsive.set(true);
                }
            });
            foreground.await;
            assert!(responsive.get());
            assert!(!first.is_ready());

            let (mut replacement, replacement_offer) =
                uncached_test_value("text/plain;charset=utf-8");
            let second_revision = replacement.revision.clone();
            let second = readiness.capture(true, &mut replacement, &connection, &reader);
            cx.run_until_parked();
            let mut writer = replacement_offer.take_writer("text/plain;charset=utf-8");
            writer
                .write_all(b"second\r\n\r\r\n\r")
                .expect("second payload");
            drop(writer);
            let snapshot = second.wait().await.expect("second snapshot");
            assert_eq!(
                snapshot
                    .item
                    .as_ref()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("second\n\r\n\r")
            );
            assert_eq!(snapshot.revision, second_revision);
            assert!(!first.is_ready());
            let mut writer = offer.take_writer("text/plain;charset=utf-8");
            writer
                .write_all("first 🦀".as_bytes())
                .expect("first payload");
            drop(writer);
            let first_snapshot = first.wait().await.expect("historical snapshot");
            let duplicate_snapshot = duplicate.wait().await.expect("deduplicated snapshot");
            assert!(Arc::ptr_eq(&first_snapshot, &duplicate_snapshot));
            for snapshot in [first_snapshot, duplicate_snapshot] {
                assert_eq!(
                    snapshot
                        .item
                        .as_ref()
                        .and_then(|item| item.text())
                        .as_deref(),
                    Some("first 🦀")
                );
                assert_eq!(snapshot.revision, first_revision);
            }
        }
    }

    #[gpui::test]
    async fn test_clipboard_capture_reuses_completed_offer(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
        let mut readiness = SelectionReadiness::default();
        readiness.authority = SelectionAuthority::Ready;
        let first = readiness.capture(true, &mut value, &connection, &reader);
        offer
            .take_writer("text/plain;charset=utf-8")
            .write_all(b"cached")
            .expect("payload");
        let snapshot = first.wait().await.expect("first capture");
        drop(first);
        let second = readiness.capture(true, &mut value, &connection, &reader);
        assert_eq!(offer.requests.get(), 1);
        assert!(second.is_ready());
        assert!(Arc::ptr_eq(
            &snapshot,
            &second.wait().await.expect("cached capture"),
        ));
    }

    #[gpui::test]
    async fn test_clipboard_capture_cache_ignores_stale_completion(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        let event_loop = EventLoop::try_new().expect("event loop");
        let mut clipboard = Clipboard::new(
            connection.clone(),
            event_loop.handle(),
            cx.background_executor.clone(),
            cx.foreground_executor.clone(),
        );
        let (mut value, old_offer) = uncached_test_value("text/plain;charset=utf-8");
        value.cached = clipboard.cached_read.clone();
        value.revision = clipboard.revision.clone();
        let old = value.capture(&connection, &reader);
        clipboard.set_offer(None).resolve();
        let (mut replacement, new_offer) = uncached_test_value("text/plain;charset=utf-8");
        replacement.cached = clipboard.cached_read.clone();
        replacement.revision = clipboard.revision.clone();
        value = replacement;
        let new = value.capture(&connection, &reader);
        new_offer
            .take_writer("text/plain;charset=utf-8")
            .write_all(b"new")
            .expect("new payload");
        let snapshot = new.wait().await.expect("new capture");
        assert!(Arc::ptr_eq(
            &snapshot,
            clipboard.cached_read.get().expect("current cache"),
        ));
        drop(new);
        old_offer
            .take_writer("text/plain;charset=utf-8")
            .write_all(b"old")
            .expect("old payload");
        assert_eq!(
            old.wait()
                .await
                .expect("old capture")
                .item
                .as_ref()
                .and_then(ClipboardItem::text)
                .as_deref(),
            Some("old"),
        );
        drop(old);
        let cached = value.capture(&connection, &reader);
        assert_eq!(new_offer.requests.get(), 1);
        assert!(cached.is_ready());
        assert!(Arc::ptr_eq(
            &snapshot,
            &cached.wait().await.expect("current capture"),
        ));
    }

    #[gpui::test]
    async fn test_clipboard_capture_idle_timeout_resets_on_progress(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        let (value, offer) = uncached_test_value("text/plain;charset=utf-8");
        let read = value.capture(&connection, &reader);
        let mut writer = offer.take_writer("text/plain;charset=utf-8");
        cx.run_until_parked();
        for length in 1..=3 {
            cx.background_executor.advance_clock(Duration::from_secs(3));
            cx.run_until_parked();
            assert!(!read.is_ready());
            let (sender, consumed) = futures::channel::oneshot::channel();
            *reader.progress.lock() = Some(sender);
            writer.write_all(b"x").expect("progress");
            assert_eq!(consumed.await.expect("read observed"), length);
        }
        drop(writer);
        assert_eq!(
            read.wait()
                .await
                .expect("progressing capture")
                .item
                .as_ref()
                .and_then(ClipboardItem::text)
                .as_deref(),
            Some("xxx"),
        );
    }

    #[gpui::test]
    async fn test_clipboard_capture_idle_timeout_and_cancellation(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
        for mime_type in ["image/png", "image/jpeg"] {
            value
                .offer
                .as_mut()
                .expect("offer")
                .add_mime_type(mime_type.to_owned());
        }
        let read = value.capture(&connection, &reader);
        offer
            .take_writer("image/jpeg")
            .write_all(b"completed fallback")
            .expect("JPEG payload");
        cx.run_until_parked();
        assert!(!read.is_ready());
        cx.background_executor.advance_clock(Duration::from_secs(3));
        let (sender, consumed) = futures::channel::oneshot::channel();
        *reader.progress.lock() = Some(sender);
        let mut writer = offer.take_writer("text/plain;charset=utf-8");
        writer.write_all(&[0xff]).expect("invalid text payload");
        assert_eq!(consumed.await.expect("text read observed"), 1);
        drop(writer);
        let (sender, consumed) = futures::channel::oneshot::channel();
        *reader.progress.lock() = Some(sender);
        let mut writer = offer.take_writer("image/png");
        writer.write_all(b"p").expect("partial image payload");
        assert_eq!(consumed.await.expect("fallback read observed"), 1);
        assert!(!read.is_ready());
        cx.background_executor.advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert!(!read.is_ready());
        cx.background_executor.advance_clock(Duration::from_secs(3));
        let snapshot = read.wait().await.expect("fallback after idle timeout");
        let [ClipboardEntry::Image(image)] =
            snapshot.item.as_ref().expect("image").entries.as_slice()
        else {
            panic!("expected JPEG fallback");
        };
        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.bytes, b"completed fallback");

        for deferred in [false, true] {
            let mut readiness = SelectionReadiness::default();
            if !deferred {
                readiness.authority = SelectionAuthority::Ready;
            }
            let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
            let first = readiness.capture(true, &mut value, &connection, &reader);
            let second = readiness.capture(true, &mut value, &connection, &reader);
            drop(first);
            if deferred {
                readiness
                    .confirm(&mut value, &connection, &reader)
                    .resolve();
            }
            cx.run_until_parked();
            assert_eq!(offer.requests.get(), 1);
            let mut writer = offer.take_writer("text/plain;charset=utf-8");
            drop(second);
            cx.run_until_parked();
            assert!(
                readiness
                    .contents
                    .as_ref()
                    .expect("deduplication entry")
                    .1
                    .upgrade()
                    .is_none()
            );
            assert_eq!(
                writer.write(b"cancelled").expect_err("closed pipe").kind(),
                ErrorKind::BrokenPipe
            );
        }

        let mut readiness = SelectionReadiness::default();
        let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
        let abandoned = readiness.capture(true, &mut value, &connection, &reader);
        drop(abandoned);
        readiness
            .confirm(&mut value, &connection, &reader)
            .resolve();
        assert_eq!(offer.requests.get(), 0);

        let (value, offer) = uncached_test_value("text/plain;charset=utf-8");
        let read = value.capture(&connection, &reader);
        let mut writer = offer.take_writer("text/plain;charset=utf-8");
        cx.run_until_parked();
        cx.background_executor.advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        assert_eq!(
            writer.write(b"late").expect_err("timed out pipe").kind(),
            ErrorKind::BrokenPipe
        );
        assert_eq!(
            resolved_test_snapshot(&read),
            Err(ClipboardReadError::Unavailable)
        );
    }

    #[gpui::test]
    async fn test_clipboard_capture_large_payload_and_failures(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        for length in [0, 8192, 64 * 1024 * 1024 + 1] {
            let (value, offer) = uncached_test_value("image/png");
            let read = value.capture(&connection, &reader);
            let writer = offer.take_writer("image/png");
            let write = cx.background_executor.spawn(async move {
                let mut writer = smol::Async::new(writer).expect("async writer");
                let chunk = [b'x'; 8192];
                let mut remaining = length;
                while remaining > 0 {
                    let count = remaining.min(chunk.len());
                    writer
                        .write_all(&chunk[..count])
                        .await
                        .expect("payload chunk");
                    remaining -= count;
                }
            });
            let result = read.wait().await;
            write.await;
            let snapshot = result.expect("complete payload");
            let item = snapshot.item.as_ref().expect("image item");
            let [ClipboardEntry::Image(image)] = item.entries.as_slice() else {
                panic!("expected one image");
            };
            assert_eq!(image.format, ImageFormat::Png);
            assert_eq!(image.bytes.len(), length);
            assert_eq!(
                image.bytes.iter().filter(|byte| **byte == b'x').count(),
                length
            );
            drop(read);
            let cached = value.capture(&connection, &reader);
            assert!(cached.is_ready());
            assert!(Arc::ptr_eq(
                &snapshot,
                &cached.wait().await.expect("cached payload")
            ));
            assert_eq!(offer.requests.get(), 1);
        }
        let (value, offer) = uncached_test_value("text/plain;charset=utf-8");
        let read = value.capture(&connection, &reader);
        offer
            .take_writer("text/plain;charset=utf-8")
            .write_all(&[0xff])
            .expect("invalid UTF-8");
        assert_eq!(
            read.wait().await.expect_err("invalid UTF-8 rejected"),
            ClipboardReadError::UnsupportedContent
        );
    }

    #[gpui::test]
    async fn test_clipboard_capture_format_fallback(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        for (first, second, format) in [
            ("text/plain;charset=utf-8", "image/png", ImageFormat::Png),
            ("image/png", "image/jpeg", ImageFormat::Jpeg),
        ] {
            let (mut value, offer) = uncached_test_value(first);
            value
                .offer
                .as_mut()
                .expect("offer")
                .add_mime_type(second.to_owned());
            let mut readiness = SelectionReadiness::default();
            readiness.authority = SelectionAuthority::Ready;
            let read = readiness.capture(true, &mut value, &connection, &reader);
            assert_eq!(offer.requests.get(), 2);
            offer.valid.set(false);
            value.offer = None;
            offer
                .take_writer(second)
                .write_all(b"fallback")
                .expect("fallback payload");
            let mut writer = offer.take_writer(first);
            if first == "image/png" {
                cx.run_until_parked();
                cx.background_executor.advance_clock(Duration::from_secs(4));
                cx.run_until_parked();
            } else {
                writer.write_all(&[0xff]).expect("invalid text");
            }
            drop(writer);
            let snapshot = read.wait().await.expect("fallback snapshot");
            let [ClipboardEntry::Image(image)] =
                snapshot.item.as_ref().expect("image").entries.as_slice()
            else {
                panic!("expected one image");
            };
            assert_eq!(image.format, format);
            assert_eq!(image.bytes, b"fallback");
            assert_eq!(snapshot.revision, value.revision);
        }
    }

    #[gpui::test]
    async fn test_clipboard_capture_preserves_format_precedence(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        for (preferred, fallback, length) in [
            ("text/plain;charset=utf-8", "image/png", 0),
            ("text/plain;charset=utf-8", "image/png", 8193),
            ("image/png", "image/jpeg", 8193),
        ] {
            let (mut value, offer) = uncached_test_value(preferred);
            for mime_type in [fallback, "image/gif"] {
                value
                    .offer
                    .as_mut()
                    .expect("offer")
                    .add_mime_type(mime_type.to_owned());
            }
            let read = value.capture(&connection, &reader);
            assert_eq!(offer.requests.get(), 3);
            offer.valid.set(false);
            value.offer = None;
            let mut fallback_writer =
                smol::Async::new(offer.take_writer(fallback)).expect("async fallback writer");
            let payload = vec![b'p'; 1024 * 1024];
            let written = fallback_writer
                .get_ref()
                .write(&payload)
                .expect("fill fallback pipe");
            assert!(written < payload.len());
            assert_eq!(
                fallback_writer
                    .get_ref()
                    .write(b"p")
                    .expect_err("full fallback pipe")
                    .kind(),
                ErrorKind::WouldBlock,
            );
            let (sender, consumed) = futures::channel::oneshot::channel();
            *reader.progress.lock() = Some(sender);
            cx.run_until_parked();
            assert_eq!(
                consumed
                    .now_or_never()
                    .expect("fallback drained before preferred payload")
                    .expect("fallback progress"),
                written.min(8192),
            );
            fallback_writer
                .write_all(&payload[written..])
                .await
                .expect("drain fallback before preferred payload");
            drop(fallback_writer);
            let mut unused_writer = offer.take_writer("image/gif");
            cx.run_until_parked();
            assert!(!read.is_ready());
            let writer = offer.take_writer(preferred);
            let write = cx.background_executor.spawn(async move {
                let mut writer = smol::Async::new(writer).expect("async writer");
                let chunk = [b'x'; 8192];
                let mut remaining = length;
                while remaining > 0 {
                    let count = remaining.min(chunk.len());
                    writer
                        .write_all(&chunk[..count])
                        .await
                        .expect("preferred payload chunk");
                    remaining -= count;
                }
            });
            let snapshot = read.wait().await.expect("preferred snapshot");
            write.await;
            let expected = if preferred == "image/png" {
                ClipboardItem {
                    entries: vec![ClipboardEntry::Image(Image::from_bytes(
                        ImageFormat::Png,
                        vec![b'x'; length],
                    ))],
                }
            } else {
                ClipboardItem::new_string("x".repeat(length))
            };
            assert_eq!(snapshot.item.as_ref(), Some(&expected));
            assert_eq!(snapshot.revision, value.revision);
            assert_eq!(
                unused_writer
                    .write(b"unused")
                    .expect_err("unused candidate canceled")
                    .kind(),
                ErrorKind::BrokenPipe
            );
        }
    }

    #[gpui::test]
    async fn test_clipboard_capture_concurrent_transfers(cx: &mut TestAppContext) {
        cx.dispatcher.allow_parking();
        let (socket, _server) = UnixStream::pair().expect("socket pair");
        let connection = Connection::from_socket(socket).expect("test connection");
        let reader = snapshot_test_reader(cx);
        for (count, images) in [(17, false), (2, true)] {
            let mut captures = Vec::new();
            for _ in 0..count {
                let (mut value, offer) = uncached_test_value("text/plain;charset=utf-8");
                if images {
                    for format in ImageFormat::iter() {
                        value
                            .offer
                            .as_mut()
                            .expect("offer")
                            .add_mime_type(format.mime_type().to_owned());
                    }
                }
                let read = value.capture(&connection, &reader);
                assert_eq!(offer.requests.get(), if images { 10 } else { 1 });
                assert!(!read.is_ready());
                captures.push((read, value, offer));
            }
            cx.run_until_parked();
            for (read, value, offer) in captures.into_iter().rev() {
                offer
                    .take_writer("text/plain;charset=utf-8")
                    .write_all(b"x")
                    .expect("payload");
                let snapshot = read.wait().await.expect("concurrent capture");
                assert_eq!(
                    snapshot
                        .item
                        .as_ref()
                        .and_then(ClipboardItem::text)
                        .as_deref(),
                    Some("x"),
                );
                assert_eq!(snapshot.revision, value.revision);
                for (_, mut writer) in offer.writers.borrow_mut().drain(..) {
                    assert_eq!(
                        writer
                            .write(b"unused")
                            .expect_err("unused candidate canceled")
                            .kind(),
                        ErrorKind::BrokenPipe,
                    );
                }
            }
        }
    }

    #[derive(Clone)]
    struct TestOffer {
        writers: Rc<RefCell<Vec<(String, File)>>>,
        requests: Rc<Cell<usize>>,
        valid: Rc<Cell<bool>>,
    }

    impl TestOffer {
        fn take_writer(&self, mime_type: &str) -> File {
            let mut writers = self.writers.borrow_mut();
            let index = writers
                .iter()
                .position(|(mime, _)| mime == mime_type)
                .expect("receive initiated");
            writers.remove(index).1
        }
    }

    impl ReceiveData for TestOffer {
        fn receive_data(&self, mime_type: String, fd: BorrowedFd<'_>) {
            assert!(self.valid.get());
            self.requests.set(self.requests.get() + 1);
            self.writers.borrow_mut().push((
                mime_type,
                File::from(fd.try_clone_to_owned().expect("duplicate writer")),
            ));
        }
    }

    fn uncached_test_value(mime_type: &str) -> (SelectionValue<TestOffer>, TestOffer) {
        let offer = TestOffer {
            writers: Rc::new(RefCell::new(Vec::new())),
            requests: Rc::new(Cell::new(0)),
            valid: Rc::new(Cell::new(true)),
        };
        let mut data_offer = DataOffer::new(offer.clone());
        data_offer.add_mime_type(mime_type.to_owned());
        (
            SelectionValue {
                source: None,
                revision: Some(ClipboardRevision::default()),
                offer: Some(data_offer),
                cached: Rc::default(),
            },
            offer,
        )
    }

    fn snapshot_test_reader(cx: &TestAppContext) -> SnapshotReader {
        SnapshotReader {
            background_executor: cx.background_executor.clone(),
            foreground_executor: cx.foreground_executor.clone(),
            timeout: PIPE_READ_TIMEOUT,
            progress: Arc::default(),
        }
    }

    fn capture_test_snapshot(clipboard: &mut Clipboard, primary: bool) -> ClipboardSnapshotRead {
        if primary {
            clipboard.capture_primary_snapshot()
        } else {
            clipboard.capture_snapshot()
        }
    }

    fn resolved_test_snapshot(
        capture: &ClipboardSnapshotRead,
    ) -> Result<(Option<String>, Option<ClipboardRevision>), ClipboardReadError> {
        let result = Rc::new(RefCell::new(None));
        capture.on_ready({
            let result = result.clone();
            move |snapshot| {
                *result.borrow_mut() = Some(snapshot.map(|snapshot| {
                    (
                        snapshot.item.as_ref().and_then(|item| item.text()),
                        snapshot.revision.clone(),
                    )
                }));
            }
        });
        result.borrow_mut().take().expect("completed capture")
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
            clipboard.set_primary_offer(Some(offer)).resolve();
        } else {
            let mut offer = DataOffer::new(WlDataOffer::inert(backend));
            offer.add_mime_type(source.mime_type.clone());
            clipboard.set_offer(Some(offer)).resolve();
        }
    }

    fn offer_test_external(clipboard: &mut Clipboard, primary: bool, text: &str) {
        let backend = clipboard.connection.backend().downgrade();
        let item = ClipboardItem::new_string(text.to_owned());
        if primary {
            let captures = std::mem::take(&mut clipboard.primary_readiness.captures);
            let mut offer = DataOffer::new(ZwpPrimarySelectionOfferV1::inert(backend));
            offer.add_mime_type("text/plain;charset=utf-8".to_owned());
            clipboard.set_primary_offer(Some(offer)).resolve();
            clipboard
                .cached_primary_read
                .set(Arc::new(ClipboardSnapshot {
                    item: Some(item),
                    revision: clipboard.primary_revision.clone(),
                }))
                .expect("uncached primary offer");
            clipboard.primary_readiness.captures = captures;
            let value = clipboard.primary_value();
            clipboard
                .primary_readiness
                .complete(&value, &clipboard.connection, &clipboard.snapshot_reader)
                .resolve();
        } else {
            let captures = std::mem::take(&mut clipboard.selection_readiness.captures);
            let mut offer = DataOffer::new(WlDataOffer::inert(backend));
            offer.add_mime_type("text/plain;charset=utf-8".to_owned());
            clipboard.set_offer(Some(offer)).resolve();
            clipboard
                .cached_read
                .set(Arc::new(ClipboardSnapshot {
                    item: Some(item),
                    revision: clipboard.revision.clone(),
                }))
                .expect("uncached clipboard offer");
            clipboard.selection_readiness.captures = captures;
            let value = clipboard.selection_value();
            clipboard
                .selection_readiness
                .complete(&value, &clipboard.connection, &clipboard.snapshot_reader)
                .resolve();
        }
    }

    fn test_snapshot(
        clipboard: &mut Clipboard,
        primary: bool,
    ) -> (Option<String>, Option<ClipboardRevision>) {
        let snapshot = if primary {
            clipboard.read_primary_snapshot()
        } else {
            clipboard.read_snapshot()
        };
        (
            snapshot.item.as_ref().and_then(|item| item.text()),
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
