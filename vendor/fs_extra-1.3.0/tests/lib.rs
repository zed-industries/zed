use std::fs::read_dir;
use std::path::Path;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

extern crate fs_extra;
use fs_extra::error::*;
use fs_extra::*;

fn files_eq<P, Q>(file1: P, file2: Q) -> bool
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let content1 = fs_extra::file::read_to_string(file1).unwrap();
    let content2 = fs_extra::file::read_to_string(file2).unwrap();
    content1 == content2
}

fn compare_dir<P, Q>(path_from: P, path_to: Q) -> bool
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut path_to = path_to.as_ref().to_path_buf();
    match path_from.as_ref().components().last() {
        None => panic!("Invalid folder from"),
        Some(dir_name) => {
            path_to.push(dir_name.as_os_str());
            if !path_to.exists() {
                return false;
            }
        }
    }

    for entry in read_dir(&path_from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if !compare_dir(path, &path_to) {
                return false;
            }
        } else {
            let mut path_to = path_to.to_path_buf();
            match path.file_name() {
                None => panic!("No file name"),
                Some(file_name) => {
                    path_to.push(file_name);
                    if !path_to.exists() {
                        return false;
                    } else if !files_eq(&path, path_to.clone()) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

const TEST_FOLDER: &'static str = "./tests/temp/lib";

#[test]
fn it_copy_work() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_work");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };
    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();

    assert!(dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(!sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    let result = copy_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(compare_dir(&dir1.0, &path_to));
    assert!(compare_dir(&dir2.0, &path_to));
    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
}

#[test]
fn it_copy_source_not_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_source_not_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    assert!(!dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(!dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(!sub.0.exists());
    assert!(!sub.1.exists());

    assert!(!file1.0.exists());
    assert!(!file1.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());

    let options = dir::CopyOptions::new();
    match copy_items(&from_paths, &path_to, &options) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => {}
        },
    };
}

#[test]
fn it_copy_exist_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_exist_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    let result = copy_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(compare_dir(&dir1.0, &path_to));
    assert!(compare_dir(&dir2.0, &path_to));
    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
}

#[test]
fn it_copy_exist_not_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_exist_not_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    match copy_items(&from_paths, &path_to, &options) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {}
            _ => panic!(format!("{}", err.to_string())),
        },
    };
}

#[test]
fn it_copy_exist_skip() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_exist_skip");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.skip_exist = true;
    let result = copy_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(16, result);
    assert!(!compare_dir(&dir1.0, &path_to));
    assert!(compare_dir(&dir2.0, &path_to));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
}

#[test]
fn it_copy_exist_overwrite_and_skip_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_exist_overwrite_and_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    let result = copy_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(compare_dir(&dir1.0, &path_to));
    assert!(compare_dir(&dir2.0, &path_to));
    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
}

