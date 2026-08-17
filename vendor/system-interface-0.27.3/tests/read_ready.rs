#[macro_use]
mod sys_common;

use std::fs::File;
use std::io::Read;
use system_interface::io::ReadReady;

#[test]
fn file_is_read_write() {
    let mut f = File::open("Cargo.toml").unwrap();

    let len = f.metadata().unwrap().len();
    assert_eq!(f.num_ready_bytes().unwrap(), len);

    let mut buf = [0u8; 6];
    f.read_exact(&mut buf).unwrap();
    assert_eq!(f.num_ready_bytes().unwrap(), len - (buf.len() as u64));

    #[cfg(target_os = "linux")]
    {
        let f = File::open("/dev/urandom").unwrap();
        let _ = f.num_ready_bytes().unwrap();

        let f = File::open("/dev/null").unwrap();
        let _ = f.num_ready_bytes().unwrap();
    }
}
