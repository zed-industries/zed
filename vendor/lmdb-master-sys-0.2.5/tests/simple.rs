use std::ffi::{c_void, CString};
use std::fs::{self, File};
use std::ptr;

use cstr::cstr;
use lmdb_master_sys::*;

// https://github.com/victorporof/lmdb/blob/mdb.master/libraries/liblmdb/moz-test.c

macro_rules! E {
    ($expr:expr) => {{
        match $expr {
            lmdb_master_sys::MDB_SUCCESS => (),
            err_code => assert!(false, "Failed with code {}", err_code),
        }
    }};
}

#[test]
#[cfg(target_pointer_width = "32")]
fn test_simple_32() {
    test_simple("./tests/fixtures/testdb-32")
}

#[test]
#[cfg(target_pointer_width = "64")]
fn test_simple_64() {
    test_simple("./tests/fixtures/testdb")
}

#[cfg(windows)]
fn get_file_fd(file: &File) -> std::os::windows::io::RawHandle {
    use std::os::windows::io::AsRawHandle;
    file.as_raw_handle()
}

#[cfg(unix)]
fn get_file_fd(file: &File) -> std::os::unix::io::RawFd {
    use std::os::unix::io::AsRawFd;
    file.as_raw_fd()
}

fn test_simple(env_path: &str) {
    let _ = fs::remove_dir_all(env_path);
    fs::create_dir_all(env_path).unwrap();

    let mut env: *mut MDB_env = ptr::null_mut();
    let mut dbi: MDB_dbi = 0;
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: ptr::null_mut(),
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: ptr::null_mut(),
    };
    let mut txn: *mut MDB_txn = ptr::null_mut();
    let sval = cstr!("foo").as_ptr() as *mut c_void;
    let dval = cstr!("bar").as_ptr() as *mut c_void;

    unsafe {
        E!(mdb_env_create(&mut env));
        E!(mdb_env_set_maxdbs(env, 2));
        let env_path = CString::new(env_path).unwrap();
        E!(mdb_env_open(env, env_path.as_ptr(), 0, 0o664));

        E!(mdb_txn_begin(env, ptr::null_mut(), 0, &mut txn));
        E!(mdb_dbi_open(
            txn,
            cstr!("subdb").as_ptr(),
            MDB_CREATE,
            &mut dbi
        ));
        E!(mdb_txn_commit(txn));

        key.mv_size = 3;
        key.mv_data = sval;
        data.mv_size = 3;
        data.mv_data = dval;

        E!(mdb_txn_begin(env, ptr::null_mut(), 0, &mut txn));
        E!(mdb_put(txn, dbi, &mut key, &mut data, 0));

        if cfg!(feature = "longer-keys") {
            // Try storing a key larger than 511 bytes (the default if MDB_MAXKEYSIZE is not set)
            let sval = cstr!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut pharetra sit amet aliquam. Sit amet nisl purus in mollis nunc. Eget egestas purus viverra accumsan in nisl nisi scelerisque. Duis ultricies lacus sed turpis tincidunt. Sem nulla pharetra diam sit. Leo vel orci porta non pulvinar. Erat pellentesque adipiscing commodo elit at imperdiet dui. Suspendisse ultrices gravida dictum fusce ut placerat orci nulla. Diam donec adipiscing tristique risus nec feugiat. In fermentum et sollicitudin ac orci. Ut sem nulla pharetra diam sit amet. Aliquam purus sit amet luctus venenatis lectus. Erat pellentesque adipiscing commodo elit at imperdiet dui accumsan. Urna duis convallis convallis tellus id interdum velit laoreet id. Ac feugiat sed lectus vestibulum mattis ullamcorper velit sed. Tincidunt arcu non sodales neque. Habitant morbi tristique senectus et netus et malesuada fames.").as_ptr() as *mut c_void;

            key.mv_size = 952;
            key.mv_data = sval;

            E!(mdb_put(txn, dbi, &mut key, &mut data, 0));
        }

        E!(mdb_txn_commit(txn));
    }

    let file = File::create("./tests/fixtures/copytestdb.mdb").unwrap();

    unsafe {
        let fd = get_file_fd(&file);
        E!(mdb_env_copyfd(env, fd));

        mdb_dbi_close(env, dbi);
        mdb_env_close(env);
    }
}