#[test]
fn it_copy_using_first_levels() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_using_first_levels");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.join("d_level_1"));
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));
    let d_level_4 = (d_level_3.0.join("d_level_4"), d_level_3.1.join("d_level_4"));
    let d_level_5 = (d_level_4.0.join("d_level_5"), d_level_4.1.join("d_level_5"));

    let d2_level_1 = (test_dir.join("d2_level_1"), path_to.join("d2_level_1"));
    let d2_level_2 = (
        d_level_1.0.join("d2_level_2"),
        d_level_1.1.join("d2_level_2"),
    );
    let d2_level_3 = (
        d_level_2.0.join("d2_level_3"),
        d_level_2.1.join("d2_level_3"),
    );
    let d2_level_4 = (
        d_level_3.0.join("d2_level_4"),
        d_level_3.1.join("d2_level_4"),
    );
    let d2_level_5 = (
        d_level_4.0.join("d2_level_5"),
        d_level_4.1.join("d2_level_5"),
    );

    let d3_level_1 = (test_dir.join("d3_level_1"), path_to.join("d3_level_1"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    let file21 = (
        d2_level_1.0.join("file21.txt"),
        d2_level_1.1.join("file21.txt"),
    );
    let file22 = (
        d2_level_2.0.join("file22.txt"),
        d2_level_2.1.join("file22.txt"),
    );
    let file23 = (
        d2_level_3.0.join("file23.txt"),
        d2_level_3.1.join("file23.txt"),
    );
    let file24 = (
        d2_level_4.0.join("file24.txt"),
        d2_level_4.1.join("file24.txt"),
    );
    let file25 = (
        d2_level_5.0.join("file25.txt"),
        d2_level_5.1.join("file25.txt"),
    );

    let file31 = (
        d3_level_1.0.join("file31.txt"),
        d3_level_1.1.join("file31.txt"),
    );

    dir::create_all(&d_level_1.0, true).unwrap();
    dir::create_all(&d_level_2.0, true).unwrap();
    dir::create_all(&d_level_3.0, true).unwrap();
    dir::create_all(&d_level_4.0, true).unwrap();
    dir::create_all(&d_level_5.0, true).unwrap();
    dir::create_all(&path_to, true).unwrap();

    dir::create_all(&d2_level_1.0, true).unwrap();
    dir::create_all(&d2_level_2.0, true).unwrap();
    dir::create_all(&d2_level_3.0, true).unwrap();
    dir::create_all(&d2_level_4.0, true).unwrap();
    dir::create_all(&d2_level_5.0, true).unwrap();

    dir::create_all(&d3_level_1.0, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d2_level_1.0.exists());
    assert!(d2_level_2.0.exists());
    assert!(d2_level_3.0.exists());
    assert!(d2_level_4.0.exists());
    assert!(d2_level_5.0.exists());

    assert!(d3_level_1.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    assert!(!d2_level_1.1.exists());
    assert!(!d2_level_2.1.exists());
    assert!(!d2_level_3.1.exists());
    assert!(!d2_level_4.1.exists());
    assert!(!d2_level_5.1.exists());

    assert!(!d3_level_1.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file21.0, "2content1").unwrap();
    fs_extra::file::write_all(&file22.0, "2content2").unwrap();
    fs_extra::file::write_all(&file23.0, "2content3").unwrap();
    fs_extra::file::write_all(&file24.0, "2content4").unwrap();
    fs_extra::file::write_all(&file25.0, "2content5").unwrap();

    fs_extra::file::write_all(&file31.0, "3content1").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    assert!(!file21.1.exists());
    assert!(!file22.1.exists());
    assert!(!file23.1.exists());
    assert!(!file24.1.exists());
    assert!(!file25.1.exists());

    assert!(!file31.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(d_level_1.0.as_path());
    from_paths.push(d2_level_1.0.as_path());
    from_paths.push(d3_level_1.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.depth = 1;
    let result = copy_items(&from_paths, path_to, &options).unwrap();

    assert_eq!(26, result);

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    assert!(file21.1.exists());
    assert!(!file22.1.exists());
    assert!(!file23.1.exists());
    assert!(!file24.1.exists());
    assert!(!file25.1.exists());

    assert!(file31.1.exists());
    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file21.0, &file21.1));
    assert!(files_eq(&file31.0, &file31.1));
}

#[test]
fn it_copy_using_four_levels() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_using_four_levels");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.join("d_level_1"));
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));
    let d_level_4 = (d_level_3.0.join("d_level_4"), d_level_3.1.join("d_level_4"));
    let d_level_5 = (d_level_4.0.join("d_level_5"), d_level_4.1.join("d_level_5"));

    let d2_level_1 = (test_dir.join("d2_level_1"), path_to.join("d2_level_1"));
    let d2_level_2 = (
        d_level_1.0.join("d2_level_2"),
        d_level_1.1.join("d2_level_2"),
    );
    let d2_level_3 = (
        d_level_2.0.join("d2_level_3"),
        d_level_2.1.join("d2_level_3"),
    );
    let d2_level_4 = (
        d_level_3.0.join("d2_level_4"),
        d_level_3.1.join("d2_level_4"),
    );
    let d2_level_5 = (
        d_level_4.0.join("d2_level_5"),
        d_level_4.1.join("d2_level_5"),
    );

    let d3_level_1 = (test_dir.join("d3_level_1"), path_to.join("d3_level_1"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    let file21 = (
        d2_level_1.0.join("file21.txt"),
        d2_level_1.1.join("file21.txt"),
    );
    let file22 = (
        d2_level_2.0.join("file22.txt"),
        d2_level_2.1.join("file22.txt"),
    );
    let file23 = (
        d2_level_3.0.join("file23.txt"),
        d2_level_3.1.join("file23.txt"),
    );
    let file24 = (
        d2_level_4.0.join("file24.txt"),
        d2_level_4.1.join("file24.txt"),
    );
    let file25 = (
        d2_level_5.0.join("file25.txt"),
        d2_level_5.1.join("file25.txt"),
    );

    let file31 = (
        d3_level_1.0.join("file31.txt"),
        d3_level_1.1.join("file31.txt"),
    );

    dir::create_all(&d_level_1.0, true).unwrap();
    dir::create_all(&d_level_2.0, true).unwrap();
    dir::create_all(&d_level_3.0, true).unwrap();
    dir::create_all(&d_level_4.0, true).unwrap();
    dir::create_all(&d_level_5.0, true).unwrap();
    dir::create_all(&path_to, true).unwrap();

    dir::create_all(&d2_level_1.0, true).unwrap();
    dir::create_all(&d2_level_2.0, true).unwrap();
    dir::create_all(&d2_level_3.0, true).unwrap();
    dir::create_all(&d2_level_4.0, true).unwrap();
    dir::create_all(&d2_level_5.0, true).unwrap();

    dir::create_all(&d3_level_1.0, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d2_level_1.0.exists());
    assert!(d2_level_2.0.exists());
    assert!(d2_level_3.0.exists());
    assert!(d2_level_4.0.exists());
    assert!(d2_level_5.0.exists());

    assert!(d3_level_1.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    assert!(!d2_level_1.1.exists());
    assert!(!d2_level_2.1.exists());
    assert!(!d2_level_3.1.exists());
    assert!(!d2_level_4.1.exists());
    assert!(!d2_level_5.1.exists());

    assert!(!d3_level_1.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file21.0, "2content1").unwrap();
    fs_extra::file::write_all(&file22.0, "2content2").unwrap();
    fs_extra::file::write_all(&file23.0, "2content3").unwrap();
    fs_extra::file::write_all(&file24.0, "2content4").unwrap();
    fs_extra::file::write_all(&file25.0, "2content5").unwrap();

    fs_extra::file::write_all(&file31.0, "3content1").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    assert!(!file21.1.exists());
    assert!(!file22.1.exists());
    assert!(!file23.1.exists());
    assert!(!file24.1.exists());
    assert!(!file25.1.exists());

    assert!(!file31.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(d_level_1.0.as_path());
    from_paths.push(d2_level_1.0.as_path());
    from_paths.push(d3_level_1.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.depth = 4;
    let result = copy_items(&from_paths, path_to, &options).unwrap();

    assert_eq!(77, result);

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    assert!(file21.1.exists());
    assert!(file22.1.exists());
    assert!(file23.1.exists());
    assert!(file24.1.exists());
    assert!(!file25.1.exists());

    assert!(file31.1.exists());
    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file21.0, &file21.1));
    assert!(files_eq(&file31.0, &file31.1));
}
#[test]

fn it_copy_content_only_opton() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_content_only_opton");
    let path_to = test_dir.join("out");

    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    match copy_items(&vec![&file1.0], &file1.1, &options) {
        Err(err) => match err.kind {
            ErrorKind::Other => {
                assert_eq!(1, 1);
            }
            _ => {
                panic!(format!("wrong error {}", err.to_string()));
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_copy_progress_work() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_progress_work");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };
    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();

    assert!(dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(!sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content22").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    let options = dir::CopyOptions::new();
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };
        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(41, result);
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(41, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(41, process_info.total_bytes);
                }
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
}

#[test]
fn it_copy_with_progress_work_dif_buf_size() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_work_dif_buf_size");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };
    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();

    assert!(dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(!sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(file1.0.as_path().to_str().unwrap().to_string());
        from_paths.push(file2.0.as_path().to_str().unwrap().to_string());
        from_paths.push(dir1.0.as_path().to_str().unwrap().to_string());
        from_paths.push(dir2.0.as_path().to_str().unwrap().to_string());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(40, result);
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));

        let mut options = dir::CopyOptions::new();
        options.buffer_size = 2;
        options.overwrite = true;
        let (tx, rx) = mpsc::channel();
        let result = thread::spawn(move || {
            let func_test = |process_info: TransitProcess| {
                tx.send(process_info).unwrap();
                dir::TransitProcessResult::ContinueOrAbort
            };
            let result =
                copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

            assert_eq!(40, result);
            assert!(compare_dir(&dir1.0, &path_to));
            assert!(compare_dir(&dir2.0, &path_to));
            assert!(files_eq(&file1.0, &file1.1));
            assert!(files_eq(&file2.0, &file2.1));
        })
        .join();
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(i * 2, process_info.copied_bytes);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(40, process_info.total_bytes);
        }
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2 + 8, process_info.copied_bytes);
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(40, process_info.total_bytes);
        }

        match result {
            Ok(_) => {}
            Err(err) => panic!(err),
        }
    })
    .join();

    for i in 1..9 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i, process_info.file_bytes_copied);
        assert_eq!(i, process_info.copied_bytes);
        assert_eq!(8, process_info.file_total_bytes);
        assert_eq!(40, process_info.total_bytes);
    }
    for i in 1..9 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i + 8, process_info.copied_bytes);
        assert_eq!(i, process_info.file_bytes_copied);
        assert_eq!(8, process_info.file_total_bytes);
        assert_eq!(40, process_info.total_bytes);
    }

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
}

