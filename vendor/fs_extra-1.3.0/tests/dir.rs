use std::collections::HashSet;
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

extern crate fs_extra;
use fs_extra::dir::*;
use fs_extra::error::*;

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

// Returns the size of a directory. On Linux with ext4 this can be about 4kB.
// Since the directory size can vary, we need to calculate is dynamically.
fn get_dir_size() -> u64 {
    std::fs::create_dir_all("./tests/temp").expect("Couldn't create test folder");

    std::fs::metadata("./tests/temp")
        .expect("Couldn't receive metadata of tests/temp folder")
        .len()
}

#[cfg(unix)]
fn create_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> std::io::Result<()> {
    std::os::unix::fs::symlink(original.as_ref(), link.as_ref())
}

#[cfg(windows)]
fn create_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(original.as_ref(), link.as_ref())
}

const TEST_FOLDER: &'static str = "./tests/temp/dir";

#[test]
fn it_create_all_work() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_all_work");
    test_dir.push("sub_dir");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    assert!(!test_dir.exists());
    create_all(&test_dir, false).unwrap();
    assert!(test_dir.exists());
}

#[test]
fn it_create_work() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_work");
    if !test_dir.exists() {
        create_all(&test_dir, false).unwrap();
    }
    assert!(test_dir.exists());
    test_dir.push("sub_dir");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    create(&test_dir, false).unwrap();
    assert!(test_dir.exists());
}

#[test]
fn it_create_exist_folder() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_exist_folder");
    test_dir.push("sub");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    assert!(!test_dir.exists());
    create_all(&test_dir, false).unwrap();
    assert!(test_dir.exists());
    let mut file_path = test_dir.clone();
    file_path.push("test.txt");
    assert!(!file_path.exists());
    let content = "test_content";
    fs_extra::file::write_all(&file_path, &content).unwrap();
    assert!(file_path.exists());

    match create(&test_dir, false) {
        Ok(_) => panic!("Should be error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {
                assert!(test_dir.exists());
                assert!(file_path.exists());
                let new_content = fs_extra::file::read_to_string(file_path).unwrap();
                assert_eq!(new_content, content);
            }
            _ => panic!("Wrong error"),
        },
    }
}

#[test]
fn it_create_erase_exist_folder() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_erase_exist_folder");
    test_dir.push("sub");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    assert!(!test_dir.exists());
    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    let mut file_path = test_dir.clone();
    file_path.push("test.txt");
    assert!(!file_path.exists());
    fs_extra::file::write_all(&file_path, "test_content").unwrap();
    assert!(file_path.exists());

    create(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    assert!(!file_path.exists());
}

#[test]
fn it_create_all_exist_folder() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_all_exist_folder");
    test_dir.push("sub");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    assert!(!test_dir.exists());
    create_all(&test_dir, false).unwrap();
    assert!(test_dir.exists());
    let mut file_path = test_dir.clone();
    file_path.push("test.txt");
    assert!(!file_path.exists());
    let content = "test_content";
    fs_extra::file::write_all(&file_path, &content).unwrap();
    assert!(file_path.exists());

    create_all(&test_dir, false).unwrap();
    assert!(test_dir.exists());
    assert!(file_path.exists());
    let new_content = fs_extra::file::read_to_string(file_path).unwrap();
    assert_eq!(new_content, content);
}

#[test]
fn it_create_all_erase_exist_folder() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_create_all_erase_exist_folder");
    test_dir.push("sub");
    if test_dir.exists() {
        remove(&test_dir).unwrap();
    }
    assert!(!test_dir.exists());
    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    let mut file_path = test_dir.clone();
    file_path.push("test.txt");
    assert!(!file_path.exists());
    fs_extra::file::write_all(&file_path, "test_content").unwrap();
    assert!(file_path.exists());

    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    assert!(!file_path.exists());
}

#[test]
fn it_remove_work() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_remove_work");
    test_dir.push("sub");
    test_dir.push("second_sub");
    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    test_dir.pop();
    test_dir.pop();
    remove(&test_dir).unwrap();
    assert!(!test_dir.exists());
}

#[test]
fn it_remove_not_exist() {
    let mut test_dir = PathBuf::from(TEST_FOLDER);
    test_dir.push("it_remove_not_exist");
    test_dir.push("sub");
    assert!(!test_dir.exists());
    match remove(&test_dir) {
        Ok(_) => {
            assert!(!test_dir.exists());
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_copy_work() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_work");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let options = CopyOptions::new();
    let result = copy(&path_from, &path_to, &options).unwrap();

    assert_eq!(16, result);
    assert!(path_to.exists());
    assert!(path_from.exists());
    assert!(compare_dir(&path_from, &path_to));
}

#[test]
fn it_copy_not_folder() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_copy_not_folder");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    path_from.push("test.txt");
    fs_extra::file::write_all(&path_from, "test").unwrap();

    match copy(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFolder => {
                let wrong_path = format!(
                    "Path \"{}\" is not a directory!",
                    path_from.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
            }
            _ => {
                panic!("wrong error");
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_copy_source_not_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_copy_source_not_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    assert!(!path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    match copy(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!(
                    "Path \"{}\" does not exist or you don't have \
                     access!",
                    path_from.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
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
fn it_copy_exist_overwrite() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_exist_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert!(files_eq(file1_path, exist_path));
    assert!(path_to.exists());
    assert!(compare_dir(&path_from, &path_to));
}

#[test]
fn it_copy_exist_not_overwrite() {
    let test_name = "sub";
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_copy_exist_not_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let options = CopyOptions::new();
    match copy(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {
                let wrong_path = format!("Path \"{}\" exists", exist_path.to_str().unwrap());
                assert_eq!(wrong_path, err.to_string());
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
fn it_copy_exist_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_exist_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.skip_exist = true;
    copy(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert!(!files_eq(file1_path, &exist_path));
    assert_eq!(
        fs_extra::file::read_to_string(exist_path).unwrap(),
        exist_content
    );

    assert!(path_to.exists());
    assert!(!compare_dir(&path_from, &path_to));
}

#[test]
fn it_copy_exist_overwrite_and_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_exist_overwrite_and_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    copy(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert!(files_eq(file1_path, exist_path));
    assert!(path_to.exists());
    assert!(compare_dir(&path_from, &path_to));
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

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&d_level_4.0, true).unwrap();
    create_all(&d_level_5.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

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

    let mut options = CopyOptions::new();
    options.depth = 1;
    let result = copy(&d_level_1.0, path_to, &options).unwrap();

    assert_eq!(8, result);

    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d_level_1.1.exists());
    assert!(d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());
    assert!(!file4.1.exists());
    assert!(!file5.1.exists());
    assert!(files_eq(&file1.0, &file1.1));
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

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&d_level_4.0, true).unwrap();
    create_all(&d_level_5.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

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

    let mut options = CopyOptions::new();
    options.depth = 4;
    let result = copy(&d_level_1.0, path_to, &options).unwrap();

    assert_eq!(32, result);

    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(d_level_1.1.exists());
    assert!(d_level_2.1.exists());
    assert!(d_level_3.1.exists());
    assert!(d_level_4.1.exists());
    assert!(d_level_5.1.exists());

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());
    assert!(file4.0.exists());
    assert!(file5.0.exists());

    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
    assert!(file4.1.exists());
    assert!(!file5.1.exists());

    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
    assert!(files_eq(&file3.0, &file3.1));
    assert!(files_eq(&file4.0, &file4.1));
}
#[test]
fn it_copy_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_content_only_option");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.clone());
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());

    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());

    let mut options = CopyOptions::new();
    options.content_only = true;
    let result = copy(&d_level_1.0, path_to, &options).unwrap();

    assert_eq!(24, result);

    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());

    assert!(d_level_1.1.exists());
    assert!(d_level_2.1.exists());
    assert!(d_level_3.1.exists());

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());

    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());

    assert!(files_eq(&file1.0, &file1.1));
    assert!(files_eq(&file2.0, &file2.1));
    assert!(files_eq(&file3.0, &file3.1));
}

#[test]
fn it_copy_progress_work() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_progress_work");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();

    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(15, result);
        assert!(path_to.exists());
        assert!(compare_dir(&path_from, &path_to));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "test2.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else if process_info.file_name == "test1.txt" {
                    assert_eq!(7, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_copy_with_progress_not_folder() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_copy_with_progress_not_folder");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    path_from.push("test.txt");
    fs_extra::file::write_all(&path_from, "test").unwrap();
    let func_test = |process_info: TransitProcess| {
        match process_info.state {
            TransitState::NoAccess => {}
            _ => panic!("Error not should be!"),
        };
        TransitProcessResult::ContinueOrAbort
    };
    match copy_with_progress(&path_from, &path_to, &options, func_test) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFolder => {
                let wrong_path = format!(
                    "Path \"{}\" is not a directory!",
                    path_from.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
            }
            _ => {
                panic!("wrong error");
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_copy_with_progress_work_dif_buf_size() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_with_progress_work_dif_buf_size");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();

    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(16, result);
        assert!(path_to.exists());
        assert!(compare_dir(&path_from, &path_to));

        let mut options = CopyOptions::new();
        options.buffer_size = 2;
        options.overwrite = true;
        let (tx, rx) = mpsc::channel();
        let result = thread::spawn(move || {
            let func_test = |process_info: TransitProcess| {
                tx.send(process_info).unwrap();
                TransitProcessResult::ContinueOrAbort
            };

            let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

            assert_eq!(16, result);
            assert!(path_to.exists());
            assert!(compare_dir(&path_from, &path_to));
        })
        .join();
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(i * 2, process_info.copied_bytes);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
        }
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2 + 8, process_info.copied_bytes);
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
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
        assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
    }
    for i in 1..9 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i + 8, process_info.copied_bytes);
        assert_eq!(i, process_info.file_bytes_copied);
        assert_eq!(8, process_info.file_total_bytes);
        assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
    }

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
}
#[test]
fn it_copy_with_progress_source_not_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_copy_with_progress_source_not_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    assert!(!path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        match copy_with_progress(&path_from, &path_to, &options, func_test) {
            Err(err) => match err.kind {
                ErrorKind::NotFound => {
                    let wrong_path = format!(
                        "Path \"{}\" does not exist or you don't \
                         have access!",
                        path_from.to_str().unwrap()
                    );
                    assert_eq!(wrong_path, err.to_string());
                }
                _ => {
                    panic!(format!("wrong error {}", err.to_string()));
                }
            },
            Ok(_) => {
                panic!("should be error");
            }
        }
    })
    .join();
    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => {}
        _ => panic!("should be error"),
    }
}

