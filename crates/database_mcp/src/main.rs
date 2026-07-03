//! `zed-database-mcp` — a stdio Model Context Protocol server that gives AI
//! agents read-only access to the database connections configured in Zed's
//! database viewer.
//!
//! Framing is newline-delimited JSON-RPC 2.0 over stdin/stdout (no
//! `Content-Length` headers). stdout carries only protocol messages; all
//! diagnostics go to stderr.

mod protocol;
mod token_store;
mod tools;
mod write_sql;

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};
use database_client::postgres::PostgresClient;
use database_client::{ConnectionConfig, DatabaseClient, SessionMode};
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};
use tokio::process::Command;
use tokio::runtime::Handle;

use crate::protocol::{RpcRequest, RpcResponse, handle_request, parse_error};
use crate::token_store::TokenStore;
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

    let write_allowed = parse_write_allowed(&settings);

    let client_factory: ClientFactory = Box::new(move |config, database, password| {
        build_client(
            config,
            database,
            password,
            statement_timeout,
            SessionMode::ReadOnly,
        )
    });
    let write_client_factory: ClientFactory = Box::new(move |config, database, password| {
        build_client(
            config,
            database,
            password,
            statement_timeout,
            SessionMode::ReadWrite,
        )
    });
    let password_source: PasswordSource = Box::new(resolve_password);
    let tokens = TokenStore::new(Duration::from_secs(300));

    let mut host = ToolHost::new(
        connections,
        max_rows,
        client_factory,
        write_client_factory,
        password_source,
        write_allowed,
        tokens,
    );

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

