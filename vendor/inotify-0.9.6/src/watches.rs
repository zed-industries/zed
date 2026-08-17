use std::{
    hash::{
        Hash,
        Hasher,
    },
    cmp::Ordering,
    os::raw::c_int,
    sync::Weak,
};

use inotify_sys as ffi;

use crate::fd_guard::FdGuard;


bitflags! {
    /// Describes a file system watch
    ///
    /// Passed to [`Inotify::add_watch`], to describe what file system events
    /// to watch for, and how to do that.
    ///
    /// # Examples
    ///
    /// `WatchMask` constants can be passed to [`Inotify::add_watch`] as is. For
    /// example, here's how to create a watch that triggers an event when a file
    /// is accessed:
    ///
    /// ``` rust
    /// # use inotify::{
    /// #     Inotify,
    /// #     WatchMask,
    /// # };
    /// #
    /// # let mut inotify = Inotify::init().unwrap();
    /// #
    /// # // Create a temporary file, so `add_watch` won't return an error.
    /// # use std::fs::File;
    /// # File::create("/tmp/inotify-rs-test-file")
    /// #     .expect("Failed to create test file");
    /// #
    /// inotify.add_watch("/tmp/inotify-rs-test-file", WatchMask::ACCESS)
    ///    .expect("Error adding watch");
    /// ```
    ///
    /// You can also combine multiple `WatchMask` constants. Here we add a watch
    /// this is triggered both when files are created or deleted in a directory:
    ///
    /// ``` rust
    /// # use inotify::{
    /// #     Inotify,
    /// #     WatchMask,
    /// # };
    /// #
    /// # let mut inotify = Inotify::init().unwrap();
    /// inotify.add_watch("/tmp/", WatchMask::CREATE | WatchMask::DELETE)
    ///    .expect("Error adding watch");
    /// ```
    ///
    /// [`Inotify::add_watch`]: struct.Inotify.html#method.add_watch
    pub struct WatchMask: u32 {
        /// File was accessed
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_ACCESS`].
        ///
        /// [`inotify_sys::IN_ACCESS`]: ../inotify_sys/constant.IN_ACCESS.html
        const ACCESS = ffi::IN_ACCESS;

        /// Metadata (permissions, timestamps, ...) changed
        ///
        /// When watching a directory, this event can be triggered for the
        /// directory itself, as well as objects inside the directory.
        ///
        /// See [`inotify_sys::IN_ATTRIB`].
        ///
        /// [`inotify_sys::IN_ATTRIB`]: ../inotify_sys/constant.IN_ATTRIB.html
        const ATTRIB = ffi::IN_ATTRIB;

        /// File opened for writing was closed
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_CLOSE_WRITE`].
        ///
        /// [`inotify_sys::IN_CLOSE_WRITE`]: ../inotify_sys/constant.IN_CLOSE_WRITE.html
        const CLOSE_WRITE = ffi::IN_CLOSE_WRITE;

        /// File or directory not opened for writing was closed
        ///
        /// When watching a directory, this event can be triggered for the
        /// directory itself, as well as objects inside the directory.
        ///
        /// See [`inotify_sys::IN_CLOSE_NOWRITE`].
        ///
        /// [`inotify_sys::IN_CLOSE_NOWRITE`]: ../inotify_sys/constant.IN_CLOSE_NOWRITE.html
        const CLOSE_NOWRITE = ffi::IN_CLOSE_NOWRITE;

        /// File/directory created in watched directory
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_CREATE`].
        ///
        /// [`inotify_sys::IN_CREATE`]: ../inotify_sys/constant.IN_CREATE.html
        const CREATE = ffi::IN_CREATE;

        /// File/directory deleted from watched directory
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_DELETE`].
        ///
        /// [`inotify_sys::IN_DELETE`]: ../inotify_sys/constant.IN_DELETE.html
        const DELETE = ffi::IN_DELETE;

        /// Watched file/directory was deleted
        ///
        /// See [`inotify_sys::IN_DELETE_SELF`].
        ///
        /// [`inotify_sys::IN_DELETE_SELF`]: ../inotify_sys/constant.IN_DELETE_SELF.html
        const DELETE_SELF = ffi::IN_DELETE_SELF;

        /// File was modified
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_MODIFY`].
        ///
        /// [`inotify_sys::IN_MODIFY`]: ../inotify_sys/constant.IN_MODIFY.html
        const MODIFY = ffi::IN_MODIFY;

        /// Watched file/directory was moved
        ///
        /// See [`inotify_sys::IN_MOVE_SELF`].
        ///
        /// [`inotify_sys::IN_MOVE_SELF`]: ../inotify_sys/constant.IN_MOVE_SELF.html
        const MOVE_SELF = ffi::IN_MOVE_SELF;

        /// File was renamed/moved; watched directory contained old name
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_MOVED_FROM`].
        ///
        /// [`inotify_sys::IN_MOVED_FROM`]: ../inotify_sys/constant.IN_MOVED_FROM.html
        const MOVED_FROM = ffi::IN_MOVED_FROM;

        /// File was renamed/moved; watched directory contains new name
        ///
        /// When watching a directory, this event is only triggered for objects
        /// inside the directory, not the directory itself.
        ///
        /// See [`inotify_sys::IN_MOVED_TO`].
        ///
        /// [`inotify_sys::IN_MOVED_TO`]: ../inotify_sys/constant.IN_MOVED_TO.html
        const MOVED_TO = ffi::IN_MOVED_TO;

        /// File or directory was opened
        ///
        /// When watching a directory, this event can be triggered for the
        /// directory itself, as well as objects inside the directory.
        ///
        /// See [`inotify_sys::IN_OPEN`].
        ///
        /// [`inotify_sys::IN_OPEN`]: ../inotify_sys/constant.IN_OPEN.html
        const OPEN = ffi::IN_OPEN;

        /// Watch for all events
        ///
        /// This constant is simply a convenient combination of the following
        /// other constants:
        ///
        /// - [`ACCESS`]
        /// - [`ATTRIB`]
        /// - [`CLOSE_WRITE`]
        /// - [`CLOSE_NOWRITE`]
        /// - [`CREATE`]
        /// - [`DELETE`]
        /// - [`DELETE_SELF`]
        /// - [`MODIFY`]
        /// - [`MOVE_SELF`]
        /// - [`MOVED_FROM`]
        /// - [`MOVED_TO`]
        /// - [`OPEN`]
        ///
        /// See [`inotify_sys::IN_ALL_EVENTS`].
        ///
        /// [`ACCESS`]: #associatedconstant.ACCESS
        /// [`ATTRIB`]: #associatedconstant.ATTRIB
        /// [`CLOSE_WRITE`]: #associatedconstant.CLOSE_WRITE
        /// [`CLOSE_NOWRITE`]: #associatedconstant.CLOSE_NOWRITE
        /// [`CREATE`]: #associatedconstant.CREATE
        /// [`DELETE`]: #associatedconstant.DELETE
        /// [`DELETE_SELF`]: #associatedconstant.DELETE_SELF
        /// [`MODIFY`]: #associatedconstant.MODIFY
        /// [`MOVE_SELF`]: #associatedconstant.MOVE_SELF
        /// [`MOVED_FROM`]: #associatedconstant.MOVED_FROM
        /// [`MOVED_TO`]: #associatedconstant.MOVED_TO
        /// [`OPEN`]: #associatedconstant.OPEN
        /// [`inotify_sys::IN_ALL_EVENTS`]: ../inotify_sys/constant.IN_ALL_EVENTS.html
        const ALL_EVENTS = ffi::IN_ALL_EVENTS;

        /// Watch for all move events
        ///
        /// This constant is simply a convenient combination of the following
        /// other constants:
        ///
        /// - [`MOVED_FROM`]
        /// - [`MOVED_TO`]
        ///
        /// See [`inotify_sys::IN_MOVE`].
        ///
        /// [`MOVED_FROM`]: #associatedconstant.MOVED_FROM
        /// [`MOVED_TO`]: #associatedconstant.MOVED_TO
        /// [`inotify_sys::IN_MOVE`]: ../inotify_sys/constant.IN_MOVE.html
        const MOVE = ffi::IN_MOVE;

        /// Watch for all close events
        ///
        /// This constant is simply a convenient combination of the following
        /// other constants:
        ///
        /// - [`CLOSE_WRITE`]
        /// - [`CLOSE_NOWRITE`]
        ///
        /// See [`inotify_sys::IN_CLOSE`].
        ///
        /// [`CLOSE_WRITE`]: #associatedconstant.CLOSE_WRITE
        /// [`CLOSE_NOWRITE`]: #associatedconstant.CLOSE_NOWRITE
        /// [`inotify_sys::IN_CLOSE`]: ../inotify_sys/constant.IN_CLOSE.html
        const CLOSE = ffi::IN_CLOSE;

        /// Don't dereference the path if it is a symbolic link
        ///
        /// See [`inotify_sys::IN_DONT_FOLLOW`].
        ///
        /// [`inotify_sys::IN_DONT_FOLLOW`]: ../inotify_sys/constant.IN_DONT_FOLLOW.html
        const DONT_FOLLOW = ffi::IN_DONT_FOLLOW;

        /// Filter events for directory entries that have been unlinked
        ///
        /// See [`inotify_sys::IN_EXCL_UNLINK`].
        ///
        /// [`inotify_sys::IN_EXCL_UNLINK`]: ../inotify_sys/constant.IN_EXCL_UNLINK.html
        const EXCL_UNLINK = ffi::IN_EXCL_UNLINK;

        /// If a watch for the inode exists, amend it instead of replacing it
        ///
        /// See [`inotify_sys::IN_MASK_ADD`].
        ///
        /// [`inotify_sys::IN_MASK_ADD`]: ../inotify_sys/constant.IN_MASK_ADD.html
        const MASK_ADD = ffi::IN_MASK_ADD;

        /// Only receive one event, then remove the watch
        ///
        /// See [`inotify_sys::IN_ONESHOT`].
        ///
        /// [`inotify_sys::IN_ONESHOT`]: ../inotify_sys/constant.IN_ONESHOT.html
        const ONESHOT = ffi::IN_ONESHOT;

        /// Only watch path, if it is a directory
        ///
        /// See [`inotify_sys::IN_ONLYDIR`].
        ///
        /// [`inotify_sys::IN_ONLYDIR`]: ../inotify_sys/constant.IN_ONLYDIR.html
        const ONLYDIR = ffi::IN_ONLYDIR;
    }
}


