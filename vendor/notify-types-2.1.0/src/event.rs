//! The `Event` type and the hierarchical `EventKind` descriptor.

use std::{
    fmt,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use bitflags::bitflags;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An event describing open or close operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AccessMode {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the file is executed, or the folder opened.
    Execute,

    /// An event emitted when the file is opened for reading.
    Read,

    /// An event emitted when the file is opened for writing.
    Write,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing non-mutating access operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "mode"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AccessKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the file is read.
    Read,

    /// An event emitted when the file, or a handle to the file, is opened.
    Open(AccessMode),

    /// An event emitted when the file, or a handle to the file, is closed.
    Close(AccessMode),

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing creation operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum CreateKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event which results in the creation of a file.
    File,

    /// An event which results in the creation of a folder.
    Folder,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the data content of a file is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum DataChange {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the size of the data is changed.
    Size,

    /// An event emitted when the content of the data is changed.
    Content,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the metadata of a file or folder is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum MetadataKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the access time of the file or folder is changed.
    AccessTime,

    /// An event emitted when the write or modify time of the file or folder is changed.
    WriteTime,

    /// An event emitted when the permissions of the file or folder are changed.
    Permissions,

    /// An event emitted when the ownership of the file or folder is changed.
    Ownership,

    /// An event emitted when an extended attribute of the file or folder is changed.
    ///
    /// If the extended attribute's name or type is known, it should be provided in the
    /// `Info` event attribute.
    Extended,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event emitted when the name of a file or folder is changed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum RenameMode {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted on the file or folder resulting from a rename.
    To,

    /// An event emitted on the file or folder that was renamed.
    From,

    /// A single event emitted with both the `From` and `To` paths.
    ///
    /// This event should be emitted when both source and target are known. The paths should be
    /// provided in this exact order (from, to).
    Both,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing mutation of content, name, or metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "mode"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ModifyKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when the data content of a file is changed.
    Data(DataChange),

    /// An event emitted when the metadata of a file or folder is changed.
    Metadata(MetadataKind),

    /// An event emitted when the name of a file or folder is changed.
    #[cfg_attr(feature = "serde", serde(rename = "rename"))]
    Name(RenameMode),

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// An event describing removal operations on files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum RemoveKind {
    /// The catch-all case, to be used when the specific kind of event is unknown.
    Any,

    /// An event emitted when a file is removed.
    File,

    /// An event emitted when a folder is removed.
    Folder,

    /// An event which specific kind is known but cannot be represented otherwise.
    Other,
}

/// Top-level event kind.
///
/// This is arguably the most important classification for events. All subkinds below this one
/// represent details that may or may not be available for any particular backend, but most tools
/// and Notify systems will only care about which of these four general kinds an event is about.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(
    all(feature = "serde", not(feature = "serialization-compat-6")),
    serde(tag = "type")
)]
pub enum EventKind {
    /// The catch-all event kind, for unsupported/unknown events.
    ///
    /// This variant should be used as the "else" case when mapping native kernel bitmasks or
    /// bitmaps, such that if the mask is ever extended with new event types the backend will not
    /// gain bugs due to not matching new unknown event types.
    ///
    /// This variant is also the default variant used when Notify is in "imprecise" mode.
    #[default]
    Any,

    /// An event describing non-mutating access operations on files.
    ///
    /// This event is about opening and closing file handles, as well as executing files, and any
    /// other such event that is about accessing files, folders, or other structures rather than
    /// mutating them.
    ///
    /// Only some platforms are capable of generating these.
    Access(AccessKind),

    /// An event describing creation operations on files.
    ///
    /// This event is about the creation of files, folders, or other structures but not about e.g.
    /// writing new content into them.
    Create(CreateKind),

    /// An event describing mutation of content, name, or metadata.
    ///
    /// This event is about the mutation of files', folders', or other structures' content, name
    /// (path), or associated metadata (attributes).
    Modify(ModifyKind),

    /// An event describing removal operations on files.
    ///
    /// This event is about the removal of files, folders, or other structures but not e.g. erasing
    /// content from them. This may also be triggered for renames/moves that move files _out of the
    /// watched subpath_.
    ///
    /// Some editors also trigger Remove events when saving files as they may opt for removing (or
    /// renaming) the original then creating a new file in-place.
    Remove(RemoveKind),

    /// An event not fitting in any of the above four categories.
    ///
    /// This may be used for meta-events about the watch itself.
    Other,
}

impl EventKind {
    /// Indicates whether an event is an Access variant.
    #[must_use]
    pub fn is_access(&self) -> bool {
        matches!(self, EventKind::Access(_))
    }

    /// Indicates whether an event is a Create variant.
    #[must_use]
    pub fn is_create(&self) -> bool {
        matches!(self, EventKind::Create(_))
    }

    /// Indicates whether an event is a Modify variant.
    #[must_use]
    pub fn is_modify(&self) -> bool {
        matches!(self, EventKind::Modify(_))
    }

    /// Indicates whether an event is a Remove variant.
    #[must_use]
    pub fn is_remove(&self) -> bool {
        matches!(self, EventKind::Remove(_))
    }

    /// Indicates whether an event is an Other variant.
    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self, EventKind::Other)
    }
}

