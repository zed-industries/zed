use anyhow::{anyhow, Result};
use clap::Parser;
use core_foundation::{
    array::{CFArray, CFIndex},
    string::kCFStringEncodingUTF8,
    url::{CFURLCreateWithBytes, CFURL},
};
use core_services::{kLSLaunchDefaults, LSLaunchURLSpec, LSOpenFromURLSpec, TCFType};
use ipc_channel::ipc::IpcOneShotServer;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process, ptr};

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

#[derive(Serialize, Deserialize)]
struct OpenResult {
    exit_status: i32,
    stdout_message: Option<String>,
    stderr_message: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (server, server_name) = IpcOneShotServer::<OpenResult>::new()?;
    let app_path = locate_app()?;
    launch_app(app_path, args.paths, server_name)?;

    let (_, result) = server.accept()?;
    if let Some(message) = result.stdout_message {
        println!("{}", message);
    }
    if let Some(message) = result.stderr_message {
        eprintln!("{}", message);
    }

    process::exit(result.exit_status)
}

fn locate_app() -> Result<PathBuf> {
    Ok("/Applications/Zed.app".into())
}

fn launch_app(app_path: PathBuf, paths_to_open: Vec<PathBuf>, server_name: String) -> Result<()> {
    let status = unsafe {
        let app_url =
            CFURL::from_path(&app_path, true).ok_or_else(|| anyhow!("invalid app path"))?;
        let mut urls_to_open = paths_to_open
            .into_iter()
            .map(|path| {
                CFURL::from_path(&path, true).ok_or_else(|| anyhow!("{:?} is invalid", path))
            })
            .collect::<Result<Vec<_>>>()?;

        let server_url = format!("zed_cli_response://{server_name}");
        urls_to_open.push(CFURL::wrap_under_create_rule(CFURLCreateWithBytes(
            ptr::null(),
            server_url.as_ptr(),
            server_url.len() as CFIndex,
            kCFStringEncodingUTF8,
            ptr::null(),
        )));

        let urls_to_open = CFArray::from_copyable(
            &urls_to_open
                .iter()
                .map(|url| url.as_concrete_TypeRef())
                .collect::<Vec<_>>(),
        );
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
        Ok(())
    } else {
        Err(anyhow!("cannot start {:?}", app_path))
    }
}
