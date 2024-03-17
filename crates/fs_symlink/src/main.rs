#![cfg(target_os = "windows")]

use std::{
    os::windows::fs::{symlink_dir, symlink_file},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{GENERIC_READ, GENERIC_WRITE},
        Storage::FileSystem::{
            CreateFileW, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING,
        },
        System::Threading::{OpenEventW, SetEvent},
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct SymlinkData {
    target: PathBuf,
    path: PathBuf,
}

const PIPE_NAME: PCWSTR = windows::core::w!("\\\\.\\pipe\\zedsymlink");
const EVNET_NAME: PCWSTR = windows::core::w!("zed-global-symlink-finish");

fn main() {
    let Ok(pipe_handle) = (unsafe {
        CreateFileW(
            PIPE_NAME,
            GENERIC_READ.0 | GENERIC_WRITE.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    }) else {
        println!("error call pipe: {}", std::io::Error::last_os_error());
        return;
    };
    let Ok(event) = (unsafe { OpenEventW(EVENT_MODIFY_STATE, false, EVNET_NAME) }) else {
        println!("unable to open event: {}", std::io::Error::last_os_error());
        return;
    };

    let mut target_buffer;
    let mut bytes_read;
    loop {
        target_buffer = vec![0u8; 1024];
        bytes_read = 0u32;
        let Ok(_) = (unsafe {
            ReadFile(
                pipe_handle,
                Some(&mut target_buffer),
                Some(&mut bytes_read as _),
                None,
            )
        }) else {
            println!("Error call pipe: {}", std::io::Error::last_os_error());
            break;
        };
        println!("{} bytes read", bytes_read);
        let Ok(symlink) =
            serde_json::from_slice::<'_, SymlinkData>(&target_buffer[..(bytes_read as usize)])
                .inspect_err(|e| println!("unable to parse data: {:?}", e))
        else {
            break;
        };

        if symlink.target.is_file() {
            if let Err(e) = symlink_file(symlink.target, symlink.path) {
                println!("error create symlink file: {:?}", e);
            }
        } else {
            if let Err(e) = symlink_dir(symlink.target, symlink.path) {
                println!("error create symlink dir: {:?}", e);
            }
        }
        unsafe {
            SetEvent(event).inspect_err(|_| {
                log::error!("error setting event: {}", std::io::Error::last_os_error())
            });
        }
    }
}