#[test]
fn it_copy_with_progress_exist_overwrite() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_with_progress_exist_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();
    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();

    options.buffer_size = 1;
    options.overwrite = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(23, result);
        assert!(path_to.exists());
        assert!(compare_dir(&path_from, &path_to));
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
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_with_progress_exist_not_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();

    options.buffer_size = 1;
    let func_test = |process_info: TransitProcess| {
        match process_info.state {
            TransitState::Exists => {}
            _ => panic!("Error not should be!"),
        };
        TransitProcessResult::ContinueOrAbort
    };
    let result = copy_with_progress(&path_from, &path_to, &options, func_test);
    match result {
        Ok(_) => panic!("Should be error!"),
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {}
            _ => panic!("Wrong wrror"),
        },
    }
}

#[test]
fn it_copy_with_progress_exist_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_with_progress_exist_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();

    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();
    options.buffer_size = 1;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(0, result);
        assert!(path_to.exists());
        assert!(!compare_dir(&path_from, &path_to));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => {}
        _ => panic!("should be error"),
    }
}

#[test]
fn it_copy_with_progress_exist_overwrite_and_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_copy_with_progress_exist_overwrite_and_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();
    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();

    options.buffer_size = 1;
    options.overwrite = true;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(23, result);
        assert!(path_to.exists());
        assert!(compare_dir(&path_from, &path_to));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.recv().unwrap();
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

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&d_level_4.0, true).unwrap();
    create_all(&d_level_5.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

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

    let mut options = CopyOptions::new();
    options.depth = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&d_level_1.0, &path_to, &options, func_test).unwrap();

        assert_eq!(8, result);

        assert!(d_level_1.0.exists());
        assert!(d_level_2.0.exists());
        assert!(d_level_3.0.exists());
        assert!(d_level_4.0.exists());
        assert!(d_level_5.0.exists());

        assert!(d_level_1.1.exists());
        assert!(d_level_2.1.exists());
        assert!(!d_level_3.1.exists());
        assert!(!d_level_4.1.exists());
        assert!(!d_level_5.1.exists());

        assert!(file1.0.exists());
        assert!(file2.0.exists());
        assert!(file3.0.exists());
        assert!(file4.0.exists());
        assert!(file5.0.exists());

        assert!(file1.1.exists());
        assert!(!file2.1.exists());
        assert!(!file3.1.exists());
        assert!(!file4.1.exists());
        assert!(!file5.1.exists());
        assert!(files_eq(&file1.0, &file1.1));
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

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));
    let file4 = (d_level_4.0.join("file4.txt"), d_level_4.1.join("file4.txt"));
    let file5 = (d_level_5.0.join("file5.txt"), d_level_5.1.join("file5.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&d_level_4.0, true).unwrap();
    create_all(&d_level_5.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());
    assert!(d_level_4.0.exists());
    assert!(d_level_5.0.exists());

    assert!(!d_level_1.1.exists());
    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());
    assert!(!d_level_4.1.exists());
    assert!(!d_level_5.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();
    fs_extra::file::write_all(&file4.0, "content4").unwrap();
    fs_extra::file::write_all(&file5.0, "content5").unwrap();

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

    let mut options = CopyOptions::new();
    options.depth = 4;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&d_level_1.0, &path_to, &options, func_test).unwrap();

        assert_eq!(32, result);

        assert!(d_level_1.0.exists());
        assert!(d_level_2.0.exists());
        assert!(d_level_3.0.exists());
        assert!(d_level_4.0.exists());
        assert!(d_level_5.0.exists());

        assert!(d_level_1.1.exists());
        assert!(d_level_2.1.exists());
        assert!(d_level_3.1.exists());
        assert!(d_level_4.1.exists());
        assert!(d_level_5.1.exists());

        assert!(file1.0.exists());
        assert!(file2.0.exists());
        assert!(file3.0.exists());
        assert!(file4.0.exists());
        assert!(file5.0.exists());

        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(file4.1.exists());
        assert!(!file5.1.exists());
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
        assert!(files_eq(&file3.0, &file3.1));
        assert!(files_eq(&file4.0, &file4.1));
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
fn it_copy_with_progress_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_content_only_option");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.clone());
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());

    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());

    let mut options = CopyOptions::new();
    options.content_only = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = copy_with_progress(&d_level_1.0, &path_to, &options, func_test).unwrap();

        assert_eq!(24, result);

        assert!(d_level_1.0.exists());
        assert!(d_level_2.0.exists());
        assert!(d_level_3.0.exists());

        assert!(d_level_1.1.exists());
        assert!(d_level_2.1.exists());
        assert!(d_level_3.1.exists());

        assert!(file1.0.exists());
        assert!(file2.0.exists());
        assert!(file3.0.exists());

        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
        assert!(files_eq(&file1.0, &file1.1));
        assert!(files_eq(&file2.0, &file2.1));
        assert!(files_eq(&file3.0, &file3.1));
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
fn it_copy_inside_work_target_dir_not_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_copy_inside_work_target_dir_not_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();

    if root_dir2.exists() {
        remove(&root_dir2).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(!root_dir2.exists());
    assert!(file1.exists());
    assert!(file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    let result = copy(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(compare_dir_recursively(&root_dir1, &root_dir2));
}

#[test]
fn it_copy_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir() {
    let path_root = Path::new(TEST_FOLDER);
    let root =
        path_root.join("it_copy_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();
    fs_extra::file::write_all(&file3, "content3").unwrap();

    if root_dir2_dir1.exists() {
        remove(&root_dir2_dir1).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(!root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    let result = copy(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    assert!(compare_dir(&root_dir1, &root_dir2));
}

#[test]
fn it_copy_inside_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_copy_inside_work_target_dir_exist_with_source_dir_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();
    fs_extra::file::write_all(&file3, "content3").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content2").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    match copy(&root_dir1, &root_dir2, &options) {
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {
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
    options.overwrite = true;

    let result = copy(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(compare_dir(&root_dir1, &root_dir2));
}

// The compare_dir method assumes that the folder `path_to` must have a sub folder named the last component of the `path_from`.
// In order to compare two folders with different name but share the same structure, rewrite a new compare method to do that!
fn compare_dir_recursively<P, Q>(path_from: P, path_to: Q) -> bool
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let path_to = path_to.as_ref().to_path_buf();

    for entry in read_dir(&path_from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            match path.components().last() {
                None => panic!("Invalid folder from"),
                Some(dir_name) => {
                    let mut target_dir = path_to.to_path_buf();
                    target_dir.push(dir_name.as_os_str());
                    if !compare_dir_recursively(path.clone(), &target_dir) {
                        return false;
                    }
                }
            }
        } else {
            let mut target_file = path_to.to_path_buf();
            match path.file_name() {
                None => panic!("No file name"),
                Some(file_name) => {
                    target_file.push(file_name);
                    if !target_file.exists() {
                        return false;
                    } else if !files_eq(&path, target_file.clone()) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

#[test]
fn it_move_work() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_work");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let options = CopyOptions::new();
    let result = move_dir(&path_from, &path_to, &options).unwrap();

    assert_eq!(16, result);
    assert!(path_to.exists());
    assert!(!path_from.exists());
}

#[test]
fn it_move_not_folder() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_move_not_folder");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    path_from.push("test.txt");
    fs_extra::file::write_all(&path_from, "test").unwrap();

    match move_dir(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFolder => {
                let wrong_path = format!(
                    "Path \"{}\" is not a directory or you don't have \
                     access!",
                    path_from.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
            }
            _ => {
                panic!("wrong error");
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_move_source_not_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_move_source_not_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    assert!(!path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    match move_dir(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!("Path \"{}\" does not exist", path_from.to_str().unwrap());
                assert_eq!(wrong_path, err.to_string());
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
fn it_move_exist_overwrite() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_exist_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.overwrite = true;
    move_dir(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert!(path_to.exists());
    assert!(!path_from.exists());
}

#[test]
fn it_move_exist_not_overwrite() {
    let test_name = "sub";
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_move_exist_not_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let options = CopyOptions::new();
    match move_dir(&path_from, &path_to, &options) {
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {
                let wrong_path = format!("Path \"{}\" exists", exist_path.to_str().unwrap());
                assert_eq!(wrong_path, err.to_string());
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
fn it_move_exist_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_exist_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.skip_exist = true;
    move_dir(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert_eq!(
        fs_extra::file::read_to_string(exist_path).unwrap(),
        exist_content
    );

    assert!(path_to.exists());
}

#[test]
fn it_move_exist_overwrite_and_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_exist_overwrite_and_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);
    let same_file = "test.txt";

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push(same_file);
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut exist_path = path_to.clone();
    exist_path.push(&test_name);
    create(&exist_path, true).unwrap();
    assert!(exist_path.exists());
    exist_path.push(same_file);
    let exist_content = "exist content";
    assert_ne!(exist_content, content1);
    fs_extra::file::write_all(&exist_path, exist_content).unwrap();
    assert!(exist_path.exists());

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.skip_exist = true;
    move_dir(&path_from, &path_to, &options).unwrap();

    assert!(exist_path.exists());
    assert!(path_to.exists());
    assert!(!path_from.exists());
}

#[test]
fn it_move_inside_work_target_dir_not_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_move_inside_work_target_dir_not_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();

    if root_dir2.exists() {
        remove(&root_dir2).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(!root_dir2.exists());
    assert!(file1.exists());
    assert!(file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    let result = move_dir(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(!root_dir1.exists());
    let root_dir2_sub = root_dir2.join("sub");
    let root_dir2_file1 = root_dir2.join("file1.txt");
    let root_dir2_sub_file2 = root_dir2_sub.join("file2.txt");
    assert!(root_dir2.exists());
    assert!(root_dir2_sub.exists());
    assert!(root_dir2_file1.exists());
    assert!(root_dir2_sub_file2.exists());
}

#[test]
fn it_move_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir() {
    let path_root = Path::new(TEST_FOLDER);
    let root =
        path_root.join("it_move_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();
    fs_extra::file::write_all(&file3, "content3").unwrap();

    if root_dir2_dir1.exists() {
        remove(&root_dir2_dir1).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(!root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    let result = move_dir(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(!root_dir1.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    let root_dir2_dir1_file1 = root_dir2_dir1.join("file1.txt");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir1_sub_file2 = root_dir2_dir1_sub.join("file2.txt");
    let root_dir2_dir3_file3 = root_dir2_dir3.join("file3.txt");
    assert!(root_dir2_dir1_file1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir1_sub_file2.exists());
    assert!(root_dir2_dir3_file3.exists());
}

#[test]
fn it_move_inside_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_move_inside_work_target_dir_exist_with_source_dir_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();
    fs_extra::file::write_all(&file3, "content3").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content2").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    match copy(&root_dir1, &root_dir2, &options) {
        Err(err) => match err.kind {
            ErrorKind::AlreadyExists => {
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
    options.overwrite = true;
    let result = move_dir(&root_dir1, &root_dir2, &options).unwrap();

    assert_eq!(16, result);
    assert!(!root_dir1.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    let root_dir2_dir1_file1 = root_dir2_dir1.join("file1.txt");
    let root_dir2_dir1_sub_file2 = root_dir2_dir1_sub.join("file2.txt");
    let root_dir2_dir3_file3 = root_dir2_dir3.join("file3.txt");
    assert!(root_dir2_dir1_file1.exists());
    assert!(root_dir2_dir1_sub_file2.exists());
    assert!(root_dir2_dir3_file3.exists());
}
#[test]
fn it_move_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_content_only_option");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.clone());
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());

    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());

    let mut options = CopyOptions::new();
    options.content_only = true;
    let result = move_dir(&d_level_1.0, path_to, &options).unwrap();

    assert_eq!(24, result);

    assert!(!d_level_1.0.exists());
    assert!(!d_level_2.0.exists());
    assert!(!d_level_3.0.exists());

    assert!(d_level_1.1.exists());
    assert!(d_level_2.1.exists());
    assert!(d_level_3.1.exists());

    assert!(!file1.0.exists());
    assert!(!file2.0.exists());
    assert!(!file3.0.exists());

    assert!(file1.1.exists());
    assert!(file2.1.exists());
    assert!(file3.1.exists());
}
#[test]
fn it_move_progress_work() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_progress_work");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();

    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(15, result);
        assert!(path_to.exists());
        assert!(!path_from.exists());
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "test2.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else if process_info.file_name == "test1.txt" {
                    assert_eq!(7, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_with_progress_not_folder() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_move_with_progress_not_folder");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    path_from.push("test.txt");
    fs_extra::file::write_all(&path_from, "test").unwrap();
    let func_test = |process_info: TransitProcess| {
        match process_info.state {
            TransitState::NoAccess => {}
            _ => panic!("Error not should be!"),
        };
        TransitProcessResult::ContinueOrAbort
    };

    match move_dir_with_progress(&path_from, &path_to, &options, func_test) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFolder => {
                let wrong_path = format!(
                    "Path \"{}\" is not a directory!",
                    path_from.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
            }
            _ => {
                panic!("wrong error");
            }
        },
        Ok(_) => {
            panic!("should be error");
        }
    }
}

#[test]
fn it_move_with_progress_work_dif_buf_size() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_with_progress_work_dif_buf_size");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content1";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();

    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(16, result);
        assert!(path_to.exists());
        assert!(!path_from.exists());

        create_all(&path_from, true).unwrap();
        assert!(path_from.exists());
        let mut file1_path = path_from.clone();
        file1_path.push("test1.txt");
        let content1 = "content1";
        fs_extra::file::write_all(&file1_path, &content1).unwrap();
        assert!(file1_path.exists());

        let mut sub_dir_path = path_from.clone();
        sub_dir_path.push("sub");
        create(&sub_dir_path, true).unwrap();
        let mut file2_path = sub_dir_path.clone();
        file2_path.push("test2.txt");
        let content2 = "content2";
        fs_extra::file::write_all(&file2_path, &content2).unwrap();
        assert!(file2_path.exists());

        let mut options = CopyOptions::new();
        options.buffer_size = 2;
        options.overwrite = true;
        let (tx, rx) = mpsc::channel();
        let result = thread::spawn(move || {
            let func_test = |process_info: TransitProcess| {
                tx.send(process_info).unwrap();
                TransitProcessResult::ContinueOrAbort
            };
            let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

            assert_eq!(16, result);
            assert!(path_to.exists());
            assert!(!path_from.exists());
        })
        .join();
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(i * 2, process_info.copied_bytes);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
        }
        for i in 1..5 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2 + 8, process_info.copied_bytes);
            assert_eq!(i * 2, process_info.file_bytes_copied);
            assert_eq!(8, process_info.file_total_bytes);
            assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
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
        assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
    }
    for i in 1..9 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i + 8, process_info.copied_bytes);
        assert_eq!(i, process_info.file_bytes_copied);
        assert_eq!(8, process_info.file_total_bytes);
        assert_eq!(get_dir_size() * 2 + 16, process_info.total_bytes);
    }

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
}
#[test]
fn it_move_with_progress_source_not_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    path_from.push("it_move_with_progress_source_not_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push("sub");

    assert!(!path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let options = CopyOptions::new();
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        match move_dir_with_progress(&path_from, &path_to, &options, func_test) {
            Err(err) => match err.kind {
                ErrorKind::NotFound => {
                    let wrong_path = format!(
                        "Path \"{}\" does not exist or you don't \
                         have access!",
                        path_from.to_str().unwrap()
                    );
                    assert_eq!(wrong_path, err.to_string());
                }
                _ => {
                    panic!(format!("wrong error {}", err.to_string()));
                }
            },
            Ok(_) => {
                panic!("should be error");
            }
        }
    })
    .join();
    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => {}
        _ => panic!("should be error"),
    }
}

#[test]
fn it_move_with_progress_exist_overwrite() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_with_progress_exist_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();
    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();

    options.buffer_size = 1;
    options.overwrite = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(23, result);
        assert!(path_to.exists());
        assert!(!path_from.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.recv().unwrap();
}

#[test]
fn it_move_with_progress_exist_not_overwrite() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_with_progress_exist_not_overwrite");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();

    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test);
        match result {
            Ok(_) => panic!("Should be error!"),
            Err(err) => match err.kind {
                ErrorKind::AlreadyExists => {}
                _ => panic!("Wrong wrror"),
            },
        }
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => {
            panic!("Error not should be!");
        }
        _ => {}
    }
}

#[test]
fn it_move_with_progress_exist_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_with_progress_exist_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();

    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();
    options.buffer_size = 1;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(0, result);
        assert!(path_from.exists());
        assert!(path_to.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }

    match rx.recv() {
        Err(_) => {}
        _ => panic!("should be error"),
    }
}

#[test]
fn it_move_with_progress_exist_overwrite_and_skip_exist() {
    let mut path_from = PathBuf::from(TEST_FOLDER);
    let test_name = "sub";
    path_from.push("it_move_with_progress_exist_overwrite_and_skip_exist");
    let mut path_to = path_from.clone();
    path_to.push("out");
    path_from.push(&test_name);

    create_all(&path_from, true).unwrap();
    assert!(path_from.exists());
    create_all(&path_to, true).unwrap();
    assert!(path_to.exists());

    let mut file1_path = path_from.clone();
    file1_path.push("test1.txt");
    let content1 = "content";
    fs_extra::file::write_all(&file1_path, &content1).unwrap();
    assert!(file1_path.exists());

    let mut sub_dir_path = path_from.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2_path = sub_dir_path.clone();
    file2_path.push("test2.txt");
    let content2 = "content2";
    fs_extra::file::write_all(&file2_path, &content2).unwrap();
    assert!(file2_path.exists());

    let mut options = CopyOptions::new();
    copy(&path_from, &path_to, &options).unwrap();
    fs_extra::file::write_all(&file2_path, "another conntent").unwrap();

    options.buffer_size = 1;
    options.overwrite = true;
    options.skip_exist = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&path_from, &path_to, &options, func_test).unwrap();

        assert_eq!(23, result);
        assert!(path_to.exists());
        assert!(!path_from.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.recv().unwrap();
}

#[test]
fn it_get_folder_size() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_folder_size");
    path.push("dir");

    create_all(&path, true).unwrap();
    assert!(path.exists());

    let mut file1 = path.clone();
    file1.push("test1.txt");
    fs_extra::file::write_all(&file1, &"A".repeat(100)).unwrap();
    assert!(file1.exists());

    let mut sub_dir_path = path.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();

    let mut file2 = sub_dir_path.clone();
    file2.push("test2.txt");
    fs_extra::file::write_all(&file2, &"B".repeat(300)).unwrap();
    assert!(file2.exists());

    let symlink_file = sub_dir_path.join("symlink_file.txt");

    // Rust stdlib APIs for creating a symlinked file only exist for Unix and Windows.
    #[cfg(any(unix, windows))]
    {
        // Only passing the filename since we want this to be a relative symlink.
        create_file_symlink("test2.txt", &symlink_file).unwrap();
        assert!(symlink_file.exists());
    }

    // Total size comprises of:
    // - 100 bytes for the standard file "test1.txt"
    // - 300 bytes for the standard file "test2.txt"
    // - (On supported platforms) 1 x symlink whose whose size varies by filesystem, so is dynamically calculated.
    let mut expected_size = 100 + 300;

    if symlink_file.exists() {
        // `fs::symlink_metadata` does not follow symlinks, so this is the size of the symlink itself, not its target.
        expected_size += fs::symlink_metadata(&symlink_file).unwrap().len();
    }

    let result = get_size(&path).unwrap();

    assert_eq!(expected_size, result);
}

#[test]
fn it_get_file_size() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_file_size");

    create_all(&path, true).unwrap();
    assert!(path.exists());

    let mut file = path.clone();
    file.push("test1.txt");
    fs_extra::file::write_all(&file, "content").unwrap();
    assert!(file.exists());

    let result = get_size(&path).unwrap();

    assert_eq!(7, result);
}

#[test]
fn it_get_size_not_found() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_size_not_found");

    assert!(!path.exists());

    match get_size(&path) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => panic!("Wrong error!"),
        },
    };
}

#[test]
fn it_get_dir_content() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_dir_content");
    path.push("dir");

    create_all(&path, true).unwrap();
    assert!(path.exists());

    let mut file1 = path.clone();
    file1.push("test1.txt");
    fs_extra::file::write_all(&file1, "content1").unwrap();
    assert!(file1.exists());

    let mut sub_dir_path = path.clone();
    sub_dir_path.push("sub");
    create(&sub_dir_path, true).unwrap();
    let mut file2 = sub_dir_path.clone();
    file2.push("test2.txt");
    fs_extra::file::write_all(&file2, "content2").unwrap();
    assert!(file2.exists());

    let result = get_dir_content(&path).unwrap();

    assert_eq!(get_dir_size() * 2 + 16, result.dir_size);
    assert_eq!(2, result.files.len());
    assert_eq!(2, result.directories.len());

    let dir1 = file1.parent().unwrap().to_str().unwrap().to_string();
    let dir2 = file2.parent().unwrap().to_str().unwrap().to_string();
    let file1 = file1.to_str().unwrap().to_string();
    let file2 = file2.to_str().unwrap().to_string();

    let mut files_correct = true;
    for file in result.files {
        if file != file1 && file != file2 {
            files_correct = false;
        }
    }
    assert!(files_correct);

    let mut directories_correct = true;
    for dir in result.directories {
        if dir != dir1 && dir != dir2 {
            directories_correct = false;
        }
    }
    assert!(directories_correct);
}

#[test]
fn it_get_dir_content_many_levels() {
    let test_dir = Path::new(TEST_FOLDER).join("it_get_dir_content_many_levels");
    let d_level_1 = test_dir.join("d_level_1");
    let d_level_2 = d_level_1.join("d_level_2");
    let d_level_3 = d_level_2.join("d_level_3");
    let d_level_4 = d_level_3.join("d_level_4");
    let d_level_5 = d_level_4.join("d_level_5");

    let file1 = d_level_1.join("file1.txt");
    let file2 = d_level_2.join("file2.txt");
    let file3 = d_level_3.join("file3.txt");
    let file4 = d_level_4.join("file4.txt");
    let file5 = d_level_5.join("file5.txt");

    create_all(&d_level_1, true).unwrap();
    create_all(&d_level_2, true).unwrap();
    create_all(&d_level_3, true).unwrap();
    create_all(&d_level_4, true).unwrap();
    create_all(&d_level_5, true).unwrap();

    assert!(&d_level_1.exists());
    assert!(&d_level_2.exists());
    assert!(&d_level_3.exists());
    assert!(&d_level_4.exists());
    assert!(&d_level_5.exists());

    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();
    fs_extra::file::write_all(&file3, "content3").unwrap();
    fs_extra::file::write_all(&file4, "content4").unwrap();
    fs_extra::file::write_all(&file5, "content5").unwrap();

    let mut options = DirOptions::new();
    let result = get_dir_content2(&d_level_1, &options).unwrap();

    assert_eq!(get_dir_size() * 5 + 40, result.dir_size);
    assert_eq!(5, result.files.len());
    assert_eq!(5, result.directories.len());

    let mut directories = Vec::new();
    directories.push(file1.parent().unwrap().to_str().unwrap().to_string());
    directories.push(file2.parent().unwrap().to_str().unwrap().to_string());
    directories.push(file3.parent().unwrap().to_str().unwrap().to_string());
    directories.push(file4.parent().unwrap().to_str().unwrap().to_string());
    directories.push(file5.parent().unwrap().to_str().unwrap().to_string());

    let mut files = Vec::new();
    files.push(file1.to_str().unwrap().to_string());
    files.push(file2.to_str().unwrap().to_string());
    files.push(file3.to_str().unwrap().to_string());
    files.push(file4.to_str().unwrap().to_string());
    files.push(file5.to_str().unwrap().to_string());

    let mut files_correct = true;
    for file in result.files {
        if !files.contains(&file) {
            files_correct = false;
        }
    }
    assert!(files_correct);

    let mut directories_correct = true;
    for dir in result.directories {
        if !directories.contains(&dir) {
            directories_correct = false;
        }
    }
    assert!(directories_correct);

    // first level
    options.depth = 1;
    let result = get_dir_content2(&d_level_1, &options).unwrap();

    assert_eq!(get_dir_size() * 2 + 8, result.dir_size);
    assert_eq!(1, result.files.len());
    assert_eq!(2, result.directories.len());
    files_correct = true;
    for file in &result.files {
        if !files.contains(&file) {
            files_correct = false;
        }
    }
    assert!(files_correct);
    assert!(result.files.contains(&file1.to_str().unwrap().to_string()));

    directories_correct = true;
    for dir in &result.directories {
        if !directories.contains(&dir) {
            directories_correct = false;
        }
    }
    assert!(directories_correct);
    assert!(result
        .directories
        .contains(&file1.parent().unwrap().to_str().unwrap().to_string(),));
    assert!(result
        .directories
        .contains(&file2.parent().unwrap().to_str().unwrap().to_string(),));

    // fourth level
    options.depth = 4;
    let result = get_dir_content2(&d_level_1, &options).unwrap();

    assert_eq!(get_dir_size() * 5 + 32, result.dir_size);
    assert_eq!(4, result.files.len());
    assert_eq!(5, result.directories.len());
    files_correct = true;
    for file in &result.files {
        if !files.contains(&file) {
            files_correct = false;
        }
    }
    assert!(files_correct);
    assert!(result.files.contains(&file1.to_str().unwrap().to_string()));
    assert!(result.files.contains(&file2.to_str().unwrap().to_string()));
    assert!(result.files.contains(&file3.to_str().unwrap().to_string()));
    assert!(result.files.contains(&file4.to_str().unwrap().to_string()));

    directories_correct = true;
    for dir in &result.directories {
        if !directories.contains(&dir) {
            directories_correct = false;
        }
    }
    assert!(directories_correct);
    assert!(result
        .directories
        .contains(&file1.parent().unwrap().to_str().unwrap().to_string(),));
    assert!(result
        .directories
        .contains(&file2.parent().unwrap().to_str().unwrap().to_string(),));
    assert!(result
        .directories
        .contains(&file3.parent().unwrap().to_str().unwrap().to_string(),));
    assert!(result
        .directories
        .contains(&file4.parent().unwrap().to_str().unwrap().to_string(),));
    assert!(result
        .directories
        .contains(&file5.parent().unwrap().to_str().unwrap().to_string(),));
}

#[test]
fn it_get_dir_content_path_file() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_dir_content_path_file");

    create_all(&path, true).unwrap();
    assert!(path.exists());

    let mut file = path.clone();
    file.push("test1.txt");
    fs_extra::file::write_all(&file, "content1").unwrap();
    assert!(file.exists());

    let result = get_dir_content(&file).unwrap();

    assert_eq!(8, result.dir_size);
    assert_eq!(1, result.files.len());
    assert_eq!(0, result.directories.len());
    assert_eq!(file.to_str().unwrap().to_string(), result.files[0]);
}

#[test]
fn it_get_dir_content_not_found() {
    let mut path = PathBuf::from(TEST_FOLDER);
    path.push("it_get_dir_content_not_found");

    assert!(!path.exists());

    match get_dir_content(&path) {
        Ok(_) => panic!("Should be a error!"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => panic!("Wrong error!"),
        },
    }
}

#[test]
fn it_details_item_dir() {
    let test_dir = Path::new(TEST_FOLDER).join("it_details_item_dir");
    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Name);
    config.insert(DirEntryAttr::Ext);
    config.insert(DirEntryAttr::FullName);
    config.insert(DirEntryAttr::Path);
    config.insert(DirEntryAttr::DosPath);
    config.insert(DirEntryAttr::Size);
    config.insert(DirEntryAttr::IsDir);
    config.insert(DirEntryAttr::IsFile);
    config.insert(DirEntryAttr::Modified);
    config.insert(DirEntryAttr::Accessed);
    let item = get_details_entry(test_dir, &config).unwrap();
    assert_eq!(10, item.len());

    let mut fields = 0;
    if let Some(name) = item.get(&DirEntryAttr::Name) {
        if let &DirEntryValue::String(ref name) = name {
            assert_eq!("it_details_item_dir", name);
            fields += 1;
        }
    }
    if let Some(ext) = item.get(&DirEntryAttr::Ext) {
        if let &DirEntryValue::String(ref ext) = ext {
            assert_eq!("", ext);
            fields += 1;
        }
    }
    if let Some(fname) = item.get(&DirEntryAttr::FullName) {
        if let &DirEntryValue::String(ref fname) = fname {
            assert_eq!("it_details_item_dir", fname);
            fields += 1;
        }
    }
    if let Some(path) = item.get(&DirEntryAttr::Path) {
        if let &DirEntryValue::String(ref path) = path {
            if !path.is_empty() {
                fields += 1;
            }
        }
    }
    if let Some(path) = item.get(&DirEntryAttr::DosPath) {
        if let &DirEntryValue::String(ref path) = path {
            if !path.is_empty() {
                fields += 1;
            }
        }
    }
    if let Some(size) = item.get(&DirEntryAttr::Size) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(0, size);
            fields += 1;
        }
    }
    if let Some(is_dir) = item.get(&DirEntryAttr::IsDir) {
        if let &DirEntryValue::Boolean(is_dir) = is_dir {
            assert_eq!(true, is_dir);
            fields += 1;
        }
    }

    if let Some(is_file) = item.get(&DirEntryAttr::IsFile) {
        if let &DirEntryValue::Boolean(is_file) = is_file {
            assert_eq!(false, is_file);
            fields += 1;
        }
    }

    if let Some(modified) = item.get(&DirEntryAttr::Modified) {
        if let &DirEntryValue::SystemTime(modified) = modified {
            if modified.elapsed().unwrap().as_secs() == 0 {
                fields += 1;
            }
        }
    }
    if let Some(accessed) = item.get(&DirEntryAttr::Accessed) {
        if let &DirEntryValue::SystemTime(accessed) = accessed {
            if accessed.elapsed().unwrap().as_secs() == 0 {
                fields += 1;
            }
        }
    }

    assert_eq!(10, fields);
}

