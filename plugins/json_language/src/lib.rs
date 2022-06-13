use plugin::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[import]
fn command(string: &str) -> Option<Vec<u8>>;

const BIN_PATH: &'static str =
    "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

#[export]
pub fn name() -> &'static str {
    "vscode-json-languageserver"
}

#[export]
pub fn server_args() -> Vec<String> {
    vec!["--stdio".into()]
}

#[export]
pub fn fetch_latest_server_version() -> Option<String> {
    #[derive(Deserialize)]
    struct NpmInfo {
        versions: Vec<String>,
    }

    // TODO: command returns error code
    let output =
        command("npm info vscode-json-languageserver --json").expect("could not run command");
    // if !output.is_ok() {
    //     return None;
    // }

    let output = String::from_utf8(output).unwrap();

    let mut info: NpmInfo = serde_json::from_str(&output).ok()?;
    info.versions.pop()
}

#[export]
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
        let output = output.map(String::from_utf8);
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

#[export]
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
        dbg!(&bin_path);
        Some(bin_path)
    } else {
        println!("no binary found");
        None
    }
}

// #[export]
// pub fn label_for_completion(
//     item: &lsp::CompletionItem,
//     // language: &language::Language,
// ) -> Option<language::CodeLabel> {
//     // TODO: Push more of this method down into the plugin.
//     use lsp::CompletionItemKind as Kind;
//     let len = item.label.len();
//     let grammar = language.grammar()?;
//     let kind = format!("{:?}", item.kind?);

//     // TODO: implementation

//     let highlight_id = grammar.highlight_id_for_name(&name)?;
//     Some(language::CodeLabel {
//         text: item.label.clone(),
//         runs: vec![(0..len, highlight_id)],
//         filter_range: 0..len,
//     })
// }

#[export]
pub fn initialization_options() -> Option<String> {
    Some("{ \"provideFormatter\": true }".to_string())
}

#[export]
pub fn id_for_language(name: String) -> Option<String> {
    if name == "JSON" {
        Some("jsonc".into())
    } else {
        None
    }
}
