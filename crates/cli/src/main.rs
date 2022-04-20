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
use serde::Deserialize;
use std::{ffi::OsStr, fs, path::PathBuf, ptr};

#[derive(Parser)]
#[clap(name = "zed", global_setting(clap::AppSettings::NoAutoVersion))]
struct Args {
    /// Wait for all of the given paths to be closed before exiting.
    #[clap(short, long)]
    wait: bool,
    /// A sequence of space-separated paths that you want to open.
    #[clap()]
    paths: Vec<PathBuf>,
    /// Print Zed's version and the app path.
    #[clap(short, long)]
    version: bool,
    /// Custom Zed.app path
    #[clap(short, long)]
    bundle_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct InfoPlist {
    #[serde(rename = "CFBundleShortVersionString")]
    bundle_short_version_string: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bundle_path = if let Some(bundle_path) = args.bundle_path {
        bundle_path.canonicalize()?
    } else {
        locate_bundle()?
    };

    if args.version {
        let plist_path = bundle_path.join("Contents/Info.plist");
        let plist = plist::from_file::<_, InfoPlist>(plist_path)?;
        println!(
            "Zed {} – {}",
            plist.bundle_short_version_string,
            bundle_path.to_string_lossy()
        );
        return Ok(());
    }

    let (tx, rx) = launch_app(bundle_path)?;

    tx.send(CliRequest::Open {
        paths: args
            .paths
            .into_iter()
            .map(|path| fs::canonicalize(path).map_err(|error| anyhow!(error)))
            .collect::<Result<Vec<PathBuf>>>()?,
        wait: args.wait,
    })?;

    while let Ok(response) = rx.recv() {
        match response {
            CliResponse::Ping => {}
            CliResponse::Stdout { message } => println!("{message}"),
            CliResponse::Stderr { message } => eprintln!("{message}"),
            CliResponse::Exit { status } => std::process::exit(status),
        }
    }

    Ok(())
}

fn locate_bundle() -> Result<PathBuf> {
    let cli_path = std::env::current_exe()?.canonicalize()?;
    let mut app_path = cli_path.clone();
    while app_path.extension() != Some(OsStr::new("app")) {
        if !app_path.pop() {
            return Err(anyhow!("cannot find app bundle containing {:?}", cli_path));
        }
    }
    Ok(app_path)
}

fn launch_app(app_path: PathBuf) -> Result<(IpcSender<CliRequest>, IpcReceiver<CliResponse>)> {
    let (server, server_name) = IpcOneShotServer::<IpcHandshake>::new()?;
    let url = format!("zed-cli://{server_name}");

    let status = unsafe {
        let app_url =
            CFURL::from_path(&app_path, true).ok_or_else(|| anyhow!("invalid app path"))?;
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