bitflags! {
    /// A bitmask specifying which event kinds to monitor.
    ///
    /// This type allows fine-grained control over which filesystem events are reported.
    /// On backends that support kernel-level filtering (inotify), the mask is
    /// translated to native flags for optimal performance. On other backends (kqueue,
    /// Windows, FSEvents, PollWatcher), filtering is applied in userspace.
    ///
    /// # Examples
    ///
    /// ```
    /// use notify_types::event::EventKindMask;
    ///
    /// // Monitor only file creations and deletions
    /// let mask = EventKindMask::CREATE | EventKindMask::REMOVE;
    ///
    /// // Monitor everything including access events
    /// let all = EventKindMask::ALL;
    ///
    /// // Default: includes all events (matches Config::default())
    /// let default = EventKindMask::default();
    /// assert_eq!(default, EventKindMask::ALL);
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct EventKindMask: u32 {
        /// Monitor file/folder creation events.
        const CREATE = 0b0000_0001;

        /// Monitor file/folder removal events.
        const REMOVE = 0b0000_0010;

        /// Monitor data modification events (content/size changes).
        const MODIFY_DATA = 0b0000_0100;

        /// Monitor metadata modification events (permissions, timestamps, etc).
        const MODIFY_META = 0b0000_1000;

        /// Monitor name/rename events.
        const MODIFY_NAME = 0b0001_0000;

        /// Monitor file open events.
        const ACCESS_OPEN = 0b0010_0000;

        /// Monitor file close events after writing.
        /// This fires when a file opened for writing is closed.
        const ACCESS_CLOSE = 0b0100_0000;

        /// Monitor file close events after read-only access.
        /// This fires when a file opened for reading (not writing) is closed.
        /// Note: This can be very noisy and may cause queue overflow on busy systems.
        const ACCESS_CLOSE_NOWRITE = 0b1000_0000;

        /// All modify events (data, metadata, and name changes).
        const ALL_MODIFY = Self::MODIFY_DATA.bits() | Self::MODIFY_META.bits() | Self::MODIFY_NAME.bits();

        /// All access events (open, close-write, and close-nowrite).
        const ALL_ACCESS = Self::ACCESS_OPEN.bits() | Self::ACCESS_CLOSE.bits() | Self::ACCESS_CLOSE_NOWRITE.bits();

        /// Core events: create, remove, and all modify events.
        const CORE = Self::CREATE.bits() | Self::REMOVE.bits() | Self::ALL_MODIFY.bits();

        /// All events including access events.
        /// This is the default.
        const ALL = Self::CORE.bits() | Self::ALL_ACCESS.bits();
    }
}

impl Default for EventKindMask {
    fn default() -> Self {
        EventKindMask::ALL
    }
}

