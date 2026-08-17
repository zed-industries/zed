#![cfg(target_os = "macos")]

use kqueue::Watcher;
use tempfile::NamedTempFile;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn test_add_hundred() {
    let mut fs = Vec::new();
    for _ in 0..100 {
        fs.push(NamedTempFile::new().unwrap());
    }

    let _profiler = dhat::Profiler::builder().testing().build();

    let mut w = Watcher::new().unwrap();
    for f in fs {
        w.add_filename(
            f.path(),
            kqueue::EventFilter::EVFILT_VNODE,
            kqueue::FilterFlag::empty(),
        )
        .unwrap();
    }

    drop(w);

    let stats = dhat::HeapStats::get();
    dhat::assert_eq!(stats.total_bytes, 18296);
    dhat::assert_eq!(stats.total_blocks, 106);

    dhat::assert_eq!(stats.curr_bytes, 0);
    dhat::assert_eq!(stats.curr_blocks, 0);
}