/// Extracts the names of connections whose `allow_mcp_writes` setting is
/// `true`. The `propose_write`/`apply_write` tools consult this to decide
/// whether they may preview/commit DML against a given connection; every
/// other connection stays read-only. An absent `database.connections` section
/// yields an empty set.
fn parse_write_allowed(settings: &serde_json::Value) -> std::collections::HashSet<String> {
    let Some(connections) = settings
        .get("database")
        .and_then(|database| database.get("connections"))
        .and_then(|connections| connections.as_array())
    else {
        return std::collections::HashSet::new();
    };
    connections
        .iter()
        .filter(|connection| {
            connection
                .get("allow_mcp_writes")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .filter_map(|connection| connection.get("name")?.as_str().map(str::to_string))
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

/// Builds a [`PostgresClient`] targeting `database` in the given `mode`.
/// `run_query`/`list_*`/`describe_table` always pass [`SessionMode::ReadOnly`]
/// (via `client_factory`); only the dedicated `write_client_factory` used by
/// `propose_write`/`apply_write` passes [`SessionMode::ReadWrite`]. The two
/// factories are never interchanged, so a cached read-only client can never be
/// used to commit a write.
///
/// The password is resolved once by the host and passed in here; we never
/// re-resolve it (that would query the keychain twice and could silently swallow
/// a second-lookup failure). The connection's initial database is overridden
/// with the requested `database` so connection-level operations that consult
/// `config.database` target the database the agent asked for.
fn build_client(
    config: &ConnectionConfig,
    database: &str,
    password: &str,
    statement_timeout: Duration,
    mode: SessionMode,
) -> Arc<dyn DatabaseClient> {
    let mut config = config.clone();
    database.clone_into(&mut config.database);
    Arc::new(PostgresClient::new(
        config,
        password.to_string(),
        statement_timeout,
        mode,
    ))
}

/// The credentials URL a connection's password is stored under. This must match
/// the URL the UI writes with (`database_ui::connection_store::credentials_url`).
fn credentials_url(connection_name: &str) -> String {
    format!("zed-database://{connection_name}")
}

/// Resolves a connection's password. The password is never logged.
///
/// Passwords can live in two backends depending on how Zed was built. Release
/// builds store them in the macOS keychain; Dev builds (this fork's default) use
/// `zed_credentials_provider`'s `DevelopmentCredentialsProvider`, a plaintext
/// JSON file at `paths::config_dir()/development_credentials`. We try the
/// keychain first, then fall back to the dev file, so a password saved by a
/// locally-run Zed UI is visible to the MCP regardless of channel.
fn resolve_password(config: &ConnectionConfig) -> Result<String> {
    let url = credentials_url(&config.name);

    match resolve_password_from_keychain(&url) {
        Ok(Some(password)) => return Ok(password),
        Ok(None) => {}
        Err(error) => {
            // A keychain error (e.g. the CLI is missing) should not prevent the
            // dev-file fallback, but is worth a diagnostic on stderr.
            eprintln!("zed-database-mcp: keychain lookup failed: {error:#}");
        }
    }

    let dev_path = paths::config_dir().join("development_credentials");
    match resolve_password_from_dev_credentials(&dev_path, &url) {
        Ok(Some(password)) => return Ok(password),
        Ok(None) => {}
        Err(error) => {
            eprintln!("zed-database-mcp: development_credentials lookup failed: {error:#}");
        }
    }

    bail!("no saved password for connection {}", config.name);
}

/// Looks up a password in the macOS keychain via `security find-internet-password`.
/// Returns `Ok(None)` when no credential exists (non-zero exit).
///
/// This is a synchronous function (it is called from the sync `ToolHost`
/// factory/password closures), so it bridges to the async `tokio::process`
/// runner via `block_in_place` on the current multi-threaded runtime.
fn resolve_password_from_keychain(url: &str) -> Result<Option<String>> {
    let output = tokio::task::block_in_place(|| {
        Handle::current().block_on(async {
            Command::new("/usr/bin/security")
                .args(["find-internet-password", "-s", url, "-w"])
                .output()
                .await
                .context("invoking /usr/bin/security")
        })
    })?;

    if !output.status.success() {
        return Ok(None);
    }

    let password = String::from_utf8(output.stdout)
        .map_err(|_| anyhow!("keychain returned a non-UTF-8 password"))?;
    // `security -w` appends a trailing newline.
    Ok(Some(password.trim_end_matches(['\n', '\r']).to_string()))
}

/// Looks up a password in the `development_credentials` JSON file written by
/// `zed_credentials_provider::DevelopmentCredentialsProvider`. The file maps
/// `url -> (username, password_bytes)`. Returns `Ok(None)` when the file is
/// absent or has no entry for `url`.
fn resolve_password_from_dev_credentials(
    path: &std::path::Path,
    url: &str,
) -> Result<Option<String>> {
    let json = match std::fs::read(path) {
        Ok(json) => json,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(anyhow::Error::from(error).context("reading development_credentials"));
        }
    };
    let credentials: std::collections::HashMap<String, (String, Vec<u8>)> =
        serde_json::from_slice(&json).context("parsing development_credentials")?;
    let Some((_username, password_bytes)) = credentials.get(url) else {
        return Ok(None);
    };
    let password = String::from_utf8(password_bytes.clone())
        .map_err(|_| anyhow!("development_credentials stored a non-UTF-8 password"))?;
    Ok(Some(password))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Write as _;

    fn write_dev_credentials(entries: &[(&str, &str, &[u8])]) -> tempfile::NamedTempFile {
        let map: HashMap<String, (String, Vec<u8>)> = entries
            .iter()
            .map(|(url, user, password)| (url.to_string(), (user.to_string(), password.to_vec())))
            .collect();
        let json = serde_json::to_vec(&map).expect("serialize dev credentials");
        let mut file = tempfile::NamedTempFile::new().expect("create temp file");
        file.write_all(&json).expect("write dev credentials");
        file.flush().expect("flush dev credentials");
        file
    }

    #[test]
    fn dev_credentials_lookup_finds_password_by_url() {
        let url = credentials_url("local-shop");
        let file = write_dev_credentials(&[
            (&url, "postgres", b"hunter2"),
            ("zed-database://other", "postgres", b"nope"),
        ]);
        let password = resolve_password_from_dev_credentials(file.path(), &url)
            .expect("lookup succeeds")
            .expect("password present");
        assert_eq!(password, "hunter2");
    }

    #[test]
    fn dev_credentials_lookup_missing_entry_returns_none() {
        let file = write_dev_credentials(&[("zed-database://other", "postgres", b"nope")]);
        let result =
            resolve_password_from_dev_credentials(file.path(), &credentials_url("local-shop"))
                .expect("lookup succeeds");
        assert!(result.is_none());
    }

    #[test]
    fn dev_credentials_lookup_absent_file_returns_none() {
        let missing = std::env::temp_dir().join("zed-database-mcp-nonexistent-credentials");
        // Ensure the path really does not exist.
        let _ = std::fs::remove_file(&missing);
        let result =
            resolve_password_from_dev_credentials(&missing, &credentials_url("local-shop"))
                .expect("absent file is not an error");
        assert!(result.is_none());
    }

    #[test]
    fn dev_credentials_lookup_rejects_non_utf8_password() {
        let url = credentials_url("local-shop");
        let file = write_dev_credentials(&[(&url, "postgres", &[0xff, 0xfe])]);
        let error = resolve_password_from_dev_credentials(file.path(), &url)
            .expect_err("non-utf8 password is an error");
        assert!(format!("{error:#}").contains("non-UTF-8"));
    }

    #[test]
    fn credentials_url_matches_ui_scheme() {
        // Must stay byte-for-byte identical to
        // `database_ui::connection_store::credentials_url`.
        assert_eq!(credentials_url("local-shop"), "zed-database://local-shop");
    }

    #[test]
    fn parse_write_allowed_collects_only_flagged_connections() {
        let settings = serde_json::json!({
            "database": { "connections": [
                { "name": "prod", "host": "h", "port": 5432, "database": "d", "user": "u" },
                { "name": "dev", "host": "h", "port": 5432, "database": "d", "user": "u", "allow_mcp_writes": true },
                { "name": "stage", "host": "h", "port": 5432, "database": "d", "user": "u", "allow_mcp_writes": false }
            ]}
        });
        let allowed = parse_write_allowed(&settings);
        assert!(allowed.contains("dev"));
        assert!(!allowed.contains("prod"));
        assert!(!allowed.contains("stage"));
        assert_eq!(allowed.len(), 1);
    }

    #[test]
    fn parse_write_allowed_empty_without_database_section() {
        assert!(parse_write_allowed(&serde_json::json!({})).is_empty());
    }
}