impl EventKindMask {
    /// Returns whether the given event kind matches this mask.
    ///
    /// `EventKind::Any` and `EventKind::Other` always pass regardless of the mask,
    /// as they represent meta-events that should not be filtered.
    ///
    /// # Examples
    ///
    /// ```
    /// use notify_types::event::{EventKindMask, EventKind, CreateKind, AccessKind, AccessMode};
    ///
    /// let mask = EventKindMask::CREATE;
    /// assert!(mask.matches(&EventKind::Create(CreateKind::File)));
    /// assert!(!mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Read))));
    ///
    /// // Any and Other always pass
    /// let empty = EventKindMask::empty();
    /// assert!(empty.matches(&EventKind::Any));
    /// assert!(empty.matches(&EventKind::Other));
    /// ```
    #[must_use]
    pub fn matches(&self, kind: &EventKind) -> bool {
        match kind {
            // Meta-events always pass
            EventKind::Any | EventKind::Other => true,

            // Create events
            EventKind::Create(_) => self.intersects(EventKindMask::CREATE),

            // Remove events
            EventKind::Remove(_) => self.intersects(EventKindMask::REMOVE),

            // Modify events - check subkind
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Data(_) => self.intersects(EventKindMask::MODIFY_DATA),
                ModifyKind::Metadata(_) => self.intersects(EventKindMask::MODIFY_META),
                ModifyKind::Name(_) => self.intersects(EventKindMask::MODIFY_NAME),
                // ModifyKind::Any and ModifyKind::Other pass if any modify flag is set
                ModifyKind::Any | ModifyKind::Other => self.intersects(EventKindMask::ALL_MODIFY),
            },

            // Access events - check subkind
            EventKind::Access(access_kind) => match access_kind {
                AccessKind::Open(_) => self.intersects(EventKindMask::ACCESS_OPEN),
                // Close after write
                AccessKind::Close(AccessMode::Write) => {
                    self.intersects(EventKindMask::ACCESS_CLOSE)
                }
                // Close after read-only (no write)
                AccessKind::Close(AccessMode::Read) => {
                    self.intersects(EventKindMask::ACCESS_CLOSE_NOWRITE)
                }
                // Close with unknown mode - match if either close flag is set
                AccessKind::Close(_) => self
                    .intersects(EventKindMask::ACCESS_CLOSE | EventKindMask::ACCESS_CLOSE_NOWRITE),
                // AccessKind::Read, Any, and Other pass if any access flag is set
                AccessKind::Read | AccessKind::Any | AccessKind::Other => {
                    self.intersects(EventKindMask::ALL_ACCESS)
                }
            },
        }
    }
}

/// Notify event.
///
/// You might want to check [`Event::need_rescan`] to make sure no event was missed before you
/// received this one.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Event {
    /// Kind or type of the event.
    ///
    /// This is a hierarchy of enums describing the event as precisely as possible. All enums in
    /// the hierarchy have two variants always present, `Any` and `Other`, accompanied by one or
    /// more specific variants.
    ///
    /// `Any` should be used when more detail about the event is not known beyond the variant
    /// already selected. For example, `AccessMode::Any` means a file has been accessed, but that's
    /// all we know.
    ///
    /// `Other` should be used when more detail _is_ available, but cannot be encoded as one of the
    /// defined variants. When specifying `Other`, the event attributes should contain an `Info`
    /// entry with a short string identifying this detail. That string is to be considered part of
    /// the interface of the backend (i.e. a change should probably be breaking).
    ///
    /// For example, `CreateKind::Other` with an `Info("mount")` may indicate the binding of a
    /// mount. The documentation of the particular backend should indicate if any `Other` events
    /// are generated, and what their description means.
    ///
    /// The `EventKind::Any` variant should be used as the "else" case when mapping native kernel
    /// bitmasks or bitmaps, such that if the mask is ever extended with new event types the
    /// backend will not gain bugs due to not matching new unknown event types.
    #[cfg_attr(
        all(feature = "serde", not(feature = "serialization-compat-6")),
        serde(flatten)
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "serialization-compat-6"),
        serde(rename = "type")
    )]
    pub kind: EventKind,

    /// Paths the event is about, if known.
    ///
    /// Notify backends report paths using the same root representation that was passed to
    /// `Watcher::watch`. For example, watching `src` reports paths like `src/lib.rs`, while
    /// watching an absolute path reports absolute event paths.
    ///
    /// If an event concerns two or more paths, and the paths are known at the time of event
    /// creation, they should all go in this `Vec`. Otherwise, using the `Tracker` attr may be more
    /// appropriate.
    ///
    /// The order of the paths is likely to be significant! For example, renames where both ends of
    /// the name change are known will have the "source" path first, and the "target" path last.
    pub paths: Vec<PathBuf>,

    // "What should be in the struct" and "what can go in the attrs" is an interesting question.
    //
    // Technically, the paths could go in the attrs. That would reduce the type size to 4 pointer
    // widths, instead of 7 like it is now. Anything 8 and below is probably good — on x64 that's
    // the size of an L1 cache line. The entire kind classification fits in 3 bytes, and an AnyMap
    // is 3 pointers. A Vec<PathBuf> is another 3 pointers.
    //
    // Type size aside, what's behind these structures? A Vec and a PathBuf is stored on the heap.
    // An AnyMap is stored on the heap. But a Vec is directly there, requiring about one access to
    // get, while retrieving anything in the AnyMap requires some accesses as overhead.
    //
    // So things that are used often should be on the struct, and things that are used more rarely
    // should go in the attrs. Additionally, arbitrary data can _only_ go in the attrs.
    //
    // The kind and the paths vie for first place on this scale, depending on how downstream wishes
    // to use the information. Everything else is secondary. So far, that's why paths live here.
    //
    // In the future, it might be possible to have more data and to benchmark things properly, so
    // the performance can be actually quantified. Also, it might turn out that I have no idea what
    // I was talking about, so the above may be discarded or reviewed. We'll see!
    //
    /// Additional attributes of the event.
    ///
    /// Arbitrary data may be added to this field, without restriction beyond the `Sync` and
    /// `Clone` properties. Some data added here is considered for comparing and hashing, but not
    /// all: at this writing this is `Tracker`, `Flag`, `Info`, and `Source`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub attrs: EventAttributes,
}

