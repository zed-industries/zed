use std::{sync::Arc, thread::JoinHandle};

use anyhow::Context;
use cli::{ipc::IpcOneShotServer, CliRequest, CliResponse, IpcHandshake};
use parking_lot::Mutex;
use release_channel::app_identifier;
use util::ResultExt;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileW, ReadFile, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE,
            OPEN_EXISTING, PIPE_ACCESS_INBOUND,
        },
        System::{
            Pipes::{
                ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_MESSAGE,
                PIPE_TYPE_MESSAGE, PIPE_WAIT,
            },
            Threading::CreateMutexW,
        },
    },
};

use crate::{Args, OpenListener};

pub fn check_single_instance(opener: OpenListener, args: &Args) -> bool {
    unsafe {
        CreateMutexW(
            None,
            false,
            &HSTRING::from(format!("{}-Instance-Mutex", app_identifier())),
        )
        .expect("Unable to create instance mutex.")
    };
    let first_instance = unsafe { GetLastError() } != ERROR_ALREADY_EXISTS;

    if first_instance {
        // We are the first instance, listen for messages sent from other instances
        std::thread::spawn(move || with_pipe(|url| opener.open_urls(vec![url])));
    } else if !args.foreground {
        // We are not the first instance, send args to the first instance
        send_args_to_instance(args).log_err();
    }

    first_instance
}

fn with_pipe(f: impl Fn(String)) {
    let pipe = unsafe {
        CreateNamedPipeW(
            &HSTRING::from(format!("\\\\.\\pipe\\{}-Named-Pipe", app_identifier())),
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

// This part of code is mostly from crates/cli/src/main.rs
fn send_args_to_instance(args: &Args) -> anyhow::Result<()> {
    if let Some(dock_menu_action_idx) = args.dock_action {
        let url = format!("zed-dock-action://{}", dock_menu_action_idx);
        return write_message_to_instance_pipe(url.as_bytes());
    }

    let (server, server_name) =
        IpcOneShotServer::<IpcHandshake>::new().context("Handshake before Zed spawn")?;
    let url = format!("zed-cli://{server_name}");

    let request = {
        let mut paths = vec![];
        let mut urls = vec![];
        for path in args.paths_or_urls.iter() {
            match std::fs::canonicalize(&path) {
                Ok(path) => paths.push(path.to_string_lossy().to_string()),
                Err(error) => {
                    if path.starts_with("zed://")
                        || path.starts_with("http://")
                        || path.starts_with("https://")
                        || path.starts_with("file://")
                        || path.starts_with("ssh://")
                    {
                        urls.push(path.clone());
                    } else {
                        log::error!("error parsing path argument: {}", error);
                    }
                }
            }
        }
        CliRequest::Open {
            paths,
            urls,
            wait: false,
            open_new_workspace: None,
            env: None,
        }
    };

    let exit_status = Arc::new(Mutex::new(None));
    let sender: JoinHandle<anyhow::Result<()>> = std::thread::spawn({
        let exit_status = exit_status.clone();
        move || {
            let (_, handshake) = server.accept().context("Handshake after Zed spawn")?;
            let (tx, rx) = (handshake.requests, handshake.responses);

            tx.send(request)?;

            while let Ok(response) = rx.recv() {
                match response {
                    CliResponse::Ping => {}
                    CliResponse::Stdout { message } => log::info!("{message}"),
                    CliResponse::Stderr { message } => log::error!("{message}"),
                    CliResponse::Exit { status } => {
                        exit_status.lock().replace(status);
                        return Ok(());
                    }
                }
            }
            Ok(())
        }
    });

    write_message_to_instance_pipe(url.as_bytes())?;
    sender.join().unwrap()?;
    if let Some(exit_status) = exit_status.lock().take() {
        std::process::exit(exit_status);
    }
    Ok(())
}

fn write_message_to_instance_pipe(message: &[u8]) -> anyhow::Result<()> {
    unsafe {
        let pipe = CreateFileW(
            &HSTRING::from(format!("\\\\.\\pipe\\{}-Named-Pipe", app_identifier())),
            GENERIC_WRITE.0,
            FILE_SHARE_MODE::default(),
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES::default(),
            None,
        )?;
        WriteFile(pipe, Some(message), None, None)?;
        CloseHandle(pipe)?;
    }
    Ok(())
}
