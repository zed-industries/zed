use std::{
    borrow::BorrowMut, ffi::OsString, fmt::Display, os::unix::ffi::OsStrExt, path::PathBuf,
    process::Stdio,
};

use crate::process::Command;

use super::encode_percents;

/// `unixexec:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-exec>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unixexec {
    path: PathBuf,
    arg0: Option<OsString>,
    args: Vec<OsString>,
}

impl Unixexec {
    /// Create a new unixexec transport with the given path and arguments.
    pub fn new(path: PathBuf, arg0: Option<OsString>, args: Vec<OsString>) -> Self {
        Self { path, arg0, args }
    }

    pub(super) fn from_options(opts: std::collections::HashMap<&str, &str>) -> crate::Result<Self> {
        let Some(path) = opts.get("path") else {
            return Err(crate::Error::Address(
                "unixexec address is missing `path`".to_owned(),
            ));
        };

        let arg0 = opts.get("argv0").map(OsString::from);

        let mut args: Vec<OsString> = Vec::new();
        let mut arg_index = 1;
        while let Some(arg) = opts.get(format!("argv{arg_index}").as_str()) {
            args.push(OsString::from(arg));
            arg_index += 1;
        }

        Ok(Self::new(PathBuf::from(path), arg0, args))
    }

    /// Binary to execute.
    ///
    /// Path of the binary to execute, either an absolute path or a binary name that is searched for
    /// in the default search path of the OS. This corresponds to the first argument of execlp().
    /// This key is mandatory.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// The executable argument.
    ///
    /// The program name to use when executing the binary. If omitted the same
    /// value as specified for path will be used. This corresponds to the
    /// second argument of execlp().
    pub fn arg0(&self) -> Option<&OsString> {
        self.arg0.as_ref()
    }

    /// Arguments.
    ///
    /// Arguments to pass to the binary.
    pub fn args(&self) -> &[OsString] {
        self.args.as_ref()
    }

    pub(super) async fn connect(&self) -> crate::Result<crate::connection::socket::Command> {
        Command::for_unixexec(self)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?
            .borrow_mut()
            .try_into()
    }
}

impl Display for Unixexec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unixexec:path=")?;
        encode_percents(f, self.path.as_os_str().as_bytes())?;

        if let Some(arg0) = self.arg0.as_ref() {
            f.write_str(",argv0=")?;
            encode_percents(f, arg0.as_bytes())?;
        }

        for (index, arg) in self.args.iter().enumerate() {
            write!(f, ",argv{}=", index + 1)?;
            encode_percents(f, arg.as_bytes())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::address::{Address, transport::Transport};

    #[test]
    fn connect() {
        let addr: Address = "unixexec:path=echo,argv1=hello,argv2=world"
            .try_into()
            .unwrap();
        let unixexec = match addr.transport() {
            Transport::Unixexec(unixexec) => unixexec,
            _ => unreachable!(),
        };
        crate::utils::block_on(unixexec.connect()).unwrap();
    }
}