#[test]
fn it_copy_with_progress_source_not_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_source_not_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    assert!(!dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(!dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(!sub.0.exists());
    assert!(!sub.1.exists());

    assert!(!file1.0.exists());
    assert!(!file1.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());

    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };

    let options = dir::CopyOptions::new();
    match copy_items_with_progress(&from_paths, &path_to, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => {}
        },
    };
}

#[test]
fn it_copy_with_progress_exist_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(40, result);
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_copy_with_progress_exist_not_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_not_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };

    match copy_items_with_progress(&from_paths, &path_to, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {}
            _ => panic!(format!("{}", err.to_string())),
        },
    };
}

#[test]
fn it_copy_with_progress_exist_skip_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(16, result);
        assert!(!compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(!files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_copy_with_progress_exist_overwrite_and_skip_exist() {
    let test_dir =
        Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_overwrite_and_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(40, result);
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_copy_with_progress_using_first_levels() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_using_first_levels");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.join("d_level_1"));
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));
    let d_level_4 = (d_level_3.0.join("d_level_4"), d_level_3.1.join("d_level_4"));
    let d_level_5 = (d_level_4.0.join("d_level_5"), d_level_4.1.join("d_level_5"));

    let d2_level_1 = (test_dir.join("d2_level_1"), path_to.join("d2_level_1"));
    let d2_level_2 = (
        d_level_1.0.join("d2_level_2"),
        d_level_1.1.join("d2_level_2"),
    );
    let d2_level_3 = (
        d_level_2.0.join("d2_level_3"),
        d_level_2.1.join("d2_level_3"),
    );
    let d2_level_4 = (
        d_level_3.0.join("d2_level_4"),
        d_level_3.1.join("d2_level_4"),
    );
    let d2_level_5 = (
        d_level_4.0.join("d2_level_5"),
        d_level_4.1.join("d2_level_5"),
    );

    let d3_level_1 = (test_dir.join("d3_level_1"), path_to.join("d3_level_1"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    let file21 = (
        d2_level_1.0.join("file21.txt"),
        d2_level_1.1.join("file21.txt"),
    );
    let file22 = (
        d2_level_2.0.join("file22.txt"),
        d2_level_2.1.join("file22.txt"),
    );
    let file23 = (
        d2_level_3.0.join("file23.txt"),
        d2_level_3.1.join("file23.txt"),
    );
    let file24 = (
        d2_level_4.0.join("file24.txt"),
        d2_level_4.1.join("file24.txt"),
    );
    let file25 = (
        d2_level_5.0.join("file25.txt"),
        d2_level_5.1.join("file25.txt"),
    );

    let file31 = (
        d3_level_1.0.join("file31.txt"),
        d3_level_1.1.join("file31.txt"),
    );

    dir::create_all(&d_level_1.0, true).unwrap();
    dir::create_all(&d_level_2.0, true).unwrap();
    dir::create_all(&d_level_3.0, true).unwrap();
    dir::create_all(&d_level_4.0, true).unwrap();
    dir::create_all(&d_level_5.0, true).unwrap();
    dir::create_all(&path_to, true).unwrap();

    dir::create_all(&d2_level_1.0, true).unwrap();
    dir::create_all(&d2_level_2.0, true).unwrap();
    dir::create_all(&d2_level_3.0, true).unwrap();
    dir::create_all(&d2_level_4.0, true).unwrap();
    dir::create_all(&d2_level_5.0, true).unwrap();

    dir::create_all(&d3_level_1.0, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d2_level_1.0.exists());
    assert!(d2_level_2.0.exists());
    assert!(d2_level_3.0.exists());
    assert!(d2_level_4.0.exists());
    assert!(d2_level_5.0.exists());

    assert!(d3_level_1.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    assert!(!d2_level_1.1.exists());
    assert!(!d2_level_2.1.exists());
    assert!(!d2_level_3.1.exists());
    assert!(!d2_level_4.1.exists());
    assert!(!d2_level_5.1.exists());

    assert!(!d3_level_1.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file21.0, "2content1").unwrap();
    fs_extra::file::write_all(&file22.0, "2content2").unwrap();
    fs_extra::file::write_all(&file23.0, "2content3").unwrap();
    fs_extra::file::write_all(&file24.0, "2content4").unwrap();
    fs_extra::file::write_all(&file25.0, "2content5").unwrap();

    fs_extra::file::write_all(&file31.0, "3content1").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    assert!(!file21.1.exists());
    assert!(!file22.1.exists());
    assert!(!file23.1.exists());
    assert!(!file24.1.exists());
    assert!(!file25.1.exists());

    assert!(!file31.1.exists());

    let mut options = dir::CopyOptions::new();
    options.depth = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(d_level_1.0.as_path());
        from_paths.push(d2_level_1.0.as_path());
        from_paths.push(d3_level_1.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(26, result);

        assert!(file1.0.exists());
        assert!(file2.0.exists());
        assert!(file3.0.exists());
        assert!(file4.0.exists());
        assert!(file5.0.exists());

        assert!(file21.0.exists());
        assert!(file22.0.exists());
        assert!(file23.0.exists());
        assert!(file24.0.exists());
        assert!(file25.0.exists());

        assert!(file31.0.exists());

        assert!(file1.1.exists());
        assert!(!file2.1.exists());
        assert!(!file3.1.exists());
        assert!(!file4.1.exists());
        assert!(!file5.1.exists());

        assert!(file21.1.exists());
        assert!(!file22.1.exists());
        assert!(!file23.1.exists());
        assert!(!file24.1.exists());
        assert!(!file25.1.exists());

        assert!(file31.1.exists());
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file21.0, &file21.1));
        assert!(files_eq(&file31.0, &file31.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_copy_with_progress_using_four_levels() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_using_four_levels");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.join("d_level_1"));
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));
    let d_level_4 = (d_level_3.0.join("d_level_4"), d_level_3.1.join("d_level_4"));
    let d_level_5 = (d_level_4.0.join("d_level_5"), d_level_4.1.join("d_level_5"));

    let d2_level_1 = (test_dir.join("d2_level_1"), path_to.join("d2_level_1"));
    let d2_level_2 = (
        d_level_1.0.join("d2_level_2"),
        d_level_1.1.join("d2_level_2"),
    );
    let d2_level_3 = (
        d_level_2.0.join("d2_level_3"),
        d_level_2.1.join("d2_level_3"),
    );
    let d2_level_4 = (
        d_level_3.0.join("d2_level_4"),
        d_level_3.1.join("d2_level_4"),
    );
    let d2_level_5 = (
        d_level_4.0.join("d2_level_5"),
        d_level_4.1.join("d2_level_5"),
    );

    let d3_level_1 = (test_dir.join("d3_level_1"), path_to.join("d3_level_1"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    let file21 = (
        d2_level_1.0.join("file21.txt"),
        d2_level_1.1.join("file21.txt"),
    );
    let file22 = (
        d2_level_2.0.join("file22.txt"),
        d2_level_2.1.join("file22.txt"),
    );
    let file23 = (
        d2_level_3.0.join("file23.txt"),
        d2_level_3.1.join("file23.txt"),
    );
    let file24 = (
        d2_level_4.0.join("file24.txt"),
        d2_level_4.1.join("file24.txt"),
    );
    let file25 = (
        d2_level_5.0.join("file25.txt"),
        d2_level_5.1.join("file25.txt"),
    );

    let file31 = (
        d3_level_1.0.join("file31.txt"),
        d3_level_1.1.join("file31.txt"),
    );

    dir::create_all(&d_level_1.0, true).unwrap();
    dir::create_all(&d_level_2.0, true).unwrap();
    dir::create_all(&d_level_3.0, true).unwrap();
    dir::create_all(&d_level_4.0, true).unwrap();
    dir::create_all(&d_level_5.0, true).unwrap();
    dir::create_all(&path_to, true).unwrap();

    dir::create_all(&d2_level_1.0, true).unwrap();
    dir::create_all(&d2_level_2.0, true).unwrap();
    dir::create_all(&d2_level_3.0, true).unwrap();
    dir::create_all(&d2_level_4.0, true).unwrap();
    dir::create_all(&d2_level_5.0, true).unwrap();

    dir::create_all(&d3_level_1.0, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d2_level_1.0.exists());
    assert!(d2_level_2.0.exists());
    assert!(d2_level_3.0.exists());
    assert!(d2_level_4.0.exists());
    assert!(d2_level_5.0.exists());

    assert!(d3_level_1.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    assert!(!d2_level_1.1.exists());
    assert!(!d2_level_2.1.exists());
    assert!(!d2_level_3.1.exists());
    assert!(!d2_level_4.1.exists());
    assert!(!d2_level_5.1.exists());

    assert!(!d3_level_1.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file21.0, "2content1").unwrap();
    fs_extra::file::write_all(&file22.0, "2content2").unwrap();
    fs_extra::file::write_all(&file23.0, "2content3").unwrap();
    fs_extra::file::write_all(&file24.0, "2content4").unwrap();
    fs_extra::file::write_all(&file25.0, "2content5").unwrap();

    fs_extra::file::write_all(&file31.0, "3content1").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file21.0.exists());
    assert!(file22.0.exists());
    assert!(file23.0.exists());
    assert!(file24.0.exists());
    assert!(file25.0.exists());

    assert!(file31.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    assert!(!file21.1.exists());
    assert!(!file22.1.exists());
    assert!(!file23.1.exists());
    assert!(!file24.1.exists());
    assert!(!file25.1.exists());

    assert!(!file31.1.exists());

    let mut options = dir::CopyOptions::new();
    options.depth = 4;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(d_level_1.0.as_path());
        from_paths.push(d2_level_1.0.as_path());
        from_paths.push(d3_level_1.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };

        let result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(77, result);

        assert!(file1.0.exists());
        assert!(file2.0.exists());
        assert!(file3.0.exists());
        assert!(file4.0.exists());
        assert!(file5.0.exists());

        assert!(file21.0.exists());
        assert!(file22.0.exists());
        assert!(file23.0.exists());
        assert!(file24.0.exists());
        assert!(file25.0.exists());

        assert!(file31.0.exists());

        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(!file5.1.exists());

        assert!(file21.1.exists());
        assert!(file22.1.exists());
        assert!(file23.1.exists());
        assert!(file24.1.exists());
        assert!(!file25.1.exists());

        assert!(file31.1.exists());
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file21.0, &file21.1));
        assert!(files_eq(&file31.0, &file31.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_copy_with_progress_content_only_opton() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_content_only_opton");
    let path_to = test_dir.join("out");

    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };
    match copy_items_with_progress(&vec![&file1.0], &file1.1, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::Other => {}
            _ => panic!(format!("wrong error {}", err.to_string())),
        },
    };
}

#[test]
fn it_move_work() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_work");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };
    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();

    assert!(dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(!sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    let result = move_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(!file1.0.exists());
    assert!(!file2.0.exists());
    assert!(!file3.0.exists());
    assert!(!file4.0.exists());
    assert!(!file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());
}

#[test]
fn it_move_source_not_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_source_not_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    assert!(!dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(!dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(!sub.0.exists());
    assert!(!sub.1.exists());

    assert!(!file1.0.exists());
    assert!(!file1.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());

    let options = dir::CopyOptions::new();
    match move_items(&from_paths, &path_to, &options) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => {}
        },
    };
}

#[test]
fn it_move_exist_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_exist_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    let result = move_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(!file1.0.exists());
    assert!(!file2.0.exists());
    assert!(!file3.0.exists());
    assert!(!file4.0.exists());
    assert!(!file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());
}

#[test]
fn it_move_exist_not_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_exist_not_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    match move_items(&from_paths, &path_to, &options) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {}
            _ => panic!(format!("{}", err.to_string())),
        },
    };
}

