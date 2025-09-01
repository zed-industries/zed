use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
    sync::Once,
};

use windows::Win32::Networking::WinSock::{
    ADDRESS_FAMILY, AF_UNIX, SOCKADDR_UN, SOCKET_ERROR, WSAGetLastError, WSAStartup,
};

pub(crate) fn init() {
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
pub(crate) fn sockaddr_un<P: AsRef<Path>>(path: P) -> Result<(SOCKADDR_UN, usize)> {
    let mut addr = SOCKADDR_UN::default();
    addr.sun_family = ADDRESS_FAMILY(AF_UNIX);

    let bytes = path
        .as_ref()
        .to_str()
        .map(|s| s.as_bytes())
        .ok_or(ErrorKind::InvalidInput)?;

    if bytes.contains(&0) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "paths may not contain interior null bytes",
        ));
    }
    if bytes.len() >= addr.sun_path.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
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

pub(crate) fn map_ret(ret: i32) -> Result<usize> {
    if ret == SOCKET_ERROR {
        Err(Error::from_raw_os_error(unsafe { WSAGetLastError().0 }))
    } else {
        Ok(ret as usize)
    }
}

fn sun_path_offset(addr: &SOCKADDR_UN) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = addr as *const _ as usize;
    let path = &addr.sun_path as *const _ as usize;
    path - base
}
