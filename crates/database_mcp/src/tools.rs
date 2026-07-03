use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use database_client::{
    ColumnInfo, ConnectionConfig, DatabaseClient, ForeignKey, IndexInfo, QueryResult, TableRef,
    TableStructure,
};

/// Constructs a [`DatabaseClient`] for a connection, target database, and the
/// already-resolved password. The password is resolved once by the host (via
/// [`PasswordSource`]) and threaded through so the keychain is not queried a
/// second time here.
pub type ClientFactory =
    Box<dyn Fn(&ConnectionConfig, &str, &str) -> Arc<dyn DatabaseClient> + Send + Sync>;

/// Resolves the password for a connection (e.g. from the system keychain).
pub type PasswordSource = Box<dyn Fn(&ConnectionConfig) -> Result<String> + Send + Sync>;

/// Owns the configured connections and lazily-built, cached database clients.
///
/// The `ToolHost` is stdio-agnostic so the tools can be exercised directly in
/// unit tests via injected `client_factory` and `password_source`.
pub struct ToolHost {
    pub connections: Vec<ConnectionConfig>,
    pub max_rows: usize,
    // Keyed by `(connection name, database)`. A tuple key avoids the ambiguity
    // of a formatted `"{name}::{database}"` string when a connection name itself
    // contains "::".
    clients: HashMap<(String, String), Arc<dyn DatabaseClient>>,
    client_factory: ClientFactory,
    password_source: PasswordSource,
}

impl ToolHost {
    pub fn new(
        connections: Vec<ConnectionConfig>,
        max_rows: usize,
        client_factory: ClientFactory,
        password_source: PasswordSource,
    ) -> Self {
        Self {
            connections,
            max_rows,
            clients: HashMap::new(),
            client_factory,
            password_source,
        }
    }

