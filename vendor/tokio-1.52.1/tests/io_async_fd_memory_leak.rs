//! Regression test for issue #7563 - Memory leak when fd closed before AsyncFd drop
//!
//! This test uses a custom global allocator to track actual memory usage,
//! avoiding false positives from RSS measurements which include freed-but-retained memory.

#![cfg(all(unix, target_os = "linux", feature = "full"))]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A tracking allocator that counts bytes currently allocated
struct TrackingAllocator {
    allocated: AtomicUsize,
}

impl TrackingAllocator {
    const fn new() -> Self {
        Self {
            allocated: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            // Subtract old size, add new size
            if new_size > layout.size() {
                self.allocated
                    .fetch_add(new_size - layout.size(), Ordering::Relaxed);
            } else {
                self.allocated
                    .fetch_sub(layout.size() - new_size, Ordering::Relaxed);
            }
        }
        new_ptr
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator::new();

fn allocated_bytes() -> usize {
    GLOBAL.allocated.load(Ordering::Relaxed)
}

#[tokio::test]
async fn memory_leak_when_fd_closed_before_drop() {
    use nix::sys::socket::{self, AddressFamily, SockFlag, SockType};
    use std::os::fd::OwnedFd;
    use std::os::unix::io::{AsRawFd, RawFd};
    use std::sync::Arc;
    use tokio::io::unix::AsyncFd;

    struct RawFdWrapper {
        fd: RawFd,
    }

    impl AsRawFd for RawFdWrapper {
        fn as_raw_fd(&self) -> RawFd {
            self.fd
        }
    }

    struct ArcFd(Arc<RawFdWrapper>);

    impl AsRawFd for ArcFd {
        fn as_raw_fd(&self) -> RawFd {
            self.0.as_raw_fd()
        }
    }

    fn set_nonblocking(fd: &OwnedFd) {
        use nix::fcntl::{OFlag, F_GETFL, F_SETFL};

        let flags = nix::fcntl::fcntl(fd, F_GETFL).expect("fcntl(F_GETFL)");

        if flags < 0 {
            panic!(
                "bad return value from fcntl(F_GETFL): {} ({:?})",
                flags,
                nix::Error::last()
            );
        }

        let flags = OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK;

        nix::fcntl::fcntl(fd, F_SETFL(flags)).expect("fcntl(F_SETFL)");
    }

    // Warm up - let runtime and allocator stabilize
    for _ in 0..100 {
        tokio::task::yield_now().await;
    }

    const ITERATIONS: usize = 1000;

    // Phase 1: Warm up allocations
    for _ in 0..ITERATIONS {
        let (fd_a, _fd_b) = socket::socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::empty(),
        )
        .unwrap();

        let raw_fd = fd_a.as_raw_fd();
        set_nonblocking(&fd_a);
        std::mem::forget(fd_a);

        let wrapper = Arc::new(RawFdWrapper { fd: raw_fd });
        let async_fd = AsyncFd::new(ArcFd(wrapper)).unwrap();

        // Close fd before dropping AsyncFd - this triggers the bug
        unsafe {
            libc::close(raw_fd);
        }

        drop(async_fd);
    }

    // Let things settle
    tokio::task::yield_now().await;
    let baseline = allocated_bytes();

    // Phase 2: Run more iterations and check for growth
    for _ in 0..ITERATIONS {
        let (fd_a, _fd_b) = socket::socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::empty(),
        )
        .unwrap();

        let raw_fd = fd_a.as_raw_fd();
        set_nonblocking(&fd_a);
        std::mem::forget(fd_a);

        let wrapper = Arc::new(RawFdWrapper { fd: raw_fd });
        let async_fd = AsyncFd::new(ArcFd(wrapper)).unwrap();

        unsafe {
            libc::close(raw_fd);
        }

        drop(async_fd);
    }

    tokio::task::yield_now().await;
    let after_phase2 = allocated_bytes();

    // Phase 3: Run even more iterations
    for _ in 0..ITERATIONS {
        let (fd_a, _fd_b) = socket::socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::empty(),
        )
        .unwrap();

        let raw_fd = fd_a.as_raw_fd();
        set_nonblocking(&fd_a);
        std::mem::forget(fd_a);

        let wrapper = Arc::new(RawFdWrapper { fd: raw_fd });
        let async_fd = AsyncFd::new(ArcFd(wrapper)).unwrap();

        unsafe {
            libc::close(raw_fd);
        }

        drop(async_fd);
    }

    tokio::task::yield_now().await;
    let after_phase3 = allocated_bytes();

    let growth_phase2 = after_phase2.saturating_sub(baseline);
    let growth_phase3 = after_phase3.saturating_sub(after_phase2);

    // If there's a leak, each phase adds ~250KB (1000 * ~256 bytes per ScheduledIo)
    // If fixed, memory should stabilize (minimal growth between phases)
    // Allow 64KB tolerance for normal allocation variance
    let threshold = 64 * 1024; // 64KB

    assert!(
        growth_phase2 < threshold || growth_phase3 < threshold,
        "Memory leak detected: allocations keep growing without stabilizing. \
         Phase 1->2: +{growth_phase2} bytes, Phase 2->3: +{growth_phase3} bytes. \
         (baseline: {baseline} bytes, phase2: {after_phase2} bytes, phase3: {after_phase3} bytes). \
         Expected at least one phase with <{threshold} bytes growth.",
    );
}
