use std::hash::{Hash, Hasher};
use std::io::{Error, ErrorKind, Result};
use std::os::unix::io::RawFd;

use libc::pid_t;

/// The watched object that fired the event
#[derive(Debug, Eq, Clone)]
pub enum Ident {
    Filename(RawFd, String),
    Fd(RawFd),
    Pid(pid_t),
    Signal(i32),
    Timer(i32),
}

/// Vnode events
///
/// These are OS-specific, and may not all be supported on your platform. Check
/// `kqueue(2)` for more information.
#[derive(Debug)]
#[non_exhaustive]
pub enum Vnode {
    /// The file was deleted
    Delete,

    /// The file received a write
    Write,

    /// The file has grown, or a directory has had new entries added
    Extend,

    /// The file was shrunk with `truncate(2)`
    Truncate,

    /// The attributes of the file were changed
    Attrib,

    /// The link count of the file was changed
    Link,

    /// The file was renamed
    Rename,

    /// Access to the file was revoked with `revoke(2)` or the fs was unmounted
    Revoke,

    /// File was opened by a process (FreeBSD-specific)
    Open,

    /// File was closed and the descriptor had write access (FreeBSD-specific)
    CloseWrite,

    /// File was closed and the descriptor had read access (FreeBSD-specific)
    Close,

    /// File was read (FreeBSD-specific)
    Read,
}

/// Process events
///
/// These are OS-specific, and may not all be supported on your platform. Check
/// `kqueue(2)` for more information.
#[derive(Debug)]
pub enum Proc {
    /// The watched process exited with the returned exit code
    Exit(usize),

    /// The process called `fork(2)`
    Fork,

    /// The process called `exec(2)`
    Exec,

    /// The process called `fork(2)`, and returned the child pid.
    Track(libc::pid_t),

    /// The process called `fork(2)`, but we were not able to track the child
    Trackerr,

    /// The process called `fork(2)`, and returned the child pid.
    // TODO: this is FreeBSD-specific. We can probably convert this to `Track`.
    Child(libc::pid_t),
}

/// Options for a `Watcher`
#[derive(Debug)]
pub struct KqueueOpts {
    /// Clear state on watched objects
    pub(crate) clear: bool,
}

impl Default for KqueueOpts {
    /// Returns the default options for a `Watcher`
    ///
    /// `clear` is set to `true`
    fn default() -> KqueueOpts {
        KqueueOpts { clear: true }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for Ident {
    fn into(self) -> i32 {
        (&self).into()
    }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for &Ident {
    fn into(self) -> i32 {
        match self {
            Ident::Filename(inner, _)
            | Ident::Fd(inner)
            | Ident::Pid(inner)
            | Ident::Signal(inner)
            | Ident::Timer(inner) => *inner,
        }
    }
}

#[allow(clippy::from_over_into)]
impl TryInto<usize> for &Ident {
    type Error = Error;

    fn try_into(self) -> Result<usize> {
        let inner = match self {
            Ident::Filename(inner, _)
            | Ident::Fd(inner)
            | Ident::Pid(inner)
            | Ident::Signal(inner)
            | Ident::Timer(inner) => inner,
        };

        let Ok(ret) = i32::try_into(*inner) else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid ident {self:?}"),
            ));
        };

        Ok(ret)
    }
}

impl PartialEq<Ident> for Ident {
    fn eq(&self, other: &Ident) -> bool {
        match *self {
            Ident::Filename(_, ref name) => {
                if let Ident::Filename(_, ref othername) = *other {
                    name == othername
                } else {
                    false
                }
            }
            _ => {
                Into::<i32>::into(self) == Into::<i32>::into(other)
                    && std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Ident::Fd(fd) => {
                fd.hash(state);
            }
            Ident::Pid(pid) => {
                pid.hash(state);
            }
            Ident::Signal(sig) => {
                sig.hash(state);
            }
            Ident::Timer(timer) => {
                timer.hash(state);
            }
            Ident::Filename(_, ref name) => {
                name.hash(state);
            }
        }
        std::mem::discriminant(self).hash(state);
    }
}

#[cfg(test)]
mod test {
    use crate::Ident;

    #[test]
    fn test_partial_eq_discriminant() {
        let first = Ident::Fd(5);
        let second = Ident::Pid(5);

        assert_ne!(
            first, second,
            "different discriminants with same value are equal"
        );
    }

    #[test]
    fn test_partial_eq() {
        let first = Ident::Fd(5);
        let second = Ident::Fd(5);

        assert_eq!(
            first, second,
            "same discriminants with same value are not equal"
        );
    }

    #[test]
    fn test_partial_eq_filename() {
        let first = Ident::Filename(5, "foo".to_string());
        let second = Ident::Filename(6, "foo".to_string());

        assert_eq!(
            first, second,
            "same filenames with different fd's are not equal"
        );
    }

    #[test]
    fn test_try_into() {
        let ident = Ident::Pid(1);
        let res: std::io::Result<usize> = (&ident).try_into();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);
    }

    #[test]
    fn test_try_into_error() {
        let ident = Ident::Signal(-1);
        let res: std::io::Result<usize> = (&ident).try_into();

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "invalid ident Signal(-1)");
    }
}
