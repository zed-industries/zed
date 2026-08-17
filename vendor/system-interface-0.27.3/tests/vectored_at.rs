#[macro_use]
mod sys_common;

use std::fs::OpenOptions;
use std::io::{self, IoSlice, IoSliceMut};
#[cfg(any(not(windows), feature = "cap_std_impls"))]
use sys_common::io::tmpdir;
use system_interface::fs::FileIoExt;
use system_interface::io::IoExt;

#[cfg(any(not(windows), feature = "cap_std_impls"))]
#[test]
fn cap_read_exact_vectored_at() {
    let tmpdir = tmpdir();
    let file = check!(tmpdir.open_with(
        "file",
        cap_std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
    ));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    check!(file.read_exact_vectored_at(&mut bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    assert_eq!(&buf0, b"efghijkl");
    assert_eq!(&buf1, b"mnopqrst");
}

/// Like `cap_read_exact_vectored_at`, but with an unexpected EOF error.
#[cfg(any(not(windows), feature = "cap_std_impls"))]
#[test]
fn cap_read_exact_vectored_at_unexpected_eof() {
    let tmpdir = tmpdir();
    let file = check!(tmpdir.open_with(
        "file",
        cap_std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
    ));
    check!(write!(&file, "abcdefghijkl"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    assert_eq!(
        file.read_exact_vectored_at(&mut bufs, 4)
            .unwrap_err()
            .kind(),
        io::ErrorKind::UnexpectedEof
    );
}

#[test]
fn read_exact_vectored_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    check!(file.read_exact_vectored_at(&mut bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    assert_eq!(&buf0, b"efghijkl");
    assert_eq!(&buf1, b"mnopqrst");
}

/// Like `read_exact_vectored_at`, but with an unexpected EOF error.
#[test]
fn read_exact_vectored_at_unexpected_eof() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijkl"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    assert_eq!(
        file.read_exact_vectored_at(&mut bufs, 4)
            .unwrap_err()
            .kind(),
        io::ErrorKind::UnexpectedEof
    );
}

#[test]
fn read_vectored_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    let nread = check!(file.read_vectored_at(&mut bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    if nread >= 8 {
        assert_eq!(&buf0, b"efghijkl");
    }
    if nread == 16 {
        assert_eq!(&buf1, b"mnopqrst");
    }
    assert!(nread <= 16);
}

#[test]
fn read_exact_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    check!(file.read_exact_at(&mut buf0, 4));
    check!(file.read_exact_at(&mut buf1, 12));
    assert_eq!(check!(file.stream_position()), 26);
    assert_eq!(&buf0, b"efghijkl");
    assert_eq!(&buf1, b"mnopqrst");
}

/// Like `read_exact_at`, but with an unexpected EOF error.
#[test]
fn read_exact_at_unexpected_eof() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijkl"));
    let mut buf1 = vec![0; 8];
    assert_eq!(
        file.read_exact_at(&mut buf1, 12).unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof
    );
}

#[test]
fn read_exact_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    check!(file.seek(std::io::SeekFrom::Start(4)));
    check!(file.read_exact_vectored(&mut bufs));
    assert_eq!(check!(file.stream_position()), 20);
    assert_eq!(&buf0, b"efghijkl");
    assert_eq!(&buf1, b"mnopqrst");
}

/// Like `read_exact_vectored`, but with an unexpected EOF error.
#[test]
fn read_exact_vectored_unexpected_eof() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijkl"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    check!(file.seek(std::io::SeekFrom::Start(4)));
    assert_eq!(
        file.read_exact_vectored(&mut bufs).unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof
    );
}

#[test]
fn read_exact() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    check!(file.seek(std::io::SeekFrom::Start(4)));
    check!(file.read_exact(&mut buf0));
    check!(file.read_exact(&mut buf1));
    assert_eq!(check!(file.stream_position()), 20);
    assert_eq!(&buf0, b"efghijkl");
    assert_eq!(&buf1, b"mnopqrst");
}

