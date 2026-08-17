# fs_extra

A Rust library that provides additional functionality not present in [`std::fs`](https://doc.rust-lang.org/std/fs/index.html).

[![Build Status](https://travis-ci.org/webdesus/fs_extra.svg)](https://travis-ci.org/webdesus/fs_extra)
[![Crates.io Status](https://img.shields.io/crates/v/fs_extra.svg)](https://crates.io/crates/fs_extra)
[![Docs](https://docs.rs/fs_extra/badge.svg)](https://docs.rs/fs_extra)

[Documentation](https://docs.rs/fs_extra)

[Migrations to 1.x.x version](https://github.com/webdesus/fs_extra/wiki/Migrations-to-1.x.x-version)


## Key features:

* Copy files (optionally with information about the progress).

* Copy directories recursively (optionally with information about the progress).

* Move files (optionally with information about the progress).

* Move directories recursively (optionally with information about the progress).

* A single method for create and write `String` content in file.

* A single method for open and read `String` content from file.

* Get folder size

* Get collection of directory entries 

## Functions:

| Function | Description |
| ------------- | ------------- |
| [fs_extra::copy_items](https://docs.rs/fs_extra/*/fs_extra/fn.copy_items.html)  | Recursively copies files and directories from one location to another |
| [fs_extra::copy_items_with_progress](https://docs.rs/fs_extra/*/fs_extra/fn.copy_items_with_progress.html)  | Recursively copies files and directories from one location to another with information about progress |
| [fs_extra::move_items](https://docs.rs/fs_extra/*/fs_extra/fn.move_items.html)  | Recursively moves files and directories from one location to another |
| [fs_extra::move_items_with_progress](https://docs.rs/fs_extra/*/fs_extra/fn.move_items_with_progress.html)  | Recursively moves files and directories from one location to another with information about progress |
| [fs_extra::remove_items](https://docs.rs/fs_extra/*/fs_extra/fn.remove_items.html)  | Removes files or directories |
| [fs_extra::file::copy](https://docs.rs/fs_extra/*/fs_extra/file/fn.copy.html)  | Copies the contents of one file to another |
| [fs_extra::file::copy_with_progress](https://docs.rs/fs_extra/*/fs_extra/file/fn.copy_with_progress.html)  | Copies the contents of one file to another with information about progress  |
| [fs_extra::file::move_file](https://docs.rs/fs_extra/*/fs_extra/file/fn.move_file.html)  | Moves a file from one location to another  |
| [fs_extra::file::move_file_with_progress](https://docs.rs/fs_extra/*/fs_extra/file/fn.move_file_with_progress.html)  | Moves a file from one location to another with information about progress  |
| [fs_extra::file::remove](https://docs.rs/fs_extra/*/fs_extra/file/fn.remove.html)  | Removes a file |
| [fs_extra::file::read_to_string](https://docs.rs/fs_extra/*/fs_extra/file/fn.read_to_string.html)  | Reads file content into a `String` |
| [fs_extra::file::write_all](https://docs.rs/fs_extra/*/fs_extra/file/fn.write_all.html)  | Writes `String` content to a file  |
| [fs_extra::dir::create](https://docs.rs/fs_extra/*/fs_extra/dir/fn.create.html)  | Creates a new, empty directory at the given path  |
| [fs_extra::dir::create_all](https://docs.rs/fs_extra/*/fs_extra/dir/fn.create_all.html)  | Recursively creates a directory and all of its parent components if they are missing  |
| [fs_extra::dir::copy](https://docs.rs/fs_extra/*/fs_extra/dir/fn.copy.html)  | Recursively copies the directory contents from one location to another |
| [fs_extra::dir::copy_with_progress](https://docs.rs/fs_extra/*/fs_extra/dir/fn.copy_with_progress.html)  | Recursively copies the directory contents from one location to another with information about progress |
| [fs_extra::dir::move_dir](https://docs.rs/fs_extra/*/fs_extra/dir/fn.move_dir.html)  | Moves directory contents from one location to another |
| [fs_extra::dir::move_dir_with_progress](https://docs.rs/fs_extra/*/fs_extra/dir/fn.move_dir_with_progress.html)  | Moves directory contents from one location to another with information about progress  |
| [fs_extra::dir::remove](https://docs.rs/fs_extra/*/fs_extra/dir/fn.remove.html)  | Removes directory  |
| [fs_extra::dir::get_size](https://docs.rs/fs_extra/*/fs_extra/dir/fn.get_size.html)  | Returns the size of the file or directory  |
| [fs_extra::dir::get_dir_content](https://docs.rs/fs_extra/*/fs_extra/dir/fn.get_dir_content.html)  | Gets details such as the size and child items of a directory |
| [fs_extra::dir::get_dir_content2](https://docs.rs/fs_extra/*/fs_extra/dir/fn.get_dir_content2.html)  | Gets details such as the size and child items of a directory using specified settings |
| [fs_extra::dir::get_details_entry](https://docs.rs/fs_extra/*/fs_extra/dir/fn.get_details_entry.html)  | Gets attributes of a directory entry |
| [fs_extra::dir::ls](https://docs.rs/fs_extra/*/fs_extra/dir/fn.ls.html)  | Gets attributes of directory entries in a directory |

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
fs_extra = "1.3.0"
```
## Examples

The following example shows how to copy a directory recursively and display progress. First a source directory `./temp/dir` containing file `test1.txt` and a subdirectory `sub` is createad with `sub` itself having a file `test2.txt`. `./temp/dir` and all contents are then copied out to `./out/dir`.

```rust
use std::path::Path;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};

extern crate fs_extra;
use fs_extra::dir::*;
use fs_extra::error::*;

fn example_copy() -> Result<()> {

    let path_from = Path::new("./temp");
    let path_to = path_from.join("out");
    let test_folder = path_from.join("test_folder");
    let dir = test_folder.join("dir");
    let sub = dir.join("sub");
    let file1 = dir.join("file1.txt");
    let file2 = sub.join("file2.txt");

    create_all(&sub, true)?;
    create_all(&path_to, true)?;
    fs_extra::file::write_all(&file1, "content1")?;
    fs_extra::file::write_all(&file2, "content2")?;

    assert!(dir.exists());
    assert!(sub.exists());
    assert!(file1.exists());
    assert!(file2.exists());


    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let handler = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            thread::sleep(time::Duration::from_millis(500));
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };
        copy_with_progress(&test_folder, &path_to, &options, handler).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                println!("{} of {} bytes",
                         process_info.copied_bytes,
                         process_info.total_bytes);
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    Ok(())

}
fn main() {
    example_copy();
}
```
