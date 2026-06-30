//! A tiny local control panel served over HTTP. The whole UI is one embedded
//! HTML page; this module serves it, exposes the configured impersonation
//! accounts, and handles the launch / cleanup actions.

use std::io::Cursor;
use std::path::Path;

use anyhow::{Context as _, Result};
use serde_json::{Map, Value};
use tiny_http::{Header, Method, Request, Response, Server};

use crate::config::AppConfig;
use crate::profile::{self, Profile};
use crate::states::SimState;

const INDEX_HTML: &str = include_str!("../ui/index.html");

type Resp = Response<Cursor<Vec<u8>>>;

/// Starts the control panel and blocks serving requests until the process exits.
pub fn serve(zed_binary: &Path, config: &AppConfig, port: u16, open_browser: bool) -> Result<()> {
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
    if config.impersonation_enabled() {
        println!(
            "Impersonation:         enabled ({} account(s), backend {})",
            config.accounts.len(),
            config.server_url.as_deref().unwrap_or("?"),
        );
    } else {
        println!("Impersonation:         disabled (see README to configure)");
    }

    if open_browser {
        if let Err(err) = open::that(&url) {
            eprintln!("Could not open a browser automatically ({err}). Open {url} manually.");
        }
    }

    for mut request in server.incoming_requests() {
        let response = route(&mut request, zed_binary, config);
        if let Err(err) = request.respond(response) {
            eprintln!("failed to send response: {err}");
        }
    }
    Ok(())
}

fn route(request: &mut Request, zed_binary: &Path, config: &AppConfig) -> Resp {
    match (request.method(), request.url()) {
        (Method::Get, "/") => html(INDEX_HTML),
        (Method::Get, "/accounts") => handle_accounts(config),
        (Method::Post, "/launch") => handle_launch(request, zed_binary, config),
        (Method::Post, "/cleanup") => handle_cleanup(),
        _ => json(404, serde_json::json!({ "error": "not found" })),
    }
}

/// Exposes the configured impersonation accounts so the UI can render them.
fn handle_accounts(config: &AppConfig) -> Resp {
    let accounts: Vec<Value> = config
        .accounts
        .iter()
        .map(|account| {
            serde_json::json!({
                "username": account.username,
                "label": account.label.clone().unwrap_or_else(|| account.username.clone()),
            })
        })
        .collect();
    json(
        200,
        serde_json::json!({
            "enabled": config.impersonation_enabled(),
            "accounts": accounts,
        }),
    )
}

fn handle_launch(request: &mut Request, zed_binary: &Path, config: &AppConfig) -> Resp {
    let mut body = String::new();
    if let Err(err) = request.as_reader().read_to_string(&mut body) {
        return json(
            400,
            serde_json::json!({ "error": format!("could not read body: {err}") }),
        );
    }

    let payload: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(err) => {
            return json(
                400,
                serde_json::json!({ "error": format!("invalid JSON: {err}") }),
            );
        }
    };

    // A request is either a static state or an impersonation by username.
    let result = if let Some(username) = payload.get("impersonate").and_then(Value::as_str) {
        launch_impersonation(username, zed_binary, config)
    } else if let Some(state_id) = payload.get("state").and_then(Value::as_str) {
        match SimState::from_id(state_id) {
            Some(state) => launch_static(state, zed_binary),
            None => Err(anyhow::anyhow!("unknown state: {state_id:?}")),
        }
    } else {
        Err(anyhow::anyhow!(
            "request must include `state` or `impersonate`"
        ))
    };

    match result {
        Ok(profile_dir) => {
            println!("Launched -> {profile_dir}");
            json(
                200,
                serde_json::json!({ "status": "launched", "profile": profile_dir }),
            )
        }
        Err(err) => {
            eprintln!("launch failed: {err:#}");
            json(400, serde_json::json!({ "error": format!("{err:#}") }))
        }
    }
}

/// Launches a static state in a fresh profile. Both static states isolate
/// credentials via a unique `credentials_url`, so a sign-in performed while
/// exploring can never touch the user's real saved login.
fn launch_static(state: SimState, zed_binary: &Path) -> Result<String> {
    let profile = Profile::create()?;

    let mut settings = Map::new();
    let _ = state; // both states share the same isolation today.
    settings.insert(
        "credentials_url".to_string(),
        Value::String(format!("zed-sim://{}", profile.id)),
    );
    profile.write_settings(settings)?;

    profile.launch(zed_binary, &[])?;
    Ok(profile.dir.display().to_string())
}

/// Launches a session impersonating `username` against the configured backend.
fn launch_impersonation(username: &str, zed_binary: &Path, config: &AppConfig) -> Result<String> {
    anyhow::ensure!(
        config.impersonation_enabled(),
        "impersonation is not configured (need a token, server URL, and at least one account)",
    );
    anyhow::ensure!(
        config.find_account(username).is_some(),
        "{username:?} is not in the configured account allow-list",
    );

    let token = config
        .token
        .clone()
        .context("missing impersonation token")?;
    let server_url = config
        .server_url
        .clone()
        .context("missing impersonation server URL")?;

    let profile = Profile::create()?;
    // Impersonation bypasses the keychain entirely, so no settings are needed —
    // the backend and identity are supplied purely via environment variables.
    profile.launch(
        zed_binary,
        &[
            ("ZED_IMPERSONATE", username.to_string()),
            ("ZED_ADMIN_API_TOKEN", token),
            ("ZED_SERVER_URL", server_url),
        ],
    )?;
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

fn json(status: u16, body: Value) -> Resp {
    Response::from_string(body.to_string())
        .with_status_code(status)
        .with_header(header("Content-Type", "application/json"))
}

fn header(key: &'static str, value: &'static str) -> Header {
    Header::from_bytes(key.as_bytes(), value.as_bytes()).expect("static header is always valid")
}
