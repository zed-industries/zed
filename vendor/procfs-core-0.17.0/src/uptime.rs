use crate::{expect, ProcResult};

use std::io::Read;
use std::str::FromStr;
use std::time::Duration;

/// The uptime of the system, based on the `/proc/uptime` file.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Uptime {
    /// The uptime of the system (including time spent in suspend).
    pub uptime: f64,

    /// The sum of how much time each core has spent idle.
    pub idle: f64,
}

impl super::FromRead for Uptime {
    fn from_read<R: Read>(mut r: R) -> ProcResult<Self> {
        let mut buf = Vec::with_capacity(128);
        r.read_to_end(&mut buf)?;

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        let mut s = buf.split(' ');
        let uptime = expect!(f64::from_str(expect!(s.next())));
        let idle = expect!(f64::from_str(expect!(s.next())));

        Ok(Uptime { uptime, idle })
    }
}

impl Uptime {
    /// The uptime of the system (including time spent in suspend).
    pub fn uptime_duration(&self) -> Duration {
        let secs = self.uptime.trunc() as u64;
        let csecs = (self.uptime.fract() * 100.0).round() as u32;
        let nsecs = csecs * 10_000_000;

        Duration::new(secs, nsecs)
    }

    /// The sum of how much time each core has spent idle.
    pub fn idle_duration(&self) -> Duration {
        let secs = self.idle.trunc() as u64;
        let csecs = (self.idle.fract() * 100.0).round() as u32;
        let nsecs = csecs * 10_000_000;

        Duration::new(secs, nsecs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromRead;
    use std::io::Cursor;

    #[test]
    fn test_uptime() {
        let reader = Cursor::new(b"2578790.61 1999230.98\n");
        let uptime = Uptime::from_read(reader).unwrap();

        assert_eq!(uptime.uptime_duration(), Duration::new(2578790, 610_000_000));
        assert_eq!(uptime.idle_duration(), Duration::new(1999230, 980_000_000));
    }
}
