//! Manual path resolution, one component at a time, with manual symlink
//! resolution, in order to enforce sandboxing.

use super::{read_link_one, CanonicalPath, CowComponent};
use crate::fs::{
    dir_options, errors, open_unchecked, path_has_trailing_dot, path_has_trailing_slash,
    stat_unchecked, FollowSymlinks, MaybeOwnedFile, Metadata, OpenOptions, OpenUncheckedError,
};
#[cfg(any(target_os = "android", target_os = "linux", target_os = "freebsd"))]
use rustix::fs::OFlags;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::{fs, io, mem};
#[cfg(windows)]
use {
    crate::fs::{open_dir_unchecked, path_really_has_trailing_dot, SymlinkKind},
    windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_DIRECTORY,
};

/// Implement `open` by breaking up the path into components, resolving each
/// component individually, and resolving symbolic links manually.
pub(crate) fn open(start: &fs::File, path: &Path, options: &OpenOptions) -> io::Result<fs::File> {
    let mut symlink_count = 0;
    let start = MaybeOwnedFile::borrowed(start);
    let maybe_owned = internal_open(start, path, options, &mut symlink_count, None)?;
    maybe_owned.into_file(options)
}

/// Context for performing manual component-at-a-time path resolution.
struct Context<'start> {
    /// The current base directory handle for path lookups.
    base: MaybeOwnedFile<'start>,

    /// The stack of directory handles below the base.
    dirs: Vec<MaybeOwnedFile<'start>>,

    /// The current worklist stack of path components to process.
    components: Vec<CowComponent<'start>>,

    /// If requested, the canonical path is constructed here.
    canonical_path: CanonicalPath<'start>,

    /// Does the path end in `/` or similar, so it requires a directory?
    dir_required: bool,

    /// Are we requesting write permissions, so we can't open a directory?
    dir_precluded: bool,

    /// Where there a trailing slash on the path?
    trailing_slash: bool,

    /// If a path ends in `.`, `..`, or `/`, including after expanding
    /// symlinks, we need to follow path resolution by opening `.` so that we
    /// obtain a full `dir_options` file descriptor and confirm that we have
    /// search rights in the last component.
    follow_with_dot: bool,

    /// A `PathBuf` that we reuse for calling `read_link_one` to minimize
    /// allocations.
    reuse: PathBuf,

    #[cfg(racy_asserts)]
    start_clone: MaybeOwnedFile<'start>,
}

impl<'start> Context<'start> {
    /// Construct a new instance of `Self`.
    fn new(
        start: MaybeOwnedFile<'start>,
        path: &'start Path,
        _options: &OpenOptions,
        canonical_path: Option<&'start mut PathBuf>,
    ) -> Self {
        let trailing_slash = path_has_trailing_slash(path);
        let trailing_dot = path_has_trailing_dot(path);
        let trailing_dotdot = path.ends_with(Component::ParentDir);

        let mut components: Vec<CowComponent> = Vec::new();

        #[cfg(windows)]
        {
            // Windows resolves `..` before doing filesystem lookups.
            for component in path.components().map(CowComponent::borrowed) {
                match component {
                    CowComponent::ParentDir
                        if !components.is_empty() && components.last().unwrap().is_normal() =>
                    {
                        let _ = components.pop();
                    }
                    _ => components.push(component),
                }
            }
            components.reverse();
        }

        #[cfg(not(windows))]
        {
            // Add the path components to the worklist. Rust's `Path`
            // normalizes away `.` components, however a trailing `.` affects
            // path lookup, so special-case it here.
            if trailing_dot {
                components.push(CowComponent::CurDir);
            }
            components.extend(path.components().rev().map(CowComponent::borrowed));
        }

        #[cfg(racy_asserts)]
        let start_clone = MaybeOwnedFile::owned(start.try_clone().unwrap());

        Self {
            base: start,
            dirs: Vec::with_capacity(components.len()),
            components,
            canonical_path: CanonicalPath::new(canonical_path),
            dir_required: trailing_slash,

            #[cfg(not(windows))]
            dir_precluded: _options.write || _options.append,

            #[cfg(windows)]
            dir_precluded: false,

            trailing_slash,

            follow_with_dot: trailing_dot | trailing_dotdot,

            reuse: PathBuf::new(),

            #[cfg(racy_asserts)]
            start_clone,
        }
    }

