mod utils {

    use std::sync::atomic::{AtomicI32, Ordering};

    use once_cell::sync::Lazy;

    // WARNING Expecting that `cargo test` won't be invoked on the same computer more than once within
    // a single millisecond
    static INSTANCE_ID: Lazy<i64> = Lazy::new(|| chrono::Local::now().timestamp_millis());
    static ID_OFFSET: AtomicI32 = AtomicI32::new(0);

    pub fn get_unique_name() -> String {
        let id = ID_OFFSET.fetch_add(1, Ordering::SeqCst);
        format!("trash-test-{}-{}", *INSTANCE_ID, id)
    }

    pub fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}

pub use utils::{get_unique_name, init_logging};

#[cfg(any(
    target_os = "windows",
    all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android"))
))]
mod os_limited {
    use super::{get_unique_name, init_logging};
    use serial_test::serial;
    use std::collections::{hash_map::Entry, HashMap};
    use std::ffi::{OsStr, OsString};
    use std::fs::File;

    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
    use std::os::unix::ffi::OsStringExt;

    use crate as trash;

    #[test]
    #[serial]
    fn list() {
        const MAX_SECONDS_DIFFERENCE: i64 = 10;
        init_logging();

        let deletion_time = chrono::Utc::now();
        let actual_unix_deletion_time = deletion_time.naive_utc().timestamp();
        assert_eq!(actual_unix_deletion_time, deletion_time.naive_local().timestamp());
        let file_name_prefix = get_unique_name();
        let batches: usize = 2;
        let files_per_batch: usize = 3;
        let names: Vec<OsString> = (0..files_per_batch).map(|i| format!("{}#{}", file_name_prefix, i).into()).collect();
        for _ in 0..batches {
            for path in names.iter() {
                File::create_new(path).unwrap();
            }
            trash::delete_all(&names).unwrap();
        }
        let items = trash::os_limited::list().unwrap();
        let items: HashMap<_, Vec<_>> = items
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .fold(HashMap::new(), |mut map, x| {
                match map.entry(x.name.clone()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(x);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![x]);
                    }
                }
                map
            });
        for name in names {
            match items.get(&name) {
                Some(items) => {
                    assert_eq!(items.len(), batches);
                    for item in items {
                        if cfg!(feature = "chrono") {
                            let diff = (item.time_deleted - actual_unix_deletion_time).abs();
                            if diff > MAX_SECONDS_DIFFERENCE {
                                panic!(
                                    "The deleted item does not have the timestamp that represents its deletion time. Expected: {}. Got: {}",
                                    actual_unix_deletion_time,
                                    item.time_deleted
                                );
                            }
                        }
                    }
                }
                None => panic!("ERROR Could not find '{:?}' in {:#?}", name, items),
            }
        }

        // Let's try to purge all the items we just created but ignore any errors
        // as this test should succeed as long as `list` works properly.
        let _ = trash::os_limited::purge_all(items.iter().flat_map(|(_name, item)| item));
    }

    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
    #[test]
    #[serial]
    fn list_invalid_utf8() {
        let mut name = OsStr::new(&get_unique_name()).to_os_string().into_encoded_bytes();
        name.push(168);
        let name = OsString::from_vec(name);
        File::create_new(&name).unwrap();

        // Delete, list, and remove file with an invalid UTF8 name
        // Listing items is already exhaustively checked above, so this test is mainly concerned
        // with checking that listing non-Unicode names does not panic
        trash::delete(&name).unwrap();
        let item = trash::os_limited::list().unwrap().into_iter().find(|item| item.name == name).unwrap();
        let _ = trash::os_limited::purge_all([item]);
    }

    #[test]
    fn purge_empty() {
        init_logging();
        trash::os_limited::purge_all::<Vec<trash::TrashItem>>(vec![]).unwrap();
    }

    #[test]
    fn restore_empty() {
        init_logging();
        trash::restore_all(vec![]).unwrap();
    }

    #[test]
    #[serial]
    fn purge() {
        init_logging();
        let file_name_prefix = get_unique_name();
        let batches: usize = 2;
        let files_per_batch: usize = 3;
        let names: Vec<_> = (0..files_per_batch).map(|i| format!("{}#{}", file_name_prefix, i)).collect();
        for _ in 0..batches {
            for path in names.iter() {
                File::create_new(path).unwrap();
            }
            trash::delete_all(&names).unwrap();
        }

        // Collect it because we need the exact number of items gathered.
        let targets: Vec<_> = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .collect();
        assert_eq!(targets.len(), batches * files_per_batch);
        trash::os_limited::purge_all(targets).unwrap();
        let remaining = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .count();
        assert_eq!(remaining, 0);
    }

    #[test]
    #[serial]
    fn restore() {
        init_logging();
        let file_name_prefix = get_unique_name();
        let file_count: usize = 3;
        let names: Vec<_> = (0..file_count).map(|i| format!("{}#{}", file_name_prefix, i)).collect();
        for path in names.iter() {
            File::create_new(path).unwrap();
        }
        trash::delete_all(&names).unwrap();

        // Collect it because we need the exact number of items gathered.
        let targets: Vec<_> = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .collect();
        assert_eq!(targets.len(), file_count);
        trash::restore_all(targets).unwrap();
        let remaining = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .count();
        assert_eq!(remaining, 0);

        // They are not in the trash anymore but they should be at their original location
        let mut missing = Vec::new();
        for path in names.iter() {
            if !std::path::Path::new(&path).is_file() {
                missing.push(path);
            }
        }
        for path in names.iter() {
            std::fs::remove_file(path).ok();
        }

        assert_eq!(missing, Vec::<&String>::new());
    }

    #[test]
    #[serial]
    fn restore_collision() {
        init_logging();
        let file_name_prefix = get_unique_name();
        let file_count: usize = 3;
        let collision_remaining = file_count - 1;
        let names: Vec<_> = (0..file_count).map(|i| format!("{}#{}", file_name_prefix, i)).collect();
        for path in names.iter() {
            File::create_new(path).unwrap();
        }
        trash::delete_all(&names).unwrap();
        for path in names.iter().skip(file_count - collision_remaining) {
            File::create_new(path).unwrap();
        }
        let mut targets: Vec<_> = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .collect();
        targets.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(targets.len(), file_count);
        let remaining_count = match trash::restore_all(targets) {
            Err(trash::Error::RestoreCollision { remaining_items, .. }) => {
                let contains = |v: &Vec<trash::TrashItem>, name: &String| {
                    for curr in v.iter() {
                        if curr.name.as_encoded_bytes() == name.as_bytes() {
                            return true;
                        }
                    }
                    false
                };
                // Are all items that got restored reside in the folder?
                for path in names.iter().filter(|filename| !contains(&remaining_items, filename)) {
                    assert!(File::open(path).is_ok());
                }
                remaining_items.len()
            }
            _ => {
                for path in names.iter() {
                    std::fs::remove_file(path).ok();
                }
                panic!("restore_all was expected to return `trash::ErrorKind::RestoreCollision` but did not.");
            }
        };
        let remaining = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .collect::<Vec<_>>();
        assert_eq!(remaining.len(), remaining_count);
        trash::os_limited::purge_all(remaining).unwrap();
        for path in names.iter() {
            // This will obviously fail on the items that both didn't collide and weren't restored.
            std::fs::remove_file(path).ok();
        }
    }

    #[test]
    #[serial]
    fn restore_twins() {
        init_logging();
        let file_name_prefix = get_unique_name();
        let file_count: usize = 4;
        let names: Vec<_> = (0..file_count).map(|i| format!("{}#{}", file_name_prefix, i)).collect();
        for path in names.iter() {
            File::create_new(path).unwrap();
        }
        trash::delete_all(&names).unwrap();

        let twin_name = &names[1];
        File::create_new(twin_name).unwrap();
        trash::delete(twin_name).unwrap();

        let mut targets: Vec<_> = trash::os_limited::list()
            .unwrap()
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .collect();
        targets.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(targets.len(), file_count + 1); // plus one for one of the twins
        match trash::restore_all(targets) {
            Err(trash::Error::RestoreTwins { path, items }) => {
                assert_eq!(path.file_name().unwrap().to_str().unwrap(), twin_name);
                trash::os_limited::purge_all(items).unwrap();
            }
            _ => panic!("restore_all was expected to return `trash::ErrorKind::RestoreTwins` but did not."),
        }
    }

    #[test]
    #[serial]
    fn is_empty_matches_list() {
        init_logging();

        let is_empty_list = trash::os_limited::list().unwrap().is_empty();
        let is_empty = trash::os_limited::is_empty().unwrap();
        assert_eq!(is_empty, is_empty_list, "is_empty() should match empty status from list()");
    }
}
