use anyhow::{anyhow, Result};
use clap::Parser;
use cli::{CliRequest, CliResponse, IpcHandshake};
use core_foundation::{
    array::{CFArray, CFIndex},
    string::kCFStringEncodingUTF8,
    url::{CFURLCreateWithBytes, CFURL},
};
use core_services::{kLSLaunchDefaults, LSLaunchURLSpec, LSOpenFromURLSpec, TCFType};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use std::{fs, path::PathBuf, ptr};

#[derive(Parser)]
#[clap(name = "zed")]
struct Args {
    /// Wait for all of the given paths to be closed before exiting.
    #[clap(short, long)]
    wait: bool,
    /// A sequence of space-separated paths that you want to open.
    #[clap()]
    paths: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let app_path = locate_app()?;
    let (tx, rx) = launch_app(app_path)?;

    tx.send(CliRequest::Open {
        paths: args
            .paths
            .into_iter()
            .map(|path| fs::canonicalize(path).map_err(|error| anyhow!(error)))
            .collect::<Result<Vec<PathBuf>>>()?,
        wait: false,
    })?;

    while let Ok(response) = rx.recv() {
        match response {
            CliResponse::Stdout { message } => println!("{message}"),
            CliResponse::Stderr { message } => eprintln!("{message}"),
            CliResponse::Exit { status } => std::process::exit(status),
        }
    }

    Ok(())
}

fn locate_app() -> Result<PathBuf> {
    Ok("/Users/nathan/src/zed/target/debug/bundle/osx/Zed.app".into())
    // Ok("/Applications/Zed.app".into())
}

fn launch_app(app_path: PathBuf) -> Result<(IpcSender<CliRequest>, IpcReceiver<CliResponse>)> {
    let (server, server_name) = IpcOneShotServer::<IpcHandshake>::new()?;

    let status = unsafe {
        let app_url =
            CFURL::from_path(&app_path, true).ok_or_else(|| anyhow!("invalid app path"))?;

        let url = format!("zed-cli://{server_name}");
        let url_to_open = CFURL::wrap_under_create_rule(CFURLCreateWithBytes(
            ptr::null(),
            url.as_ptr(),
            url.len() as CFIndex,
            kCFStringEncodingUTF8,
            ptr::null(),
        ));

        let urls_to_open = CFArray::from_copyable(&[url_to_open.as_concrete_TypeRef()]);

        LSOpenFromURLSpec(
            &LSLaunchURLSpec {
                appURL: app_url.as_concrete_TypeRef(),
                itemURLs: urls_to_open.as_concrete_TypeRef(),
                passThruParams: ptr::null(),
                launchFlags: kLSLaunchDefaults,
                asyncRefCon: ptr::null_mut(),
            },
            ptr::null_mut(),
        )
    };

    if status == 0 {
        let (_, handshake) = server.accept()?;
        Ok((handshake.requests, handshake.responses))
    } else {
        Err(anyhow!("cannot start {:?}", app_path))
    }
}
