use std::{
    fs::File,
    io,
    os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
    path::Path,
    process,
};

/// A handle represents an owned and valid Windows handle to a file-like
/// object.
///
/// When an owned handle is dropped, then the underlying raw handle is closed.
/// To get a borrowed handle, use `HandleRef`.
#[derive(Debug)]
pub struct Handle(File);

impl AsRawHandle for Handle {
    fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}

impl FromRawHandle for Handle {
    unsafe fn from_raw_handle(handle: RawHandle) -> Handle {
        Handle(File::from_raw_handle(handle))
    }
}

impl IntoRawHandle for Handle {
    fn into_raw_handle(self) -> RawHandle {
        self.0.into_raw_handle()
    }
}

impl Handle {
    /// Create an owned handle to the given file.
    ///
    /// When the returned handle is dropped, the file is closed.
    ///
    /// Note that if the given file represents a handle to a directory, then
    /// it is generally required that it have been opened with the
    /// [`FILE_FLAG_BACKUP_SEMANTICS`] flag in order to use it in various
    /// calls such as `information` or `typ`. To have this done automatically
    /// for you, use the `from_path_any` constructor.
    ///
    /// [`FILE_FLAG_BACKUP_SEMANTICS`]: https://docs.microsoft.com/en-us/windows/desktop/api/FileAPI/nf-fileapi-createfilea
    pub fn from_file(file: File) -> Handle {
        Handle(file)
    }

    /// Open a file to the given file path, and return an owned handle to that
    /// file.
    ///
    /// When the returned handle is dropped, the file is closed.
    ///
    /// If there was a problem opening the file, then the corresponding error
    /// is returned.
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Handle> {
        Ok(Handle::from_file(File::open(path)?))
    }

    /// Like `from_path`, but supports opening directory handles as well.
    ///
    /// If you use `from_path` on a directory, then subsequent queries using
    /// that handle will fail.
    pub fn from_path_any<P: AsRef<Path>>(path: P) -> io::Result<Handle> {
        use std::fs::OpenOptions;
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;

        let file = OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .open(path)?;
        Ok(Handle::from_file(file))
    }

    /// Return this handle as a standard `File` reference.
    pub fn as_file(&self) -> &File {
        &self.0
    }

    /// Return this handle as a standard `File` mutable reference.
    pub fn as_file_mut(&mut self) -> &mut File {
        &mut self.0
    }
}

/// Represents a borrowed and valid Windows handle to a file-like object, such
/// as stdin/stdout/stderr or an actual file.
///
/// When a borrowed handle is dropped, then the underlying raw handle is
/// **not** closed. To get an owned handle, use `Handle`.
#[derive(Debug)]
pub struct HandleRef(HandleRefInner);

/// The representation of a HandleRef, on which we define a custom Drop impl
/// that avoids closing the underlying raw handle.
#[derive(Debug)]
struct HandleRefInner(Option<File>);

impl Drop for HandleRefInner {
    fn drop(&mut self) {
        self.0.take().unwrap().into_raw_handle();
    }
}

impl AsRawHandle for HandleRef {
    fn as_raw_handle(&self) -> RawHandle {
        self.as_file().as_raw_handle()
    }
}

impl Clone for HandleRef {
    fn clone(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl HandleRef {
    /// Create a borrowed handle to stdin.
    ///
    /// When the returned handle is dropped, stdin is not closed.
    pub fn stdin() -> HandleRef {
        unsafe { HandleRef::from_raw_handle(io::stdin().as_raw_handle()) }
    }

    /// Create a handle to stdout.
    ///
    /// When the returned handle is dropped, stdout is not closed.
    pub fn stdout() -> HandleRef {
        unsafe { HandleRef::from_raw_handle(io::stdout().as_raw_handle()) }
    }

    /// Create a handle to stderr.
    ///
    /// When the returned handle is dropped, stderr is not closed.
    pub fn stderr() -> HandleRef {
        unsafe { HandleRef::from_raw_handle(io::stderr().as_raw_handle()) }
    }

    /// Create a borrowed handle to the given file.
    ///
    /// When the returned handle is dropped, the file is not closed.
    pub fn from_file(file: &File) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(file.as_raw_handle()) }
    }

    /// Create a borrowed handle from the given raw handle.
    ///
    /// Note that unlike the `FromRawHandle` trait, this constructor does
    /// **not** consume ownership of the given handle. That is, when the
    /// borrowed handle created by this constructor is dropped, the underlying
    /// handle will not be closed.
    ///
    /// # Safety
    ///
    /// This is unsafe because there is no guarantee that the given raw handle
    /// is a valid handle. The caller must ensure this is true before invoking
    /// this constructor.
    pub unsafe fn from_raw_handle(handle: RawHandle) -> HandleRef {
        HandleRef(HandleRefInner(Some(File::from_raw_handle(handle))))
    }

    /// Return this handle as a standard `File` reference.
    pub fn as_file(&self) -> &File {
        (self.0).0.as_ref().unwrap()
    }

    /// Return this handle as a standard `File` mutable reference.
    pub fn as_file_mut(&mut self) -> &mut File {
        (self.0).0.as_mut().unwrap()
    }
}

/// Construct borrowed and valid Windows handles from file-like objects.
pub trait AsHandleRef {
    /// A borrowed handle that wraps the raw handle of the `Self` object.
    fn as_handle_ref(&self) -> HandleRef;

    /// A convenience routine for extracting a `HandleRef` from `Self`, and
    /// then extracting a raw handle from the `HandleRef`.
    fn as_raw(&self) -> RawHandle {
        self.as_handle_ref().as_raw_handle()
    }
}

impl<'a, T: AsHandleRef> AsHandleRef for &'a T {
    fn as_handle_ref(&self) -> HandleRef {
        (**self).as_handle_ref()
    }
}

impl AsHandleRef for Handle {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for HandleRef {
    fn as_handle_ref(&self) -> HandleRef {
        self.clone()
    }
}

impl AsHandleRef for File {
    fn as_handle_ref(&self) -> HandleRef {
        HandleRef::from_file(self)
    }
}

impl AsHandleRef for io::Stdin {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for io::Stdout {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for io::Stderr {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for process::ChildStdin {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for process::ChildStdout {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}

impl AsHandleRef for process::ChildStderr {
    fn as_handle_ref(&self) -> HandleRef {
        unsafe { HandleRef::from_raw_handle(self.as_raw_handle()) }
    }
}
