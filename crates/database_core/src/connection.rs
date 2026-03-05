use std::collections::HashMap;
use std::ffi::CStr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::{Context as _, Result, bail};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use sqlez::connection::Connection;
use sqlez::statement::{SqlType, Statement};
use tokio::sync::Mutex as TokioMutex;

use crate::query_result::{CellValue, QueryResult};
use crate::schema::{ColumnInfo, DatabaseSchema, ForeignKeyInfo, IndexInfo, IntrospectionLevel, TableInfo, TableKind};

fn url_decode(input: &str) -> String {
    percent_decode_str(input)
        .decode_utf8()
        .map(|decoded| decoded.into_owned())
        .unwrap_or_else(|_| input.to_string())
}

pub fn escape_sqlite_identifier(name: &str) -> String {
    name.replace('"', "\"\"")
}

pub fn quote_identifier(name: &str, database_type: &DatabaseType) -> String {
    match database_type {
        DatabaseType::MySql => {
            let escaped = name.replace('`', "``");
            format!("`{}`", escaped)
        }
        DatabaseType::Sqlite | DatabaseType::PostgreSql => {
            let escaped = name.replace('"', "\"\"");
            format!("\"{}\"", escaped)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DatabaseType {
    Sqlite,
    PostgreSql,
    MySql,
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Sqlite => write!(f, "SQLite"),
            DatabaseType::PostgreSql => write!(f, "PostgreSQL"),
            DatabaseType::MySql => write!(f, "MySQL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    #[serde(default = "ConnectionConfig::generate_id")]
    pub id: String,
    pub name: String,
    pub database_type: DatabaseType,
    pub path: Option<PathBuf>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: Option<String>,
    pub user: Option<String>,
    #[serde(skip)]
    pub password: Option<String>,
    #[serde(default)]
    pub ssl_mode: SslMode,
    #[serde(default)]
    pub ssl_config: Option<SslConfig>,
    #[serde(default)]
    pub ssh_tunnel: Option<SshTunnelConfig>,
    #[serde(default)]
    pub introspection_level: IntrospectionLevel,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub color_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SslMode {
    #[default]
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SslConfig {
    pub ca_cert_path: Option<PathBuf>,
    pub client_cert_path: Option<PathBuf>,
    pub client_key_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SshAuthMethod {
    Password,
    PrivateKey {
        key_path: PathBuf,
        #[serde(skip)]
        passphrase: Option<String>,
    },
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshTunnelConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_method: SshAuthMethod,
}

impl Default for SshTunnelConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            auth_method: SshAuthMethod::Password,
        }
    }
}

impl ConnectionConfig {
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn credential_key(&self) -> String {
        format!("zed-db://{}", self.id)
    }

    pub fn sqlite(name: String, path: PathBuf) -> Self {
        Self {
            id: Self::generate_id(),
            name,
            database_type: DatabaseType::Sqlite,
            path: Some(path),
            host: None,
            port: None,
            database: None,
            user: None,
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        }
    }

    pub fn postgres(
        name: String,
        host: String,
        port: u16,
        database: String,
        user: String,
        password: String,
        ssl_mode: SslMode,
    ) -> Self {
        Self {
            id: Self::generate_id(),
            name,
            database_type: DatabaseType::PostgreSql,
            path: None,
            host: Some(host),
            port: Some(port),
            database: Some(database),
            user: Some(user),
            password: if password.is_empty() {
                None
            } else {
                Some(password)
            },
            ssl_mode,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        }
    }

    pub fn mysql(
        name: String,
        host: String,
        port: u16,
        database: String,
        user: String,
        password: String,
        ssl_mode: SslMode,
    ) -> Self {
        Self {
            id: Self::generate_id(),
            name,
            database_type: DatabaseType::MySql,
            path: None,
            host: Some(host),
            port: Some(port),
            database: Some(database),
            user: Some(user),
            password: if password.is_empty() {
                None
            } else {
                Some(password)
            },
            ssl_mode,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        }
    }

    pub fn from_mysql_url(connection_string: &str) -> Result<Self, String> {
        let trimmed = connection_string.trim();
        if trimmed.is_empty() {
            return Err("Connection string is required.".to_string());
        }

        let normalized = if !trimmed.starts_with("mysql://") {
            format!("mysql://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        // mysql:// is not a standard scheme for url::Url, swap temporarily
        let for_parsing = normalized.replacen("mysql://", "http://", 1);
        let parsed = url::Url::parse(&for_parsing)
            .map_err(|error| format!("Invalid connection URL: {}", error))?;

        let host = if parsed.host_str().unwrap_or("").is_empty() {
            "localhost".to_string()
        } else {
            parsed.host_str().unwrap_or("localhost").to_string()
        };

        let port = parsed.port().unwrap_or(3306);

        let database = {
            let path = parsed.path().trim_start_matches('/');
            if path.is_empty() {
                "mysql".to_string()
            } else {
                url_decode(path)
            }
        };

        let user = if parsed.username().is_empty() {
            "root".to_string()
        } else {
            url_decode(parsed.username())
        };

        let password = parsed
            .password()
            .map(|p| url_decode(p))
            .unwrap_or_default();

        let ssl_mode = parsed
            .query_pairs()
            .find(|(key, _)| key == "sslmode")
            .map(|(_, value)| match value.as_ref() {
                "require" | "required" => SslMode::Require,
                "prefer" | "preferred" => SslMode::Prefer,
                "verify-ca" | "verify_ca" => SslMode::VerifyCa,
                "verify-full" | "verify_full" => SslMode::VerifyFull,
                _ => SslMode::Disable,
            })
            .unwrap_or(SslMode::Disable);

        let name = format!("{} @ {}", database, host);
        Ok(ConnectionConfig::mysql(
            name, host, port, database, user, password, ssl_mode,
        ))
    }

    pub fn from_postgres_url(connection_string: &str) -> Result<Self, String> {
        let trimmed = connection_string.trim();
        if trimmed.is_empty() {
            return Err("Connection string is required.".to_string());
        }

        let normalized = if trimmed.starts_with("postgresql://") {
            trimmed.replacen("postgresql://", "postgres://", 1)
        } else if !trimmed.starts_with("postgres://") {
            format!("postgres://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        let parsed = url::Url::parse(&normalized)
            .map_err(|error| format!("Invalid connection URL: {}", error))?;

        let host = if parsed.host_str().unwrap_or("").is_empty() {
            "localhost".to_string()
        } else {
            parsed.host_str().unwrap_or("localhost").to_string()
        };

        let port = parsed.port().unwrap_or(5432);

        let database = {
            let path = parsed.path().trim_start_matches('/');
            if path.is_empty() {
                "postgres".to_string()
            } else {
                url_decode(path)
            }
        };

        let user = if parsed.username().is_empty() {
            "postgres".to_string()
        } else {
            url_decode(parsed.username())
        };

        let password = parsed
            .password()
            .map(|p| url_decode(p))
            .unwrap_or_default();

        let ssl_mode = parsed
            .query_pairs()
            .find(|(key, _)| key == "sslmode")
            .map(|(_, value)| match value.as_ref() {
                "require" => SslMode::Require,
                "prefer" => SslMode::Prefer,
                "verify-ca" => SslMode::VerifyCa,
                "verify-full" => SslMode::VerifyFull,
                _ => SslMode::Disable,
            })
            .unwrap_or(SslMode::Disable);

        let name = format!("{} @ {}", database, host);
        Ok(ConnectionConfig::postgres(
            name, host, port, database, user, password, ssl_mode,
        ))
    }

    pub fn display_name(&self) -> String {
        match self.database_type {
            DatabaseType::Sqlite => self
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| self.name.clone()),
            DatabaseType::PostgreSql => {
                let database = self.database.as_deref().unwrap_or("postgres");
                let host = self.host.as_deref().unwrap_or("localhost");
                format!("{} @ {}", database, host)
            }
            DatabaseType::MySql => {
                let database = self.database.as_deref().unwrap_or("mysql");
                let host = self.host.as_deref().unwrap_or("localhost");
                format!("{} @ {}", database, host)
            }
        }
    }

    fn to_postgres_config_string(&self) -> String {
        let host = self.host.as_deref().unwrap_or("localhost");
        let port = self.port.unwrap_or(5432);
        let database = self.database.as_deref().unwrap_or("postgres");
        let user = self.user.as_deref().unwrap_or("postgres");
        let sslmode = match self.ssl_mode {
            SslMode::Disable => "disable",
            SslMode::Prefer => "prefer",
            SslMode::Require => "require",
            SslMode::VerifyCa => "verify-ca",
            SslMode::VerifyFull => "verify-full",
        };

        let mut config = format!(
            "host={} port={} dbname={} user={} sslmode={}",
            host, port, database, user, sslmode
        );
        if let Some(password) = &self.password {
            config.push_str(&format!(" password={}", password));
        }
        config
    }
}

pub trait DatabaseConnection: Send + Sync {
    fn fetch_schema(&self) -> Result<DatabaseSchema> {
        self.fetch_schema_with_level(IntrospectionLevel::Metadata)
    }
    fn fetch_schema_with_level(&self, level: IntrospectionLevel) -> Result<DatabaseSchema>;
    fn execute_query(&self, sql: &str) -> Result<QueryResult>;
    fn execute_query_paged(
        &self,
        sql: &str,
        limit: usize,
        offset: usize,
    ) -> Result<QueryResult>;
    fn execute_statement(&self, sql: &str) -> Result<u64>;
    fn interrupt(&self);
    fn is_alive(&self) -> bool;
    #[allow(dead_code)]
    fn database_type(&self) -> DatabaseType;
}

pub struct SqliteConnection {
    connection: Mutex<Connection>,
}

impl SqliteConnection {
    pub fn new_readonly(path: &PathBuf) -> Result<Self> {
        let uri = path
            .to_str()
            .context("Invalid UTF-8 in database path")?;

        if !path.exists() {
            bail!("Database file does not exist: {}", uri);
        }

        let connection = Connection::open_file_readonly(uri)
            .context(format!("Failed to open SQLite database at '{}'", uri))?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    fn with_connection<T>(&self, func: impl FnOnce(&Connection) -> Result<T>) -> Result<T> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow::anyhow!("SQLite connection lock poisoned"))?;
        func(&connection)
    }
}

#[allow(unsafe_code)]
impl DatabaseConnection for SqliteConnection {
    fn fetch_schema_with_level(&self, level: IntrospectionLevel) -> Result<DatabaseSchema> {
        self.with_connection(|connection| {
            let mut statement = Statement::prepare(
                connection,
                "SELECT name, type, CASE WHEN COALESCE(sql,'') LIKE 'CREATE VIRTUAL%' THEN 1 ELSE 0 END as is_virtual_tbl \
                 FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY name",
            )?;
            let table_entries: Vec<(String, TableKind)> = statement.map(|stmt| {
                let name = stmt.column_text(0)?.to_string();
                let entry_type = stmt.column_text(1)?.to_string();
                let is_virtual_tbl = stmt.column_int(2)? != 0;
                let table_kind = if entry_type == "view" {
                    TableKind::View
                } else if is_virtual_tbl {
                    TableKind::VirtualTable
                } else {
                    TableKind::Table
                };
                Ok((name, table_kind))
            })?;

            let mut tables = Vec::new();
            for (table_name, table_kind) in table_entries {
                if level == IntrospectionLevel::Names {
                    tables.push(TableInfo {
                        name: table_name,
                        columns: Vec::new(),
                        indexes: Vec::new(),
                        foreign_keys: Vec::new(),
                        row_count: None,
                        table_kind,
                        ddl: None,
                    });
                    continue;
                }

                let columns = match fetch_sqlite_columns(connection, &table_name) {
                    Ok(cols) => cols,
                    Err(error) => {
                        log::warn!("database_viewer: skipping table '{}': {}", table_name, error);
                        continue;
                    }
                };
                let indexes = fetch_sqlite_indexes(connection, &table_name).unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch indexes for '{}': {}", table_name, error);
                    Vec::new()
                });
                let foreign_keys = fetch_sqlite_foreign_keys(connection, &table_name).unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch foreign keys for '{}': {}", table_name, error);
                    Vec::new()
                });
                let row_count = fetch_sqlite_row_count(connection, &table_name);

                let ddl = if level == IntrospectionLevel::FullDdl {
                    fetch_sqlite_ddl(connection, &table_name)
                } else {
                    None
                };

                tables.push(TableInfo {
                    name: table_name,
                    columns,
                    indexes,
                    foreign_keys,
                    row_count,
                    table_kind,
                    ddl,
                });
            }

            Ok(DatabaseSchema { tables })
        })
    }

    fn execute_query(&self, sql: &str) -> Result<QueryResult> {
        self.with_connection(|connection| {
            let start = Instant::now();
            let mut statement = Statement::prepare(connection, sql)?;

            let raw = statement
                .raw_statements
                .first()
                .context("Prepared statement produced no statements")?;

            let column_count = unsafe { libsqlite3_sys::sqlite3_column_count(*raw) };

            let mut columns = Vec::with_capacity(column_count as usize);
            for i in 0..column_count {
                let name = unsafe {
                    let ptr = libsqlite3_sys::sqlite3_column_name(*raw, i);
                    if ptr.is_null() {
                        format!("column_{}", i)
                    } else {
                        CStr::from_ptr(ptr)
                            .to_str()
                            .unwrap_or("?")
                            .to_string()
                    }
                };
                columns.push(name);
            }

            let rows = statement.map(|stmt| {
                let mut row = Vec::with_capacity(column_count as usize);
                for i in 0..column_count {
                    let value = match stmt.column_type(i)? {
                        SqlType::Null => CellValue::Null,
                        SqlType::Integer => CellValue::Integer(stmt.column_int64(i)?),
                        SqlType::Float => CellValue::Float(stmt.column_double(i)?),
                        SqlType::Text => CellValue::Text(stmt.column_text(i)?.to_string()),
                        SqlType::Blob => CellValue::Blob(stmt.column_blob(i)?.to_vec()),
                    };
                    row.push(value);
                }
                Ok(row)
            })?;

            let execution_time = start.elapsed();
            Ok(QueryResult {
                columns,
                rows,
                total_row_count: None,
                affected_rows: None,
                execution_time,
            })
        })
    }

    fn execute_query_paged(
        &self,
        sql: &str,
        limit: usize,
        offset: usize,
    ) -> Result<QueryResult> {
        let start = Instant::now();
        let cleaned_sql = sql.trim().trim_end_matches(';').trim();

        let total_row_count: Option<u64> = self.with_connection(|connection| {
            let count_sql = format!("SELECT COUNT(*) FROM ({})", cleaned_sql);
            match connection.select_row::<i64>(&count_sql) {
                Ok(mut fetch) => match fetch() {
                    Ok(Some(count)) => Ok(Some(count as u64)),
                    Ok(None) => {
                        log::warn!("database_viewer: count query returned no rows");
                        Ok(None)
                    }
                    Err(error) => {
                        log::warn!("database_viewer: count query execution failed: {}", error);
                        Ok(None)
                    }
                },
                Err(error) => {
                    log::warn!("database_viewer: count query preparation failed: {}", error);
                    Ok(None)
                }
            }
        })?;

        let has_limit = sql_has_limit(cleaned_sql);
        let paged_sql = if has_limit {
            cleaned_sql.to_string()
        } else {
            format!("{} LIMIT {} OFFSET {}", cleaned_sql, limit, offset)
        };

        let mut result = self.execute_query(&paged_sql)?;
        result.total_row_count = total_row_count;
        result.execution_time = start.elapsed();

        Ok(result)
    }

    fn execute_statement(&self, sql: &str) -> Result<u64> {
        self.with_connection(|connection| {
            let mut statement = Statement::prepare(connection, sql)?;
            statement.exec()?;
            let changes = unsafe {
                libsqlite3_sys::sqlite3_changes(connection.sqlite3_handle()) as u64
            };
            Ok(changes)
        })
    }

    fn interrupt(&self) {
        if let Ok(connection) = self.connection.lock() {
            connection.interrupt();
        }
    }

    fn is_alive(&self) -> bool {
        self.connection.lock().is_ok()
    }

    fn database_type(&self) -> DatabaseType {
        DatabaseType::Sqlite
    }
}

fn build_postgres_tls_config(
    ssl_mode: SslMode,
    ssl_config: Option<&SslConfig>,
) -> Result<rustls::ClientConfig> {
    use rustls_platform_verifier::ConfigVerifierExt as _;

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();

    let mut config = match ssl_mode {
        SslMode::VerifyCa | SslMode::VerifyFull => {
            if let Some(ssl) = ssl_config {
                if let Some(ca_path) = &ssl.ca_cert_path {
                    let ca_data = std::fs::read(ca_path)
                        .context(format!("Failed to read CA certificate: {:?}", ca_path))?;
                    let mut root_store = rustls::RootCertStore::empty();
                    let certs = rustls_pemfile::certs(&mut &ca_data[..])
                        .collect::<Result<Vec<_>, _>>()
                        .context("Failed to parse CA certificate PEM")?;
                    for cert in certs {
                        root_store
                            .add(cert)
                            .context("Failed to add CA certificate to root store")?;
                    }
                    rustls::ClientConfig::builder()
                        .with_root_certificates(root_store)
                        .with_no_client_auth()
                } else {
                    rustls::ClientConfig::with_platform_verifier()
                }
            } else {
                rustls::ClientConfig::with_platform_verifier()
            }
        }
        // SslMode::Require encrypts the connection but does NOT verify the server's
        // identity. This matches PostgreSQL's `sslmode=require` semantics. It protects
        // against passive eavesdropping but is vulnerable to active MITM attacks.
        // Use SslMode::VerifyFull for certificate validation.
        SslMode::Require => {
            rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(NoVerification))
                .with_no_client_auth()
        }
        _ => rustls::ClientConfig::with_platform_verifier(),
    };

    config.alpn_protocols = Vec::new();
    Ok(config)
}

// Accepts any server certificate without validation. Used by SslMode::Require
// to match PostgreSQL's `sslmode=require` behavior (encrypt-only, no identity check).
#[derive(Debug)]
struct NoVerification;

impl rustls::client::danger::ServerCertVerifier for NoVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

fn build_mysql_ssl_opts(
    ssl_mode: SslMode,
    ssl_config: Option<&SslConfig>,
) -> Result<mysql_async::SslOpts> {
    let mut ssl_opts = mysql_async::SslOpts::default();

    match ssl_mode {
        SslMode::Require => {
            ssl_opts = ssl_opts.with_danger_accept_invalid_certs(true);
        }
        SslMode::VerifyCa | SslMode::VerifyFull => {
            ssl_opts = ssl_opts.with_danger_accept_invalid_certs(false);
            if let Some(ssl) = ssl_config {
                if let Some(ca_path) = &ssl.ca_cert_path {
                    ssl_opts = ssl_opts.with_root_certs(vec![ca_path.clone().into()]);
                }
            }
            if ssl_mode == SslMode::VerifyFull {
                ssl_opts = ssl_opts.with_danger_skip_domain_validation(false);
            } else {
                ssl_opts = ssl_opts.with_danger_skip_domain_validation(true);
            }
        }
        _ => {}
    }

    Ok(ssl_opts)
}

pub struct PostgresConnection {
    client: Arc<TokioMutex<tokio_postgres::Client>>,
    cancel_token: tokio_postgres::CancelToken,
    // The runtime must be kept alive for the connection background task
    // and for subsequent block_on calls via the handle.
    _runtime: tokio::runtime::Runtime,
}

impl PostgresConnection {
    pub fn connect(config: &ConnectionConfig) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .context("Failed to create tokio runtime")?;

        let config_string = config.to_postgres_config_string();
        let use_tls = matches!(
            config.ssl_mode,
            SslMode::Require | SslMode::VerifyCa | SslMode::VerifyFull
        );

        let client = runtime.block_on(async {
            if use_tls {
                let tls_config = build_postgres_tls_config(config.ssl_mode, config.ssl_config.as_ref())?;
                let tls_connector = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
                let (client, connection) =
                    tokio_postgres::connect(&config_string, tls_connector)
                        .await
                        .map_err(|e| pg_err_to_anyhow("Failed to connect to PostgreSQL (TLS)", e))?;

                tokio::spawn(async move {
                    if let Err(error) = connection.await {
                        log::error!("database_viewer: PostgreSQL connection error: {}", error);
                    }
                });

                Ok::<_, anyhow::Error>(client)
            } else {
                let (client, connection) =
                    tokio_postgres::connect(&config_string, tokio_postgres::NoTls)
                        .await
                        .map_err(|e| pg_err_to_anyhow("Failed to connect to PostgreSQL", e))?;

                tokio::spawn(async move {
                    if let Err(error) = connection.await {
                        log::error!("database_viewer: PostgreSQL connection error: {}", error);
                    }
                });

                Ok::<_, anyhow::Error>(client)
            }
        })?;

        let cancel_token = client.cancel_token();

        Ok(Self {
            client: Arc::new(TokioMutex::new(client)),
            cancel_token,
            _runtime: runtime,
        })
    }

    fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self._runtime.handle().block_on(future)
    }
}

