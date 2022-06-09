use plugin::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

// #[import]
// fn command(string: &str) -> Option<String>;

// #[no_mangle]
// // TODO: switch len from usize to u32?
// pub extern "C" fn #outer_fn_name(ptr: *const u8, len: usize) -> *const ::plugin::__Buffer {
//     // setup
//     let buffer = ::plugin::__Buffer { ptr, len };
//     let data = unsafe { buffer.to_vec() };

//     // operation
//     let data: #ty = match ::plugin::bincode::deserialize(&data) {
//         Ok(d) => d,
//         Err(e) => panic!("Data passed to function not deserializable."),
//     };
//     let result = #inner_fn_name(#args);
//     let new_data: Result<Vec<u8>, _> = ::plugin::bincode::serialize(&result);
//     let new_data = new_data.unwrap();

//     // teardown
//     let new_buffer = unsafe { ::plugin::__Buffer::from_vec(new_data) };
//     return new_buffer.leak_to_heap();
// }

extern "C" {
    fn __command(buffer: u64) -> u64;
}

#[no_mangle]
fn command(string: &str) -> Option<Vec<u8>> {
    dbg!("executing command: {}", string);
    // setup
    let data = string;
    let data = ::plugin::bincode::serialize(&data).unwrap();
    let buffer = unsafe { ::plugin::__Buffer::from_vec(data) };

    // operation
    let new_buffer = unsafe { __command(buffer.into_u64()) };
    let new_data = unsafe { ::plugin::__Buffer::from_u64(new_buffer).to_vec() };
    let new_data: Option<Vec<u8>> = match ::plugin::bincode::deserialize(&new_data) {
        Ok(d) => d,
        Err(e) => panic!("Data returned from function not deserializable."),
    };

    // teardown
    return new_data;
}

// TODO: some sort of macro to generate ABI bindings
// extern "C" {
//     pub fn hello(item: u32) -> u32;
//     pub fn bye(item: u32) -> u32;
// }

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

#[export]
pub fn name() -> &'static str {
    // let number = unsafe { hello(27) };
    // let number = unsafe { bye(28) };
    // println!("got: {}", number);
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
    dbg!(&output);

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
        dbg!(&output);
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
    println!("reading directory");
    let mut entries = fs::read_dir(&container_dir).ok()?;

    while let Some(entry) = entries.next() {
        println!("looking at entries");
        let entry = entry.ok()?;
        println!("some more stuff");
        if entry.file_type().ok()?.is_dir() {
            println!("this is it!");

            last_version_dir = Some(entry.path());
        }
    }

    let last_version_dir = last_version_dir?;
    println!("here we go");
    let bin_path = last_version_dir.join(BIN_PATH);
    if bin_path.exists() {
        dbg!(&bin_path);
        Some(bin_path)
    } else {
        println!("no binary found");
        None
    }
}

#[export]
pub fn label_for_completion(label: String) -> Option<String> {
    None
}

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