    fn check_dot_access(&self) -> io::Result<()> {
        // Manually check that we have permissions to search `self.base` to
        // search for `.` in it, since we otherwise resolve `.` and `..`
        // ourselves by just manipulating the `dirs` stack.
        #[cfg(not(windows))]
        {
            // Use `faccess` with `AT_EACCESS`. `AT_EACCESS` is not often the
            // right tool for the job; in POSIX, it's better to ask for errno
            // than to ask for permission. But we use `check_dot_access` to
            // check access for opening `.` and `..` in situations where we
            // already have open handles to them, and now we're accessing them
            // through different paths, and we need to check whether these
            // paths allow us access.
            //
            // Android and Emscripten lack `AT_EACCESS`.
            // <https://android.googlesource.com/platform/bionic/+/master/libc/bionic/faccessat.cpp>
            #[cfg(any(target_os = "emscripten", target_os = "android"))]
            let at_flags = rustix::fs::AtFlags::empty();
            #[cfg(not(any(target_os = "emscripten", target_os = "android")))]
            let at_flags = rustix::fs::AtFlags::EACCESS;

            // Always use `CurDir`, even though this code is used to check
            // permissions for both `.` and `..`, because in both cases we
            // already know we can access the referenced directory, and we
            // just need to check for the ability to search for `.` or `..`
            // within `self.base`, which should always be the same. And
            // using `.` means we avoid asking the OS to access a `..` path
            // for us.
            Ok(rustix::fs::accessat(
                &*self.base,
                Component::CurDir.as_os_str(),
                rustix::fs::Access::EXEC_OK,
                at_flags,
            )?)
        }
        #[cfg(windows)]
        open_dir_unchecked(&self.base, Component::CurDir.as_ref()).map(|_| ())
    }

    /// Handle a "." path component.
    fn cur_dir(&mut self) -> io::Result<()> {
        // This is a no-op. If this occurs at the end of the path, it does
        // imply that we need search access to the directory, and it requires
        // we open a directory, however we'll handle that in the
        // `follow_with_dot` check.
        Ok(())
    }

    /// Handle a ".." path component.
    fn parent_dir(&mut self) -> io::Result<()> {
        #[cfg(racy_asserts)]
        if !self.dirs.is_empty() {
            assert_different_file!(&self.start_clone, &self.base);
        }

        // We hold onto all the parent directory descriptors so that we
        // don't have to re-open anything when we encounter a `..`. This
        // way, even if the directory is concurrently moved, we don't have
        // to worry about `..` leaving the sandbox.
        match self.dirs.pop() {
            Some(dir) => {
                // Check that we have permission to look up `..`.
                self.check_dot_access()?;

                // Looks good.
                self.base = dir;
            }
            None => return Err(errors::escape_attempt()),
        }
        assert!(self.canonical_path.pop());

        Ok(())
    }