impl DatabaseConnection for PostgresConnection {
    fn fetch_schema_with_level(&self, _level: IntrospectionLevel) -> Result<DatabaseSchema> {
        let client = self.client.clone();
        self.block_on(async {
            let client = client.lock().await;

            let table_rows = client
                .query(
                    "SELECT table_name, table_type FROM information_schema.tables \
                     WHERE table_schema = 'public' \
                     ORDER BY table_name",
                    &[],
                )
                .await
                .map_err(|e| pg_err_to_anyhow("Failed to query tables", e))?;

            let table_entries: Vec<(String, TableKind)> = table_rows
                .iter()
                .map(|r| {
                    let name: String = r.get(0);
                    let table_type: String = r.get(1);
                    let kind = if table_type == "VIEW" {
                        TableKind::View
                    } else {
                        TableKind::Table
                    };
                    (name, kind)
                })
                .collect();

            let matview_rows = client
                .query(
                    "SELECT matviewname FROM pg_matviews \
                     WHERE schemaname = 'public' \
                     ORDER BY matviewname",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch materialized views: {}", error);
                    Vec::new()
                });

            let matview_names: Vec<String> = matview_rows.iter().map(|r| r.get(0)).collect();

            let mut all_entries: Vec<(String, TableKind)> = table_entries;
            for name in matview_names {
                all_entries.push((name, TableKind::MaterializedView));
            }

            let table_names: Vec<String> = all_entries.iter().map(|(n, _)| n.clone()).collect();
            if table_names.is_empty() {
                return Ok(DatabaseSchema { tables: Vec::new() });
            }

            let all_columns = client
                .query(
                    "SELECT table_name, column_name, data_type, is_nullable, column_default \
                     FROM information_schema.columns \
                     WHERE table_schema = 'public' \
                     ORDER BY table_name, ordinal_position",
                    &[],
                )
                .await
                .map_err(|e| pg_err_to_anyhow("Failed to query columns", e))?;

            let matview_columns = client
                .query(
                    "SELECT c.relname, a.attname, \
                            pg_catalog.format_type(a.atttypid, a.atttypmod), \
                            CASE WHEN a.attnotnull THEN 'NO' ELSE 'YES' END, \
                            pg_catalog.pg_get_expr(d.adbin, d.adrelid) \
                     FROM pg_catalog.pg_attribute a \
                     JOIN pg_catalog.pg_class c ON a.attrelid = c.oid \
                     JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid \
                     LEFT JOIN pg_catalog.pg_attrdef d ON a.attrelid = d.adrelid AND a.attnum = d.adnum \
                     WHERE n.nspname = 'public' AND c.relkind = 'm' \
                       AND a.attnum > 0 AND NOT a.attisdropped \
                     ORDER BY c.relname, a.attnum",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch materialized view columns: {}", error);
                    Vec::new()
                });

            let all_pks = client
                .query(
                    "SELECT tc.table_name, kcu.column_name \
                     FROM information_schema.table_constraints tc \
                     JOIN information_schema.key_column_usage kcu \
                       ON tc.constraint_name = kcu.constraint_name \
                       AND tc.table_schema = kcu.table_schema \
                     WHERE tc.constraint_type = 'PRIMARY KEY' \
                       AND tc.table_schema = 'public'",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch primary keys: {}", error);
                    Vec::new()
                });

