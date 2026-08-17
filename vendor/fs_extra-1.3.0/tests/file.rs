// use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

extern crate fs_extra;
use fs_extra::error::*;
use fs_extra::file::*;

const TEST_FOLDER: &'static str = "./tests/temp/file";

fn files_eq<P, Q>(file1: P, file2: Q) -> Result<bool>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let content1 = read_to_string(file1)?;
    let content2 = read_to_string(file2)?;
    Ok(content1 == content2)
}

#[test]
fn it_read_and_write_work() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_read_and_write_work");
    test_file.push("test.txt");
    fs_extra::dir::create_all(test_file.parent().unwrap(), true).unwrap();
    let content1 = "test_1";
    let content2 = "test_2";
    write_all(&test_file, &content1).unwrap();
    assert!(test_file.exists());
    let read1 = read_to_string(&test_file).unwrap();
    assert_eq!(content1, read1);
    write_all(&test_file, &content2).unwrap();
    let read2 = read_to_string(&test_file).unwrap();
    assert_eq!(content2, read2);
}

#[test]
fn it_read_not_exist_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_read_not_exist_file");
    test_file.push("test.txt");
    fs_extra::dir::create_all(test_file.parent().unwrap(), true).unwrap();
    assert!(!test_file.exists());
    match read_to_string(&test_file) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {}
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_read_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_read_not_file");
    fs_extra::dir::create_all(&test_file, true).unwrap();
    match read_to_string(&test_file) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {}
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_write_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_write_not_file");
    test_file.push("test.txt");
    fs_extra::dir::create_all(test_file.parent().unwrap(), true).unwrap();
    assert!(!test_file.exists());
    test_file.pop();
    match write_all(test_file, "content") {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {}
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_remove_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_remove_file");
    test_file.push("test.txt");
    fs_extra::dir::create_all(test_file.parent().unwrap(), true).unwrap();
    write_all(&test_file, "test").unwrap();
    assert!(test_file.exists());
    remove(&test_file).unwrap();
    assert!(!test_file.exists());
}

#[test]
fn it_copy_work() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(test_file_out.exists());
    assert_eq!(test_file.file_name(), test_file_out.file_name());
    assert!(files_eq(test_file, test_file_out).unwrap());
}

#[test]
fn it_copy_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    test_file.pop();
    let options = CopyOptions::new();

    match copy(&test_file, &test_file_out, &options) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {
                let wrong_path = format!("Path \"{}\" is not a file!", test_file.to_str().unwrap());
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
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_source_not_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test1.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    assert!(!test_file.exists());
    let options = CopyOptions::new();
    match copy(&test_file, test_file_out, &options) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!(
                    "Path \"{}\" does not exist or you don't have \
                     access!",
                    test_file.to_str().unwrap()
                );
                assert_eq!(wrong_path, err.to_string());
                ()
            }
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_copy_exist_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_exist_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    write_all(&test_file, "test_data2").unwrap();
    match copy(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_copy_exist_not_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_exist_not_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.overwrite = false;
    write_all(&test_file, "test_data2").unwrap();
    match copy(&test_file, &test_file_out, &options) {
        Ok(_) => panic!("should be error"),
        Err(err) => {
            let file_path = format!("Path \"{}\" exists", test_file_out.to_str().unwrap());
            assert_eq!(file_path, err.to_string());
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
    }
}

#[test]
fn it_copy_exist_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_exist_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    match copy(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
        Err(_) => panic!("should be error"),
    }
}

#[test]
fn it_copy_exist_overwrite_and_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_exist_overwrite_and_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    match copy(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_copy_with_progress_work() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
        };
        copy_with_progress(&test_file, &test_file_out, &options, func_test).unwrap();
        assert!(test_file.exists());
        assert!(test_file_out.exists());
        assert_eq!(test_file.file_name(), test_file_out.file_name());
        assert!(files_eq(test_file, test_file_out).unwrap());
    });
    for i in 1..10 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i, process_info.copied_bytes);
        assert_eq!(9, process_info.total_bytes);
    }
}

