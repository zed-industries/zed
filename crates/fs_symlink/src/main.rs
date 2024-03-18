#![cfg(target_os = "windows")]

use std::{
    os::windows::fs::{symlink_dir, symlink_file},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileW, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_WRITE_ATTRIBUTES,
            OPEN_EXISTING,
        },
        System::Pipes::{SetNamedPipeHandleState, PIPE_READMODE_MESSAGE},
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct SymlinkData {
    target: PathBuf,
    path: PathBuf,
}

const PIPE_NAME: PCWSTR = windows::core::w!("\\\\.\\pipe\\zedsymlink");

fn main() {
    let Ok(pipe_handle) = (unsafe {
        CreateFileW(
            PIPE_NAME,
            GENERIC_READ.0 | FILE_WRITE_ATTRIBUTES.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    }) else {
        println!("error open pipe: {}", std::io::Error::last_os_error());
        return;
    };
    let mode = PIPE_READMODE_MESSAGE;
    unsafe {
        if SetNamedPipeHandleState(pipe_handle, Some(&mode as _), None, None).is_err() {
            println!(
                "unable to configure pipe: {}",
                std::io::Error::last_os_error()
            );
            destroy(pipe_handle);
            return;
        }
    }

    let mut target_buffer;
    let mut bytes_read;
    loop {
        target_buffer = vec![0u8; 2048];
        bytes_read = 0u32;
        let Ok(_) = (unsafe {
            ReadFile(
                pipe_handle,
                Some(&mut target_buffer),
                Some(&mut bytes_read as _),
                None,
            )
        }) else {
            println!("Error read from pipe: {}", std::io::Error::last_os_error());
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
    }
    destroy(pipe_handle);
}

#[inline]
fn destroy(handle: HANDLE) {
    unsafe {
        let _ = CloseHandle(handle).inspect_err(|_| {
            println!(
                "unable to close handle: {}",
                std::io::Error::last_os_error()
            )
        });
    }
}