#[test]
fn it_move_exist_skip() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_exist_skip");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.skip_exist = true;
    let result = move_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(16, result);
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!file2.0.exists());
    assert!(!files_eq(&file3.0, &file3.1));
    assert!(!files_eq(&file4.0, &file4.1));
    assert!(!file5.0.exists());
}

#[test]
fn it_move_exist_overwrite_and_skip_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_exist_overwrite_and_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    let result = move_items(&from_paths, &path_to, &options).unwrap();

    assert_eq!(40, result);
    assert!(!file1.0.exists());
    assert!(!file2.0.exists());
    assert!(!file3.0.exists());
    assert!(!file4.0.exists());
    assert!(!file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());
}
#[test]
fn it_move_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_content_only_option");
    let path_to = test_dir.join("out");

    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    match move_items(&vec![&file1.0], &file1.1, &options) {
        Err(err) => match err.kind {
            ErrorKind::Other => {
                assert_eq!(1, 1);
            }
            _ => {
                panic!(format!("wrong error {}", err.to_string()));
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_move_progress_work() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_progress_work");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };
    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();

    assert!(dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(!sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content22").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());

    let options = dir::CopyOptions::new();
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };
        let result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(41, result);
        assert!(!file1.0.exists());
        assert!(!file2.0.exists());
        assert!(!file3.0.exists());
        assert!(!file4.0.exists());
        assert!(!file5.0.exists());
        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(file5.1.exists());
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(41, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(41, process_info.total_bytes);
                }
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
}

#[test]
fn it_move_with_progress_source_not_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_source_not_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    assert!(!dir1.0.exists());
    assert!(!dir1.1.exists());
    assert!(!dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(!sub.0.exists());
    assert!(!sub.1.exists());

    assert!(!file1.0.exists());
    assert!(!file1.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());

    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };
    let options = dir::CopyOptions::new();
    match move_items_with_progress(&from_paths, &path_to, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => {}
        },
    };
}

#[test]
fn it_move_with_progress_exist_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };
        let result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(40, result);
        assert!(!file1.0.exists());
        assert!(!file2.0.exists());
        assert!(!file3.0.exists());
        assert!(!file4.0.exists());
        assert!(!file5.0.exists());
        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(file5.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_move_with_progress_exist_not_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_not_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.0.as_path());
    from_paths.push(dir2.0.as_path());
    from_paths.push(file1.0.as_path());
    from_paths.push(file2.0.as_path());

    let options = dir::CopyOptions::new();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };
    match move_items_with_progress(&from_paths, &path_to, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {}
            _ => panic!(format!("{}", err.to_string())),
        },
    };
}

