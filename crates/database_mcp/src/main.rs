//! `zed-database-mcp` — a stdio Model Context Protocol server that gives AI
//! agents read-only access to the database connections configured in Zed's
//! database viewer.
//!
//! Framing is newline-delimited JSON-RPC 2.0 over stdin/stdout (no
//! `Content-Length` headers). stdout carries only protocol messages; all
//! diagnostics go to stderr.

mod protocol;
mod tools;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};
use database_client::postgres::PostgresClient;
use database_client::{ConnectionConfig, DatabaseClient, SessionMode};
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};
use tokio::process::Command;
use tokio::runtime::Handle;

use crate::protocol::{RpcRequest, RpcResponse, handle_request, parse_error};
use crate::tools::{ClientFactory, PasswordSource, ToolHost};

/// Default when `database.mcp_max_rows` is absent from settings.
const DEFAULT_MAX_ROWS: usize = 200;
/// Default when `database.query_timeout_seconds` is absent from settings.
const DEFAULT_QUERY_TIMEOUT_SECONDS: u64 = 30;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = load_settings();
    let connections = parse_connections(&settings);
    let max_rows = parse_max_rows(&settings);
    let statement_timeout = parse_query_timeout(&settings);

    eprintln!(
        "zed-database-mcp: {} connection(s) configured, max_rows={max_rows}, statement_timeout={}s",
        connections.len(),
        statement_timeout.as_secs()
    );

    let client_factory: ClientFactory =
        Box::new(move |config, database| build_client(config, database, statement_timeout));
    let password_source: PasswordSource = Box::new(resolve_password);

    let mut host = ToolHost::new(connections, max_rows, client_factory, password_source);

    run_stdio_loop(&mut host).await
}

/// The read/dispatch/write loop over newline-delimited JSON-RPC on stdio.
async fn run_stdio_loop(host: &mut ToolHost) -> Result<()> {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = lines.next_line().await.context("reading stdin")? {
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<RpcRequest>(&line) {
            Ok(request) => handle_request(request, host).await,
            Err(error) => {
                eprintln!("zed-database-mcp: failed to parse request: {error}");
                Some(parse_error(format!("parse error: {error}")))
            }
        };
        if let Some(response) = response {
            write_response(&mut stdout, &response).await?;
        }
    }
    Ok(())
}

async fn write_response(stdout: &mut tokio::io::Stdout, response: &RpcResponse) -> Result<()> {
    let mut serialized = serde_json::to_string(response).context("serializing response")?;
    serialized.push('\n');
    stdout
        .write_all(serialized.as_bytes())
        .await
        .context("writing response")?;
    stdout.flush().await.context("flushing stdout")?;
    Ok(())
}

/// Reads and parses `settings.json` once at startup. A missing or unparseable
/// file yields an empty object so the server still starts (with no
/// connections); the tools then return clear errors.
fn load_settings() -> serde_json::Value {
    let path = paths::settings_file();
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!(
                "zed-database-mcp: could not read settings at {}: {error}",
                path.display()
            );
            return serde_json::Value::Object(Default::default());
        }
    };
    // Zed's settings.json is JSONC (comments, trailing commas), so use the
    // lenient parser that the settings crate uses.
    match serde_json_lenient::from_str::<serde_json::Value>(&contents) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("zed-database-mcp: could not parse settings: {error}");
            serde_json::Value::Object(Default::default())
        }
    }
}

/// Extracts the configured connections from the `database.connections` array.
/// An absent section yields an empty list.
fn parse_connections(settings: &serde_json::Value) -> Vec<ConnectionConfig> {
    let Some(connections) = settings
        .get("database")
        .and_then(|database| database.get("connections"))
        .and_then(|connections| connections.as_array())
    else {
        return Vec::new();
    };

    connections
        .iter()
        .filter_map(|connection| {
            Some(ConnectionConfig {
                name: connection.get("name")?.as_str()?.to_string(),
                host: connection.get("host")?.as_str()?.to_string(),
                port: u16::try_from(connection.get("port")?.as_u64()?).ok()?,
                database: connection.get("database")?.as_str()?.to_string(),
                user: connection.get("user")?.as_str()?.to_string(),
            })
        })
        .collect()
}

fn parse_max_rows(settings: &serde_json::Value) -> usize {
    settings
        .get("database")
        .and_then(|database| database.get("mcp_max_rows"))
        .and_then(|value| value.as_u64())
        .map(|value| value as usize)
        .unwrap_or(DEFAULT_MAX_ROWS)
}

fn parse_query_timeout(settings: &serde_json::Value) -> Duration {
    let seconds = settings
        .get("database")
        .and_then(|database| database.get("query_timeout_seconds"))
        .and_then(|value| value.as_u64())
        .unwrap_or(DEFAULT_QUERY_TIMEOUT_SECONDS);
    Duration::from_secs(seconds)
}

/// Builds a read-only [`PostgresClient`]. MCP sessions must never write, so the
/// mode is hard-coded to [`SessionMode::ReadOnly`].
fn build_client(
    config: &ConnectionConfig,
    _database: &str,
    statement_timeout: Duration,
) -> Arc<dyn DatabaseClient> {
    // The password is resolved separately by the host's `password_source`; the
    // client only needs it when it actually connects, so we resolve it here as
    // well. A failure to resolve is surfaced by the host before we reach this
    // point, so a lookup failure here degrades to an empty password (the
    // connection then fails with a clear libpq error at call time).
    let password = resolve_password(config).unwrap_or_default();
    Arc::new(PostgresClient::new(
        config.clone(),
        password,
        statement_timeout,
        SessionMode::ReadOnly,
    ))
}

/// Resolves a connection's password from the macOS keychain via `security`.
/// The password is never logged. A non-zero exit means no saved credential.
///
/// This is a synchronous function (it is called from the sync `ToolHost`
/// factory/password closures), so it bridges to the async `tokio::process`
/// runner via `block_in_place` on the current multi-threaded runtime.
fn resolve_password(config: &ConnectionConfig) -> Result<String> {
    let service = format!("zed-database://{}", config.name);
    let output = tokio::task::block_in_place(|| {
        Handle::current().block_on(async {
            Command::new("/usr/bin/security")
                .args(["find-internet-password", "-s", &service, "-w"])
                .output()
                .await
                .context("invoking /usr/bin/security")
        })
    })?;

    if !output.status.success() {
        bail!("no saved password for connection {}", config.name);
    }

    let password = String::from_utf8(output.stdout)
        .map_err(|_| anyhow!("keychain returned a non-UTF-8 password"))?;
    // `security -w` appends a trailing newline.
    Ok(password.trim_end_matches(['\n', '\r']).to_string())
}
