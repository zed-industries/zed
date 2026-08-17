// // Copyright (c) Microsoft Corporation.
// // Licensed under the MIT License.

// mod common;
// use std::{env, path::PathBuf, sync::Once};

// use common::resolve_test_path;
// use pet_python_utils::cache::{get_cache_directory, set_cache_directory};

// static INIT: Once = Once::new();

// /// Setup function that is only run once, even if called multiple times.
// fn setup() {
//     INIT.call_once(|| {
//         env_logger::builder()
//             .filter(None, log::LevelFilter::Trace)
//             .init();

//         set_cache_directory(env::temp_dir().join("pet_cache"));
//     });
// }

// #[cfg_attr(
//     any(
//         feature = "ci", // Try to run this in all ci jobs/environments
//         feature = "ci-jupyter-container",
//         feature = "ci-homebrew-container",
//         feature = "ci-poetry-global",
//         feature = "ci-poetry-project",
//         feature = "ci-poetry-custom",
//     ),
//     test
// )]
// #[allow(dead_code)]
// fn verify_cache() {
//     use std::fs;

//     use pet_python_utils::{
//         cache::{clear_cache, create_cache},
//         env::ResolvedPythonEnv,
//         fs_cache::generate_cache_file,
//     };

//     setup();

//     let cache_dir = get_cache_directory().unwrap();
//     let prefix: PathBuf = resolve_test_path(&["unix", "executables", ".venv"]).into();
//     let bin = prefix.join("bin");
//     let python = bin.join("python");
//     let python3 = bin.join("python3");
//     let resolve_env = ResolvedPythonEnv {
//         executable: python.clone(),
//         version: "3.9.9".to_string(),
//         prefix: prefix.clone(),
//         is64_bit: true,
//         symlinks: Some(vec![python.clone(), python3.clone()]),
//     };

//     // Ensure the file does not exist.
//     let cache_file = generate_cache_file(&cache_dir, &resolve_env.executable);
//     let _ = fs::remove_file(&cache_file);

//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     // No cache file, so we should not have a value.
//     assert!(cache.get().is_none());
//     assert!(!cache_file.exists());

//     // Store the value in cache and verify the file exists.
//     cache.store(resolve_env.clone());

//     assert!(cache.get().is_some());
//     assert!(cache_file.exists());
//     drop(cache);

//     // Creating a new cache should load the value from the file.
//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     assert!(cache.get().is_some());
//     assert!(cache_file.exists());
//     drop(cache);

//     // Deleting the cache file and Creating a new cache should not load the value from the file.
//     let _ = clear_cache();
//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     assert!(cache.get().is_none());
//     assert!(!cache_file.exists());
// }

// #[cfg_attr(
//     any(
//         feature = "ci", // Try to run this in all ci jobs/environments
//         feature = "ci-jupyter-container",
//         feature = "ci-homebrew-container",
//         feature = "ci-poetry-global",
//         feature = "ci-poetry-project",
//         feature = "ci-poetry-custom",
//     ),
//     test
// )]
// #[allow(dead_code)]
// fn verify_invalidating_cache() {
//     use std::{fs, time::SystemTime};

//     use pet_python_utils::{
//         cache::create_cache, env::ResolvedPythonEnv, fs_cache::generate_cache_file,
//     };

//     setup();

//     let cache_dir = get_cache_directory().unwrap();
//     let prefix: PathBuf = resolve_test_path(&["unix", "executables", ".venv"]).into();
//     let bin = prefix.join("bin");
//     let python = bin.join("python");
//     let python3 = bin.join("python3");
//     let resolve_env = ResolvedPythonEnv {
//         executable: python.clone(),
//         version: "3.9.9".to_string(),
//         prefix: prefix.clone(),
//         is64_bit: true,
//         symlinks: Some(vec![python.clone(), python3.clone()]),
//     };

//     // Ensure the file does not exist.
//     let cache_file = generate_cache_file(&cache_dir, &resolve_env.executable);
//     let _ = fs::remove_file(&cache_file);

//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     // Store the value in cache and verify the file exists.
//     cache.store(resolve_env.clone());

//     assert!(cache.get().is_some());
//     assert!(cache_file.exists());

//     // Next update the executable, so as to cause the mtime to change.
//     // As a result of this the cache should no longer be valid.
//     let _ = fs::write(python.clone(), format!("{:?}", SystemTime::now()));
//     assert!(cache.get().is_none());
//     assert!(!cache_file.exists());
// }

// #[cfg_attr(
//     any(
//         feature = "ci", // Try to run this in all ci jobs/environments
//         feature = "ci-jupyter-container",
//         feature = "ci-homebrew-container",
//         feature = "ci-poetry-global",
//         feature = "ci-poetry-project",
//         feature = "ci-poetry-custom",
//     ),
//     test
// )]
// #[allow(dead_code)]
// fn verify_invalidating_cache_due_to_hash_conflicts() {
//     use std::fs;

//     use pet_python_utils::{
//         cache::{clear_cache, create_cache},
//         env::ResolvedPythonEnv,
//         fs_cache::generate_cache_file,
//     };

//     setup();

//     let cache_dir = get_cache_directory().unwrap();
//     let prefix: PathBuf = resolve_test_path(&["unix", "executables", ".venv"]).into();
//     let bin = prefix.join("bin");
//     let python = bin.join("python");
//     let python3 = bin.join("python3");
//     let resolve_env = ResolvedPythonEnv {
//         executable: python.clone(),
//         version: "3.9.9".to_string(),
//         prefix: prefix.clone(),
//         is64_bit: true,
//         symlinks: Some(vec![python.clone(), python3.clone()]),
//     };

//     // Ensure the file does not exist.
//     let cache_file = generate_cache_file(&cache_dir, &resolve_env.executable);
//     let _ = fs::remove_file(&cache_file);

//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     // Store the value in cache and verify the file exists.
//     cache.store(resolve_env.clone());
//     assert!(cache.get().is_some());
//     assert!(cache_file.exists());
//     drop(cache);

//     // Simulate a hash collision by changing the executable to a different value.
//     // I.e. the cached file points to another executable.
//     let contents = fs::read_to_string(&cache_file.clone()).unwrap();
//     let contents = contents.replace(
//         python.to_string_lossy().to_string().as_str(),
//         "/usr/bin/python",
//     );
//     let contents = contents.replace(
//         python
//             .to_string_lossy()
//             .to_string()
//             .replace("\\", "\\\\") // For windows paths stored in JSON
//             .as_str(),
//         "/usr/bin/python",
//     );
//     let contents = contents.replace(
//         python3.to_string_lossy().to_string().as_str(),
//         "/usr/bin/python3",
//     );
//     let contents = contents.replace(
//         python3
//             .to_string_lossy()
//             .to_string()
//             .replace("\\", "\\\\") // For windows paths stored in JSON
//             .as_str(),
//         "/usr/bin/python3",
//     );

//     let _ = clear_cache(); // Clear in memory cache as well as the files..
//     let _ = fs::create_dir_all(&cache_dir).unwrap();
//     let _ = fs::write(&cache_file, contents.clone()); // Create the cache file with the invalid details.
//     let cache = create_cache(resolve_env.executable.clone());
//     let cache = cache.lock().unwrap();

//     assert!(cache.get().is_none());
// }