#[test]
fn it_copy_progress_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_progress_not_file");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    test_file.pop();
    let options = CopyOptions::new();
    let func_test = |process_info: TransitProcess| println!("{}", process_info.total_bytes);

    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {
                let wrong_path = format!("Path \"{}\" is not a file!", test_file.to_str().unwrap());
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
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_work_dif_buf_size");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data_").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
        };
        copy_with_progress(&test_file, &test_file_out, &options, func_test).unwrap();
        assert!(test_file.exists());
        assert!(test_file_out.exists());
        assert_eq!(test_file.file_name(), test_file_out.file_name());
        assert!(files_eq(&test_file, &test_file_out).unwrap());

        let mut options = CopyOptions::new();
        options.buffer_size = 2;
        options.overwrite = true;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let func_test = |process_info: TransitProcess| {
                tx.send(process_info).unwrap();
            };
            copy_with_progress(&test_file, &test_file_out, &options, func_test).unwrap();
        });
        for i in 1..6 {
            let process_info: TransitProcess = rx.recv().unwrap();
            assert_eq!(i * 2, process_info.copied_bytes);
            assert_eq!(10, process_info.total_bytes);
        }
    });
    for i in 1..11 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i, process_info.copied_bytes);
        assert_eq!(10, process_info.total_bytes);
    }
}

#[test]
fn it_copy_with_progress_source_not_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_source_not_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test1.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    assert!(!test_file.exists());
    let options = CopyOptions::new();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!(
                    "Path \"{}\" does not exist or you don't have \
                     access!",
                    test_file.to_str().unwrap()
                );

                assert_eq!(wrong_path, err.to_string());
                ()
            }
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_copy_with_progress_exist_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_exist_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_copy_with_progress_exist_not_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_exist_not_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.overwrite = false;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => panic!("should be error"),
        Err(err) => {
            let file_path = format!("Path \"{}\" exists", test_file_out.to_str().unwrap());

            assert_eq!(file_path, err.to_string());
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
    }
}

#[test]
fn it_copy_with_progress_exist_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_exist_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
        Err(_) => panic!("should be error"),
    }
}

#[test]
fn it_copy_with_progress_exist_overwrite_and_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_copy_with_progress_exist_overwrite_and_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match copy_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_move_work() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let options = CopyOptions::new();
    let old_name = test_file.file_name();
    let old_content = read_to_string(&test_file).unwrap();
    move_file(&test_file, &test_file_out, &options).unwrap();
    assert!(!test_file.exists());
    assert!(test_file_out.exists());
    assert_eq!(old_name, test_file_out.file_name());
    let new_content = read_to_string(&test_file_out).unwrap();
    assert_eq!(old_content, new_content);
}

#[test]
fn it_move_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    test_file.pop();
    let options = CopyOptions::new();

    match move_file(&test_file, &test_file_out, &options) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {
                let wrong_path = format!("Path \"{}\" is not a file!", test_file.to_str().unwrap());
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
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_source_not_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test1.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    assert!(!test_file.exists());
    let options = CopyOptions::new();
    match move_file(&test_file, &test_file_out, &options) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!(
                    "Path \"{}\" does not exist or you don't have \
                     access!",
                    test_file.to_str().unwrap()
                );

                assert_eq!(wrong_path, err.to_string());
                ()
            }
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_move_exist_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_exist_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    write_all(&test_file, "test_data2").unwrap();
    match move_file(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(!test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_move_exist_not_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_exist_not_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.overwrite = false;
    write_all(&test_file, "test_data2").unwrap();
    match move_file(&test_file, &test_file_out, &options) {
        Ok(_) => panic!("should be error"),
        Err(err) => {
            let file_path = format!("Path \"{}\" exists", test_file_out.to_str().unwrap());

            assert_eq!(file_path, err.to_string());
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
    }
}

#[test]
fn it_move_exist_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_exist_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    match move_file(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
        Err(_) => panic!("should be error"),
    }
}

#[test]
fn it_move_exist_overwrite_and_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_exist_overwrite_and_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    match move_file(&test_file, &test_file_out, &options) {
        Ok(_) => {
            assert!(!test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_move_with_progress_work() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_work");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let old_name = test_file.file_name();
        let old_content = read_to_string(&test_file).unwrap();
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
        };
        move_file_with_progress(&test_file, &test_file_out, &options, func_test).unwrap();
        assert!(!test_file.exists());
        assert!(test_file_out.exists());
        assert_eq!(old_name, test_file_out.file_name());
        let new_content = read_to_string(&test_file_out).unwrap();
        assert_eq!(old_content, new_content);
    });
    for i in 1..10 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i, process_info.copied_bytes);
        assert_eq!(9, process_info.total_bytes);
    }
}

