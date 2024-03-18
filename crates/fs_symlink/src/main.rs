#[cfg(target_os = "windows")]
fn main() {
    use std::{
        os::windows::fs::{symlink_dir, symlink_file},
        path::PathBuf,
    };

    use serde::{Deserialize, Serialize};
    use windows::{
        core::PCWSTR,
        Win32::{
            Foundation::{CloseHandle, GENERIC_READ, HANDLE},
            Storage::FileSystem::{
                CreateFileW, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ,
                FILE_WRITE_ATTRIBUTES, OPEN_EXISTING,
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

    // actual code goes here
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
        #[cfg(debug_assertions)]
        println!("linking: {:#?}", symlink);

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

    #[cfg(debug_assertions)]
    {
        use std::io::Read;

        println!("Press any key to exit...");
        let mut buffer = [0; 1];
        std::io::stdin()
            .read_exact(&mut buffer)
            .expect("unable to read input");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