#[test]
fn it_move_with_progress_exist_skip_exist() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();
    file::write_all(&file5.1, "old content5").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.skip_exist = true;
    options.overwrite = false;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };
        let result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(8, result);
        assert!(file1.0.exists());
        assert!(!file2.0.exists());
        assert!(file3.0.exists());
        assert!(file4.0.exists());
        assert!(file5.0.exists());
        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(file5.1.exists());
        assert!(!files_eq(&file1.0, &file1.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_move_with_progress_exist_overwrite_and_skip_exist() {
    let test_dir =
        Path::new(TEST_FOLDER).join("it_move_with_progress_exist_overwrite_and_skip_exist");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    match dir::create_all(&path_to, true) {
        Ok(_) => {}
        Err(_) => {}
    };

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(dir1.0.exists());
    assert!(dir1.1.exists());
    assert!(dir2.0.exists());
    assert!(!dir2.1.exists());
    assert!(sub.0.exists());
    assert!(sub.1.exists());

    file::write_all(&file1.0, "content1").unwrap();
    file::write_all(&file2.0, "content2").unwrap();
    file::write_all(&file3.0, "content3").unwrap();
    file::write_all(&file4.0, "content4").unwrap();
    file::write_all(&file5.0, "content5").unwrap();

    file::write_all(&file1.1, "old content1").unwrap();
    file::write_all(&file3.1, "old content3").unwrap();
    file::write_all(&file4.1, "old content4").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            dir::TransitProcessResult::ContinueOrAbort
        };
        let result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();

        assert_eq!(40, result);
        assert!(!file1.0.exists());
        assert!(!file2.0.exists());
        assert!(!file3.0.exists());
        assert!(!file4.0.exists());
        assert!(!file5.0.exists());
        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(file5.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => panic!("Errors should not be!"),
        _ => {}
    }
}

#[test]
fn it_move_with_progress_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_content_only_option");
    let path_to = test_dir.join("out");

    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));

    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        dir::TransitProcessResult::ContinueOrAbort
    };
    match move_items_with_progress(&vec![&file1.0], &file1.1, &options, func_test) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::Other => {}
            _ => panic!(format!("wrong error {}", err.to_string())),
        },
    };
}