/// Represents a watch on an inode
///
/// Can be obtained from [`Inotify::add_watch`] or from an [`Event`]. A watch
/// descriptor can be used to get inotify to stop watching an inode by passing
/// it to [`Inotify::rm_watch`].
///
/// [`Inotify::add_watch`]: struct.Inotify.html#method.add_watch
/// [`Inotify::rm_watch`]: struct.Inotify.html#method.rm_watch
/// [`Event`]: struct.Event.html
#[derive(Clone, Debug)]
pub struct WatchDescriptor{
    pub(crate) id: c_int,
    pub(crate) fd: Weak<FdGuard>,
}

impl Eq for WatchDescriptor {}

impl PartialEq for WatchDescriptor {
    fn eq(&self, other: &Self) -> bool {
        let self_fd  = self.fd.upgrade();
        let other_fd = other.fd.upgrade();

        self.id == other.id && self_fd.is_some() && self_fd == other_fd
    }
}

impl Ord for WatchDescriptor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for WatchDescriptor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for WatchDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This function only takes `self.id` into account, as `self.fd` is a
        // weak pointer that might no longer be available. Since neither
        // panicking nor changing the hash depending on whether it's available
        // is acceptable, we just don't look at it at all.
        // I don't think that this influences storage in a `HashMap` or
        // `HashSet` negatively, as storing `WatchDescriptor`s from different
        // `Inotify` instances seems like something of an anti-pattern anyway.
        self.id.hash(state);
    }
}