/// Additional attributes of the event.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventAttributes {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: Option<Box<EventAttributesInner>>,
}

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct EventAttributesInner {
    /// Tracking ID for events that are related.
    ///
    /// For events generated by backends with the `TrackRelated` capability. Those backends _may_
    /// emit events that are related to each other, and tag those with an identical "tracking id"
    /// or "cookie". The value is normalised to `usize`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    tracker: Option<usize>,

    /// Special Notify flag on the event.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    flag: Option<Flag>,

    /// Additional information on the event.
    ///
    /// This is to be used for all `Other` variants of the event kind hierarchy. The variant
    /// indicates that a consumer should look into the `attrs` for an `Info` value; if that value
    /// is missing it should be considered a backend bug.
    ///
    /// This attribute may also be present for non-`Other` variants of the event kind, if doing so
    /// provides useful precision. For example, the `Modify(Metadata(Extended))` kind suggests
    /// using this attribute when information about _what_ extended metadata changed is available.
    ///
    /// This should be a short string, and changes may be considered breaking.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    info: Option<String>,

    /// The source of the event.
    ///
    /// In most cases this should be a short string, identifying the backend unambiguously. In some
    /// cases this may be dynamically generated, but should contain a prefix to make it unambiguous
    /// between backends.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    source: Option<String>,

    /// The process ID of the originator of the event.
    ///
    /// This attribute is experimental and, while included in Notify itself, is not considered
    /// stable or standard enough to be part of the serde, eq, hash, and debug representations.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing, skip_deserializing)
    )]
    process_id: Option<u32>,
}

impl EventAttributes {
    /// Creates a new `EventAttributes`.
    #[must_use]
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// Retrieves the tracker ID for an event directly, if present.
    #[must_use]
    pub fn tracker(&self) -> Option<usize> {
        self.inner.as_ref().and_then(|inner| inner.tracker)
    }

    /// Retrieves the Notify flag for an event directly, if present.
    #[must_use]
    pub fn flag(&self) -> Option<Flag> {
        self.inner.as_ref().and_then(|inner| inner.flag)
    }