    /// The tool definitions advertised via `tools/list`, as a JSON array.
    pub fn tool_definitions() -> serde_json::Value {
        serde_json::json!([
            {
                "name": "list_connections",
                "description": "List configured database connections. Passwords are never returned.",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }
            },
            {
                "name": "list_tables",
                "description": "List schemas and their tables for a connection's database.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "connection": {
                            "type": "string",
                            "description": "Name of a configured connection."
                        },
                        "database": {
                            "type": "string",
                            "description": "Database to inspect. Defaults to the connection's initial database."
                        }
                    },
                    "required": ["connection"],
                    "additionalProperties": false
                }
            },
            {
                "name": "describe_table",
                "description": "Describe a table's columns, primary key, foreign keys, and indexes.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "connection": {
                            "type": "string",
                            "description": "Name of a configured connection."
                        },
                        "table": {
                            "type": "string",
                            "description": "Table name, optionally schema-qualified as \"schema.table\". Defaults to the public schema."
                        },
                        "database": {
                            "type": "string",
                            "description": "Database containing the table. Defaults to the connection's initial database."
                        }
                    },
                    "required": ["connection", "table"],
                    "additionalProperties": false
                }
            },
            {
                "name": "run_query",
                "description": "Run a read-only SQL query and return the result rows.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "connection": {
                            "type": "string",
                            "description": "Name of a configured connection."
                        },
                        "sql": {
                            "type": "string",
                            "description": "SQL to execute. The session is read-only."
                        },
                        "database": {
                            "type": "string",
                            "description": "Database to query. Defaults to the connection's initial database."
                        }
                    },
                    "required": ["connection", "sql"],
                    "additionalProperties": false
                }
            }
        ])
    }

    /// Dispatches a tool call by name. Unknown tools return an error, which the
    /// protocol layer surfaces as an `isError` tool result (not a JSON-RPC
    /// error).
    pub async fn call(
        &mut self,
        name: &str,
        arguments: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        match name {
            "list_connections" => self.list_connections(),
            "list_tables" => self.list_tables(arguments).await,
            "describe_table" => self.describe_table(arguments).await,
            "run_query" => self.run_query(arguments).await,
            other => Err(anyhow!("unknown tool: {other}")),
        }
    }

    fn list_connections(&self) -> Result<serde_json::Value> {
        let connections: Vec<serde_json::Value> = self
            .connections
            .iter()
            .map(|connection| {
                serde_json::json!({
                    "name": connection.name,
                    "host": connection.host,
                    "port": connection.port,
                    "database": connection.database,
                    "user": connection.user,
                })
            })
            .collect();
        Ok(serde_json::Value::Array(connections))
    }

    async fn list_tables(&mut self, arguments: &serde_json::Value) -> Result<serde_json::Value> {
        let connection_name = required_str(arguments, "connection")?;
        let config = self.connection(connection_name)?.clone();
        let database = optional_str(arguments, "database")
            .unwrap_or(config.database.as_str())
            .to_string();

        let client = self.client(&config, &database)?;

        let schema_names = client
            .list_schemas(&database)
            .await
            .with_context(|| format!("listing schemas in {database}"))?;

        let mut schemas = Vec::with_capacity(schema_names.len());
        for schema_name in schema_names {
            let tables = client
                .list_tables(&database, &schema_name)
                .await
                .with_context(|| format!("listing tables in {schema_name}"))?;
            let tables: Vec<serde_json::Value> = tables
                .into_iter()
                .map(|table| serde_json::json!({ "name": table.name, "is_view": table.is_view }))
                .collect();
            schemas.push(serde_json::json!({ "name": schema_name, "tables": tables }));
        }

        Ok(serde_json::json!({ "database": database, "schemas": schemas }))
    }

    async fn describe_table(&mut self, arguments: &serde_json::Value) -> Result<serde_json::Value> {
        let connection_name = required_str(arguments, "connection")?;
        let config = self.connection(connection_name)?.clone();
        let database = optional_str(arguments, "database")
            .unwrap_or(config.database.as_str())
            .to_string();
        let table_arg = required_str(arguments, "table")?;
        let (schema, table) = split_table_name(table_arg);

        let client = self.client(&config, &database)?;
        let table_ref = TableRef {
            database: database.clone(),
            schema: schema.clone(),
            name: table.clone(),
        };
        let structure = client
            .table_structure(&table_ref)
            .await
            .with_context(|| format!("describing {schema}.{table}"))?;

        Ok(structure_to_json(&database, &schema, &table, &structure))
    }

    async fn run_query(&mut self, arguments: &serde_json::Value) -> Result<serde_json::Value> {
        let connection_name = required_str(arguments, "connection")?;
        let config = self.connection(connection_name)?.clone();
        let database = optional_str(arguments, "database")
            .unwrap_or(config.database.as_str())
            .to_string();
        let sql = required_str(arguments, "sql")?;

        let client = self.client(&config, &database)?;
        // The client's `run_query` already wraps failures with a "running query"
        // context; adding another here would double the prefix in the tool's
        // error text (`format!("{error:#}")`), so we forward the error as-is.
        let result = client.run_query(&database, sql, self.max_rows).await?;

        Ok(query_result_to_json(&result))
    }

    /// Looks up a configured connection by name.
    fn connection(&self, name: &str) -> Result<&ConnectionConfig> {
        self.connections
            .iter()
            .find(|connection| connection.name == name)
            .ok_or_else(|| anyhow!("unknown connection: {name}"))
    }

    /// Returns a cached client for `(connection, database)`, building and
    /// caching one on first use. Building resolves the password exactly once and
    /// passes it to the factory, so a resolution failure surfaces as a tool
    /// error rather than silently degrading to an empty password.
    fn client(
        &mut self,
        config: &ConnectionConfig,
        database: &str,
    ) -> Result<Arc<dyn DatabaseClient>> {
        let key = (config.name.clone(), database.to_string());
        if let Some(client) = self.clients.get(&key) {
            return Ok(client.clone());
        }
        // Resolve the password once here and thread it through to the factory;
        // the factory must not re-resolve it (that would double keychain
        // prompts and could silently swallow a second-lookup failure).
        let password = (self.password_source)(config)
            .with_context(|| format!("resolving password for connection {}", config.name))?;
        let client = (self.client_factory)(config, database, &password);
        self.clients.insert(key, client.clone());
        Ok(client)
    }
}

/// Splits `"schema.table"` into `(schema, table)`, defaulting to the public
/// schema for an unqualified name.
fn split_table_name(raw: &str) -> (String, String) {
    match raw.split_once('.') {
        Some((schema, table)) => (schema.to_string(), table.to_string()),
        None => ("public".to_string(), raw.to_string()),
    }
}