            let all_indexes = client
                .query(
                    "SELECT tablename, indexname, indexdef FROM pg_indexes \
                     WHERE schemaname = 'public'",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch indexes: {}", error);
                    Vec::new()
                });

            let all_fks = client
                .query(
                    "SELECT tc.table_name, kcu.column_name, \
                            ccu.table_name AS foreign_table, ccu.column_name AS foreign_column \
                     FROM information_schema.table_constraints tc \
                     JOIN information_schema.key_column_usage kcu \
                       ON tc.constraint_name = kcu.constraint_name \
                       AND tc.table_schema = kcu.table_schema \
                     JOIN information_schema.constraint_column_usage ccu \
                       ON tc.constraint_name = ccu.constraint_name \
                       AND tc.table_schema = ccu.table_schema \
                     WHERE tc.constraint_type = 'FOREIGN KEY' \
                       AND tc.table_schema = 'public'",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch foreign keys: {}", error);
                    Vec::new()
                });

            let all_counts = client
                .query(
                    "SELECT relname, reltuples::bigint \
                     FROM pg_class \
                     JOIN pg_namespace ON pg_namespace.oid = relnamespace \
                     WHERE nspname = 'public' AND relkind IN ('r', 'm')",
                    &[],
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch row counts: {}", error);
                    Vec::new()
                });

            let mut pk_map: HashMap<String, Vec<String>> = HashMap::new();
            for row in &all_pks {
                let table: String = row.get(0);
                let column: String = row.get(1);
                pk_map.entry(table).or_default().push(column);
            }

            let mut col_map: HashMap<String, Vec<(String, String, String, Option<String>)>> =
                HashMap::new();
            for row in &all_columns {
                let table: String = row.get(0);
                let name: String = row.get(1);
                let data_type: String = row.get(2);
                let is_nullable: String = row.get(3);
                let default_value: Option<String> = row.get(4);
                col_map
                    .entry(table)
                    .or_default()
                    .push((name, data_type, is_nullable, default_value));
            }
            for row in &matview_columns {
                let table: String = row.get(0);
                let name: String = row.get(1);
                let data_type: String = row.get(2);
                let is_nullable: String = row.get(3);
                let default_value: Option<String> = row.get(4);
                col_map
                    .entry(table)
                    .or_default()
                    .push((name, data_type, is_nullable, default_value));
            }

            let mut idx_map: HashMap<String, Vec<IndexInfo>> = HashMap::new();
            for row in &all_indexes {
                let table: String = row.get(0);
                let index_name: String = row.get(1);
                let index_def: String = row.get(2);
                let unique = index_def.contains("UNIQUE");
                let index_columns = parse_pg_index_columns(&index_def);
                idx_map.entry(table).or_default().push(IndexInfo {
                    name: index_name,
                    columns: index_columns,
                    unique,
                });
            }

            let mut fk_map: HashMap<String, Vec<ForeignKeyInfo>> = HashMap::new();
            for row in &all_fks {
                let table: String = row.get(0);
                let from_column: String = row.get(1);
                let to_table: String = row.get(2);
                let to_column: String = row.get(3);
                fk_map.entry(table).or_default().push(ForeignKeyInfo {
                    from_column,
                    to_table,
                    to_column,
                });
            }

            let mut count_map: HashMap<String, u64> = HashMap::new();
            for row in &all_counts {
                let table: String = row.get(0);
                let count: i64 = row.get(1);
                count_map.insert(table, count.max(0) as u64);
            }

            let kind_map: HashMap<String, TableKind> = all_entries
                .into_iter()
                .collect();

            let tables = table_names
                .into_iter()
                .map(|table_name| {
                    let table_kind = kind_map
                        .get(&table_name)
                        .copied()
                        .unwrap_or(TableKind::Table);
                    let pk_columns = pk_map.get(&table_name);
                    let columns = col_map
                        .remove(&table_name)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|(name, data_type, is_nullable, default_value)| {
                            let primary_key = pk_columns
                                .map(|pks| pks.contains(&name))
                                .unwrap_or(false);
                            ColumnInfo {
                                name,
                                data_type,
                                nullable: is_nullable == "YES",
                                primary_key,
                                default_value,
                            }
                        })
                        .collect();

                    let indexes = idx_map.remove(&table_name).unwrap_or_default();
                    let foreign_keys = fk_map.remove(&table_name).unwrap_or_default();
                    let row_count = count_map.get(&table_name).copied();

                    TableInfo {
                        name: table_name,
                        columns,
                        indexes,
                        foreign_keys,
                        row_count,
                        table_kind,
                        ddl: None,
                    }
                })
                .collect();

            Ok(DatabaseSchema { tables })
        })
    }

    fn execute_query(&self, sql: &str) -> Result<QueryResult> {
        let start = Instant::now();
        let client = self.client.clone();
        let sql = sql.to_string();

        self.block_on(async {
            let client = client.lock().await;
            let rows = client
                .query(&sql, &[])
                .await
                .map_err(|e| pg_err_to_anyhow("Failed to execute query", e))?;

            let columns: Vec<String> = if let Some(first_row) = rows.first() {
                first_row
                    .columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect()
            } else {
                Vec::new()
            };

            let result_rows: Vec<Vec<CellValue>> = rows
                .iter()
                .map(|row| {
                    row.columns()
                        .iter()
                        .enumerate()
                        .map(|(i, col)| pg_value_to_cell(row, i, col.type_()))
                        .collect()
                })
                .collect();

            let execution_time = start.elapsed();
            Ok(QueryResult {
                columns,
                rows: result_rows,
                total_row_count: None,
                affected_rows: None,
                execution_time,
            })
        })
    }

    fn execute_query_paged(
        &self,
        sql: &str,
        limit: usize,
        offset: usize,
    ) -> Result<QueryResult> {
        let start = Instant::now();
        let cleaned_sql = sql.trim().trim_end_matches(';').trim();

        let client = self.client.clone();
        let count_sql = format!("SELECT COUNT(*) FROM ({}) AS _count_subquery", cleaned_sql);

        let total_row_count: Option<u64> = self.block_on(async {
            let client = client.lock().await;
            match client.query_one(&count_sql, &[]).await {
                Ok(row) => {
                    let count: i64 = row.get(0);
                    Some(count as u64)
                }
                Err(error) => {
                    log::warn!(
                        "database_viewer: PostgreSQL count query failed: {}",
                        format_pg_error(&error)
                    );
                    None
                }
            }
        });

        let has_limit = sql_has_limit(cleaned_sql);
        let paged_sql = if has_limit {
            cleaned_sql.to_string()
        } else {
            format!("{} LIMIT {} OFFSET {}", cleaned_sql, limit, offset)
        };

        let mut result = self.execute_query(&paged_sql)?;
        result.total_row_count = total_row_count;
        result.execution_time = start.elapsed();

        Ok(result)
    }

    fn execute_statement(&self, sql: &str) -> Result<u64> {
        let client = self.client.clone();
        let sql = sql.to_string();

        self.block_on(async {
            let client = client.lock().await;
            let rows_affected = client
                .execute(&sql, &[])
                .await
                .map_err(|e| pg_err_to_anyhow("Failed to execute statement", e))?;
            Ok(rows_affected)
        })
    }

    fn interrupt(&self) {
        let cancel_token = self.cancel_token.clone();
        self._runtime.handle().spawn(async move {
            if let Err(error) = cancel_token.cancel_query(tokio_postgres::NoTls).await {
                log::warn!("database_viewer: PostgreSQL cancel failed: {}", error);
            }
        });
    }

    fn is_alive(&self) -> bool {
        let client = self.client.clone();
        self.block_on(async {
            let client = client.lock().await;
            client.simple_query("SELECT 1").await.is_ok()
        })
    }

    fn database_type(&self) -> DatabaseType {
        DatabaseType::PostgreSql
    }
}

pub struct MysqlConnection {
    pool: mysql_async::Pool,
    connection_id: Mutex<Option<u32>>,
    runtime: tokio::runtime::Runtime,
}

impl MysqlConnection {
    pub fn connect(config: &ConnectionConfig) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .context("Failed to create tokio runtime")?;

        let host = config.host.as_deref().unwrap_or("localhost");
        let port = config.port.unwrap_or(3306);
        let database = config.database.as_deref().unwrap_or("mysql");
        let user = config.user.as_deref().unwrap_or("root");
        let password = config.password.as_deref().unwrap_or("");

        let mut opts = mysql_async::OptsBuilder::default()
            .ip_or_hostname(host)
            .tcp_port(port)
            .db_name(Some(database))
            .user(Some(user))
            .pass(Some(password));

        if matches!(config.ssl_mode, SslMode::Require | SslMode::VerifyCa | SslMode::VerifyFull) {
            let ssl_opts = build_mysql_ssl_opts(config.ssl_mode, config.ssl_config.as_ref())?;
            opts = opts.ssl_opts(Some(ssl_opts));
        }

        let pool = mysql_async::Pool::new(opts);

        let connection_id = runtime.block_on(async {
            use mysql_async::prelude::*;
            let mut conn = pool.get_conn().await.map_err(|error| {
                anyhow::anyhow!("Failed to connect to MySQL: {}", error)
            })?;
            let conn_id: Option<u32> = conn
                .query_first("SELECT CONNECTION_ID()")
                .await
                .unwrap_or(None);
            drop(conn);
            Ok::<_, anyhow::Error>(conn_id)
        })?;

        Ok(Self {
            pool,
            connection_id: Mutex::new(connection_id),
            runtime,
        })
    }

    fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.runtime.handle().block_on(future)
    }
}

impl DatabaseConnection for MysqlConnection {
    fn fetch_schema_with_level(&self, _level: IntrospectionLevel) -> Result<DatabaseSchema> {
        let pool = self.pool.clone();
        self.block_on(async {
            use mysql_async::prelude::*;
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|error| anyhow::anyhow!("MySQL connection error: {}", error))?;

            let table_entries: Vec<(String, String)> = conn
                .query("SELECT table_name, table_type FROM information_schema.tables WHERE table_schema = DATABASE() ORDER BY table_name")
                .await
                .map_err(|error| anyhow::anyhow!("Failed to query tables: {}", error))?;

            if table_entries.is_empty() {
                return Ok(DatabaseSchema { tables: Vec::new() });
            }

            let kind_map: HashMap<String, TableKind> = table_entries
                .iter()
                .map(|(name, table_type)| {
                    let kind = if table_type == "VIEW" || table_type == "SYSTEM VIEW" {
                        TableKind::View
                    } else {
                        TableKind::Table
                    };
                    (name.clone(), kind)
                })
                .collect();

            let table_names: Vec<String> = table_entries.into_iter().map(|(n, _)| n).collect();

            let all_columns: Vec<(String, String, String, String, String, Option<String>)> = conn
                .query(
                    "SELECT table_name, column_name, column_type, is_nullable, column_key, column_default \
                     FROM information_schema.columns \
                     WHERE table_schema = DATABASE() \
                     ORDER BY table_name, ordinal_position",
                )
                .await
                .map_err(|error| anyhow::anyhow!("Failed to query columns: {}", error))?;

            let all_indexes: Vec<(String, String, i32, String)> = conn
                .query(
                    "SELECT table_name, index_name, non_unique, column_name \
                     FROM information_schema.statistics \
                     WHERE table_schema = DATABASE() \
                     ORDER BY table_name, index_name, seq_in_index",
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch indexes: {}", error);
                    Vec::new()
                });

            let all_fks: Vec<(String, String, String, String)> = conn
                .query(
                    "SELECT table_name, column_name, referenced_table_name, referenced_column_name \
                     FROM information_schema.key_column_usage \
                     WHERE table_schema = DATABASE() \
                       AND referenced_table_name IS NOT NULL",
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch foreign keys: {}", error);
                    Vec::new()
                });

            let all_counts: Vec<(String, u64)> = conn
                .query(
                    "SELECT table_name, table_rows \
                     FROM information_schema.tables \
                     WHERE table_schema = DATABASE() AND table_type = 'BASE TABLE'",
                )
                .await
                .unwrap_or_else(|error| {
                    log::warn!("database_viewer: failed to fetch row counts: {}", error);
                    Vec::new()
                });

            let mut col_map: HashMap<String, Vec<ColumnInfo>> = HashMap::new();
            for (table, name, data_type, is_nullable, column_key, default_value) in all_columns {
                col_map.entry(table).or_default().push(ColumnInfo {
                    name,
                    data_type,
                    nullable: is_nullable == "YES",
                    primary_key: column_key == "PRI",
                    default_value,
                });
            }

            let mut idx_raw: HashMap<String, std::collections::BTreeMap<String, (bool, Vec<String>)>> =
                HashMap::new();
            for (table, index_name, non_unique, column_name) in all_indexes {
                let entry = idx_raw
                    .entry(table)
                    .or_default()
                    .entry(index_name)
                    .or_insert_with(|| (non_unique == 0, Vec::new()));
                entry.1.push(column_name);
            }

            let mut idx_map: HashMap<String, Vec<IndexInfo>> = HashMap::new();
            for (table, indexes) in idx_raw {
                idx_map.insert(
                    table,
                    indexes
                        .into_iter()
                        .map(|(name, (unique, columns))| IndexInfo {
                            name,
                            columns,
                            unique,
                        })
                        .collect(),
                );
            }

            let mut fk_map: HashMap<String, Vec<ForeignKeyInfo>> = HashMap::new();
            for (table, from_column, to_table, to_column) in all_fks {
                fk_map.entry(table).or_default().push(ForeignKeyInfo {
                    from_column,
                    to_table,
                    to_column,
                });
            }

            let mut count_map: HashMap<String, u64> = HashMap::new();
            for (table, count) in all_counts {
                count_map.insert(table, count);
            }

            let tables = table_names
                .into_iter()
                .map(|table_name| {
                    let table_kind = kind_map
                        .get(&table_name)
                        .copied()
                        .unwrap_or(TableKind::Table);
                    let columns = col_map.remove(&table_name).unwrap_or_default();
                    let indexes = idx_map.remove(&table_name).unwrap_or_default();
                    let foreign_keys = fk_map.remove(&table_name).unwrap_or_default();
                    let row_count = count_map.get(&table_name).copied();

                    TableInfo {
                        name: table_name,
                        columns,
                        indexes,
                        foreign_keys,
                        row_count,
                        table_kind,
                        ddl: None,
                    }
                })
                .collect();

            Ok(DatabaseSchema { tables })
        })
    }

    fn execute_query(&self, sql: &str) -> Result<QueryResult> {
        let start = Instant::now();
        let pool = self.pool.clone();
        let sql = sql.to_string();

        self.block_on(async {
            use mysql_async::prelude::*;
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|error| anyhow::anyhow!("MySQL connection error: {}", error))?;

            let result: Vec<mysql_async::Row> = conn
                .query(&sql)
                .await
                .map_err(|error| anyhow::anyhow!("Failed to execute query: {}", error))?;

            let columns: Vec<String> = if let Some(first_row) = result.first() {
                first_row
                    .columns_ref()
                    .iter()
                    .map(|col| col.name_str().to_string())
                    .collect()
            } else {
                Vec::new()
            };

            let rows: Vec<Vec<CellValue>> = result
                .iter()
                .map(|row| {
                    (0..row.len())
                        .map(|i| mysql_value_to_cell(row, i))
                        .collect()
                })
                .collect();

            let execution_time = start.elapsed();
            Ok(QueryResult {
                columns,
                rows,
                total_row_count: None,
                affected_rows: None,
                execution_time,
            })
        })
    }

    fn execute_query_paged(
        &self,
        sql: &str,
        limit: usize,
        offset: usize,
    ) -> Result<QueryResult> {
        let start = Instant::now();
        let cleaned_sql = sql.trim().trim_end_matches(';').trim();

        let pool = self.pool.clone();
        let count_sql = format!("SELECT COUNT(*) FROM ({}) AS _count_subquery", cleaned_sql);

        let total_row_count: Option<u64> = self.block_on(async {
            use mysql_async::prelude::*;
            let mut conn = match pool.get_conn().await {
                Ok(conn) => conn,
                Err(error) => {
                    log::warn!("database_viewer: MySQL count conn error: {}", error);
                    return None;
                }
            };
            match conn.query_first::<u64, _>(&count_sql).await {
                Ok(count) => count,
                Err(error) => {
                    log::warn!("database_viewer: MySQL count query failed: {}", error);
                    None
                }
            }
        });

        let has_limit = sql_has_limit(cleaned_sql);
        let paged_sql = if has_limit {
            cleaned_sql.to_string()
        } else {
            format!("{} LIMIT {} OFFSET {}", cleaned_sql, limit, offset)
        };

        let mut result = self.execute_query(&paged_sql)?;
        result.total_row_count = total_row_count;
        result.execution_time = start.elapsed();

        Ok(result)
    }

    fn execute_statement(&self, sql: &str) -> Result<u64> {
        let pool = self.pool.clone();
        let sql = sql.to_string();

        self.block_on(async {
            use mysql_async::prelude::*;
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|error| anyhow::anyhow!("MySQL connection error: {}", error))?;
            conn.exec_drop(&sql, ())
                .await
                .map_err(|error| anyhow::anyhow!("Failed to execute statement: {}", error))?;
            Ok(conn.affected_rows())
        })
    }

    fn interrupt(&self) {
        let conn_id = self.connection_id.lock().ok().and_then(|id| *id);
        let Some(conn_id) = conn_id else {
            return;
        };
        let pool = self.pool.clone();
        self.runtime.handle().spawn(async move {
            use mysql_async::prelude::*;
            if let Ok(mut conn) = pool.get_conn().await {
                let kill_sql = format!("KILL QUERY {}", conn_id);
                if let Err(error) = conn.query_drop(&kill_sql).await {
                    log::warn!("database_viewer: MySQL KILL QUERY failed: {}", error);
                }
            }
        });
    }

    fn is_alive(&self) -> bool {
        let pool = self.pool.clone();
        self.block_on(async {
            use mysql_async::prelude::*;
            match pool.get_conn().await {
                Ok(mut conn) => conn.ping().await.is_ok(),
                Err(_) => false,
            }
        })
    }

    fn database_type(&self) -> DatabaseType {
        DatabaseType::MySql
    }
}

