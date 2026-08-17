#[cfg(feature = "logging")]
use crate::log::warn;
use crate::KeyLog;
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

// Internal mutable state for KeyLogFile
struct KeyLogFileInner {
    file: Option<File>,
    buf: Vec<u8>,
}

impl KeyLogFileInner {
    fn new(var: Result<String, env::VarError>) -> Self {
        let path = match var {
            Ok(ref s) => Path::new(s),
            Err(env::VarError::NotUnicode(ref s)) => Path::new(s),
            Err(env::VarError::NotPresent) => {
                return Self {
                    file: None,
                    buf: Vec::new(),
                };
            }
        };

        #[cfg_attr(not(feature = "logging"), allow(unused_variables))]
        let file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
        {
            Ok(f) => Some(f),
            Err(e) => {
                warn!("unable to create key log file {:?}: {}", path, e);
                None
            }
        };

        Self {
            file,
            buf: Vec::new(),
        }
    }

    fn try_write(&mut self, label: &str, client_random: &[u8], secret: &[u8]) -> io::Result<()> {
        let mut file = match self.file {
            None => {
                return Ok(());
            }
            Some(ref f) => f,
        };

        self.buf.truncate(0);
        write!(self.buf, "{} ", label)?;
        for b in client_random.iter() {
            write!(self.buf, "{:02x}", b)?;
        }
        write!(self.buf, " ")?;
        for b in secret.iter() {
            write!(self.buf, "{:02x}", b)?;
        }
        writeln!(self.buf)?;
        file.write_all(&self.buf)
    }
}

/// [`KeyLog`] implementation that opens a file whose name is
/// given by the `SSLKEYLOGFILE` environment variable, and writes
/// keys into it.
///
/// If `SSLKEYLOGFILE` is not set, this does nothing.
///
/// If such a file cannot be opened, or cannot be written then
/// this does nothing but logs errors at warning-level.
pub struct KeyLogFile(Mutex<KeyLogFileInner>);

impl KeyLogFile {
    /// Makes a new `KeyLogFile`.  The environment variable is
    /// inspected and the named file is opened during this call.
    pub fn new() -> Self {
        let var = env::var("SSLKEYLOGFILE");
        Self(Mutex::new(KeyLogFileInner::new(var)))
    }
}

impl KeyLog for KeyLogFile {
    fn log(&self, label: &str, client_random: &[u8], secret: &[u8]) {
        #[cfg_attr(not(feature = "logging"), allow(unused_variables))]
        match self
            .0
            .lock()
            .unwrap()
            .try_write(label, client_random, secret)
        {
            Ok(()) => {}
            Err(e) => {
                warn!("error writing to key log file: {}", e);
            }
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_env_var_is_not_unicode() {
        init();
        let mut inner = KeyLogFileInner::new(Err(env::VarError::NotUnicode(
            "/tmp/keylogfileinnertest".into(),
        )));
        assert!(inner
            .try_write("label", b"random", b"secret")
            .is_ok());
    }

    #[test]
    fn test_env_var_is_not_set() {
        init();
        let mut inner = KeyLogFileInner::new(Err(env::VarError::NotPresent));
        assert!(inner
            .try_write("label", b"random", b"secret")
            .is_ok());
    }

    #[test]
    fn test_env_var_cannot_be_opened() {
        init();
        let mut inner = KeyLogFileInner::new(Ok("/dev/does-not-exist".into()));
        assert!(inner
            .try_write("label", b"random", b"secret")
            .is_ok());
    }

    #[test]
    fn test_env_var_cannot_be_written() {
        init();
        let mut inner = KeyLogFileInner::new(Ok("/dev/full".into()));
        assert!(inner
            .try_write("label", b"random", b"secret")
            .is_err());
    }
}
