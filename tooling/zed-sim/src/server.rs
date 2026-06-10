//! A tiny local control panel served over HTTP. The whole UI is one embedded
//! HTML page; this module just serves it and handles two POST actions.

use std::io::Cursor;
use std::path::Path;

use anyhow::{Context as _, Result};
use tiny_http::{Header, Method, Request, Response, Server};

use crate::profile::{self, Profile};
use crate::states::SimState;

const INDEX_HTML: &str = include_str!("../ui/index.html");

type Resp = Response<Cursor<Vec<u8>>>;

/// Starts the control panel and blocks serving requests until the process exits.
pub fn serve(zed_binary: &Path, port: u16, open_browser: bool) -> Result<()> {
    let server = Server::http(("127.0.0.1", port))
        .map_err(|err| anyhow::anyhow!("failed to start control server: {err}"))?;
    let address = server
        .server_addr()
        .to_ip()
        .context("control server is not bound to an IP address")?;
    let url = format!("http://{address}");

    println!("Zed Sim control panel: {url}");
    println!("Using Zed binary:      {}", zed_binary.display());
    println!(
        "Scratch profiles:      {}",
        profile::profiles_root().display()
    );

    if open_browser {
        if let Err(err) = open::that(&url) {
            eprintln!("Could not open a browser automatically ({err}). Open {url} manually.");
        }
    }

    for mut request in server.incoming_requests() {
        let response = route(&mut request, zed_binary);
        if let Err(err) = request.respond(response) {
            eprintln!("failed to send response: {err}");
        }
    }
    Ok(())
}

fn route(request: &mut Request, zed_binary: &Path) -> Resp {
    match (request.method(), request.url()) {
        (Method::Get, "/") => html(INDEX_HTML),
        (Method::Post, "/launch") => handle_launch(request, zed_binary),
        (Method::Post, "/cleanup") => handle_cleanup(),
        _ => json(404, serde_json::json!({ "error": "not found" })),
    }
}

fn handle_launch(request: &mut Request, zed_binary: &Path) -> Resp {
    let mut body = String::new();
    if let Err(err) = request.as_reader().read_to_string(&mut body) {
        return json(
            400,
            serde_json::json!({ "error": format!("could not read body: {err}") }),
        );
    }

    let payload: serde_json::Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(err) => {
            return json(
                400,
                serde_json::json!({ "error": format!("invalid JSON: {err}") }),
            );
        }
    };

    let state_id = payload
        .get("state")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    let Some(state) = SimState::from_id(state_id) else {
        return json(
            400,
            serde_json::json!({ "error": format!("unknown state: {state_id:?}") }),
        );
    };

    match launch_state(state, zed_binary) {
        Ok(profile_dir) => {
            println!("Launched {state_id} -> {profile_dir}");
            json(
                200,
                serde_json::json!({ "status": "launched", "profile": profile_dir }),
            )
        }
        Err(err) => {
            eprintln!("launch failed: {err:#}");
            json(500, serde_json::json!({ "error": format!("{err:#}") }))
        }
    }
}

fn launch_state(state: SimState, zed_binary: &Path) -> Result<String> {
    let profile = Profile::create(state)?;
    profile.launch(zed_binary)?;
    Ok(profile.dir.display().to_string())
}

fn handle_cleanup() -> Resp {
    match profile::wipe_profiles() {
        Ok(removed) => {
            println!("Wiped {removed} scratch profile(s)");
            json(
                200,
                serde_json::json!({ "status": "wiped", "removed": removed }),
            )
        }
        Err(err) => {
            eprintln!("cleanup failed: {err:#}");
            json(500, serde_json::json!({ "error": format!("{err:#}") }))
        }
    }
}

fn html(body: &str) -> Resp {
    Response::from_string(body).with_header(header("Content-Type", "text/html; charset=utf-8"))
}

fn json(status: u16, body: serde_json::Value) -> Resp {
    Response::from_string(body.to_string())
        .with_status_code(status)
        .with_header(header("Content-Type", "application/json"))
}

fn header(key: &'static str, value: &'static str) -> Header {
    Header::from_bytes(key.as_bytes(), value.as_bytes()).expect("static header is always valid")
}