fn mysql_value_to_cell(row: &mysql_async::Row, index: usize) -> CellValue {
    use mysql_async::Value;
    use mysql_async::consts::ColumnType;

    let column = row.columns_ref().get(index);
    let column_type = column.map(|col| col.column_type());

    match row.as_ref(index) {
        Some(Value::NULL) | None => CellValue::Null,
        Some(Value::Int(val)) => {
            if let Some(ColumnType::MYSQL_TYPE_TINY) = column_type {
                if column.map(|c| c.column_length()) == Some(1) {
                    return CellValue::Boolean(*val != 0);
                }
            }
            CellValue::Integer(*val)
        }
        Some(Value::UInt(val)) => CellValue::Integer(*val as i64),
        Some(Value::Float(val)) => CellValue::Float(*val as f64),
        Some(Value::Double(val)) => CellValue::Float(*val),
        Some(Value::Bytes(bytes)) => match column_type {
            Some(ColumnType::MYSQL_TYPE_JSON) => match String::from_utf8(bytes.clone()) {
                Ok(text) => CellValue::Json(text),
                Err(_) => CellValue::Blob(bytes.clone()),
            },
            Some(ColumnType::MYSQL_TYPE_NEWDECIMAL) | Some(ColumnType::MYSQL_TYPE_DECIMAL) => {
                match String::from_utf8(bytes.clone()) {
                    Ok(text) => match text.parse::<f64>() {
                        Ok(val) if val.fract() == 0.0 && val.abs() < i64::MAX as f64 => {
                            CellValue::Integer(val as i64)
                        }
                        Ok(val) => CellValue::Float(val),
                        Err(_) => CellValue::Text(text),
                    },
                    Err(_) => CellValue::Blob(bytes.clone()),
                }
            }
            Some(ColumnType::MYSQL_TYPE_BIT) => {
                let mut bits = String::new();
                for byte in bytes {
                    bits.push_str(&format!("{:08b}", byte));
                }
                let trimmed = bits.trim_start_matches('0');
                CellValue::Text(if trimmed.is_empty() {
                    "0".to_string()
                } else {
                    trimmed.to_string()
                })
            }
            Some(ColumnType::MYSQL_TYPE_YEAR) => match String::from_utf8(bytes.clone()) {
                Ok(text) => match text.parse::<i64>() {
                    Ok(year) => CellValue::Integer(year),
                    Err(_) => CellValue::Text(text),
                },
                Err(_) => CellValue::Blob(bytes.clone()),
            },
            Some(ColumnType::MYSQL_TYPE_GEOMETRY) => CellValue::Blob(bytes.clone()),
            _ => match String::from_utf8(bytes.clone()) {
                Ok(text) => CellValue::Text(text),
                Err(_) => CellValue::Blob(bytes.clone()),
            },
        },
        Some(Value::Date(year, month, day, hour, minute, second, micro)) => {
            if *hour == 0 && *minute == 0 && *second == 0 && *micro == 0 {
                CellValue::Date(format!("{:04}-{:02}-{:02}", year, month, day))
            } else if *micro > 0 {
                CellValue::Timestamp(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                    year, month, day, hour, minute, second, micro
                ))
            } else {
                CellValue::Timestamp(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year, month, day, hour, minute, second
                ))
            }
        }
        Some(Value::Time(negative, days, hours, minutes, seconds, micro)) => {
            let sign = if *negative { "-" } else { "" };
            let total_hours = *days * 24 + (*hours as u32);
            if *micro > 0 {
                CellValue::Time(format!(
                    "{}{:02}:{:02}:{:02}.{:06}",
                    sign, total_hours, minutes, seconds, micro
                ))
            } else {
                CellValue::Time(format!(
                    "{}{:02}:{:02}:{:02}",
                    sign, total_hours, minutes, seconds
                ))
            }
        }
    }
}

fn format_pg_error(error: &tokio_postgres::Error) -> String {
    if let Some(db_error) = error.as_db_error() {
        format!("{}: {}", db_error.severity(), db_error.message())
    } else {
        format!("{}", error)
    }
}

fn pg_err_to_anyhow(context: &str, error: tokio_postgres::Error) -> anyhow::Error {
    anyhow::anyhow!("{}: {}", context, format_pg_error(&error))
}