#[test]
fn read_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let mut bufs = vec![IoSliceMut::new(&mut buf0), IoSliceMut::new(&mut buf1)];
    check!(file.seek(std::io::SeekFrom::Start(4)));
    let nread = check!(file.read_vectored(&mut bufs));
    assert_eq!(check!(file.stream_position()), (4 + nread) as u64);
    if nread >= 8 {
        assert_eq!(&buf0, b"efghijkl");
    }
    if nread == 16 {
        assert_eq!(&buf1, b"mnopqrst");
    }
    assert!(nread <= 16);
}

#[test]
fn read_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    let nread0 = check!(file.read_at(&mut buf0, 4));
    let nread1 = check!(file.read_at(&mut buf1, 12));
    assert_eq!(check!(file.stream_position()), 26);
    if nread0 == 8 {
        assert_eq!(&buf0, b"efghijkl");
        if nread1 == 8 {
            assert_eq!(&buf1, b"mnopqrst");
        }
    }
    assert!(nread0 <= 8);
    assert!(nread1 <= 8);
}

#[test]
fn read() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let mut buf0 = vec![0; 8];
    let mut buf1 = vec![0; 8];
    check!(file.seek(std::io::SeekFrom::Start(4)));
    let nread0 = check!(file.read(&mut buf0));
    let nread1 = check!(file.read(&mut buf1));
    assert_eq!(check!(file.stream_position()), (4 + nread0 + nread1) as u64);
    if nread0 == 8 {
        assert_eq!(&buf0, b"efghijkl");
        if nread1 == 8 {
            assert_eq!(&buf1, b"mnopqrst");
        }
    }
    assert!(nread0 <= 8);
    assert!(nread1 <= 8);
}

#[cfg(any(not(windows), feature = "cap_std_impls"))]
#[test]
fn cap_write_all_vectored_at() {
    let tmpdir = tmpdir();
    let file = check!(tmpdir.open_with(
        "file",
        cap_std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
    ));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.write_all_vectored_at(&mut bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdEFGHIJKLMNOPQRSTuvwxyz");
}

#[cfg(any(not(windows), feature = "cap_std_impls"))]
#[test]
fn cap_write_all_vectored_after_end() {
    let tmpdir = tmpdir();
    let file = check!(tmpdir.open_with(
        "file",
        cap_std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
    ));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.write_all_vectored_at(&mut bufs, 32));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(
        back,
        "abcdefghijklmnopqrstuvwxyz\0\0\0\0\0\0EFGHIJKLMNOPQRST"
    );
}

#[test]
fn write_all_vectored_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.write_all_vectored_at(&mut bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdEFGHIJKLMNOPQRSTuvwxyz");
}

#[test]
fn write_all_vectored_after_end() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.write_all_vectored_at(&mut bufs, 32));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(
        back,
        "abcdefghijklmnopqrstuvwxyz\0\0\0\0\0\0EFGHIJKLMNOPQRST"
    );
}

#[test]
fn write_vectored_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    let nwritten = check!(file.write_vectored_at(&bufs, 4));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    if nwritten >= 8 {
        assert_eq!(&back[..12], "abcdEFGHIJKL");
    }
    if nwritten == 16 {
        assert_eq!(back, "abcdEFGHIJKLMNOPQRSTuvwxyz");
    }
    assert!(nwritten <= 16);
}

#[test]
fn write_vectored_after_end() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    let nwritten = check!(file.write_vectored_at(&bufs, 32));
    assert!(nwritten > 0);
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(&back[..26], "abcdefghijklmnopqrstuvwxyz");
    if nwritten >= 8 {
        assert_eq!(&back[26..40], "\0\0\0\0\0\0EFGHIJKL");
    }
    if nwritten == 16 {
        assert_eq!(&back[26..], "\0\0\0\0\0\0EFGHIJKLMNOPQRST");
    }
    assert!(nwritten <= 16);
}

