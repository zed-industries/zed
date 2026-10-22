use super::*;

#[test]
fn equivalent_path_spellings_share_watch_delivery_and_lifetime() {
    for case_insensitive in [false, true] {
        for spelling in ["/repo/project/", "/repo//project", "/repo/./project"] {
            let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
            let watcher = test_os_watcher(OsWatcherKind::Native, Some(backend.clone()));
            let (first_fired, first_callback) = fired_count();
            let first = watcher
                .add(Path::new(spelling).into(), case_insensitive, first_callback)
                .expect("add alternate spelling")
                .expect("registered alternate spelling");
            watcher.dispatch(Ok(modify_event("/repo/project/file.txt")));
            assert_eq!(
                *first_fired.lock(),
                1,
                "{spelling}, folded={case_insensitive}"
            );

            let (second_fired, second_callback) = fired_count();
            let second = watcher
                .add(
                    Path::new("/repo/project").into(),
                    case_insensitive,
                    second_callback,
                )
                .expect("add ordinary spelling")
                .expect("registered ordinary spelling");
            assert_eq!(backend.lock().watch_calls.len(), 1);
            watcher.dispatch(Ok(modify_event("/repo/project/file.txt")));
            assert_eq!(*first_fired.lock(), 2);
            assert_eq!(*second_fired.lock(), 1);

            watcher.remove(first);
            assert!(backend.lock().unwatch_calls.is_empty());
            watcher.dispatch(Ok(modify_event("/repo/project/file.txt")));
            assert_eq!(*first_fired.lock(), 2);
            assert_eq!(*second_fired.lock(), 2);
            watcher.remove(second);
            assert_eq!(backend.lock().unwatch_calls.len(), 1);
            assert!(backend.lock().watched_paths.is_empty());
        }
    }
}

#[test]
fn folded_keys_preserve_root_and_parent_component_identity() {
    for path in ["/", "/repo/../project", "./project", "project"] {
        let ordinary = Path::new(path);
        let with_separator = PathBuf::from(format!("{path}/"));
        assert_eq!(ordinary, with_separator);
        assert_eq!(
            WatchKey::folded(SanitizedPath::new(ordinary)),
            WatchKey::folded(SanitizedPath::new(&with_separator)),
        );
    }
    for (left, right) in [
        ("/", ""),
        ("/repo/../project", "/project"),
        ("./project", "project"),
    ] {
        assert_ne!(
            WatchKey::folded(SanitizedPath::new(Path::new(left))),
            WatchKey::folded(SanitizedPath::new(Path::new(right))),
        );
    }
}