fn required_str<'a>(arguments: &'a serde_json::Value, key: &str) -> Result<&'a str> {
    arguments
        .get(key)
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("missing required argument: {key}"))
}

fn optional_str<'a>(arguments: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    arguments
        .get(key)
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
}

fn structure_to_json(
    database: &str,
    schema: &str,
    table: &str,
    structure: &TableStructure,
) -> serde_json::Value {
    let columns: Vec<serde_json::Value> = structure.columns.iter().map(column_to_json).collect();
    let primary_key: Vec<&str> = structure
        .columns
        .iter()
        .filter(|column| column.is_primary_key)
        .map(|column| column.name.as_str())
        .collect();
    let foreign_keys: Vec<serde_json::Value> = structure
        .foreign_keys
        .iter()
        .map(foreign_key_to_json)
        .collect();
    let indexes: Vec<serde_json::Value> = structure.indexes.iter().map(index_to_json).collect();

    serde_json::json!({
        "database": database,
        "schema": schema,
        "table": table,
        "columns": columns,
        "primary_key": primary_key,
        "foreign_keys": foreign_keys,
        "indexes": indexes,
    })
}

fn column_to_json(column: &ColumnInfo) -> serde_json::Value {
    serde_json::json!({
        "name": column.name,
        "data_type": column.data_type,
        "udt_name": column.udt_name,
        "is_nullable": column.is_nullable,
        "default": column.default,
        "is_primary_key": column.is_primary_key,
    })
}

fn foreign_key_to_json(foreign_key: &ForeignKey) -> serde_json::Value {
    serde_json::json!({
        "column": foreign_key.column,
        "references_schema": foreign_key.references_schema,
        "references_table": foreign_key.references_table,
        "references_column": foreign_key.references_column,
    })
}

fn index_to_json(index: &IndexInfo) -> serde_json::Value {
    serde_json::json!({ "name": index.name, "definition": index.definition })
}