#[test]
fn write_all_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    check!(file.write_all_at(&buf0, 4));
    check!(file.write_all_at(&buf1, 12));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdEFGHIJKLMNOPQRSTuvwxyz");
}

#[test]
fn write_all_after_end() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    check!(file.write_all_at(&buf0, 32));
    check!(file.write_all_at(&buf1, 40));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(
        back,
        "abcdefghijklmnopqrstuvwxyz\0\0\0\0\0\0EFGHIJKLMNOPQRST"
    );
}

#[test]
fn write_all_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let mut bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    check!(file.write_all_vectored(&mut bufs));
    assert_eq!(check!(file.stream_position()), 42);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
}

#[test]
fn write_all() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    check!(file.write_all(&buf0));
    check!(file.write_all(&buf1));
    assert_eq!(check!(file.stream_position()), 42);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
}

#[test]
fn write_vectored() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let bufs = vec![IoSlice::new(&buf0), IoSlice::new(&buf1)];
    let nwritten = check!(file.write_vectored(&bufs));
    assert_eq!(check!(file.stream_position()), (26 + nwritten) as u64);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(
        back,
        &"abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST"[..26 + nwritten]
    );
}

#[test]
fn write_at() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let nwritten0 = check!(file.write_at(&buf0, 4));
    let nwritten1 = check!(file.write_at(&buf1, 12));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    if nwritten0 == 8 {
        if nwritten1 == 8 {
            assert_eq!(back, "abcdEFGHIJKLMNOPQRSTuvwxyz");
        } else {
            assert_eq!(&back.as_bytes()[0..4 + nwritten0], b"abcdEFGHIJKL");
        }
    }
    assert!(nwritten0 <= 8);
    assert!(nwritten1 <= 8);
}

#[test]
fn write_after_end() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let nwritten0 = check!(file.write_at(&buf0, 32));
    let nwritten1 = check!(file.write_at(&buf1, 40));
    assert_eq!(check!(file.stream_position()), 26);
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    assert_eq!(&back[..26], "abcdefghijklmnopqrstuvwxyz");
    if nwritten0 > 0 {
        assert_eq!(
            &back[26..26 + 6 + nwritten0],
            &"\0\0\0\0\0\0EFGHIJKL"[..6 + nwritten0]
        );
    }
    if nwritten1 > 0 {
        assert_eq!(&back[26..26 + 6], "\0\0\0\0\0\0");
        assert_eq!(
            &back[26 + 6 + 8..26 + 6 + 8 + nwritten1],
            &"MNOPQRST"[..nwritten1]
        );
    }
    assert!(nwritten0 <= 8);
    assert!(nwritten1 <= 8);
}

#[test]
fn write() {
    let dir = tempfile::tempdir().unwrap();
    let file = check!(OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(dir.path().join("file")));
    check!(write!(&file, "abcdefghijklmnopqrstuvwxyz"));
    let buf0 = b"EFGHIJKL".to_vec();
    let buf1 = b"MNOPQRST".to_vec();
    let nwritten0 = check!(file.write(&buf0));
    let nwritten1 = check!(file.write(&buf1));
    assert_eq!(
        check!(file.stream_position()),
        (26 + nwritten0 + nwritten1) as u64
    );
    let mut back = String::new();
    check!(file.seek(std::io::SeekFrom::Start(0)));
    check!(file.read_to_string(&mut back));
    if nwritten0 == 8 {
        if nwritten1 == 8 {
            assert_eq!(back, "abcdefghijklmnopqrstuvwxyzEFGHIJKLMNOPQRST");
        } else {
            assert_eq!(&back.as_bytes()[22..26 + nwritten0], b"wxyzEFGHIJKL");
        }
    }
    assert!(nwritten0 <= 8);
    assert!(nwritten1 <= 8);
}
