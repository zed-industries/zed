use rustix::cstr;
use rustix::fs::{openat, statat, AtFlags, Mode, OFlags, CWD};
use rustix::io::Errno;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static WORKING: AtomicBool = AtomicBool::new(false);
static CHECKED: AtomicBool = AtomicBool::new(false);

#[inline]
pub(crate) fn beneath_supported() -> bool {
    if WORKING.load(Relaxed) {
        return true;
    }
    if CHECKED.load(Relaxed) {
        return false;
    }
    check_beneath_supported()
}

#[cold]
fn check_beneath_supported() -> bool {
    // `RESOLVE_BENEATH` was introduced in FreeBSD 13, but opening `..` within
    // the root directory re-opened the root directory. In FreeBSD 14, it fails
    // as cap-std expects.
    if let Ok(root) = openat(
        CWD,
        cstr!("/"),
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
    ) {
        // Unknown O_ flags get ignored but AT_ flags have strict checks, so we use that.
        if let Err(Errno::NOTCAPABLE) = statat(root, cstr!(".."), AtFlags::RESOLVE_BENEATH) {
            WORKING.store(true, Relaxed);
            return true;
        }
    }

    CHECKED.store(true, Relaxed);
    false
}