    /// Returns additional, backend-provided information about this event, if present.
    ///
    /// This attribute is primarily used to disambiguate the various `Other` variants in the
    /// [`EventKind`] hierarchy (for example [`EventKind::Other`], [`CreateKind::Other`],
    /// [`ModifyKind::Other`], [`RemoveKind::Other`], [`MetadataKind::Other`], ...).
    ///
    /// When a backend emits an `Other` kind, it should also set this attribute to a short,
    /// human-readable string explaining what happened. If an event kind indicates `Other` but
    /// this value is missing, treat it as a backend bug.
    ///
    /// Backends may also set this attribute for non-`Other` kinds, if doing so provides useful
    /// precision. For example, the `Modify(Metadata(Extended))` kind suggests using this
    /// attribute when information about _what_ extended metadata changed is available.
    ///
    /// # Stability
    ///
    /// The exact string values are backend-defined. They are intended as a hint for consumers and
    /// may change between notify versions.
    ///
    /// # Examples
    ///
    /// `notify` backends currently use values such as:
    ///
    /// - `unmount` (inotify unmount events)
    /// - `rescan: user dropped` / `rescan: kernel dropped` (FSEvents queue overflow hints)
    /// - `root changed`, `mount`
    /// - `is: symlink`, `is: hardlink`, `is: clone`
    /// - `meta: finder info`
    /// - `override` (debouncer-generated synthetic events)
    #[must_use]
    pub fn info(&self) -> Option<&str> {
        self.inner.as_ref().and_then(|inner| inner.info.as_deref())
    }

    /// Retrieves the source for an event directly, if present.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.inner
            .as_ref()
            .and_then(|inner| inner.source.as_deref())
    }

    /// The process ID of the originator of the event.
    ///
    /// This attribute is experimental and, while included in Notify itself, is not considered
    /// stable or standard enough to be part of the serde, eq, hash, and debug representations.
    #[must_use]
    pub fn process_id(&self) -> Option<u32> {
        self.inner.as_ref().and_then(|inner| inner.process_id)
    }

    /// Sets the tracker.
    pub fn set_tracker(&mut self, tracker: usize) {
        self.inner_mut().tracker = Some(tracker);
    }

    /// Sets the Notify flag onto the event.
    pub fn set_flag(&mut self, flag: Flag) {
        self.inner_mut().flag = Some(flag);
    }

    /// Sets [`EventAttributes::info`].
    ///
    /// This should be a short, human-readable string.
    pub fn set_info(&mut self, info: &str) {
        self.inner_mut().info = Some(info.to_string());
    }

    /// Sets the process id onto the event.
    pub fn set_process_id(&mut self, process_id: u32) {
        self.inner_mut().process_id = Some(process_id)
    }

    fn inner_mut(&mut self) -> &mut EventAttributesInner {
        self.inner.get_or_insert_with(Box::default)
    }
}

/// Special Notify flag on the event.
///
/// This attribute is used to flag certain kinds of events that Notify either marks or generates in
/// particular ways.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    all(feature = "serde", not(feature = "serialization-compat-6")),
    serde(rename_all = "camelCase")
)]
pub enum Flag {
    /// Rescan notices are emitted by some platforms (and may also be emitted by Notify itself).
    /// They indicate either a lapse in the events or a change in the filesystem such that events
    /// received so far can no longer be relied on to represent the state of the filesystem now.
    ///
    /// An application that simply reacts to file changes may not care about this. An application
    /// that keeps an in-memory representation of the filesystem will need to care, and will need
    /// to refresh that representation directly from the filesystem.
    Rescan,
}

impl Event {
    /// Returns whether some events may have been missed. If true, you should assume any file or
    /// folder might have been modified.
    ///
    /// See [`Flag::Rescan`] for more information.
    #[must_use]
    pub fn need_rescan(&self) -> bool {
        matches!(self.flag(), Some(Flag::Rescan))
    }
    /// Retrieves the tracker ID for an event directly, if present.
    #[must_use]
    pub fn tracker(&self) -> Option<usize> {
        self.attrs.tracker()
    }

    /// Retrieves the Notify flag for an event directly, if present.
    #[must_use]
    pub fn flag(&self) -> Option<Flag> {
        self.attrs.flag()
    }

    /// Returns [`EventAttributes::info`].
    #[must_use]
    pub fn info(&self) -> Option<&str> {
        self.attrs.info()
    }