    /// Handle a "normal" path component.
    fn normal(
        &mut self,
        one: &OsStr,
        options: &OpenOptions,
        symlink_count: &mut u8,
    ) -> io::Result<()> {
        // If there are more named components left, this will be a base
        // directory from which to open subsequent components, so use "path"
        // options (`O_PATH` on Linux).
        let use_options = if self.components.is_empty() {
            options.clone()
        } else {
            dir_options()
        };

        // If the last path component ended in a slash, re-add the slash,
        // as Rust's `Path` will have removed it, and we need it to get the
        // same behavior from the OS.
        let use_path: Cow<OsStr> = if self.components.is_empty() && self.trailing_slash {
            let mut p = one.to_os_string();
            p.push("/");
            Cow::Owned(p)
        } else {
            Cow::Borrowed(one)
        };

        let dir_required = self.dir_required || use_options.dir_required;

        #[allow(clippy::redundant_clone)]
        match open_unchecked(
            &self.base,
            use_path.as_ref(),
            use_options
                .clone()
                .follow(FollowSymlinks::No)
                .dir_required(dir_required),
        ) {
            Ok(file) => {
                // Emulate `O_PATH` + `FollowSymlinks::Yes` on Linux. If `file`
                // is a symlink, follow it.
                #[cfg(any(target_os = "android", target_os = "linux", target_os = "freebsd"))]
                if should_emulate_o_path(&use_options) {
                    match read_link_one(
                        &file,
                        Default::default(),
                        symlink_count,
                        mem::take(&mut self.reuse),
                    ) {
                        Ok(destination) => {
                            return self.push_symlink_destination(destination);
                        }
                        // If it isn't a symlink, handle it as normal.
                        // `readlinkat` returns `ENOENT` if the file isn't a
                        // symlink in this situation.
                        Err(err) if err.kind() == io::ErrorKind::NotFound => (),
                        // If `readlinkat` fails any other way, pass it on.
                        Err(err) => return Err(err),
                    }
                }

                // Normal case
                let prev_base = self.base.descend_to(MaybeOwnedFile::owned(file));
                self.dirs.push(prev_base);
                self.canonical_path.push(one);

                Ok(())
            }
            #[cfg(not(windows))]
            Err(OpenUncheckedError::Symlink(err, ())) => {
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            #[cfg(windows)]
            Err(OpenUncheckedError::Symlink(err, SymlinkKind::Dir)) => {
                // If this is a Windows directory symlink, require a directory.
                self.dir_required |= self.components.is_empty();
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            #[cfg(windows)]
            Err(OpenUncheckedError::Symlink(err, SymlinkKind::File)) => {
                // If this is a Windows file symlink, preclude a directory.
                self.dir_precluded = true;
                self.maybe_last_component_symlink(one, symlink_count, options.follow, err)
            }
            Err(OpenUncheckedError::NotFound(err)) => Err(err),
            Err(OpenUncheckedError::Other(err)) => {
                // An error occurred. If this was the last component, and the
                // error wasn't due to invalid inputs (eg. the path has an
                // embedded NUL), record it as the last component of the
                // canonical path, even if we couldn't open it.
                if self.components.is_empty() && err.kind() != io::ErrorKind::InvalidInput {
                    self.canonical_path.push(one);
                    self.canonical_path.complete();
                }
                Err(err)
            }
        }
    }

    /// Dereference one symlink level.
    fn symlink(&mut self, one: &OsStr, symlink_count: &mut u8) -> io::Result<()> {
        let destination =
            read_link_one(&self.base, one, symlink_count, mem::take(&mut self.reuse))?;
        self.push_symlink_destination(destination)
    }

    /// Push the components of `destination` onto the worklist stack.
    fn push_symlink_destination(&mut self, destination: PathBuf) -> io::Result<()> {
        let at_end = self.components.is_empty();
        let trailing_slash = path_has_trailing_slash(&destination);
        let trailing_dot = path_has_trailing_dot(&destination);
        let trailing_dotdot = destination.ends_with(Component::ParentDir);

        #[cfg(windows)]
        {
            // `path_has_trailing_dot` returns false so that we don't open `.`
            // at the end of path resolution. But for determining the Windows
            // symlink restrictions, we need to know whether the path really
            // ends in a `.`.
            let trailing_dot_really = path_really_has_trailing_dot(&destination);

            // Windows appears to disallow symlinks to paths with trailing
            // slashes, slashdots, or slashdotdots.
            if trailing_slash
                || (trailing_dot_really && destination.as_os_str() != Component::CurDir.as_os_str())
                || (trailing_dotdot && destination.as_os_str() != Component::ParentDir.as_os_str())
            {
                return Err(io::Error::from_raw_os_error(123));
            }

            // Windows resolves `..` before doing filesystem lookups.
            let mut components: Vec<CowComponent> = Vec::new();
            for component in destination.components().map(CowComponent::owned) {
                match component {
                    CowComponent::ParentDir
                        if !components.is_empty() && components.last().unwrap().is_normal() =>
                    {
                        let _ = components.pop();
                    }
                    _ => components.push(component),
                }
            }
            self.components.extend(components.into_iter().rev());
        }

        #[cfg(not(windows))]
        {
            // Rust's `Path` hides a trailing dot, so handle it manually.
            if trailing_dot {
                self.components.push(CowComponent::CurDir);
            }
            self.components
                .extend(destination.components().rev().map(CowComponent::owned));
        }

        // Record whether the new components ended with a path that implies
        // an open of `.` at the end of path resolution.
        if at_end {
            self.follow_with_dot |= trailing_dot | trailing_dotdot;
            self.trailing_slash |= trailing_slash;
            self.dir_required |= trailing_slash;
        }

        // As an optimization, hold onto the `PathBuf` buffer for later reuse.
        self.reuse = destination;

        Ok(())
    }

    /// Check whether this is the last component and we don't need
    /// to dereference; otherwise call `Self::symlink`.
    fn maybe_last_component_symlink(
        &mut self,
        one: &OsStr,
        symlink_count: &mut u8,
        follow: FollowSymlinks,
        err: io::Error,
    ) -> io::Result<()> {
        if follow == FollowSymlinks::No && !self.trailing_slash && self.components.is_empty() {
            self.canonical_path.push(one);
            self.canonical_path.complete();
            return Err(err);
        }

        self.symlink(one, symlink_count)
    }
}

/// Internal implementation of manual `open`, exposing some additional
/// parameters.
///
/// Callers can request the canonical path by passing `Some` to
/// `canonical_path`. If the complete canonical path is processed, it will be
/// stored in the provided `&mut PathBuf`, even if the actual open fails. If
/// a failure occurs before the complete canonical path is processed, the
/// provided `&mut PathBuf` is cleared to empty.
///
/// A note on lifetimes: `path` and `canonical_path` here don't strictly
/// need `'start`, but using them makes it easier to store them in the
/// `Context` struct.
pub(super) fn internal_open<'start>(
    start: MaybeOwnedFile<'start>,
    path: &'start Path,
    options: &OpenOptions,
    symlink_count: &mut u8,
    canonical_path: Option<&'start mut PathBuf>,
) -> io::Result<MaybeOwnedFile<'start>> {
    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut ctx = Context::new(start, path, options, canonical_path);