fn query_result_to_json(result: &QueryResult) -> serde_json::Value {
    serde_json::json!({
        "columns": result.columns,
        "rows": result.rows,
        "truncated": result.truncated,
        "command_tag": result.command_tag,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use database_client::fake::FakeDatabaseClient;
    use database_client::{QueryResult, TableInfo};
    use std::sync::{Arc, Mutex};

    fn config(name: &str) -> ConnectionConfig {
        ConnectionConfig {
            name: name.to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "app".to_string(),
            user: "postgres".to_string(),
        }
    }

    /// Builds a host whose factory returns the supplied fake client and whose
    /// password source always succeeds.
    fn host_with(
        connections: Vec<ConnectionConfig>,
        max_rows: usize,
        fake: Arc<FakeDatabaseClient>,
    ) -> ToolHost {
        // Record what database the factory was asked to build so tests can
        // assert on schema/table resolution.
        ToolHost::new(
            connections,
            max_rows,
            Box::new(move |_config, _database, _password| fake.clone() as Arc<dyn DatabaseClient>),
            Box::new(|_config| Ok("pw".to_string())),
        )
    }

    #[tokio::test]
    async fn list_connections_returns_configs_without_passwords() {
        let fake = Arc::new(FakeDatabaseClient::new());
        let mut host = host_with(vec![config("primary")], 200, fake);

        let result = host
            .call("list_connections", &serde_json::json!({}))
            .await
            .unwrap();
        let array = result.as_array().unwrap();
        assert_eq!(array.len(), 1);
        let entry = &array[0];
        assert_eq!(entry["name"], "primary");
        assert_eq!(entry["host"], "localhost");
        assert_eq!(entry["port"], 5432);
        assert_eq!(entry["database"], "app");
        assert_eq!(entry["user"], "postgres");
        // No password field of any kind is leaked.
        let serialized = serde_json::to_string(&result).unwrap();
        assert!(!serialized.contains("password"));
        assert!(!serialized.contains("pw"));
    }

    #[tokio::test]
    async fn run_query_forwards_max_rows() {
        // Truncation itself lives in `PostgresClient::run_query`, which the fake
        // does not emulate; this test only asserts that the host forwards its
        // configured `max_rows` to the client (the value the client would then
        // truncate against). Actual truncation reporting is covered by
        // `run_query_reports_truncation_flag`.
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["id".into()],
            rows: vec![vec![Some("1".into())]],
            truncated: false,
            command_tag: Some("SELECT 1".into()),
        };
        let fake = Arc::new(fake);
        let mut host = host_with(vec![config("primary")], 2, fake.clone());

        let result = host
            .call(
                "run_query",
                &serde_json::json!({ "connection": "primary", "sql": "select 1" }),
            )
            .await
            .unwrap();

        assert!(fake.calls().iter().any(|call| call.contains("max_rows=2")));
        assert_eq!(result["command_tag"], "SELECT 1");
        assert!(result["columns"].is_array());
    }

    #[tokio::test]
    async fn run_query_reports_truncation_flag() {
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["id".into()],
            rows: vec![vec![Some("1".into())], vec![Some("2".into())]],
            truncated: true,
            command_tag: Some("SELECT 2".into()),
        };
        let mut host = host_with(vec![config("primary")], 2, Arc::new(fake));

        let result = host
            .call(
                "run_query",
                &serde_json::json!({ "connection": "primary", "sql": "select 1" }),
            )
            .await
            .unwrap();

        assert_eq!(result["truncated"], true);
        assert_eq!(result["rows"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn describe_table_parses_schema_qualified_name() {
        // "public.users" and "users" must both resolve to schema=public,
        // name=users, producing the same table_structure call on the client.
        let fake_qualified = Arc::new(FakeDatabaseClient::new());
        let mut host = host_with(vec![config("primary")], 200, fake_qualified.clone());
        host.call(
            "describe_table",
            &serde_json::json!({ "connection": "primary", "table": "public.users" }),
        )
        .await
        .unwrap();

        let fake_bare = Arc::new(FakeDatabaseClient::new());
        let mut host = host_with(vec![config("primary")], 200, fake_bare.clone());
        let result = host
            .call(
                "describe_table",
                &serde_json::json!({ "connection": "primary", "table": "users" }),
            )
            .await
            .unwrap();

        assert_eq!(fake_qualified.calls(), fake_bare.calls());
        assert_eq!(result["schema"], "public");
        assert_eq!(result["table"], "users");
        // Primary key derived from columns.
        assert_eq!(result["primary_key"], serde_json::json!(["id"]));
    }

    #[tokio::test]
    async fn unknown_connection_is_error() {
        let fake = Arc::new(FakeDatabaseClient::new());
        let mut host = host_with(vec![config("primary")], 200, fake);

        let error = host
            .call(
                "run_query",
                &serde_json::json!({ "connection": "nope", "sql": "select 1" }),
            )
            .await
            .unwrap_err();
        assert!(error.to_string().contains("unknown connection"));
        assert!(error.to_string().contains("nope"));
    }

    #[tokio::test]
    async fn unknown_tool_is_error() {
        let fake = Arc::new(FakeDatabaseClient::new());
        let mut host = host_with(vec![config("primary")], 200, fake);
        let error = host
            .call("bogus", &serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(error.to_string().contains("unknown tool"));
    }

    #[tokio::test]
    async fn list_tables_collects_schemas_and_tables() {
        let mut fake = FakeDatabaseClient::new();
        fake.schemas = vec!["public".into(), "billing".into()];
        fake.tables = vec![
            TableInfo {
                name: "users".into(),
                is_view: false,
            },
            TableInfo {
                name: "orders_view".into(),
                is_view: true,
            },
        ];
        let mut host = host_with(vec![config("primary")], 200, Arc::new(fake));

        let result = host
            .call(
                "list_tables",
                &serde_json::json!({ "connection": "primary" }),
            )
            .await
            .unwrap();

        assert_eq!(result["database"], "app");
        let schemas = result["schemas"].as_array().unwrap();
        assert_eq!(schemas.len(), 2);
        assert_eq!(schemas[0]["name"], "public");
        let tables = schemas[0]["tables"].as_array().unwrap();
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0]["name"], "users");
        assert_eq!(tables[0]["is_view"], false);
        assert_eq!(tables[1]["is_view"], true);
    }

    #[tokio::test]
    async fn password_source_failure_surfaces_as_error() {
        let fake = Arc::new(FakeDatabaseClient::new());
        let mut host = ToolHost::new(
            vec![config("primary")],
            200,
            Box::new(move |_config, _database, _password| fake.clone() as Arc<dyn DatabaseClient>),
            Box::new(|config| Err(anyhow!("no saved password for connection {}", config.name))),
        );
        let error = host
            .call(
                "run_query",
                &serde_json::json!({ "connection": "primary", "sql": "select 1" }),
            )
            .await
            .unwrap_err();
        // The inner cause is preserved in the anyhow context chain; the
        // protocol layer surfaces it via `{error:#}`.
        assert!(format!("{error:#}").contains("no saved password"));
    }

    #[tokio::test]
    async fn clients_are_cached_per_connection_and_database() {
        // The factory increments a counter each call; caching means a second
        // tool call against the same (connection, database) does not rebuild.
        let build_count = Arc::new(Mutex::new(0usize));
        let counter = build_count.clone();
        let mut host = ToolHost::new(
            vec![config("primary")],
            200,
            Box::new(move |_config, _database, _password| {
                *counter.lock().unwrap() += 1;
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(|_config| Ok("pw".to_string())),
        );

        host.call(
            "run_query",
            &serde_json::json!({ "connection": "primary", "sql": "select 1" }),
        )
        .await
        .unwrap();
        host.call(
            "run_query",
            &serde_json::json!({ "connection": "primary", "sql": "select 2" }),
        )
        .await
        .unwrap();
        assert_eq!(*build_count.lock().unwrap(), 1);

        // A different database rebuilds.
        host.call(
            "run_query",
            &serde_json::json!({ "connection": "primary", "sql": "select 3", "database": "other" }),
        )
        .await
        .unwrap();
        assert_eq!(*build_count.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn resolved_password_is_threaded_to_factory_once() {
        // The password source must be consulted exactly once per built client,
        // and the resolved value must reach the factory (rather than being
        // discarded and re-resolved). This guards against the double-resolution
        // and silent empty-password degradation.
        let resolve_count = Arc::new(Mutex::new(0usize));
        let seen_password = Arc::new(Mutex::new(None::<String>));
        let counter = resolve_count.clone();
        let recorder = seen_password.clone();
        let mut host = ToolHost::new(
            vec![config("primary")],
            200,
            Box::new(move |_config, _database, password| {
                *recorder.lock().unwrap() = Some(password.to_string());
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(move |_config| {
                *counter.lock().unwrap() += 1;
                Ok("s3cret".to_string())
            }),
        );

        host.call(
            "run_query",
            &serde_json::json!({ "connection": "primary", "sql": "select 1" }),
        )
        .await
        .unwrap();
        // A second call against the same (connection, database) reuses the cached
        // client and must not resolve the password again.
        host.call(
            "run_query",
            &serde_json::json!({ "connection": "primary", "sql": "select 2" }),
        )
        .await
        .unwrap();

        assert_eq!(*resolve_count.lock().unwrap(), 1);
        assert_eq!(seen_password.lock().unwrap().as_deref(), Some("s3cret"));
    }

    #[tokio::test]
    async fn cache_key_does_not_collide_on_names_containing_separator() {
        // A connection named "prod::analytics" (initial db "app") and a
        // connection "prod" queried with database "analytics::app" would collapse
        // to the same `"{name}::{database}"` string. With a tuple key they build
        // distinct clients, so the query hits the intended server.
        let build_count = Arc::new(Mutex::new(0usize));
        let counter = build_count.clone();
        let mut host = ToolHost::new(
            vec![config("prod::analytics"), config("prod")],
            200,
            Box::new(move |_config, _database, _password| {
                *counter.lock().unwrap() += 1;
                Arc::new(FakeDatabaseClient::new()) as Arc<dyn DatabaseClient>
            }),
            Box::new(|_config| Ok("pw".to_string())),
        );

        // (prod::analytics, app) — the connection's initial database.
        host.call(
            "run_query",
            &serde_json::json!({ "connection": "prod::analytics", "sql": "select 1" }),
        )
        .await
        .unwrap();
        // (prod, analytics::app) — collides with the above under string keying.
        host.call(
            "run_query",
            &serde_json::json!({ "connection": "prod", "sql": "select 2", "database": "analytics::app" }),
        )
        .await
        .unwrap();

        assert_eq!(*build_count.lock().unwrap(), 2);
    }
}