    /// Retrieves the source for an event directly, if present.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.attrs.source()
    }

    /// Creates a new `Event` given a kind.
    #[must_use]
    pub fn new(kind: EventKind) -> Self {
        Self {
            kind,
            paths: Vec::new(),
            attrs: EventAttributes::new(),
        }
    }

    /// Sets the kind.
    #[must_use]
    pub fn set_kind(mut self, kind: EventKind) -> Self {
        self.kind = kind;
        self
    }

    /// Adds a path to the event.
    #[must_use]
    pub fn add_path(mut self, path: PathBuf) -> Self {
        self.paths.push(path);
        self
    }

    /// Adds a path to the event if the argument is Some.
    #[must_use]
    pub fn add_some_path(self, path: Option<PathBuf>) -> Self {
        if let Some(path) = path {
            self.add_path(path)
        } else {
            self
        }
    }

    /// Sets the tracker.
    #[must_use]
    pub fn set_tracker(mut self, tracker: usize) -> Self {
        self.attrs.set_tracker(tracker);
        self
    }

    /// Sets [`EventAttributes::info`].
    #[must_use]
    pub fn set_info(mut self, info: &str) -> Self {
        self.attrs.set_info(info);
        self
    }

    /// Sets the Notify flag onto the event.
    #[must_use]
    pub fn set_flag(mut self, flag: Flag) -> Self {
        self.attrs.set_flag(flag);
        self
    }

    /// Sets the process id onto the event.
    #[must_use]
    pub fn set_process_id(mut self, process_id: u32) -> Self {
        self.attrs.set_process_id(process_id);
        self
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("kind", &self.kind)
            .field("paths", &self.paths)
            .field("attr:tracker", &self.tracker())
            .field("attr:flag", &self.flag())
            .field("attr:info", &self.info())
            .field("attr:source", &self.source())
            .finish()
    }
}
impl Default for Event {
    fn default() -> Self {
        Self {
            kind: EventKind::default(),
            paths: Vec::new(),
            attrs: EventAttributes::new(),
        }
    }
}

impl Eq for Event {}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
            && self.paths.eq(&other.paths)
            && self.tracker().eq(&other.tracker())
            && self.flag().eq(&other.flag())
            && self.info().eq(&other.info())
            && self.source().eq(&other.source())
    }
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.paths.hash(state);
        self.tracker().hash(state);
        self.flag().hash(state);
        self.info().hash(state);
        self.source().hash(state);
    }
}

#[cfg(test)]
mod event_kind_mask_tests {
    use super::*;

    #[test]
    fn default_is_all() {
        assert_eq!(EventKindMask::default(), EventKindMask::ALL);
    }

    #[test]
    fn matches_create_events() {
        let mask = EventKindMask::CREATE;
        assert!(mask.matches(&EventKind::Create(CreateKind::File)));
        assert!(mask.matches(&EventKind::Create(CreateKind::Folder)));
        assert!(mask.matches(&EventKind::Create(CreateKind::Any)));
        assert!(mask.matches(&EventKind::Create(CreateKind::Other)));
        assert!(!mask.matches(&EventKind::Remove(RemoveKind::File)));
    }

    #[test]
    fn matches_remove_events() {
        let mask = EventKindMask::REMOVE;
        assert!(mask.matches(&EventKind::Remove(RemoveKind::File)));
        assert!(mask.matches(&EventKind::Remove(RemoveKind::Folder)));
        assert!(mask.matches(&EventKind::Remove(RemoveKind::Any)));
        assert!(!mask.matches(&EventKind::Create(CreateKind::File)));
    }