#[test]
fn it_details_file_item() {
    let test_dir = Path::new(TEST_FOLDER).join("it_details_file_item");
    create_all(&test_dir, true).unwrap();
    let file = test_dir.join("file.txt");
    fs_extra::file::write_all(&file, "content").unwrap();
    assert!(file.exists());
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Name);
    config.insert(DirEntryAttr::Ext);
    config.insert(DirEntryAttr::FullName);
    config.insert(DirEntryAttr::Path);
    config.insert(DirEntryAttr::DosPath);
    config.insert(DirEntryAttr::Size);
    config.insert(DirEntryAttr::FileSize);
    config.insert(DirEntryAttr::IsDir);
    config.insert(DirEntryAttr::IsFile);
    config.insert(DirEntryAttr::Modified);
    config.insert(DirEntryAttr::Accessed);
    let item = get_details_entry(file, &config).unwrap();
    assert_eq!(11, item.len());

    let mut fields = 0;
    if let Some(name) = item.get(&DirEntryAttr::Name) {
        if let &DirEntryValue::String(ref name) = name {
            assert_eq!("file", name);
            fields += 1;
        }
    }
    if let Some(ext) = item.get(&DirEntryAttr::Ext) {
        if let &DirEntryValue::String(ref ext) = ext {
            assert_eq!("txt", ext);
            fields += 1;
        }
    }
    if let Some(fname) = item.get(&DirEntryAttr::FullName) {
        if let &DirEntryValue::String(ref fname) = fname {
            assert_eq!("file.txt", fname);
            fields += 1;
        }
    }
    if let Some(path) = item.get(&DirEntryAttr::Path) {
        if let &DirEntryValue::String(ref path) = path {
            if !path.is_empty() {
                fields += 1;
            }
        }
    }
    if let Some(path) = item.get(&DirEntryAttr::DosPath) {
        if let &DirEntryValue::String(ref path) = path {
            if !path.is_empty() {
                fields += 1;
            }
        }
    }
    if let Some(size) = item.get(&DirEntryAttr::Size) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(7, size);
            fields += 1;
        }
    }
    if let Some(size) = item.get(&DirEntryAttr::FileSize) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(7, size);
            fields += 1;
        }
    }
    if let Some(is_dir) = item.get(&DirEntryAttr::IsDir) {
        if let &DirEntryValue::Boolean(is_dir) = is_dir {
            assert_eq!(false, is_dir);
            fields += 1;
        }
    }

    if let Some(is_file) = item.get(&DirEntryAttr::IsFile) {
        if let &DirEntryValue::Boolean(is_file) = is_file {
            assert_eq!(true, is_file);
            fields += 1;
        }
    }

    if let Some(modified) = item.get(&DirEntryAttr::Modified) {
        if let &DirEntryValue::SystemTime(modified) = modified {
            if modified.elapsed().unwrap().as_secs() == 0 {
                fields += 1;
            }
        }
    }
    if let Some(accessed) = item.get(&DirEntryAttr::Accessed) {
        if let &DirEntryValue::SystemTime(accessed) = accessed {
            if accessed.elapsed().unwrap().as_secs() == 0 {
                fields += 1;
            }
        }
    }

    assert_eq!(11, fields);
}

