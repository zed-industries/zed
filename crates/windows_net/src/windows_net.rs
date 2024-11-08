#![cfg(target_os = "windows")]

use std::{path::Path, sync::Once};

use windows::Win32::Networking::WinSock::{
    WSAStartup, ADDRESS_FAMILY, AF_UNIX, SOCKADDR_UN, SOCKET_ERROR,
};

pub mod listener;
pub mod socket;
pub mod stream;
pub mod util;

pub fn init() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        let mut wsa_data = std::mem::zeroed();
        let result = WSAStartup(0x202, &mut wsa_data);
        if result != 0 {
            panic!("WSAStartup failed: {}", result);
        }
    });
}

// https://devblogs.microsoft.com/commandline/af_unix-comes-to-windows/
fn sockaddr_un(path: &Path) -> std::io::Result<(SOCKADDR_UN, usize)> {
    let mut addr = SOCKADDR_UN::default();
    addr.sun_family = ADDRESS_FAMILY(AF_UNIX);

    let bytes = path
        .to_str()
        .map(|s| s.as_bytes())
        .ok_or(std::io::ErrorKind::InvalidInput)?;

    if bytes.contains(&0) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "paths may not contain interior null bytes",
        ));
    }
    if bytes.len() >= addr.sun_path.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path must be shorter than SUN_LEN",
        ));
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            addr.sun_path.as_mut_ptr().cast(),
            bytes.len(),
        );
    }

    let mut len = sun_path_offset(&addr) + bytes.len();
    match bytes.first() {
        Some(&0) | None => {}
        Some(_) => len += 1,
    }
    Ok((addr, len))
}

fn sun_path_offset(addr: &SOCKADDR_UN) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = addr as *const _ as usize;
    let path = &addr.sun_path as *const _ as usize;
    path - base
}

fn map_ret(ret: i32) -> std::io::Result<()> {
    if ret == SOCKET_ERROR {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