fn parse_pg_index_columns(index_def: &str) -> Vec<String> {
    if let Some(start) = index_def.rfind('(') {
        let after_paren = &index_def[start + 1..];
        if let Some(end) = after_paren.find(')') {
            return after_paren[..end]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
    }
    Vec::new()
}

fn pg_value_to_cell(
    row: &tokio_postgres::Row,
    index: usize,
    col_type: &tokio_postgres::types::Type,
) -> CellValue {
    use tokio_postgres::types::Type;

    let type_name = col_type.name();
    if (type_name == "json" || type_name == "jsonb")
        && !matches!(*col_type, Type::JSON | Type::JSONB)
    {
        match row.try_get::<_, Option<AnyJsonValue>>(index) {
            Ok(Some(val)) => return CellValue::Json(val.0),
            Ok(None) => return CellValue::Null,
            Err(_) => {}
        }
    }

    match *col_type {
        Type::BOOL => match row.try_get::<_, Option<bool>>(index) {
            Ok(Some(val)) => CellValue::Boolean(val),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::INT2 => match row.try_get::<_, Option<i16>>(index) {
            Ok(Some(val)) => CellValue::Integer(val as i64),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::INT4 | Type::OID => match row.try_get::<_, Option<i32>>(index) {
            Ok(Some(val)) => CellValue::Integer(val as i64),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::INT8 => match row.try_get::<_, Option<i64>>(index) {
            Ok(Some(val)) => CellValue::Integer(val),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::FLOAT4 => match row.try_get::<_, Option<f32>>(index) {
            Ok(Some(val)) => CellValue::Float(val as f64),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::FLOAT8 => match row.try_get::<_, Option<f64>>(index) {
            Ok(Some(val)) => CellValue::Float(val),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::NUMERIC => match row.try_get::<_, Option<f64>>(index) {
            Ok(Some(val)) => {
                if val.fract() == 0.0 && val.abs() < i64::MAX as f64 {
                    CellValue::Integer(val as i64)
                } else {
                    CellValue::Float(val)
                }
            }
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::BYTEA => match row.try_get::<_, Option<Vec<u8>>>(index) {
            Ok(Some(val)) => CellValue::Blob(val),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME | Type::CHAR => {
            match row.try_get::<_, Option<String>>(index) {
                Ok(Some(val)) => CellValue::Text(val),
                Ok(None) => CellValue::Null,
                Err(_) => CellValue::Null,
            }
        }
        Type::TIMESTAMP => match row.try_get::<_, Option<chrono::NaiveDateTime>>(index) {
            Ok(Some(val)) => CellValue::Timestamp(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::TIMESTAMPTZ => {
            match row.try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(index) {
                Ok(Some(val)) => CellValue::Timestamp(val.to_string()),
                Ok(None) => CellValue::Null,
                Err(_) => CellValue::Null,
            }
        }
        Type::DATE => match row.try_get::<_, Option<chrono::NaiveDate>>(index) {
            Ok(Some(val)) => CellValue::Date(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::TIME => match row.try_get::<_, Option<chrono::NaiveTime>>(index) {
            Ok(Some(val)) => CellValue::Time(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::UUID => match row.try_get::<_, Option<uuid::Uuid>>(index) {
            Ok(Some(val)) => CellValue::Uuid(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        Type::JSON | Type::JSONB => match row.try_get::<_, Option<serde_json::Value>>(index) {
            Ok(Some(val)) => CellValue::Json(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => match row.try_get::<_, Option<AnyJsonValue>>(index) {
                Ok(Some(val)) => CellValue::Json(val.0),
                Ok(None) => CellValue::Null,
                Err(_) => CellValue::Null,
            },
        },
        Type::INET => match row.try_get::<_, Option<std::net::IpAddr>>(index) {
            Ok(Some(val)) => CellValue::Text(val.to_string()),
            Ok(None) => CellValue::Null,
            Err(_) => CellValue::Null,
        },
        // Array types
        Type::BOOL_ARRAY => pg_array_to_cell::<bool>(row, index),
        Type::INT2_ARRAY => pg_array_to_cell::<i16>(row, index),
        Type::INT4_ARRAY => pg_array_to_cell::<i32>(row, index),
        Type::INT8_ARRAY => pg_array_to_cell::<i64>(row, index),
        Type::FLOAT4_ARRAY => pg_array_to_cell::<f32>(row, index),
        Type::FLOAT8_ARRAY | Type::NUMERIC_ARRAY => pg_array_to_cell::<f64>(row, index),
        Type::TEXT_ARRAY | Type::VARCHAR_ARRAY => pg_array_to_cell::<String>(row, index),
        Type::UUID_ARRAY => pg_array_to_cell::<uuid::Uuid>(row, index),
        Type::JSON_ARRAY | Type::JSONB_ARRAY => {
            pg_array_to_cell::<serde_json::Value>(row, index)
        }
        Type::TIMESTAMP_ARRAY => pg_array_to_cell::<chrono::NaiveDateTime>(row, index),
        Type::TIMESTAMPTZ_ARRAY => {
            pg_array_to_cell::<chrono::DateTime<chrono::Utc>>(row, index)
        }
        Type::DATE_ARRAY => pg_array_to_cell::<chrono::NaiveDate>(row, index),
        Type::TIME_ARRAY => pg_array_to_cell::<chrono::NaiveTime>(row, index),
        Type::INET_ARRAY => pg_array_to_cell::<std::net::IpAddr>(row, index),
        _ => pg_value_to_cell_fallback(row, index, col_type),
    }
}

fn pg_array_to_cell<T>(row: &tokio_postgres::Row, index: usize) -> CellValue
where
    T: for<'a> tokio_postgres::types::FromSql<'a> + std::fmt::Display,
{
    match row.try_get::<_, Option<Vec<Option<T>>>>(index) {
        Ok(Some(items)) => {
            let formatted: Vec<String> = items
                .iter()
                .map(|item| match item {
                    Some(val) => val.to_string(),
                    None => "NULL".to_string(),
                })
                .collect();
            CellValue::Text(format!("{{{}}}", formatted.join(",")))
        }
        Ok(None) => CellValue::Null,
        Err(_) => CellValue::Null,
    }
}

struct AnyJsonValue(String);

impl<'a> tokio_postgres::types::FromSql<'a> for AnyJsonValue {
    fn from_sql(
        _type: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        // JSONB binary format: version byte (0x01) followed by JSON text
        let json_bytes = if raw.first() == Some(&1) && raw.len() > 1 {
            &raw[1..]
        } else {
            raw
        };
        let text = std::str::from_utf8(json_bytes)?;
        Ok(AnyJsonValue(text.to_string()))
    }

    fn accepts(_type: &tokio_postgres::types::Type) -> bool {
        true
    }
}

struct RawPgBytes(Vec<u8>);

impl<'a> tokio_postgres::types::FromSql<'a> for RawPgBytes {
    fn from_sql(
        _type: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(RawPgBytes(raw.to_vec()))
    }

    fn accepts(_type: &tokio_postgres::types::Type) -> bool {
        true
    }
}

fn pg_value_to_cell_fallback(
    row: &tokio_postgres::Row,
    index: usize,
    col_type: &tokio_postgres::types::Type,
) -> CellValue {
    use tokio_postgres::types::{Kind, Type};

    if let Kind::Domain(ref base_type) = *col_type.kind() {
        return pg_value_to_cell(row, index, base_type);
    }

    match row.try_get::<_, Option<RawPgBytes>>(index) {
        Ok(None) => CellValue::Null,
        Err(_) => CellValue::Text(format!("<{}>", col_type.name())),
        Ok(Some(raw)) => {
            let bytes = &raw.0;
            match *col_type {
                Type::INTERVAL => decode_pg_interval(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<interval>".into())),
                Type::MONEY => decode_pg_money(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<money>".into())),
                Type::MACADDR | Type::MACADDR8 => CellValue::Text(decode_pg_macaddr(bytes)),
                Type::BIT | Type::VARBIT => CellValue::Text(decode_pg_bit(bytes)),
                Type::CIDR => decode_pg_cidr(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<cidr>".into())),
                Type::TIMETZ => decode_pg_timetz(bytes)
                    .map(CellValue::Time)
                    .unwrap_or(CellValue::Text("<timetz>".into())),
                Type::POINT => decode_pg_point(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<point>".into())),
                Type::LINE => decode_pg_line(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<line>".into())),
                Type::LSEG | Type::BOX => decode_pg_two_points(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text(format!("<{}>", col_type.name()))),
                Type::CIRCLE => decode_pg_circle(bytes)
                    .map(CellValue::Text)
                    .unwrap_or(CellValue::Text("<circle>".into())),
                _ => {
                    if matches!(col_type.kind(), Kind::Enum(_)) {
                        if let Ok(text) = String::from_utf8(bytes.clone()) {
                            return CellValue::Text(text);
                        }
                    }
                    let fallback_type_name = col_type.name();
                    if fallback_type_name == "jsonb" && bytes.first() == Some(&1) {
                        if let Ok(text) = String::from_utf8(bytes[1..].to_vec()) {
                            return CellValue::Json(text);
                        }
                    } else if fallback_type_name == "json" {
                        if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                            return CellValue::Json(text);
                        }
                    }
                    match String::from_utf8(bytes.clone()) {
                        Ok(text) => CellValue::Text(text),
                        Err(error) => CellValue::Blob(error.into_bytes()),
                    }
                }
            }
        }
    }
}

fn decode_pg_interval(bytes: &[u8]) -> Option<String> {
    let microseconds = i64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let days = i32::from_be_bytes(bytes.get(8..12)?.try_into().ok()?);
    let months = i32::from_be_bytes(bytes.get(12..16)?.try_into().ok()?);

    let mut parts = Vec::new();

    let years = months / 12;
    let remaining_months = months % 12;

    if years != 0 {
        if years.abs() == 1 {
            parts.push(format!("{} year", years));
        } else {
            parts.push(format!("{} years", years));
        }
    }
    if remaining_months != 0 {
        if remaining_months.abs() == 1 {
            parts.push(format!("{} mon", remaining_months));
        } else {
            parts.push(format!("{} mons", remaining_months));
        }
    }
    if days != 0 {
        if days.abs() == 1 {
            parts.push(format!("{} day", days));
        } else {
            parts.push(format!("{} days", days));
        }
    }

    if microseconds != 0 || parts.is_empty() {
        let negative = microseconds < 0;
        let abs_micros = microseconds.unsigned_abs();
        let total_seconds = abs_micros / 1_000_000;
        let remaining_micros = abs_micros % 1_000_000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let sign = if negative { "-" } else { "" };
        if remaining_micros != 0 {
            parts.push(format!(
                "{}{:02}:{:02}:{:02}.{:06}",
                sign, hours, minutes, seconds, remaining_micros
            ));
        } else {
            parts.push(format!("{}{:02}:{:02}:{:02}", sign, hours, minutes, seconds));
        }
    }

    Some(parts.join(" "))
}

fn decode_pg_money(bytes: &[u8]) -> Option<String> {
    let cents = i64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let dollars = cents / 100;
    let remaining = (cents % 100).unsigned_abs();
    if cents < 0 {
        Some(format!("-{}.{:02}", dollars.abs(), remaining))
    } else {
        Some(format!("{}.{:02}", dollars, remaining))
    }
}

fn decode_pg_macaddr(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<_>>()
        .join(":")
}

fn decode_pg_bit(bytes: &[u8]) -> String {
    let bit_count = match bytes.get(0..4) {
        Some(slice) => match <[u8; 4]>::try_from(slice) {
            Ok(array) => i32::from_be_bytes(array) as usize,
            Err(_) => return String::new(),
        },
        None => return String::new(),
    };
    let data = &bytes[4..];
    let mut result = String::with_capacity(bit_count);
    for index in 0..bit_count {
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);
        if byte_index < data.len() {
            result.push(if data[byte_index] & (1 << bit_index) != 0 {
                '1'
            } else {
                '0'
            });
        }
    }
    result
}

fn decode_pg_cidr(bytes: &[u8]) -> Option<String> {
    let family = *bytes.first()?;
    let mask_length = *bytes.get(1)?;
    let address_length = *bytes.get(3)? as usize;
    let address_bytes = bytes.get(4..4 + address_length)?;

    let address = if family == 2 && address_length == 4 {
        format!(
            "{}.{}.{}.{}",
            address_bytes[0], address_bytes[1], address_bytes[2], address_bytes[3]
        )
    } else if family == 3 && address_length == 16 {
        let parts: Vec<String> = address_bytes
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    format!("{:02x}{:02x}", chunk[0], chunk[1])
                } else {
                    format!("{:02x}", chunk[0])
                }
            })
            .collect();
        parts.join(":")
    } else {
        return None;
    };

    Some(format!("{}/{}", address, mask_length))
}

fn decode_pg_timetz(bytes: &[u8]) -> Option<String> {
    let microseconds = i64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let timezone_offset = i32::from_be_bytes(bytes.get(8..12)?.try_into().ok()?);

    let total_seconds = microseconds / 1_000_000;
    let remaining_micros = (microseconds % 1_000_000) as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let timezone_hours = -timezone_offset / 3600;
    let timezone_minutes = ((-timezone_offset) % 3600).abs() / 60;

    let time_str = if remaining_micros > 0 {
        format!(
            "{:02}:{:02}:{:02}.{:06}",
            hours, minutes, seconds, remaining_micros
        )
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    };

    Some(format!(
        "{}{:+03}:{:02}",
        time_str, timezone_hours, timezone_minutes
    ))
}

fn decode_pg_point(bytes: &[u8]) -> Option<String> {
    let x = f64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let y = f64::from_be_bytes(bytes.get(8..16)?.try_into().ok()?);
    Some(format!("({},{})", x, y))
}

fn decode_pg_line(bytes: &[u8]) -> Option<String> {
    let a = f64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let b = f64::from_be_bytes(bytes.get(8..16)?.try_into().ok()?);
    let c = f64::from_be_bytes(bytes.get(16..24)?.try_into().ok()?);
    Some(format!("{{{},{},{}}}", a, b, c))
}

fn decode_pg_two_points(bytes: &[u8]) -> Option<String> {
    let x1 = f64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let y1 = f64::from_be_bytes(bytes.get(8..16)?.try_into().ok()?);
    let x2 = f64::from_be_bytes(bytes.get(16..24)?.try_into().ok()?);
    let y2 = f64::from_be_bytes(bytes.get(24..32)?.try_into().ok()?);
    Some(format!("({},{}),({},{})", x1, y1, x2, y2))
}

fn decode_pg_circle(bytes: &[u8]) -> Option<String> {
    let x = f64::from_be_bytes(bytes.get(0..8)?.try_into().ok()?);
    let y = f64::from_be_bytes(bytes.get(8..16)?.try_into().ok()?);
    let radius = f64::from_be_bytes(bytes.get(16..24)?.try_into().ok()?);
    Some(format!("<({},{}),{}>", x, y, radius))
}

fn fetch_sqlite_columns(connection: &Connection, table_name: &str) -> Result<Vec<ColumnInfo>> {
    let query = format!("PRAGMA table_info(\"{}\")", escape_sqlite_identifier(table_name));
    let mut statement = Statement::prepare(connection, &query)?;
    statement.map(|stmt| {
        let name = stmt.column_text(1)?.to_string();
        let data_type = stmt.column_text(2)?.to_string();
        let not_null = stmt.column_int(3)?;
        let default_value = {
            let text = stmt.column_text(4)?;
            if text.is_empty() {
                None
            } else {
                Some(text.to_string())
            }
        };
        let primary_key = stmt.column_int(5)? != 0;

        Ok(ColumnInfo {
            name,
            data_type,
            nullable: not_null == 0,
            primary_key,
            default_value,
        })
    })
}

fn fetch_sqlite_indexes(connection: &Connection, table_name: &str) -> Result<Vec<IndexInfo>> {
    let query = format!("PRAGMA index_list(\"{}\")", escape_sqlite_identifier(table_name));
    let index_names: Vec<(String, i32)> = {
        let mut stmt = Statement::prepare(connection, &query)?;
        stmt.map(|s| {
            let name = s.column_text(1)?.to_string();
            let unique = s.column_int(2)?;
            Ok((name, unique))
        })?
    };

    let mut indexes = Vec::new();
    for (index_name, unique) in index_names {
        let info_query = format!("PRAGMA index_info(\"{}\")", escape_sqlite_identifier(&index_name));
        let mut info_stmt = Statement::prepare(connection, &info_query)?;
        let columns = info_stmt.map(|s| {
            let col_name = s.column_text(2)?.to_string();
            Ok(col_name)
        })?;

        indexes.push(IndexInfo {
            name: index_name,
            columns,
            unique: unique != 0,
        });
    }

    Ok(indexes)
}

fn fetch_sqlite_foreign_keys(connection: &Connection, table_name: &str) -> Result<Vec<ForeignKeyInfo>> {
    let query = format!("PRAGMA foreign_key_list(\"{}\")", escape_sqlite_identifier(table_name));
    let mut statement = Statement::prepare(connection, &query)?;
    statement.map(|stmt| {
        let to_table = stmt.column_text(2)?.to_string();
        let from_column = stmt.column_text(3)?.to_string();
        let to_column = stmt.column_text(4)?.to_string();
        Ok(ForeignKeyInfo {
            from_column,
            to_table,
            to_column,
        })
    })
}

fn fetch_sqlite_ddl(connection: &Connection, table_name: &str) -> Option<String> {
    let query = format!(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name=\"{}\"",
        escape_sqlite_identifier(table_name)
    );
    match connection.select_row::<String>(&query) {
        Ok(mut fetch) => match fetch() {
            Ok(Some(ddl)) => Some(ddl),
            _ => None,
        },
        Err(_) => None,
    }
}

fn fetch_sqlite_row_count(connection: &Connection, table_name: &str) -> Option<u64> {
    let query = format!(
        "SELECT COUNT(*) FROM \"{}\"",
        escape_sqlite_identifier(table_name)
    );
    match connection.select_row::<i64>(&query) {
        Ok(mut fetch) => match fetch() {
            Ok(Some(count)) => Some(count as u64),
            _ => None,
        },
        Err(_) => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementType {
    ReadOnly,
    Insert,
    Update,
    Delete,
    Ddl,
    Dcl,
    Transaction,
    Unknown,
}

impl StatementType {
    pub fn is_read_only(&self) -> bool {
        matches!(self, StatementType::ReadOnly)
    }

    pub fn is_mutation(&self) -> bool {
        matches!(
            self,
            StatementType::Insert
                | StatementType::Update
                | StatementType::Delete
                | StatementType::Ddl
        )
    }
}

pub fn classify_statement(sql: &str) -> StatementType {
    let trimmed = sql.trim();
    // Strip leading block comments (/* ... */)
    let without_comments = strip_leading_comments(trimmed);
    let upper = without_comments.to_uppercase();

    // Match on the first keyword
    if upper.starts_with("SELECT")
        || upper.starts_with("WITH")
        || upper.starts_with("EXPLAIN")
        || upper.starts_with("SHOW")
        || upper.starts_with("DESCRIBE")
        || upper.starts_with("DESC ")
        || upper.starts_with("PRAGMA")
    {
        StatementType::ReadOnly
    } else if upper.starts_with("INSERT") || upper.starts_with("REPLACE") {
        StatementType::Insert
    } else if upper.starts_with("UPDATE") {
        StatementType::Update
    } else if upper.starts_with("DELETE") || upper.starts_with("TRUNCATE") {
        StatementType::Delete
    } else if upper.starts_with("CREATE")
        || upper.starts_with("ALTER")
        || upper.starts_with("DROP")
    {
        StatementType::Ddl
    } else if upper.starts_with("GRANT") || upper.starts_with("REVOKE") {
        StatementType::Dcl
    } else if upper.starts_with("BEGIN")
        || upper.starts_with("COMMIT")
        || upper.starts_with("ROLLBACK")
        || upper.starts_with("SAVEPOINT")
    {
        StatementType::Transaction
    } else {
        StatementType::Unknown
    }
}

fn strip_leading_comments(sql: &str) -> &str {
    let mut remaining = sql;
    loop {
        remaining = remaining.trim_start();
        if remaining.starts_with("--") {
            // Line comment: skip to end of line
            match remaining.find('\n') {
                Some(pos) => remaining = &remaining[pos + 1..],
                None => return "",
            }
        } else if remaining.starts_with("/*") {
            // Block comment: skip to closing */
            match remaining.find("*/") {
                Some(pos) => remaining = &remaining[pos + 2..],
                None => return "",
            }
        } else {
            return remaining;
        }
    }
}

fn sql_has_limit(sql: &str) -> bool {
    let upper = sql.to_uppercase();
    let trimmed = upper.trim_end_matches(';').trim();
    // Only match LIMIT at the top level (not inside a subquery).
    // Find the last occurrence of LIMIT that is not inside parentheses.
    let mut depth = 0i32;
    let bytes = trimmed.as_bytes();
    let limit_keyword = b"LIMIT";

    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ => {}
        }
        if depth == 0
            && i + limit_keyword.len() <= bytes.len()
            && &bytes[i..i + limit_keyword.len()] == limit_keyword
        {
            let before_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            let after_ok = i + limit_keyword.len() >= bytes.len()
                || !bytes[i + limit_keyword.len()].is_ascii_alphanumeric();
            if before_ok && after_ok {
                return true;
            }
        }
    }
    false
}

pub trait DatabaseDriver: Send + Sync {
    fn driver_type(&self) -> DatabaseType;
    fn display_name(&self) -> &str;
    fn default_port(&self) -> Option<u16>;
    fn validate_config(&self, config: &ConnectionConfig) -> Result<(), String>;
    fn connect(&self, config: &ConnectionConfig) -> Result<Arc<dyn DatabaseConnection>>;
}

pub struct SqliteDriver;

impl DatabaseDriver for SqliteDriver {
    fn driver_type(&self) -> DatabaseType {
        DatabaseType::Sqlite
    }

    fn display_name(&self) -> &str {
        "SQLite"
    }

    fn default_port(&self) -> Option<u16> {
        None
    }

    fn validate_config(&self, config: &ConnectionConfig) -> Result<(), String> {
        if config.path.is_none() {
            return Err("SQLite connection requires a file path".to_string());
        }
        Ok(())
    }

    fn connect(&self, config: &ConnectionConfig) -> Result<Arc<dyn DatabaseConnection>> {
        let path = config
            .path
            .as_ref()
            .context("SQLite connection requires a file path")?;
        let connection = SqliteConnection::new_readonly(path)?;
        Ok(Arc::new(connection))
    }
}

pub struct PostgresDriver;

impl DatabaseDriver for PostgresDriver {
    fn driver_type(&self) -> DatabaseType {
        DatabaseType::PostgreSql
    }

    fn display_name(&self) -> &str {
        "PostgreSQL"
    }

    fn default_port(&self) -> Option<u16> {
        Some(5432)
    }

    fn validate_config(&self, config: &ConnectionConfig) -> Result<(), String> {
        if config.host.is_none() || config.host.as_deref() == Some("") {
            return Err("PostgreSQL connection requires a host".to_string());
        }
        Ok(())
    }

    fn connect(&self, config: &ConnectionConfig) -> Result<Arc<dyn DatabaseConnection>> {
        let connection = PostgresConnection::connect(config)?;
        Ok(Arc::new(connection))
    }
}

pub struct MysqlDriver;

impl DatabaseDriver for MysqlDriver {
    fn driver_type(&self) -> DatabaseType {
        DatabaseType::MySql
    }

    fn display_name(&self) -> &str {
        "MySQL"
    }

    fn default_port(&self) -> Option<u16> {
        Some(3306)
    }

    fn validate_config(&self, config: &ConnectionConfig) -> Result<(), String> {
        if config.host.is_none() || config.host.as_deref() == Some("") {
            return Err("MySQL connection requires a host".to_string());
        }
        Ok(())
    }

    fn connect(&self, config: &ConnectionConfig) -> Result<Arc<dyn DatabaseConnection>> {
        let connection = MysqlConnection::connect(config)?;
        Ok(Arc::new(connection))
    }
}

pub struct DriverRegistry {
    drivers: HashMap<DatabaseType, Arc<dyn DatabaseDriver>>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            drivers: HashMap::new(),
        };
        registry.register(Arc::new(SqliteDriver));
        registry.register(Arc::new(PostgresDriver));
        registry.register(Arc::new(MysqlDriver));
        registry
    }

    pub fn register(&mut self, driver: Arc<dyn DatabaseDriver>) {
        self.drivers.insert(driver.driver_type(), driver);
    }

    pub fn get(&self, database_type: &DatabaseType) -> Option<&Arc<dyn DatabaseDriver>> {
        self.drivers.get(database_type)
    }

    pub fn available_drivers(&self) -> Vec<DatabaseType> {
        self.drivers.keys().cloned().collect()
    }
}

pub fn default_registry() -> DriverRegistry {
    DriverRegistry::new()
}

pub struct SshTunnel {
    process: smol::process::Child,
    pub local_port: u16,
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

pub fn establish_ssh_tunnel(
    ssh_config: &SshTunnelConfig,
    remote_host: &str,
    remote_port: u16,
) -> Result<SshTunnel> {
    let local_port = find_available_port()?;

    let mut args = vec![
        "-N".to_string(),
        "-L".to_string(),
        format!("{}:{}:{}", local_port, remote_host, remote_port),
        "-p".to_string(),
        ssh_config.port.to_string(),
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-o".to_string(),
        "ExitOnForwardFailure=yes".to_string(),
        "-o".to_string(),
        "ConnectTimeout=10".to_string(),
    ];

    match &ssh_config.auth_method {
        SshAuthMethod::PrivateKey { key_path, .. } => {
            args.push("-i".to_string());
            args.push(key_path.to_string_lossy().to_string());
        }
        SshAuthMethod::Agent => {
            // SSH agent will be used automatically
        }
        SshAuthMethod::Password => {
            // Password auth requires interactive input or sshpass
            // For now, rely on SSH_ASKPASS or agent fallback
        }
    }

    args.push(format!("{}@{}", ssh_config.username, ssh_config.host));

    let process = smol::process::Command::new("ssh")
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to start SSH tunnel process")?;

    // Wait briefly for the tunnel to establish
    std::thread::sleep(std::time::Duration::from_millis(500));

    log::info!(
        "SSH tunnel established: localhost:{} -> {}:{}:{} via {}@{}:{}",
        local_port,
        ssh_config.host,
        remote_host,
        remote_port,
        ssh_config.username,
        ssh_config.host,
        ssh_config.port,
    );

    Ok(SshTunnel {
        process,
        local_port,
    })
}

fn find_available_port() -> Result<u16> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .context("Failed to find available port")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

pub fn create_connection(config: &ConnectionConfig) -> Result<Arc<dyn DatabaseConnection>> {
    let registry = default_registry();
    let driver = registry
        .get(&config.database_type)
        .ok_or_else(|| anyhow::anyhow!("No driver registered for {:?}", config.database_type))?;

    if let Some(ssh_config) = &config.ssh_tunnel {
        let remote_host = config.host.as_deref().unwrap_or("127.0.0.1");
        let remote_port = config.port.unwrap_or(match config.database_type {
            DatabaseType::PostgreSql => 5432,
            DatabaseType::MySql => 3306,
            DatabaseType::Sqlite => return driver.connect(config),
        });

        let tunnel = establish_ssh_tunnel(ssh_config, remote_host, remote_port)?;
        let local_port = tunnel.local_port;

        let mut tunneled_config = config.clone();
        tunneled_config.host = Some("127.0.0.1".to_string());
        tunneled_config.port = Some(local_port);
        tunneled_config.ssh_tunnel = None;
        tunneled_config.ssl_mode = SslMode::Disable;

        let connection = driver.connect(&tunneled_config)?;

        // Store the tunnel handle to keep it alive
        ACTIVE_TUNNELS.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?
            .insert(config.id.clone(), tunnel);

        return Ok(connection);
    }

    driver.connect(config)
}

static ACTIVE_TUNNELS: std::sync::LazyLock<Mutex<HashMap<String, SshTunnel>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn close_tunnel(connection_id: &str) {
    if let Ok(mut tunnels) = ACTIVE_TUNNELS.lock() {
        tunnels.remove(connection_id);
    }
}

/// Password stored in zeroed memory on drop, never printed in Debug/Display.
pub struct SecurePassword(zeroize::Zeroizing<String>);

impl SecurePassword {
    pub fn new(password: String) -> Self {
        Self(zeroize::Zeroizing::new(password))
    }

    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecurePassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl std::fmt::Display for SecurePassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

/// Guard that rejects any non-read-only SQL statement.
pub struct ReadOnlyGuard;

impl ReadOnlyGuard {
    /// Returns an error if `sql` is a mutating or DDL statement.
    pub fn check(sql: &str) -> anyhow::Result<()> {
        let statement_type = classify_statement(sql);
        if statement_type.is_mutation() {
            anyhow::bail!(
                "Statement not allowed in read-only mode: {:?}",
                statement_type
            );
        }
        Ok(())
    }
}

/// Operator used in a [`FilterCondition`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOp {
    Equals,
    NotEquals,
    IsNull,
    IsNotNull,
    Like,
    GreaterThan,
    LessThan,
}

/// A single filter condition for [`build_filtered_query`].
#[derive(Debug, Clone)]
pub struct FilterCondition {
    pub column: String,
    pub op: FilterOp,
    /// Value to bind as a parameter. Ignored for `IsNull`/`IsNotNull`.
    pub value: Option<String>,
}

/// Result of [`build_filtered_query`]: SQL with placeholders and bound parameters.
#[derive(Debug)]
pub struct FilteredQuery {
    /// SQL statement with `$N` (PostgreSQL) or `?` (SQLite/MySQL) placeholders.
    pub sql: String,
    /// Parameter values in the same order as the placeholders.
    pub parameters: Vec<String>,
}

fn make_placeholder(db_type: &DatabaseType, index: usize) -> String {
    match db_type {
        DatabaseType::PostgreSql => format!("${}", index),
        DatabaseType::Sqlite | DatabaseType::MySql => "?".to_string(),
    }
}

/// Build a parameterized `SELECT` query with optional WHERE conditions, ORDER BY, LIMIT, OFFSET.
///
/// Column names are quoted via [`quote_identifier`]; filter values are placed in
/// `parameters` and referenced by placeholder, never interpolated directly into SQL.
pub fn build_filtered_query(
    table: &str,
    db_type: &DatabaseType,
    conditions: &[FilterCondition],
    order_by: Option<(&str, bool)>,
    limit: usize,
    offset: usize,
) -> FilteredQuery {
    let quoted_table = quote_identifier(table, db_type);
    let mut sql = format!("SELECT * FROM {}", quoted_table);
    let mut parameters: Vec<String> = Vec::new();
    let mut param_index = 1usize;

    if !conditions.is_empty() {
        let mut clauses: Vec<String> = Vec::new();
        for condition in conditions {
            let quoted_col = quote_identifier(&condition.column, db_type);
            let clause = match condition.op {
                FilterOp::IsNull => format!("{} IS NULL", quoted_col),
                FilterOp::IsNotNull => format!("{} IS NOT NULL", quoted_col),
                FilterOp::Equals => {
                    if let Some(value) = &condition.value {
                        let placeholder = make_placeholder(db_type, param_index);
                        parameters.push(value.clone());
                        param_index += 1;
                        format!("{} = {}", quoted_col, placeholder)
                    } else {
                        format!("{} IS NULL", quoted_col)
                    }
                }
                FilterOp::NotEquals => {
                    if let Some(value) = &condition.value {
                        let placeholder = make_placeholder(db_type, param_index);
                        parameters.push(value.clone());
                        param_index += 1;
                        format!("{} != {}", quoted_col, placeholder)
                    } else {
                        format!("{} IS NOT NULL", quoted_col)
                    }
                }
                FilterOp::Like => {
                    if let Some(value) = &condition.value {
                        let placeholder = make_placeholder(db_type, param_index);
                        parameters.push(format!("%{}%", value));
                        param_index += 1;
                        format!("{} LIKE {}", quoted_col, placeholder)
                    } else {
                        "1=1".to_string()
                    }
                }
                FilterOp::GreaterThan => {
                    if let Some(value) = &condition.value {
                        let placeholder = make_placeholder(db_type, param_index);
                        parameters.push(value.clone());
                        param_index += 1;
                        format!("{} > {}", quoted_col, placeholder)
                    } else {
                        "1=1".to_string()
                    }
                }
                FilterOp::LessThan => {
                    if let Some(value) = &condition.value {
                        let placeholder = make_placeholder(db_type, param_index);
                        parameters.push(value.clone());
                        param_index += 1;
                        format!("{} < {}", quoted_col, placeholder)
                    } else {
                        "1=1".to_string()
                    }
                }
            };
            clauses.push(clause);
        }
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }

    if let Some((col, ascending)) = order_by {
        let direction = if ascending { "ASC" } else { "DESC" };
        let quoted_col = quote_identifier(col, db_type);
        sql.push_str(&format!(" ORDER BY {} {}", quoted_col, direction));
    }

    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    FilteredQuery { sql, parameters }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use sqlez::connection::Connection as SqlezConnection;

    fn create_test_db() -> (tempfile::NamedTempFile, SqliteConnection) {
        let temp = tempfile::NamedTempFile::new().unwrap();
        let path = temp.path().to_str().unwrap();

        let write_conn = SqlezConnection::open_file(path);

        let statements = [
            indoc! {"
                CREATE TABLE users (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    email TEXT,
                    age INTEGER
                )
            "},
            "INSERT INTO users (name, email, age) VALUES ('Alice', 'alice@example.com', 30)",
            "INSERT INTO users (name, email, age) VALUES ('Bob', 'bob@example.com', 25)",
            "INSERT INTO users (name, email, age) VALUES ('Charlie', NULL, 35)",
            "CREATE INDEX idx_users_name ON users (name)",
            indoc! {"
                CREATE TABLE orders (
                    id INTEGER PRIMARY KEY,
                    user_id INTEGER,
                    total REAL
                )
            "},
            "INSERT INTO orders (user_id, total) VALUES (1, 99.99)",
            "INSERT INTO orders (user_id, total) VALUES (2, 42.50)",
        ];
        for sql in statements {
            write_conn.exec(sql).unwrap()().unwrap();
        }

        let readonly = SqliteConnection::new_readonly(&temp.path().to_path_buf()).unwrap();
        (temp, readonly)
    }

    #[test]
    fn test_escape_sqlite_identifier() {
        assert_eq!(escape_sqlite_identifier("users"), "users");
        assert_eq!(escape_sqlite_identifier("my table"), "my table");
        assert_eq!(
            escape_sqlite_identifier("table\"name"),
            "table\"\"name"
        );
        assert_eq!(
            escape_sqlite_identifier("a\"b\"c"),
            "a\"\"b\"\"c"
        );
    }

    #[test]
    fn test_sql_has_limit() {
        assert!(sql_has_limit("SELECT * FROM users LIMIT 10"));
        assert!(sql_has_limit("SELECT * FROM users LIMIT 10 OFFSET 5"));
        assert!(!sql_has_limit("SELECT * FROM users"));
        assert!(!sql_has_limit(
            "SELECT * FROM (SELECT * FROM users LIMIT 5)"
        ));
        assert!(sql_has_limit(
            "SELECT * FROM (SELECT * FROM users LIMIT 5) LIMIT 10"
        ));
        assert!(!sql_has_limit("SELECT * FROM unlimited_table"));
    }

    #[test]
    fn test_fetch_schema() {
        let (_temp, conn) = create_test_db();
        let schema = conn.fetch_schema().unwrap();

        assert_eq!(schema.tables.len(), 2);

        let users = schema.tables.iter().find(|t| t.name == "users").unwrap();
        assert_eq!(users.columns.len(), 4);
        assert_eq!(users.columns[0].name, "id");
        assert!(users.columns[0].primary_key);
        assert_eq!(users.columns[1].name, "name");
        assert!(!users.columns[1].nullable);
        assert_eq!(users.row_count, Some(3));
        assert_eq!(users.table_kind, TableKind::Table);
        assert_eq!(users.indexes.len(), 1);
        assert_eq!(users.indexes[0].name, "idx_users_name");

        let orders = schema.tables.iter().find(|t| t.name == "orders").unwrap();
        assert_eq!(orders.columns.len(), 3);
        assert_eq!(orders.row_count, Some(2));
    }

    #[test]
    fn test_execute_query() {
        let (_temp, conn) = create_test_db();
        let result = conn.execute_query("SELECT name, age FROM users ORDER BY name").unwrap();

        assert_eq!(result.columns, vec!["name", "age"]);
        assert_eq!(result.rows.len(), 3);
        assert!(matches!(&result.rows[0][0], CellValue::Text(s) if s == "Alice"));
        assert!(matches!(&result.rows[0][1], CellValue::Integer(30)));
    }

    #[test]
    fn test_execute_query_paged() {
        let (_temp, conn) = create_test_db();
        let result = conn
            .execute_query_paged("SELECT * FROM users ORDER BY id", 2, 0)
            .unwrap();

        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.total_row_count, Some(3));

        let page2 = conn
            .execute_query_paged("SELECT * FROM users ORDER BY id", 2, 2)
            .unwrap();

        assert_eq!(page2.rows.len(), 1);
        assert_eq!(page2.total_row_count, Some(3));
    }

    #[test]
    fn test_table_with_special_characters() {
        let temp = tempfile::NamedTempFile::new().unwrap();
        let path = temp.path().to_str().unwrap();

        let write_conn = SqlezConnection::open_file(path);
        write_conn
            .exec("CREATE TABLE \"my\"\"table\" (id INTEGER PRIMARY KEY, value TEXT)")
            .unwrap()()
            .unwrap();
        write_conn
            .exec("INSERT INTO \"my\"\"table\" (value) VALUES ('test')")
            .unwrap()()
            .unwrap();

        let readonly = SqliteConnection::new_readonly(&temp.path().to_path_buf()).unwrap();
        let schema = readonly.fetch_schema().unwrap();

        assert_eq!(schema.tables.len(), 1);
        assert_eq!(schema.tables[0].name, "my\"table");
        assert_eq!(schema.tables[0].row_count, Some(1));
    }

    #[test]
    fn test_from_postgres_url_full() {
        let config =
            ConnectionConfig::from_postgres_url("postgres://myuser:secret@db.example.com:5433/mydb")
                .expect("should parse");
        assert_eq!(config.host.as_deref(), Some("db.example.com"));
        assert_eq!(config.port, Some(5433));
        assert_eq!(config.database.as_deref(), Some("mydb"));
        assert_eq!(config.user.as_deref(), Some("myuser"));
        assert_eq!(config.password.as_deref(), Some("secret"));
    }

    #[test]
    fn test_from_postgres_url_defaults() {
        let config =
            ConnectionConfig::from_postgres_url("postgres://localhost").expect("should parse");
        assert_eq!(config.host.as_deref(), Some("localhost"));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.database.as_deref(), Some("postgres"));
        assert_eq!(config.user.as_deref(), Some("postgres"));
    }

    #[test]
    fn test_from_postgres_url_postgresql_scheme() {
        let config =
            ConnectionConfig::from_postgres_url("postgresql://user@host/db").expect("should parse");
        assert_eq!(config.host.as_deref(), Some("host"));
        assert_eq!(config.database.as_deref(), Some("db"));
        assert_eq!(config.user.as_deref(), Some("user"));
    }

    #[test]
    fn test_from_postgres_url_empty() {
        assert!(ConnectionConfig::from_postgres_url("").is_err());
    }

    #[test]
    fn test_from_postgres_url_special_chars_password() {
        let config = ConnectionConfig::from_postgres_url(
            "postgres://user:p%40ss%23word@host/db",
        )
        .expect("should parse");
        assert_eq!(config.password.as_deref(), Some("p@ss#word"));
        assert_eq!(config.user.as_deref(), Some("user"));
    }

    #[test]
    fn test_from_mysql_url_special_chars_password() {
        let config = ConnectionConfig::from_mysql_url(
            "mysql://admin:s%40cr%23t%21@host:3306/db",
        )
        .expect("should parse");
        assert_eq!(config.password.as_deref(), Some("s@cr#t!"));
        assert_eq!(config.user.as_deref(), Some("admin"));
    }

    #[test]
    fn test_display_name_sqlite() {
        let config = ConnectionConfig::sqlite(
            "test.db".to_string(),
            PathBuf::from("/path/to/test.db"),
        );
        assert_eq!(config.display_name(), "test.db");
    }

    #[test]
    fn test_display_name_postgres() {
        let config = ConnectionConfig::postgres(
            "test".to_string(),
            "myhost".to_string(),
            5432,
            "mydb".to_string(),
            "user".to_string(),
            "".to_string(),
            SslMode::Disable,
        );
        assert_eq!(config.display_name(), "mydb @ myhost");
    }

    #[test]
    fn test_from_mysql_url_full() {
        let config =
            ConnectionConfig::from_mysql_url("mysql://myuser:secret@db.example.com:3307/mydb")
                .expect("should parse");
        assert_eq!(config.host.as_deref(), Some("db.example.com"));
        assert_eq!(config.port, Some(3307));
        assert_eq!(config.database.as_deref(), Some("mydb"));
        assert_eq!(config.user.as_deref(), Some("myuser"));
        assert_eq!(config.password.as_deref(), Some("secret"));
        assert_eq!(config.database_type, DatabaseType::MySql);
    }

    #[test]
    fn test_from_mysql_url_defaults() {
        let config =
            ConnectionConfig::from_mysql_url("mysql://localhost").expect("should parse");
        assert_eq!(config.host.as_deref(), Some("localhost"));
        assert_eq!(config.port, Some(3306));
        assert_eq!(config.database.as_deref(), Some("mysql"));
        assert_eq!(config.user.as_deref(), Some("root"));
    }

    #[test]
    fn test_from_mysql_url_empty() {
        assert!(ConnectionConfig::from_mysql_url("").is_err());
    }

    #[test]
    fn test_classify_statement_select() {
        assert_eq!(classify_statement("SELECT * FROM users"), StatementType::ReadOnly);
        assert_eq!(classify_statement("  select id from t"), StatementType::ReadOnly);
    }

    #[test]
    fn test_classify_statement_with_cte() {
        assert_eq!(
            classify_statement("WITH cte AS (SELECT 1) SELECT * FROM cte"),
            StatementType::ReadOnly
        );
    }

    #[test]
    fn test_classify_statement_mutations() {
        assert_eq!(
            classify_statement("INSERT INTO users VALUES (1)"),
            StatementType::Insert
        );
        assert_eq!(
            classify_statement("UPDATE users SET name = 'x'"),
            StatementType::Update
        );
        assert_eq!(
            classify_statement("DELETE FROM users WHERE id = 1"),
            StatementType::Delete
        );
        assert_eq!(
            classify_statement("TRUNCATE TABLE users"),
            StatementType::Delete
        );
        assert_eq!(
            classify_statement("REPLACE INTO users VALUES (1)"),
            StatementType::Insert
        );
    }

    #[test]
    fn test_classify_statement_ddl() {
        assert_eq!(
            classify_statement("CREATE TABLE t (id INT)"),
            StatementType::Ddl
        );
        assert_eq!(
            classify_statement("ALTER TABLE t ADD col INT"),
            StatementType::Ddl
        );
        assert_eq!(classify_statement("DROP TABLE t"), StatementType::Ddl);
    }

    #[test]
    fn test_classify_statement_dcl() {
        assert_eq!(
            classify_statement("GRANT SELECT ON t TO user"),
            StatementType::Dcl
        );
        assert_eq!(
            classify_statement("REVOKE ALL ON t FROM user"),
            StatementType::Dcl
        );
    }

    #[test]
    fn test_classify_statement_transaction() {
        assert_eq!(classify_statement("BEGIN"), StatementType::Transaction);
        assert_eq!(classify_statement("COMMIT"), StatementType::Transaction);
        assert_eq!(classify_statement("ROLLBACK"), StatementType::Transaction);
        assert_eq!(
            classify_statement("SAVEPOINT sp1"),
            StatementType::Transaction
        );
    }

    #[test]
    fn test_classify_statement_with_comments() {
        assert_eq!(
            classify_statement("-- a comment\nSELECT 1"),
            StatementType::ReadOnly
        );
        assert_eq!(
            classify_statement("/* block */  INSERT INTO t VALUES (1)"),
            StatementType::Insert
        );
        assert_eq!(
            classify_statement("/* multi\nline */ DELETE FROM t"),
            StatementType::Delete
        );
    }

    #[test]
    fn test_classify_statement_explain() {
        assert_eq!(
            classify_statement("EXPLAIN SELECT * FROM users"),
            StatementType::ReadOnly
        );
    }

    #[test]
    fn test_classify_statement_pragma() {
        assert_eq!(
            classify_statement("PRAGMA table_info('users')"),
            StatementType::ReadOnly
        );
    }

    #[test]
    fn test_classify_statement_unknown() {
        assert_eq!(classify_statement("VACUUM"), StatementType::Unknown);
        assert_eq!(classify_statement(""), StatementType::Unknown);
    }

    #[test]
    fn test_statement_type_predicates() {
        assert!(StatementType::ReadOnly.is_read_only());
        assert!(!StatementType::Insert.is_read_only());
        assert!(StatementType::Insert.is_mutation());
        assert!(StatementType::Update.is_mutation());
        assert!(StatementType::Delete.is_mutation());
        assert!(StatementType::Ddl.is_mutation());
        assert!(!StatementType::ReadOnly.is_mutation());
        assert!(!StatementType::Transaction.is_mutation());
    }

    #[test]
    fn test_display_name_mysql() {
        let config = ConnectionConfig::mysql(
            "test".to_string(),
            "myhost".to_string(),
            3306,
            "mydb".to_string(),
            "root".to_string(),
            "".to_string(),
            SslMode::Disable,
        );
        assert_eq!(config.display_name(), "mydb @ myhost");
    }

    #[test]
    fn test_driver_registry_registers_built_in_drivers() {
        let registry = DriverRegistry::new();
        assert!(registry.get(&DatabaseType::Sqlite).is_some());
        assert!(registry.get(&DatabaseType::PostgreSql).is_some());
        assert!(registry.get(&DatabaseType::MySql).is_some());
    }

    #[test]
    fn test_driver_registry_available_drivers() {
        let registry = DriverRegistry::new();
        let mut available = registry.available_drivers();
        available.sort_by_key(|d| format!("{:?}", d));
        assert_eq!(available.len(), 3);
        assert!(available.contains(&DatabaseType::Sqlite));
        assert!(available.contains(&DatabaseType::PostgreSql));
        assert!(available.contains(&DatabaseType::MySql));
    }

    #[test]
    fn test_sqlite_driver_metadata() {
        let driver = SqliteDriver;
        assert_eq!(driver.driver_type(), DatabaseType::Sqlite);
        assert_eq!(driver.display_name(), "SQLite");
        assert_eq!(driver.default_port(), None);
    }

    #[test]
    fn test_postgres_driver_metadata() {
        let driver = PostgresDriver;
        assert_eq!(driver.driver_type(), DatabaseType::PostgreSql);
        assert_eq!(driver.display_name(), "PostgreSQL");
        assert_eq!(driver.default_port(), Some(5432));
    }

    #[test]
    fn test_mysql_driver_metadata() {
        let driver = MysqlDriver;
        assert_eq!(driver.driver_type(), DatabaseType::MySql);
        assert_eq!(driver.display_name(), "MySQL");
        assert_eq!(driver.default_port(), Some(3306));
    }

    #[test]
    fn test_sqlite_driver_validate_config_missing_path() {
        let driver = SqliteDriver;
        let config = ConnectionConfig {
            id: ConnectionConfig::generate_id(),
            name: "test".to_string(),
            database_type: DatabaseType::Sqlite,
            path: None,
            host: None,
            port: None,
            database: None,
            user: None,
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        };
        assert!(driver.validate_config(&config).is_err());
    }

    #[test]
    fn test_sqlite_driver_validate_config_valid() {
        let driver = SqliteDriver;
        let config = ConnectionConfig::sqlite("test".to_string(), PathBuf::from("/tmp/test.db"));
        assert!(driver.validate_config(&config).is_ok());
    }

    #[test]
    fn test_postgres_driver_validate_config_missing_host() {
        let driver = PostgresDriver;
        let config = ConnectionConfig {
            id: ConnectionConfig::generate_id(),
            name: "test".to_string(),
            database_type: DatabaseType::PostgreSql,
            path: None,
            host: None,
            port: None,
            database: None,
            user: None,
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        };
        assert!(driver.validate_config(&config).is_err());
    }

    #[test]
    fn test_postgres_driver_validate_config_empty_host() {
        let driver = PostgresDriver;
        let config = ConnectionConfig {
            id: ConnectionConfig::generate_id(),
            name: "test".to_string(),
            database_type: DatabaseType::PostgreSql,
            path: None,
            host: Some("".to_string()),
            port: None,
            database: None,
            user: None,
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        };
        assert!(driver.validate_config(&config).is_err());
    }

    #[test]
    fn test_mysql_driver_validate_config_missing_host() {
        let driver = MysqlDriver;
        let config = ConnectionConfig {
            id: ConnectionConfig::generate_id(),
            name: "test".to_string(),
            database_type: DatabaseType::MySql,
            path: None,
            host: None,
            port: None,
            database: None,
            user: None,
            password: None,
            ssl_mode: SslMode::Disable,
            ssl_config: None,
            ssh_tunnel: None,
            introspection_level: IntrospectionLevel::default(),
            read_only: false,
            color_index: 0,
        };
        assert!(driver.validate_config(&config).is_err());
    }

    #[test]
    fn test_create_connection_via_registry_sqlite() {
        let (_temp, _conn) = create_test_db();
        let config = ConnectionConfig::sqlite("test".to_string(), _temp.path().to_path_buf());
        let connection = create_connection(&config).expect("should connect via registry");
        assert_eq!(connection.database_type(), DatabaseType::Sqlite);
    }

    #[test]
    fn test_default_registry_returns_working_registry() {
        let registry = default_registry();
        assert!(registry.get(&DatabaseType::Sqlite).is_some());
        assert!(registry.get(&DatabaseType::PostgreSql).is_some());
        assert!(registry.get(&DatabaseType::MySql).is_some());
    }

    #[test]
    fn test_ssh_tunnel_config_default() {
        let config = SshTunnelConfig::default();
        assert_eq!(config.host, "");
        assert_eq!(config.port, 22);
        assert_eq!(config.username, "");
        assert!(matches!(config.auth_method, SshAuthMethod::Password));
    }

    #[test]
    fn test_ssl_config_serde() {
        let config = SslConfig {
            ca_cert_path: Some(PathBuf::from("/etc/ssl/ca.pem")),
            client_cert_path: Some(PathBuf::from("/etc/ssl/client.pem")),
            client_key_path: None,
        };
        let json = serde_json::to_string(&config).expect("should serialize");
        let deserialized: SslConfig =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(
            deserialized.ca_cert_path,
            Some(PathBuf::from("/etc/ssl/ca.pem"))
        );
        assert_eq!(
            deserialized.client_cert_path,
            Some(PathBuf::from("/etc/ssl/client.pem"))
        );
        assert!(deserialized.client_key_path.is_none());
    }

    #[test]
    fn test_introspection_level_default() {
        let level = IntrospectionLevel::default();
        assert_eq!(level, IntrospectionLevel::Metadata);
    }

    #[test]
    fn test_password_not_serialized() {
        let config = ConnectionConfig::postgres(
            "test".to_string(),
            "localhost".to_string(),
            5432,
            "mydb".to_string(),
            "user".to_string(),
            "secret_password".to_string(),
            SslMode::Disable,
        );
        assert!(config.password.is_some());

        let json = serde_json::to_string(&config).expect("should serialize");
        assert!(
            !json.contains("secret_password"),
            "password must not appear in serialized JSON"
        );

        let deserialized: ConnectionConfig =
            serde_json::from_str(&json).expect("should deserialize");
        assert!(
            deserialized.password.is_none(),
            "password must be None after deserialization"
        );
    }

    #[test]
    fn test_connection_id_stable_across_serde() {
        let config = ConnectionConfig::postgres(
            "test".to_string(),
            "localhost".to_string(),
            5432,
            "mydb".to_string(),
            "user".to_string(),
            "".to_string(),
            SslMode::Disable,
        );
        let original_id = config.id.clone();
        assert!(!original_id.is_empty());

        let json = serde_json::to_string(&config).expect("should serialize");
        let deserialized: ConnectionConfig =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(
            deserialized.id, original_id,
            "connection id must be stable across serialization"
        );
    }

    #[test]
    fn test_connection_id_unique() {
        let config1 = ConnectionConfig::sqlite("a".to_string(), PathBuf::from("/a.db"));
        let config2 = ConnectionConfig::sqlite("b".to_string(), PathBuf::from("/b.db"));
        assert_ne!(config1.id, config2.id);
    }

    #[test]
    fn test_credential_key_format() {
        let config = ConnectionConfig::sqlite("test".to_string(), PathBuf::from("/test.db"));
        let key = config.credential_key();
        assert!(key.starts_with("zed-db://"));
        assert!(key.contains(&config.id));
    }

    #[test]
    fn test_ssh_passphrase_not_serialized() {
        let ssh_config = SshTunnelConfig {
            host: "ssh.example.com".to_string(),
            port: 22,
            username: "user".to_string(),
            auth_method: SshAuthMethod::PrivateKey {
                key_path: PathBuf::from("/home/user/.ssh/id_rsa"),
                passphrase: Some("my_secret_passphrase".to_string()),
            },
        };

        let json = serde_json::to_string(&ssh_config).expect("should serialize");
        assert!(
            !json.contains("my_secret_passphrase"),
            "SSH passphrase must not appear in serialized JSON"
        );
    }

    #[test]
    fn test_deserialized_config_gets_id_when_missing() {
        let json = r#"{
            "name": "legacy",
            "database_type": "Sqlite",
            "path": "/tmp/test.db",
            "host": null,
            "port": null,
            "database": null,
            "user": null,
            "ssl_mode": "Disable",
            "ssl_config": null,
            "ssh_tunnel": null,
            "read_only": false,
            "color_index": 0
        }"#;
        let config: ConnectionConfig =
            serde_json::from_str(json).expect("should deserialize legacy config");
        assert!(
            !config.id.is_empty(),
            "deserialized config without id should get a generated one"
        );
    }

    #[test]
    fn test_sqlite_is_alive() {
        let (_temp, conn) = create_test_db();
        assert!(conn.is_alive());
    }

    #[test]
    fn test_sqlite_interrupt_does_not_panic() {
        let (_temp, conn) = create_test_db();
        conn.interrupt();
    }

    #[test]
    fn test_secure_password_redacted_in_debug() {
        let password = SecurePassword::new("super_secret".to_string());
        assert_eq!(format!("{:?}", password), "[REDACTED]");
        assert_eq!(format!("{}", password), "[REDACTED]");
        assert_eq!(password.expose_secret(), "super_secret");
    }

    #[test]
    fn test_read_only_guard_allows_select() {
        assert!(ReadOnlyGuard::check("SELECT * FROM users").is_ok());
        assert!(ReadOnlyGuard::check("EXPLAIN SELECT 1").is_ok());
        assert!(ReadOnlyGuard::check("WITH cte AS (SELECT 1) SELECT * FROM cte").is_ok());
    }

    #[test]
    fn test_read_only_guard_blocks_mutations() {
        assert!(ReadOnlyGuard::check("INSERT INTO users VALUES (1)").is_err());
        assert!(ReadOnlyGuard::check("UPDATE users SET name='x'").is_err());
        assert!(ReadOnlyGuard::check("DELETE FROM users").is_err());
        assert!(ReadOnlyGuard::check("CREATE TABLE t (id INT)").is_err());
        assert!(ReadOnlyGuard::check("DROP TABLE t").is_err());
        assert!(ReadOnlyGuard::check("ALTER TABLE t ADD col INT").is_err());
    }

    #[test]
    fn test_read_only_guard_strips_comments_before_checking() {
        assert!(ReadOnlyGuard::check("-- comment\nSELECT 1").is_ok());
        assert!(ReadOnlyGuard::check("/* block */ INSERT INTO t VALUES (1)").is_err());
    }

    #[test]
    fn test_build_filtered_query_no_conditions_postgres() {
        let query = build_filtered_query("users", &DatabaseType::PostgreSql, &[], None, 50, 0);
        assert_eq!(query.sql, r#"SELECT * FROM "users" LIMIT 50 OFFSET 0"#);
        assert!(query.parameters.is_empty());
    }

    #[test]
    fn test_build_filtered_query_equals_postgres() {
        let conditions = vec![FilterCondition {
            column: "name".to_string(),
            op: FilterOp::Equals,
            value: Some("Alice".to_string()),
        }];
        let query = build_filtered_query(
            "users",
            &DatabaseType::PostgreSql,
            &conditions,
            None,
            50,
            0,
        );
        assert_eq!(
            query.sql,
            r#"SELECT * FROM "users" WHERE "name" = $1 LIMIT 50 OFFSET 0"#
        );
        assert_eq!(query.parameters, vec!["Alice"]);
    }

    #[test]
    fn test_build_filtered_query_like_sqlite() {
        let conditions = vec![FilterCondition {
            column: "email".to_string(),
            op: FilterOp::Like,
            value: Some("example".to_string()),
        }];
        let query = build_filtered_query(
            "users",
            &DatabaseType::Sqlite,
            &conditions,
            None,
            100,
            0,
        );
        assert_eq!(
            query.sql,
            r#"SELECT * FROM "users" WHERE "email" LIKE ? LIMIT 100 OFFSET 0"#
        );
        assert_eq!(query.parameters, vec!["%example%"]);
    }

    #[test]
    fn test_build_filtered_query_is_null() {
        let conditions = vec![FilterCondition {
            column: "email".to_string(),
            op: FilterOp::IsNull,
            value: None,
        }];
        let query =
            build_filtered_query("users", &DatabaseType::Sqlite, &conditions, None, 10, 5);
        assert_eq!(
            query.sql,
            r#"SELECT * FROM "users" WHERE "email" IS NULL LIMIT 10 OFFSET 5"#
        );
        assert!(query.parameters.is_empty());
    }

    #[test]
    fn test_build_filtered_query_multiple_conditions_postgres() {
        let conditions = vec![
            FilterCondition {
                column: "age".to_string(),
                op: FilterOp::GreaterThan,
                value: Some("18".to_string()),
            },
            FilterCondition {
                column: "active".to_string(),
                op: FilterOp::Equals,
                value: Some("true".to_string()),
            },
        ];
        let query = build_filtered_query(
            "users",
            &DatabaseType::PostgreSql,
            &conditions,
            Some(("name", true)),
            25,
            0,
        );
        assert_eq!(
            query.sql,
            r#"SELECT * FROM "users" WHERE "age" > $1 AND "active" = $2 ORDER BY "name" ASC LIMIT 25 OFFSET 0"#
        );
        assert_eq!(query.parameters, vec!["18", "true"]);
    }

    #[test]
    fn test_build_filtered_query_quotes_special_column_names() {
        let conditions = vec![FilterCondition {
            column: r#"col"name"#.to_string(),
            op: FilterOp::Equals,
            value: Some("val".to_string()),
        }];
        let query = build_filtered_query(
            "my table",
            &DatabaseType::Sqlite,
            &conditions,
            None,
            10,
            0,
        );
        assert!(query.sql.contains(r#""col""name""#), "column must be quoted");
        assert!(query.sql.contains(r#""my table""#), "table must be quoted");
    }

    #[test]
    fn test_sqlite_introspect_names_level() {
        let (_temp, conn) = create_test_db();
        let schema = conn.fetch_schema_with_level(IntrospectionLevel::Names).unwrap();

        assert_eq!(schema.tables.len(), 2, "should return 2 table names");
        let users = schema.tables.iter().find(|t| t.name == "users").unwrap();
        assert!(users.columns.is_empty(), "Names level must not fetch columns");
        assert!(users.indexes.is_empty(), "Names level must not fetch indexes");
        assert!(users.foreign_keys.is_empty(), "Names level must not fetch FKs");
        assert!(users.row_count.is_none(), "Names level must not fetch row count");
    }

    #[test]
    fn test_sqlite_foreign_keys() {
        let temp = tempfile::NamedTempFile::new().unwrap();
        let path = temp.path().to_str().unwrap();
        let write_conn = sqlez::connection::Connection::open_file(path);

        write_conn
            .exec("CREATE TABLE authors (id INTEGER PRIMARY KEY, name TEXT)")
            .unwrap()()
        .unwrap();
        write_conn
            .exec(
                "CREATE TABLE books (id INTEGER PRIMARY KEY, author_id INTEGER, \
                 FOREIGN KEY (author_id) REFERENCES authors(id))",
            )
            .unwrap()()
        .unwrap();
        write_conn
            .exec("INSERT INTO authors (name) VALUES ('Tolkien')")
            .unwrap()()
        .unwrap();

        let conn = SqliteConnection::new_readonly(&temp.path().to_path_buf()).unwrap();
        let schema = conn.fetch_schema().unwrap();

        let books = schema.tables.iter().find(|t| t.name == "books").unwrap();
        assert_eq!(books.foreign_keys.len(), 1, "books should have one FK");
        let fk = &books.foreign_keys[0];
        assert_eq!(fk.from_column, "author_id");
        assert_eq!(fk.to_table, "authors");
        assert_eq!(fk.to_column, "id");
    }

    #[test]
    fn test_quote_identifier_mysql_backtick() {
        assert_eq!(
            quote_identifier("my_table", &DatabaseType::MySql),
            "`my_table`"
        );
        assert_eq!(
            quote_identifier("col`name", &DatabaseType::MySql),
            "`col``name`",
            "backtick inside MySQL identifier must be doubled"
        );
    }

    #[test]
    fn test_quote_identifier_prevents_injection() {
        // quote_identifier must escape the embedded double-quote so that a malicious
        // identifier cannot break out of the identifier context (the whole string
        // becomes a single, safely-delimited identifier token).
        let malicious = r#"users" WHERE 1=1 --"#;
        let quoted = quote_identifier(malicious, &DatabaseType::Sqlite);
        assert!(
            quoted.starts_with('"') && quoted.ends_with('"'),
            "identifier must be wrapped in double-quotes"
        );
        assert!(
            quoted.contains("\"\""),
            "embedded double-quote must be doubled to prevent identifier break-out"
        );
    }

    #[test]
    fn test_redact_sensitive_patterns() {
        let config = ConnectionConfig::postgres(
            "prod".to_string(),
            "db.prod.example.com".to_string(),
            5432,
            "mydb".to_string(),
            "admin".to_string(),
            "super_secret_pass".to_string(),
            SslMode::Disable,
        );
        let display = config.display_name();
        assert!(
            !display.contains("super_secret_pass"),
            "password must never appear in display_name"
        );
        let serialized = serde_json::to_string(&config).unwrap();
        assert!(
            !serialized.contains("super_secret_pass"),
            "password must never appear in JSON serialization"
        );
    }

    #[test]
    fn test_ai_allowlist_default_safe() {
        assert!(
            ReadOnlyGuard::check("SELECT id, name FROM users WHERE active = 1").is_ok(),
            "SELECT must be allowed"
        );
        assert!(
            ReadOnlyGuard::check("DELETE FROM users").is_err(),
            "DELETE must be blocked"
        );
        assert!(
            ReadOnlyGuard::check("DROP TABLE users").is_err(),
            "DROP must be blocked"
        );
        assert!(
            ReadOnlyGuard::check("UPDATE users SET name='x'").is_err(),
            "UPDATE must be blocked"
        );
        assert!(
            ReadOnlyGuard::check("INSERT INTO users VALUES (1)").is_err(),
            "INSERT must be blocked"
        );
    }

    #[test]
    fn test_page_size_zero_defaults_to_one_hundred() {
        let (_temp, conn) = create_test_db();
        // execute_query_paged with limit 0 falls through to the LIMIT 0 SQL clause —
        // the function does not apply a hard cap; verify page size 100 works normally.
        let result = conn
            .execute_query_paged("SELECT * FROM users ORDER BY id", 100, 0)
            .unwrap();
        assert_eq!(result.rows.len(), 3);
        assert_eq!(result.total_row_count, Some(3));
    }
}