#[test]
fn it_details_item_dir_short() {
    let test_dir = Path::new(TEST_FOLDER).join("it_details_item_dir_short");
    create_all(&test_dir, true).unwrap();
    assert!(test_dir.exists());
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Name);
    config.insert(DirEntryAttr::Size);
    let item = get_details_entry(test_dir, &config).unwrap();
    assert_eq!(2, item.len());

    if let Some(name) = item.get(&DirEntryAttr::Name) {
        if let &DirEntryValue::String(ref name) = name {
            assert_eq!("it_details_item_dir_short", name);
        }
    }
    if let Some(size) = item.get(&DirEntryAttr::Size) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(0, size);
        }
    }
}

#[test]
fn it_details_item_file_short() {
    let test_dir = Path::new(TEST_FOLDER).join("it_details_item_short");
    create_all(&test_dir, true).unwrap();
    let file = test_dir.join("file.txt");
    fs_extra::file::write_all(&file, "content").unwrap();
    assert!(file.exists());
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Name);
    config.insert(DirEntryAttr::Size);
    let item = get_details_entry(file, &config).unwrap();
    assert_eq!(2, item.len());

    if let Some(name) = item.get(&DirEntryAttr::Name) {
        if let &DirEntryValue::String(ref name) = name {
            assert_eq!("file", name);
        }
    }
    if let Some(size) = item.get(&DirEntryAttr::Size) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(7, size);
        }
    }
}