#[test]
fn it_remove_work() {
    let test_dir = Path::new(TEST_FOLDER).join("it_remove_work");
    let dir1 = test_dir.join("dir1");
    let dir2 = test_dir.join("dir2");
    let sub = dir1.join("sub");
    let file1 = test_dir.join("file1.txt");
    let file2 = test_dir.join("file2.txt");
    let file3 = dir1.join("file3.txt");
    let file4 = sub.join("file4.txt");
    let file5 = dir2.join("file5.txt");

    dir::create_all(&dir1, true).unwrap();
    dir::create_all(&dir2, true).unwrap();
    dir::create_all(&sub, true).unwrap();

    assert!(dir1.exists());
    assert!(dir2.exists());
    assert!(sub.exists());

    file::write_all(&file1, "content1").unwrap();
    file::write_all(&file2, "content2").unwrap();
    file::write_all(&file3, "content3").unwrap();
    file::write_all(&file4, "content4").unwrap();
    file::write_all(&file5, "content5").unwrap();

    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(file4.exists());
    assert!(file5.exists());

    let mut from_paths = Vec::new();
    from_paths.push(dir1.as_path());
    from_paths.push(dir2.as_path());
    from_paths.push(file1.as_path());
    from_paths.push(file2.as_path());

    remove_items(&from_paths).unwrap();
    assert!(!file1.exists());
    assert!(!file2.exists());
    assert!(!file3.exists());
    assert!(!file4.exists());
    assert!(!file5.exists());
}

