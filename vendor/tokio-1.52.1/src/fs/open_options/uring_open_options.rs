use std::{io, os::unix::fs::OpenOptionsExt};

#[cfg(test)]
use super::mock_open_options::MockOpenOptions as StdOpenOptions;
#[cfg(not(test))]
use std::fs::OpenOptions as StdOpenOptions;

#[derive(Debug, Clone)]
pub(crate) struct UringOpenOptions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) append: bool,
    pub(crate) truncate: bool,
    pub(crate) create: bool,
    pub(crate) create_new: bool,
    pub(crate) mode: libc::mode_t,
    pub(crate) custom_flags: libc::c_int,
}

impl UringOpenOptions {
    pub(crate) fn new() -> Self {
        Self {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            mode: 0o666,
            custom_flags: 0,
        }
    }

    pub(crate) fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    pub(crate) fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    pub(crate) fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }

    pub(crate) fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    pub(crate) fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub(crate) fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub(crate) fn mode(&mut self, mode: u32) -> &mut Self {
        self.mode = mode as libc::mode_t;
        self
    }

    pub(crate) fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.custom_flags = flags;
        self
    }

    // Equivalent to https://github.com/rust-lang/rust/blob/64c81fd10509924ca4da5d93d6052a65b75418a5/library/std/src/sys/fs/unix.rs#L1118-L1127
    pub(crate) fn access_mode(&self) -> io::Result<libc::c_int> {
        match (self.read, self.write, self.append) {
            (true, false, false) => Ok(libc::O_RDONLY),
            (false, true, false) => Ok(libc::O_WRONLY),
            (true, true, false) => Ok(libc::O_RDWR),
            (false, _, true) => Ok(libc::O_WRONLY | libc::O_APPEND),
            (true, _, true) => Ok(libc::O_RDWR | libc::O_APPEND),
            (false, false, false) => Err(io::Error::from_raw_os_error(libc::EINVAL)),
        }
    }

    // Equivalent to https://github.com/rust-lang/rust/blob/64c81fd10509924ca4da5d93d6052a65b75418a5/library/std/src/sys/fs/unix.rs#L1129-L1151
    pub(crate) fn creation_mode(&self) -> io::Result<libc::c_int> {
        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(io::Error::from_raw_os_error(libc::EINVAL));
                }
            }
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(io::Error::from_raw_os_error(libc::EINVAL));
                }
            }
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => 0,
            (true, false, false) => libc::O_CREAT,
            (false, true, false) => libc::O_TRUNC,
            (true, true, false) => libc::O_CREAT | libc::O_TRUNC,
            (_, _, true) => libc::O_CREAT | libc::O_EXCL,
        })
    }
}

impl From<UringOpenOptions> for StdOpenOptions {
    fn from(value: UringOpenOptions) -> Self {
        let mut std = StdOpenOptions::new();

        std.append(value.append);
        std.create(value.create);
        std.create_new(value.create_new);
        std.read(value.read);
        std.truncate(value.truncate);
        std.write(value.write);

        std.mode(value.mode);
        std.custom_flags(value.custom_flags);

        std
    }
}