    while let Some(c) = ctx.components.pop() {
        match c {
            CowComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            CowComponent::CurDir => ctx.cur_dir()?,
            CowComponent::ParentDir => ctx.parent_dir()?,
            CowComponent::Normal(one) => ctx.normal(&one, options, symlink_count)?,
        }
    }

    // We've now finished all the path components other than any trailing `.`s,
    // so we have the complete canonical path.
    ctx.canonical_path.complete();

    // If the path ended in `.` (explicit or implied) or `..`, we may have
    // opened the directory with eg. `O_PATH` on Linux, or we may have skipped
    // checking for search access to `.`, so re-open it.
    if ctx.follow_with_dot {
        if ctx.dir_precluded {
            return Err(errors::is_directory());
        }
        ctx.base = MaybeOwnedFile::owned(open_unchecked(
            &ctx.base,
            Component::CurDir.as_ref(),
            options,
        )?);
    }

    #[cfg(racy_asserts)]
    check_internal_open(&ctx, path, options);

    Ok(ctx.base)
}

/// Implement manual `stat` in a similar manner as manual `open`.
pub(crate) fn stat(start: &fs::File, path: &Path, follow: FollowSymlinks) -> io::Result<Metadata> {
    // POSIX returns `ENOENT` on an empty path. TODO: On Windows, we should
    // be compatible with what Windows does instead.
    if path.as_os_str().is_empty() {
        return Err(errors::no_such_file_or_directory());
    }

    let mut options = OpenOptions::new();
    options.follow(follow);
    let mut symlink_count = 0;
    let mut ctx = Context::new(MaybeOwnedFile::borrowed(start), path, &options, None);
    assert!(!ctx.dir_precluded);

    while let Some(c) = ctx.components.pop() {
        match c {
            CowComponent::PrefixOrRootDir => return Err(errors::escape_attempt()),
            CowComponent::CurDir => ctx.cur_dir()?,
            CowComponent::ParentDir => ctx.parent_dir()?,
            CowComponent::Normal(one) => {
                if ctx.components.is_empty() {
                    // If this is the last component, do a non-following
                    // `stat_unchecked` on it.
                    let stat = stat_unchecked(&ctx.base, one.as_ref(), FollowSymlinks::No)?;

                    // If we weren't asked to follow symlinks, or it wasn't a
                    // symlink, we're done.
                    if options.follow == FollowSymlinks::No || !stat.file_type().is_symlink() {
                        if stat.is_dir() {
                            if ctx.dir_precluded {
                                return Err(errors::is_directory());
                            }
                        } else if ctx.dir_required {
                            return Err(errors::is_not_directory());
                        }
                        return Ok(stat);
                    }

                    // On Windows, symlinks know whether they are a file or
                    // directory.
                    #[cfg(windows)]
                    if stat.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0 {
                        ctx.dir_required = true;
                    } else {
                        ctx.dir_precluded = true;
                    }

                    // If it was a symlink and we're asked to follow symlinks,
                    // dereference it.
                    ctx.symlink(&one, &mut symlink_count)?
                } else {
                    // Otherwise open the path component normally.
                    ctx.normal(&one, &options, &mut symlink_count)?
                }
            }
        }
    }

    // If the path ended in `.` (explicit or implied) or `..`, we may have
    // opened the directory with eg. `O_PATH` on Linux, or we may have skipped
    // checking for search access to `.`, so re-check it.
    if ctx.follow_with_dot {
        if ctx.dir_precluded {
            return Err(errors::is_directory());
        }

        ctx.check_dot_access()?;
    }

    // If the path ended in `.` or `..`, we already have it open, so just do
    // `.metadata()` on it.
    Metadata::from_file(&ctx.base)
}