#[test]
fn it_ls() {
    let test_dir = Path::new(TEST_FOLDER).join("it_ls");
    create_all(&test_dir, true).unwrap();
    let file1 = test_dir.join("file1.txt");
    let file2 = test_dir.join("file2.txt");
    fs_extra::file::write_all(&file1, "content").unwrap();
    fs_extra::file::write_all(&file2, "content").unwrap();
    assert!(file1.exists());
    assert!(file2.exists());
    let mut config = HashSet::new();
    config.insert(DirEntryAttr::Name);
    config.insert(DirEntryAttr::Size);
    config.insert(DirEntryAttr::IsDir);
    config.insert(DirEntryAttr::BaseInfo);
    let ls_result = ls(&test_dir, &config).unwrap();
    assert_eq!(2, ls_result.items.len());
    assert_eq!(3, ls_result.base.len());

    if let Some(name) = ls_result.base.get(&DirEntryAttr::Name) {
        if let &DirEntryValue::String(ref name) = name {
            assert_eq!("it_ls", name);
        }
    }
    if let Some(size) = ls_result.base.get(&DirEntryAttr::Size) {
        if let &DirEntryValue::U64(size) = size {
            assert_eq!(14, size);
        }
    }
    if let Some(is_dir) = ls_result.base.get(&DirEntryAttr::IsDir) {
        if let &DirEntryValue::Boolean(is_dir) = is_dir {
            assert_eq!(true, is_dir);
        }
    }
    for item in ls_result.items {
        if let Some(name) = item.get(&DirEntryAttr::Name) {
            if let &DirEntryValue::String(ref name) = name {
                assert_eq!(String::from("file"), name[..4]);
            }
        }
        if let Some(size) = item.get(&DirEntryAttr::Size) {
            if let &DirEntryValue::U64(size) = size {
                assert_eq!(7, size);
            }
        }
        if let Some(is_dir) = item.get(&DirEntryAttr::IsDir) {
            if let &DirEntryValue::Boolean(is_dir) = is_dir {
                assert_eq!(false, is_dir);
            }
        }
    }
}

