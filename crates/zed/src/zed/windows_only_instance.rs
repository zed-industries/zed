use anyhow::Context;
use release_channel::APP_IDENTIFIER;
use util::ResultExt;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS, HANDLE},
        Storage::FileSystem::{ReadFile, PIPE_ACCESS_INBOUND},
        System::{
            Pipes::{
                ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_MESSAGE,
                PIPE_TYPE_MESSAGE, PIPE_WAIT,
            },
            Threading::CreateMutexW,
        },
    },
};

use super::OpenListener;

pub fn check_single_instance(opener: OpenListener) -> bool {
    unsafe {
        CreateMutexW(
            None,
            false,
            &HSTRING::from(format!("{}-Instance-Mutex", *APP_IDENTIFIER)),
        )
        .expect("Unable to create instance sync event")
    };
    let first_instance = unsafe { GetLastError() } != ERROR_ALREADY_EXISTS;
    if first_instance {
        // We are the first instance, listen for messages sent from other instances
        std::thread::spawn(move || with_pipe(|url| opener.open_urls(vec![url])));
    }
    first_instance
}

fn with_pipe(f: impl Fn(String)) {
    let pipe = unsafe {
        CreateNamedPipeW(
            &HSTRING::from(format!("\\\\.\\pipe\\{}-Named-Pipe", *APP_IDENTIFIER)),
            PIPE_ACCESS_INBOUND,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            1,
            128,
            128,
            0,
            None,
        )
    };
    if pipe.is_invalid() {
        log::error!("Failed to create named pipe: {:?}", unsafe {
            GetLastError()
        });
        return;
    }

    loop {
        if let Some(message) = retrieve_message_from_pipe(pipe)
            .context("Failed to read from named pipe")
            .log_err()
        {
            f(message);
        }
    }
}

fn retrieve_message_from_pipe(pipe: HANDLE) -> anyhow::Result<String> {
    unsafe { ConnectNamedPipe(pipe, None)? };
    let message = retrieve_message_from_pipe_inner(pipe);
    unsafe { DisconnectNamedPipe(pipe).log_err() };
    message
}

fn retrieve_message_from_pipe_inner(pipe: HANDLE) -> anyhow::Result<String> {
    let mut buffer = [0u8; 128];
    unsafe {
        ReadFile(pipe, Some(&mut buffer), None, None)?;
    }
    let message = std::ffi::CStr::from_bytes_until_nul(&buffer)?;
    Ok(message.to_string_lossy().to_string())
}
