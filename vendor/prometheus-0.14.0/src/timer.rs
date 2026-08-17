use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;

/// Milliseconds since ANCHOR.
static RECENT: AtomicU64 = AtomicU64::new(0);
lazy_static! {
    static ref ANCHOR: Instant = Instant::now();
}

/// Convert a duration to millisecond.
#[inline]
pub fn duration_to_millis(dur: Duration) -> u64 {
    dur.as_secs() * 1000 + dur.subsec_millis() as u64
}

/// Returns milliseconds since ANCHOR.
///
/// ANCHOR is some fixed point in history.
pub fn now_millis() -> u64 {
    let res = Instant::now();
    let t = duration_to_millis(res.saturating_duration_since(*ANCHOR));
    let mut recent = RECENT.load(Ordering::Relaxed);
    loop {
        if recent > t {
            return recent;
        }
        match RECENT.compare_exchange_weak(recent, t, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return t,
            Err(r) => recent = r,
        }
    }
}

/// Returns recent returned value by `now_millis`.
pub fn recent_millis() -> u64 {
    RECENT.load(Ordering::Relaxed)
}

lazy_static! {
    static ref UPDATER_IS_RUNNING: AtomicBool = AtomicBool::new(false);
}

const CHECK_UPDATE_INTERVAL: Duration = Duration::from_millis(200);

/// Ensures background updater is running, which will call `now_millis` periodically.
pub fn ensure_updater() {
    if UPDATER_IS_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        std::thread::Builder::new()
            .name("time updater".to_owned())
            .spawn(|| loop {
                thread::sleep(CHECK_UPDATE_INTERVAL);
                now_millis();
            })
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_duration_to_millis() {
        let cases = vec![(1, 1, 1000), (0, 1_000_000, 1), (3, 103_000_000, 3103)];
        for (secs, nanos, exp) in cases {
            let dur = Duration::new(secs, nanos);
            assert_eq!(super::duration_to_millis(dur), exp);
        }
    }

    #[test]
    fn test_time_update() {
        assert_eq!(super::recent_millis(), 0);
        let now = super::now_millis();
        assert_eq!(super::recent_millis(), now);
        super::ensure_updater();
        thread::sleep(super::CHECK_UPDATE_INTERVAL * 2);
        assert!(super::recent_millis() > now);
    }
}