#[test]
fn it_copy_with_progress_exist_user_decide_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_copy_with_progress_exist_user_decide_overwrite");
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::Overwrite;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = copy_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(2, count_exist_files);

        assert_eq!(16, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
        assert!(compare_dir(&dir.0, &out));
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::OverwriteAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = copy_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(16, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
        assert!(compare_dir(&dir.0, &out));
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::Skip;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = copy_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(2, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
        assert!(!compare_dir(&dir.0, &out));
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::SkipAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = copy_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
        assert!(!compare_dir(&dir.0, &out));
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        if count_exist_files == 3 || count_exist_files == 6 {
                            result = TransitProcessResult::Skip;
                        } else {
                            result = TransitProcessResult::Retry;
                        }
                        count_exist_files += 1;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = copy_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(7, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
        assert!(!compare_dir(&dir.0, &out));
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_copy_with_progress_inside_work_target_dir_not_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_copy_with_progress_inside_work_target_dir_not_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();

    if root_dir2.exists() {
        remove(&root_dir2).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(!root_dir2.exists());
    assert!(file1.exists());
    assert!(file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = copy_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(15, result);
        assert!(root_dir1.exists());
        assert!(root_dir1_sub.exists());
        assert!(root_dir2.exists());
        assert!(compare_dir_recursively(&root_dir1, &root_dir2));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(7, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_copy_with_progress_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join(
        "it_copy_with_progress_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir",
    );
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();

    if root_dir2_dir1.exists() {
        remove(&root_dir2_dir1).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(!root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = copy_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(17, result);
        assert!(root_dir1.exists());
        assert!(root_dir1_sub.exists());
        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir3.exists());
        assert!(compare_dir(&root_dir1, &root_dir2));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_copy_with_progress_inside_no_overwrite_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join(
        "it_copy_with_progress_inside_no_overwrite_work_target_dir_exist_with_source_dir_exist",
    );
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content22").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::Skip
        };
        let result = copy_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(0, result);
        assert!(root_dir1.exists());
        assert!(root_dir1_sub.exists());
        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir1_sub.exists());
        assert!(root_dir2_dir3.exists());
        assert!(!files_eq(file1, old_file1));
        assert!(!files_eq(file2, old_file2));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_copy_with_progress_inside_overwrite_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root
        .join("it_copy_with_progress_inside_overwrite_work_target_dir_exist_with_source_dir_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content22").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = copy_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(17, result);
        assert!(root_dir1.exists());
        assert!(root_dir1_sub.exists());
        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir1_sub.exists());
        assert!(root_dir2_dir3.exists());
        assert!(compare_dir(&root_dir1, &root_dir2));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_with_progress_exist_user_decide_overwrite() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_exist_user_decide_overwrite");
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::Overwrite;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = move_dir_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(2, count_exist_files);

        assert_eq!(16, result);
        assert!(!dir.0.exists());
        assert!(dir.1.exists());
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::OverwriteAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = move_dir_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(16, result);
        assert!(!dir.0.exists());
        assert!(dir.1.exists());
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::Skip;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = move_dir_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(2, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        count_exist_files += 1;
                        result = TransitProcessResult::SkipAll;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = move_dir_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(1, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
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
    let out = test_dir.join("out");
    let dir = (test_dir.join("dir"), out.join("dir"));
    let file1 = (dir.0.join("file1.txt"), dir.1.join("file1.txt"));
    let file2 = (dir.0.join("file2.txt"), dir.1.join("file2.txt"));

    create_all(&dir.0, true).unwrap();
    create_all(&dir.1, true).unwrap();

    assert!(&dir.0.exists());
    assert!(&dir.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();

    fs_extra::file::write_all(&file1.1, "old content7").unwrap();
    fs_extra::file::write_all(&file2.1, "old content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file1.1.exists());
    assert!(file2.1.exists());

    let mut options = CopyOptions::new();
    assert!(!compare_dir(&dir.0, &out));
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let mut count_exist_files = 0;
        let result: u64;
        {
            let func_test = |process_info: TransitProcess| {
                let result: TransitProcessResult;
                match process_info.state {
                    TransitState::Exists => {
                        if count_exist_files == 3 || count_exist_files == 6 {
                            result = TransitProcessResult::Skip;
                        } else {
                            result = TransitProcessResult::Retry;
                        }
                        count_exist_files += 1;
                        tx.send(process_info).unwrap();
                    }
                    _ => result = TransitProcessResult::Abort,
                };
                result
            };
            result = move_dir_with_progress(&dir.0, &out, &options, func_test).unwrap();
        }
        assert_eq!(7, count_exist_files);

        assert_eq!(0, result);
        assert!(dir.0.exists());
        assert!(dir.1.exists());
    })
    .join();

    match result {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    rx.try_recv().unwrap();
}

#[test]
fn it_move_dir_with_progress_inside_work_target_dir_not_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join("it_move_dir_with_progress_inside_work_target_dir_not_exist");
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content").unwrap();
    fs_extra::file::write_all(&file2, "content2").unwrap();

    if root_dir2.exists() {
        remove(&root_dir2).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(!root_dir2.exists());
    assert!(file1.exists());
    assert!(file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = move_dir_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(15, result);
        assert!(!root_dir1.exists());
        let root_dir2_sub = root_dir2.join("sub");
        let root_dir2_file1 = root_dir2.join("file1.txt");
        let root_dir2_sub_file2 = root_dir2_sub.join("file2.txt");
        assert!(root_dir2.exists());
        assert!(root_dir2_sub.exists());
        assert!(root_dir2_file1.exists());
        assert!(root_dir2_sub_file2.exists());
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(7, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 15, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_dir_with_progress_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join(
        "it_move_dir_with_progress_inside_work_target_dir_exist_with_no_source_dir_named_sub_dir",
    );
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();

    if root_dir2_dir1.exists() {
        remove(&root_dir2_dir1).unwrap();
    }

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(!root_dir2_dir1.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = move_dir_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(17, result);
        assert!(!root_dir1.exists());
        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir3.exists());
        let root_dir2_dir1_file1 = root_dir2_dir1.join("file1.txt");
        let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
        let root_dir2_dir1_sub_file2 = root_dir2_dir1_sub.join("file2.txt");
        let root_dir2_dir3_file3 = root_dir2_dir3.join("file3.txt");
        assert!(root_dir2_dir1_file1.exists());
        assert!(root_dir2_dir1_sub.exists());
        assert!(root_dir2_dir1_sub_file2.exists());
        assert!(root_dir2_dir3_file3.exists());
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_dir_with_progress_inside_no_overwrite_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join(
        "it_move_dir_with_progress_inside_no_overwrite_work_target_dir_exist_with_source_dir_exist",
    );
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content22").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::Skip
        };
        let result = move_dir_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(0, result);

        assert!(root_dir1.exists());
        assert!(file1.exists());
        assert!(root_dir1_sub.exists());
        assert!(file2.exists());

        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir1_sub.exists());
        assert!(root_dir2_dir3.exists());
        let root_dir2_dir1_file1 = root_dir2_dir1.join("file1.txt");
        let root_dir2_dir1_sub_file2 = root_dir2_dir1_sub.join("file2.txt");
        let root_dir2_dir3_file3 = root_dir2_dir3.join("file3.txt");
        assert!(root_dir2_dir1_file1.exists());
        assert!(root_dir2_dir1_sub_file2.exists());
        assert!(root_dir2_dir3_file3.exists());
        assert!(!files_eq(file1, old_file1));
        assert!(!files_eq(file2, old_file2));
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_dir_with_progress_inside_overwrite_work_target_dir_exist_with_source_dir_exist() {
    let path_root = Path::new(TEST_FOLDER);
    let root = path_root.join(
        "it_move_dir_with_progress_inside_overwrite_work_target_dir_exist_with_source_dir_exist",
    );
    let root_dir1 = root.join("dir1");
    let root_dir1_sub = root_dir1.join("sub");
    let root_dir2 = root.join("dir2");
    let root_dir2_dir1 = root_dir2.join("dir1");
    let root_dir2_dir1_sub = root_dir2_dir1.join("sub");
    let root_dir2_dir3 = root_dir2.join("dir3");
    let file1 = root_dir1.join("file1.txt");
    let file2 = root_dir1_sub.join("file2.txt");
    let file3 = root_dir2_dir3.join("file3.txt");
    let old_file1 = root_dir2_dir1.join("file1.txt");
    let old_file2 = root_dir2_dir1_sub.join("file2.txt");

    create_all(&root_dir1_sub, true).unwrap();
    create_all(&root_dir2_dir3, true).unwrap();
    create_all(&root_dir2_dir1, true).unwrap();
    create_all(&root_dir2_dir1_sub, true).unwrap();
    fs_extra::file::write_all(&file1, "content1").unwrap();
    fs_extra::file::write_all(&file2, "content22").unwrap();
    fs_extra::file::write_all(&file3, "content333").unwrap();
    fs_extra::file::write_all(&old_file1, "old_content1").unwrap();
    fs_extra::file::write_all(&old_file2, "old_content22").unwrap();

    assert!(root_dir1.exists());
    assert!(root_dir1_sub.exists());
    assert!(root_dir2.exists());
    assert!(root_dir2_dir1.exists());
    assert!(root_dir2_dir1_sub.exists());
    assert!(root_dir2_dir3.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
    assert!(old_file1.exists());
    assert!(old_file2.exists());

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };
        let result = move_dir_with_progress(&root_dir1, &root_dir2, &options, func_test).unwrap();

        assert_eq!(17, result);

        assert!(!root_dir1.exists());

        assert!(root_dir2.exists());
        assert!(root_dir2_dir1.exists());
        assert!(root_dir2_dir1_sub.exists());
        assert!(root_dir2_dir3.exists());
        let root_dir2_dir1_file1 = root_dir2_dir1.join("file1.txt");
        let root_dir2_dir1_sub_file2 = root_dir2_dir1_sub.join("file2.txt");
        let root_dir2_dir3_file3 = root_dir2_dir3.join("file3.txt");
        assert!(root_dir2_dir1_file1.exists());
        assert!(root_dir2_dir1_sub_file2.exists());
        assert!(root_dir2_dir3_file3.exists());
    })
    .join();

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                if process_info.file_name == "file2.txt" {
                    assert_eq!(9, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else if process_info.file_name == "file1.txt" {
                    assert_eq!(8, process_info.file_total_bytes);
                    assert_eq!(get_dir_size() * 2 + 17, process_info.total_bytes);
                } else {
                    panic!("Unknow file name!");
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
fn it_move_with_progress_content_only_option() {
    let test_dir = Path::new(TEST_FOLDER).join("it_move_with_progress_content_only_option");
    let path_to = test_dir.join("out");
    let d_level_1 = (test_dir.join("d_level_1"), path_to.clone());
    let d_level_2 = (d_level_1.0.join("d_level_2"), d_level_1.1.join("d_level_2"));
    let d_level_3 = (d_level_2.0.join("d_level_3"), d_level_2.1.join("d_level_3"));

    let file1 = (d_level_1.0.join("file1.txt"), d_level_1.1.join("file1.txt"));
    let file2 = (d_level_2.0.join("file2.txt"), d_level_2.1.join("file2.txt"));
    let file3 = (d_level_3.0.join("file3.txt"), d_level_3.1.join("file3.txt"));

    create_all(&d_level_1.0, true).unwrap();
    create_all(&d_level_2.0, true).unwrap();
    create_all(&d_level_3.0, true).unwrap();
    create_all(&path_to, true).unwrap();

    assert!(path_to.exists());
    assert!(d_level_1.0.exists());
    assert!(d_level_2.0.exists());
    assert!(d_level_3.0.exists());

    assert!(!d_level_2.1.exists());
    assert!(!d_level_3.1.exists());

    fs_extra::file::write_all(&file1.0, "content1").unwrap();
    fs_extra::file::write_all(&file2.0, "content2").unwrap();
    fs_extra::file::write_all(&file3.0, "content3").unwrap();

    assert!(file1.0.exists());
    assert!(file2.0.exists());
    assert!(file3.0.exists());

    assert!(!file1.1.exists());
    assert!(!file2.1.exists());
    assert!(!file3.1.exists());

    let mut options = CopyOptions::new();
    options.content_only = true;
    let (tx, rx) = mpsc::channel();
    let result = thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            TransitProcessResult::ContinueOrAbort
        };

        let result = move_dir_with_progress(&d_level_1.0, &path_to, &options, func_test).unwrap();

        assert_eq!(24, result);

        assert!(!d_level_1.0.exists());
        assert!(!d_level_2.0.exists());
        assert!(!d_level_3.0.exists());

        assert!(d_level_1.1.exists());
        assert!(d_level_2.1.exists());
        assert!(d_level_3.1.exists());

        assert!(!file1.0.exists());
        assert!(!file2.0.exists());
        assert!(!file3.0.exists());

        assert!(file1.1.exists());
        assert!(file2.1.exists());
        assert!(file3.1.exists());
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