#[test]
fn it_copy_with_progress_exist_user_decide_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::Overwrite;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(5, count_exist_files);

        assert_eq!(40, result);
        assert!(dir1.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.0.exists());
        assert!(dir2.1.exists());
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_copy_with_progress_exist_user_decide_overwrite_all() {
    let test_dir =
        Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_overwrite_all");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::OverwriteAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(40, result);
        assert!(dir1.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.0.exists());
        assert!(dir2.1.exists());
        assert!(compare_dir(&dir1.0, &path_to));
        assert!(compare_dir(&dir2.0, &path_to));
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_copy_with_progress_exist_user_decide_skip() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_skip");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::Skip;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(5, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.0.exists());
        assert!(dir2.1.exists());
        assert!(!compare_dir(&dir1.0, &path_to));
        assert!(!compare_dir(&dir2.0, &path_to));
        assert!(!files_eq(&file1.0, &file1.1));
        assert!(!files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_copy_with_progress_exist_user_decide_skip_all() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_skip_all");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::SkipAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.0.exists());
        assert!(dir2.1.exists());
        assert!(!compare_dir(&dir1.0, &path_to));
        assert!(!compare_dir(&dir2.0, &path_to));
        assert!(!files_eq(&file1.0, &file1.1));
        assert!(!files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_copy_with_progress_exist_user_decide_retry() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_retry");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        if count_exist_files == 3 || count_exist_files > 6 {
                            result = dir::TransitProcessResult::Skip;
                        } else {
                            result = dir::TransitProcessResult::Retry;
                        }

                        count_exist_files += 1;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = copy_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(11, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.0.exists());
        assert!(dir2.1.exists());
        assert!(!compare_dir(&dir1.0, &path_to));
        assert!(!compare_dir(&dir2.0, &path_to));
        assert!(!files_eq(&file1.0, &file1.1));
        assert!(!files_eq(&file2.0, &file2.1));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_user_decide_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_overwrite");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::Overwrite;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(5, count_exist_files);

        assert_eq!(40, result);
        assert!(!dir1.0.exists());
        assert!(!dir2.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_user_decide_overwrite_all() {
    let test_dir =
        Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_overwrite_all");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::OverwriteAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(40, result);
        assert!(!dir1.0.exists());
        assert!(!dir2.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_user_decide_skip() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_skip");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::Skip;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(5, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir2.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_user_decide_skip_all() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_skip_all");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        count_exist_files += 1;
                        result = dir::TransitProcessResult::SkipAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir2.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.1.exists());
        assert!(file1.0.exists());
        assert!(file2.0.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_user_decide_retry() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_retry");
    let path_to = test_dir.join("out");
    let dir1 = (test_dir.join("dir1"), path_to.join("dir1"));
    let dir2 = (test_dir.join("dir2"), path_to.join("dir2"));
    let sub = (dir1.0.join("sub"), dir1.1.join("sub"));
    let file1 = (test_dir.join("file1.txt"), path_to.join("file1.txt"));
    let file2 = (test_dir.join("file2.txt"), path_to.join("file2.txt"));
    let file3 = (dir1.0.join("file3.txt"), dir1.1.join("file3.txt"));
    let file4 = (sub.0.join("file4.txt"), sub.1.join("file4.txt"));
    let file5 = (dir2.0.join("file5.txt"), dir2.1.join("file5.txt"));

    dir::create_all(&dir1.0, true).unwrap();
    dir::create_all(&dir1.1, true).unwrap();
    dir::create_all(&dir2.0, true).unwrap();
    dir::create_all(&dir2.1, true).unwrap();
    dir::create_all(&sub.0, true).unwrap();
    dir::create_all(&sub.1, true).unwrap();

    assert!(&dir1.0.exists());
    assert!(&dir1.1.exists());
    assert!(&dir2.0.exists());
    assert!(&dir2.1.exists());
    assert!(&sub.0.exists());
    assert!(&sub.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

    fs_extra::file::write_all(&file1.1, "old content11").unwrap();
    fs_extra::file::write_all(&file2.1, "old content12").unwrap();
    fs_extra::file::write_all(&file3.1, "old content13").unwrap();
    fs_extra::file::write_all(&file4.1, "old content14").unwrap();
    fs_extra::file::write_all(&file5.1, "old content15").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(file5.1.exists());

    let mut options = dir::CopyOptions::new();
    assert!(!compare_dir(&dir1.0, &dir1.1));
    assert!(!compare_dir(&dir2.0, &dir2.1));
    assert!(!files_eq(&file1.0, &file1.1));
    assert!(!files_eq(&file2.0, &file2.1));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let mut from_paths = Vec::new();
        from_paths.push(dir1.0.as_path());
        from_paths.push(dir2.0.as_path());
        from_paths.push(file1.0.as_path());
        from_paths.push(file2.0.as_path());

        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: dir::TransitProcessResult;
                match process_info.state {
                    dir::TransitState::Exists => {
                        if count_exist_files == 3 || count_exist_files > 6 {
                            result = dir::TransitProcessResult::Skip;
                        } else {
                            result = dir::TransitProcessResult::Retry;
                        }

                        count_exist_files += 1;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = dir::TransitProcessResult::Abort,
                };
                result
            };

            result = move_items_with_progress(&from_paths, &path_to, &options, func_test).unwrap();
        }
        assert_eq!(11, count_exist_files);

        assert_eq!(0, result);
        assert!(dir1.0.exists());
        assert!(dir2.0.exists());
        assert!(dir1.1.exists());
        assert!(dir2.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}