    #[test]
    fn matches_access_open_events() {
        let mask = EventKindMask::ACCESS_OPEN;
        assert!(mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Read))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Write))));
        assert!(!mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
        assert!(!mask.matches(&EventKind::Create(CreateKind::File)));
    }

    #[test]
    fn matches_access_close_events() {
        // ACCESS_CLOSE only matches Close(Write), not Close(Read)
        let mask = EventKindMask::ACCESS_CLOSE;
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Any)))); // Any could be write
        assert!(!mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Read)))); // Read goes to NOWRITE
        assert!(!mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
    }

    #[test]
    fn matches_access_close_nowrite_events() {
        // ACCESS_CLOSE_NOWRITE matches Close(Read) - files opened read-only
        let mask = EventKindMask::ACCESS_CLOSE_NOWRITE;
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Read))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Any)))); // Any could be read
        assert!(!mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write)))); // Write goes to ACCESS_CLOSE
        assert!(!mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
    }

    #[test]
    fn combined_close_masks_match_both() {
        // When both ACCESS_CLOSE and ACCESS_CLOSE_NOWRITE are set, match both
        let mask = EventKindMask::ACCESS_CLOSE | EventKindMask::ACCESS_CLOSE_NOWRITE;
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Read))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Any))));
    }

    #[test]
    fn all_access_matches_open_close_read_any_other() {
        let mask = EventKindMask::ALL_ACCESS;
        assert!(mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Read))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
        assert!(mask.matches(&EventKind::Access(AccessKind::Read)));
        assert!(mask.matches(&EventKind::Access(AccessKind::Any)));
        assert!(mask.matches(&EventKind::Access(AccessKind::Other)));
    }

    #[test]
    fn matches_modify_data_events() {
        let mask = EventKindMask::MODIFY_DATA;
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Size))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Content))));
        assert!(!mask.matches(&EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any))));
        assert!(!mask.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::From))));
    }

    #[test]
    fn matches_modify_metadata_events() {
        let mask = EventKindMask::MODIFY_META;
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Metadata(
            MetadataKind::Permissions
        ))));
        assert!(!mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
    }

    #[test]
    fn matches_modify_name_events() {
        let mask = EventKindMask::MODIFY_NAME;
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::From))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::To))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::Both))));
        assert!(!mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
    }

    #[test]
    fn all_modify_matches_data_meta_name() {
        let mask = EventKindMask::ALL_MODIFY;
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::From))));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Any)));
        assert!(mask.matches(&EventKind::Modify(ModifyKind::Other)));
    }

    #[test]
    fn any_and_other_always_pass() {
        let empty = EventKindMask::empty();
        assert!(empty.matches(&EventKind::Any));
        assert!(empty.matches(&EventKind::Other));
    }

    #[test]
    fn core_excludes_access() {
        let core = EventKindMask::CORE;
        assert!(core.matches(&EventKind::Create(CreateKind::File)));
        assert!(core.matches(&EventKind::Remove(RemoveKind::File)));
        assert!(core.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(core.matches(&EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any))));
        assert!(core.matches(&EventKind::Modify(ModifyKind::Name(RenameMode::From))));
        assert!(!core.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
        assert!(!core.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
    }

    #[test]
    fn empty_mask_only_passes_any_other() {
        let empty = EventKindMask::empty();
        assert!(empty.matches(&EventKind::Any));
        assert!(empty.matches(&EventKind::Other));
        assert!(!empty.matches(&EventKind::Create(CreateKind::File)));
        assert!(!empty.matches(&EventKind::Remove(RemoveKind::File)));
        assert!(!empty.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(!empty.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
    }

    #[test]
    fn bitwise_or_combines_masks() {
        let mask = EventKindMask::CREATE | EventKindMask::REMOVE;
        assert!(mask.matches(&EventKind::Create(CreateKind::File)));
        assert!(mask.matches(&EventKind::Remove(RemoveKind::Folder)));
        assert!(!mask.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(!mask.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
    }

    #[test]
    fn all_matches_everything() {
        let all = EventKindMask::ALL;
        assert!(all.matches(&EventKind::Any));
        assert!(all.matches(&EventKind::Other));
        assert!(all.matches(&EventKind::Create(CreateKind::File)));
        assert!(all.matches(&EventKind::Remove(RemoveKind::File)));
        assert!(all.matches(&EventKind::Modify(ModifyKind::Data(DataChange::Any))));
        assert!(all.matches(&EventKind::Access(AccessKind::Open(AccessMode::Any))));
        assert!(all.matches(&EventKind::Access(AccessKind::Close(AccessMode::Write))));
    }

    #[test]
    fn access_read_and_any_match_with_all_access() {
        // Edge case: AccessKind::Read and AccessKind::Any should match if ALL_ACCESS is set
        let mask = EventKindMask::ALL_ACCESS;
        assert!(mask.matches(&EventKind::Access(AccessKind::Read)));
        assert!(mask.matches(&EventKind::Access(AccessKind::Any)));
        assert!(mask.matches(&EventKind::Access(AccessKind::Other)));
    }
}

#[cfg(all(test, feature = "serde", not(feature = "serialization-compat-6")))]
mod tests {
    use super::*;

    use insta::assert_snapshot;
    use rstest::rstest;

    #[rustfmt::skip]
    #[rstest]
    #[case("any", EventKind::Any)]
    #[case("access-any", EventKind::Access(AccessKind::Any))]
    #[case("access-read", EventKind::Access(AccessKind::Read))]
    #[case("access-open-any", EventKind::Access(AccessKind::Open(AccessMode::Any)))]
    #[case("access-open-execute", EventKind::Access(AccessKind::Open(AccessMode::Execute)))]
    #[case("access-open-read", EventKind::Access(AccessKind::Open(AccessMode::Read)))]
    #[case("access-open-write", EventKind::Access(AccessKind::Open(AccessMode::Write)))]
    #[case("access-open-other", EventKind::Access(AccessKind::Open(AccessMode::Other)))]
    #[case("access-close-any", EventKind::Access(AccessKind::Close(AccessMode::Any)))]
    #[case("access-close-execute", EventKind::Access(AccessKind::Close(AccessMode::Execute)))]
    #[case("access-close-read", EventKind::Access(AccessKind::Close(AccessMode::Read)))]
    #[case("access-close-write", EventKind::Access(AccessKind::Close(AccessMode::Write)))]
    #[case("access-close-other", EventKind::Access(AccessKind::Close(AccessMode::Other)))]
    #[case("access-other", EventKind::Access(AccessKind::Other))]
    #[case("create-any", EventKind::Create(CreateKind::Any))]
    #[case("create-file", EventKind::Create(CreateKind::File))]
    #[case("create-folder", EventKind::Create(CreateKind::Folder))]
    #[case("create-other", EventKind::Create(CreateKind::Other))]
    #[case("modify-any", EventKind::Modify(ModifyKind::Any))]
    #[case("modify-data-any", EventKind::Modify(ModifyKind::Data(DataChange::Any)))]
    #[case("modify-data-size", EventKind::Modify(ModifyKind::Data(DataChange::Size)))]
    #[case("modify-data-content", EventKind::Modify(ModifyKind::Data(DataChange::Content)))]
    #[case("modify-data-other", EventKind::Modify(ModifyKind::Data(DataChange::Other)))]
    #[case("modify-metadata-any", EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)))]
    #[case("modify-metadata-accesstime", EventKind::Modify(ModifyKind::Metadata(MetadataKind::AccessTime)))]
    #[case("modify-metadata-writetime", EventKind::Modify(ModifyKind::Metadata(MetadataKind::WriteTime)))]
    #[case("modify-metadata-permissions", EventKind::Modify(ModifyKind::Metadata(MetadataKind::Permissions)))]
    #[case("modify-metadata-ownership", EventKind::Modify(ModifyKind::Metadata(MetadataKind::Ownership)))]
    #[case("modify-metadata-extended", EventKind::Modify(ModifyKind::Metadata(MetadataKind::Extended)))]
    #[case("modify-metadata-other", EventKind::Modify(ModifyKind::Metadata(MetadataKind::Other)))]
    #[case("modify-name-any", EventKind::Modify(ModifyKind::Name(RenameMode::Any)))]
    #[case("modify-name-to", EventKind::Modify(ModifyKind::Name(RenameMode::To)))]
    #[case("modify-name-from", EventKind::Modify(ModifyKind::Name(RenameMode::From)))]
    #[case("modify-name-both", EventKind::Modify(ModifyKind::Name(RenameMode::Both)))]
    #[case("modify-name-other", EventKind::Modify(ModifyKind::Name(RenameMode::Other)))]
    #[case("modify-other", EventKind::Modify(ModifyKind::Other))]
    #[case("remove-any", EventKind::Remove(RemoveKind::Any))]
    #[case("remove-file", EventKind::Remove(RemoveKind::File))]
    #[case("remove-folder", EventKind::Remove(RemoveKind::Folder))]
    #[case("remove-other", EventKind::Remove(RemoveKind::Other))]
    #[case("other", EventKind::Other)]
    fn serialize_event_kind(
        #[case] name: &str,
        #[case] event_kind: EventKind,
    ) {
        let event = Event::new(event_kind);
        let json = serde_json::to_string(&event).unwrap();
        assert_snapshot!(name, json);
    }

    #[test]
    fn serialize_event_with_attrs() {
        let event = Event::new(EventKind::Any)
            .set_tracker(123)
            .set_flag(Flag::Rescan)
            .set_info("test event")
            .set_process_id(0);
        let json = serde_json::to_string(&event).unwrap();
        assert_snapshot!(json);
    }
}