/// Test whether the given options imply that we should treat an open file as
/// potentially being a symlink we need to follow, due to use of `O_PATH`.
#[cfg(any(target_os = "android", target_os = "linux", target_os = "freebsd"))]
fn should_emulate_o_path(use_options: &OpenOptions) -> bool {
    (use_options.ext.custom_flags & (OFlags::PATH.bits() as i32)) == (OFlags::PATH.bits() as i32)
        && use_options.follow == FollowSymlinks::Yes
}

#[cfg(racy_asserts)]
fn check_internal_open(ctx: &Context, path: &Path, options: &OpenOptions) {
    match open_unchecked(
        &ctx.start_clone,
        ctx.canonical_path.debug.as_ref(),
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    ) {
        Ok(unchecked_file) => {
            assert_same_file!(
                &ctx.base,
                &unchecked_file,
                "path resolution inconsistency: start='{:?}', path='{}'; canonical_path='{}'",
                ctx.start_clone,
                path.display(),
                ctx.canonical_path.debug.display(),
            );
        }
        Err(_unchecked_error) => {
            /* TODO: Check error messages.
            panic!(
                "unexpected success opening result={:?} start='{:?}', path='{}'; canonical_path='{}'; \
                 expected {:?}",
                ctx.base,
                ctx.start_clone,
                path.display(),
                ctx.canonical_path.debug.display(),
                unchecked_error,
            */
        }
    }
}