#[test]
fn it_move_progress_not_file() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_progress_not_file");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    test_file.pop();
    let options = CopyOptions::new();
    let func_test = |process_info: TransitProcess| println!("{}", process_info.total_bytes);

    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Err(err) => match err.kind {
            ErrorKind::InvalidFile => {
                let wrong_path = format!("Path \"{}\" is not a file!", test_file.to_str().unwrap());
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
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_work_dif_buf_size");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data_").unwrap();
    assert!(test_file.exists());
    assert!(!test_file_out.exists());
    let mut options = CopyOptions::new();
    options.buffer_size = 2;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let old_name = test_file.file_name();
        let old_content = read_to_string(&test_file).unwrap();
        let func_test = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
        };
        move_file_with_progress(&test_file, &test_file_out, &options, func_test).unwrap();
        assert!(!test_file.exists());
        assert!(test_file_out.exists());
        assert_eq!(old_name, test_file_out.file_name());
        let new_content = read_to_string(&test_file_out).unwrap();
        assert_eq!(old_content, new_content);
    });
    for i in 1..6 {
        let process_info: TransitProcess = rx.recv().unwrap();
        assert_eq!(i * 2, process_info.copied_bytes);
        assert_eq!(10, process_info.total_bytes);
    }
}

#[test]
fn it_move_with_progress_source_not_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_source_not_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test1.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    assert!(!test_file.exists());
    let options = CopyOptions::new();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => panic!("should be error"),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                let wrong_path = format!(
                    "Path \"{}\" does not exist or you don't have \
                     access!",
                    test_file.to_str().unwrap()
                );

                assert_eq!(wrong_path, err.to_string());
                ()
            }
            _ => panic!("wrong error"),
        },
    }
}

#[test]
fn it_move_with_progress_exist_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_exist_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(!test_file.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}

#[test]
fn it_move_with_progress_exist_not_overwrite() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_exist_not_overwrite");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.overwrite = false;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => panic!("should be error"),
        Err(err) => {
            let file_path = format!("Path \"{}\" exists", test_file_out.to_str().unwrap());

            assert_eq!(file_path, err.to_string());
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
    }
}

#[test]
fn it_move_with_progress_exist_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_exist_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(!files_eq(test_file, test_file_out).unwrap());
            ()
        }
        Err(_) => panic!("should be error"),
    }
}

#[test]
fn it_move_with_progress_exist_overwrite_and_skip_exist() {
    let mut test_file = PathBuf::from(TEST_FOLDER);
    test_file.push("it_move_with_progress_exist_overwrite_and_skip_exist");
    let mut test_file_out = test_file.clone();
    test_file.push("test.txt");
    test_file_out.push("out");
    test_file_out.push("test.txt");
    fs_extra::dir::create_all(&test_file.parent().unwrap(), true).unwrap();
    fs_extra::dir::create_all(&test_file_out.parent().unwrap(), true).unwrap();

    write_all(&test_file, "test_data").unwrap();
    let mut options = CopyOptions::new();
    copy(&test_file, &test_file_out, &options).unwrap();
    assert!(test_file.exists());
    assert!(files_eq(&test_file, &test_file_out).unwrap());
    options.overwrite = true;
    options.skip_exist = true;
    write_all(&test_file, "test_data2").unwrap();
    let func_test = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
    };
    match move_file_with_progress(&test_file, &test_file_out, &options, func_test) {
        Ok(_) => {
            assert!(!test_file.exists());
            assert!(test_file_out.exists());
            assert_eq!(read_to_string(test_file_out).unwrap(), "test_data2");
            ()
        }
        Err(err) => panic!(err.to_string()),
    }
}
