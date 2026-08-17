#![cfg(target_os = "macos")]

use kqueue::Watcher;
use tempfile::NamedTempFile;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn test_rm_single() {
    let f = NamedTempFile::new().unwrap();

    let _profiler = dhat::Profiler::builder().testing().build();

    let mut w = Watcher::new().unwrap();
    w.add_filename(
        f.path(),
        kqueue::EventFilter::EVFILT_VNODE,
        kqueue::FilterFlag::empty(),
    )
    .unwrap();

    w.watch().unwrap();

    w.remove_filename(f.path(), kqueue::EventFilter::EVFILT_VNODE)
        .unwrap();

    drop(w);

    let stats = dhat::HeapStats::get();
    dhat::assert_eq!(stats.total_bytes, 354);
    dhat::assert_eq!(stats.total_blocks, 4);

    dhat::assert_eq!(stats.curr_bytes, 0);
    dhat::assert_eq!(stats.curr_blocks, 0);
}
