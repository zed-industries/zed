use plugin::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

// #[import]
fn command(string: &str) -> Option<String> {
    None
}

// TODO: some sort of macro to generate ABI bindings
extern "C" {
    pub fn hello(item: u32) -> u32;
    pub fn bye(item: u32) -> u32;
}

// #[bind]
// pub async fn name(u32) -> u32 {

// }

// #[no_mangle]
// pub extern "C" fn very_unique_name_of_course() -> impl std::future::Future<Output = u32> {
//     async move {
//         std::fs::read_to_string("heck.txt").unwrap().len() as u32
//     }
// }

const BIN_PATH: &'static str =
    "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

#[bind]
pub fn name() -> &'static str {
    // let number = unsafe { hello(27) };
    // println!("got: {}", number);
    // let number = unsafe { bye(28) };
    // println!("got: {}", number);
    "vscode-json-languageserver"
}

#[bind]
pub fn server_args() -> Vec<String> {
    vec!["--stdio".into()]
}

#[bind]
pub fn fetch_latest_server_version() -> Option<String> {
    #[derive(Deserialize)]
    struct NpmInfo {
        versions: Vec<String>,
    }

    // TODO: command returns error code
    let output = command("npm info vscode-json-languageserver --json")?;
    // if !output.is_ok() {
    //     return None;
    // }

    let mut info: NpmInfo = serde_json::from_str(&output).ok()?;
    info.versions.pop()
}

#[bind]
pub fn fetch_server_binary(container_dir: PathBuf, version: String) -> Result<PathBuf, String> {
    let version_dir = container_dir.join(version.as_str());
    fs::create_dir_all(&version_dir)
        .map_err(|_| "failed to create version directory".to_string())?;
    let binary_path = version_dir.join(BIN_PATH);

    if fs::metadata(&binary_path).is_err() {
        let output = command(&format!(
            "npm install vscode-json-languageserver@{}",
            version
        ));
        if output.is_none() {
            return Err("failed to install vscode-json-languageserver".to_string());
        }

        if let Some(mut entries) = fs::read_dir(&container_dir).ok() {
            while let Some(entry) = entries.next() {
                if let Some(entry) = entry.ok() {
                    let entry_path = entry.path();
                    if entry_path.as_path() != version_dir {
                        fs::remove_dir_all(&entry_path).ok();
                    }
                }
            }
        }
    }

    Ok(binary_path)
}

#[bind]
pub fn cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    let mut last_version_dir = None;
    let mut entries = fs::read_dir(&container_dir).ok()?;

    while let Some(entry) = entries.next() {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_dir() {
            last_version_dir = Some(entry.path());
        }
    }

    let last_version_dir = last_version_dir?;
    let bin_path = last_version_dir.join(BIN_PATH);
    if bin_path.exists() {
        Some(bin_path)
    } else {
        None
    }
}

#[bind]
pub fn label_for_completion(label: String) -> Option<String> {
    None
}

#[bind]
pub fn initialization_options() -> Option<String> {
    Some("{ \"provideFormatter\": true }".to_string())
}

#[bind]
pub fn id_for_language(name: String) -> Option<String> {
    if name == "JSON" {
        Some("jsonc".into())
    } else {
        None
    }
}
