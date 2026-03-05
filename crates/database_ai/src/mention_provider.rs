use database_core::{describe_object_core, get_schema_core, list_objects_core};

/// Represents a database-specific mention typed by the user in the AI chat.
///
/// The URI format is: `db:<type>:<connection>[:<object>]`
/// Examples:
///   - `db:connection:mydb`       → includes all tables summary
///   - `db:schema:mydb`           → includes full DDL schema
///   - `db:table:mydb:users`      → includes users table schema
///   - `db:query:mydb:SELECT 1`   → executes and includes query results
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseMention {
    Connection {
        connection_id: String,
    },
    Schema {
        connection: String,
        schema: String,
    },
    Table {
        connection: String,
        table: String,
    },
    Query {
        connection: String,
        sql: String,
    },
}

impl DatabaseMention {
    /// Parses a URI string into a `DatabaseMention`.
    ///
    /// Returns `None` if the URI is not a recognized database mention.
    pub fn parse(uri: &str) -> Option<Self> {
        let remainder = uri.strip_prefix("db:")?;
        let mut parts = remainder.splitn(4, ':');
        let mention_type = parts.next()?;
        let connection = parts.next()?.to_string();

        match mention_type {
            "connection" => Some(DatabaseMention::Connection {
                connection_id: connection,
            }),
            "schema" => {
                let schema = parts.next().unwrap_or("public").to_string();
                Some(DatabaseMention::Schema { connection, schema })
            }
            "table" => {
                let table = parts.next()?.to_string();
                Some(DatabaseMention::Table { connection, table })
            }
            "query" => {
                let sql = parts.next()?.to_string();
                Some(DatabaseMention::Query { connection, sql })
            }
            _ => None,
        }
    }

    /// Resolves this mention to a text string suitable for inclusion in an AI prompt.
    pub fn resolve(&self) -> Result<String, String> {
        match self {
            DatabaseMention::Connection { connection_id } => {
                list_objects_core(connection_id, Some("all"))
            }
            DatabaseMention::Schema { connection, .. } => get_schema_core(connection, &[]),
            DatabaseMention::Table { connection, table } => {
                describe_object_core(table, connection)
            }
            DatabaseMention::Query { connection, sql } => {
                database_core::execute_query_core(sql, connection, 100)
            }
        }
    }

    /// Returns a short human-readable label for this mention, used in the UI chip.
    pub fn label(&self) -> String {
        match self {
            DatabaseMention::Connection { connection_id } => {
                format!("@db:{connection_id}")
            }
            DatabaseMention::Schema { connection, schema } => {
                format!("@schema:{connection}/{schema}")
            }
            DatabaseMention::Table { connection, table } => {
                format!("@table:{connection}/{table}")
            }
            DatabaseMention::Query { connection, sql } => {
                let short_sql = if sql.len() > 30 {
                    format!("{}…", &sql[..30])
                } else {
                    sql.clone()
                };
                format!("@query:{connection}/{short_sql}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_connection_mention() {
        let mention = DatabaseMention::parse("db:connection:mydb").unwrap();
        assert_eq!(
            mention,
            DatabaseMention::Connection {
                connection_id: "mydb".to_string()
            }
        );
    }

    #[test]
    fn test_parse_table_mention() {
        let mention = DatabaseMention::parse("db:table:mydb:users").unwrap();
        assert_eq!(
            mention,
            DatabaseMention::Table {
                connection: "mydb".to_string(),
                table: "users".to_string()
            }
        );
    }

    #[test]
    fn test_parse_schema_mention_with_explicit_schema() {
        let mention = DatabaseMention::parse("db:schema:mydb:public").unwrap();
        assert_eq!(
            mention,
            DatabaseMention::Schema {
                connection: "mydb".to_string(),
                schema: "public".to_string()
            }
        );
    }

    #[test]
    fn test_parse_schema_mention_defaults_to_public() {
        let mention = DatabaseMention::parse("db:schema:mydb").unwrap();
        assert_eq!(
            mention,
            DatabaseMention::Schema {
                connection: "mydb".to_string(),
                schema: "public".to_string()
            }
        );
    }

    #[test]
    fn test_parse_query_mention() {
        let mention = DatabaseMention::parse("db:query:mydb:SELECT 1").unwrap();
        assert_eq!(
            mention,
            DatabaseMention::Query {
                connection: "mydb".to_string(),
                sql: "SELECT 1".to_string()
            }
        );
    }

    #[test]
    fn test_parse_unknown_type_returns_none() {
        assert!(DatabaseMention::parse("db:unknown:mydb").is_none());
    }

    #[test]
    fn test_parse_non_db_prefix_returns_none() {
        assert!(DatabaseMention::parse("file://something").is_none());
        assert!(DatabaseMention::parse("table:mydb:users").is_none());
    }

    #[test]
    fn test_label_connection() {
        let mention = DatabaseMention::Connection {
            connection_id: "prod".to_string(),
        };
        assert_eq!(mention.label(), "@db:prod");
    }

    #[test]
    fn test_label_table() {
        let mention = DatabaseMention::Table {
            connection: "prod".to_string(),
            table: "orders".to_string(),
        };
        assert_eq!(mention.label(), "@table:prod/orders");
    }

    #[test]
    fn test_label_query_truncates_long_sql() {
        let long_sql = "SELECT id, name, email, created_at FROM users WHERE active = true";
        let mention = DatabaseMention::Query {
            connection: "dev".to_string(),
            sql: long_sql.to_string(),
        };
        let label = mention.label();
        assert!(label.contains("…"));
        assert!(label.len() < long_sql.len() + 20);
    }

    #[test]
    fn test_resolve_missing_connection_returns_error() {
        let mention = DatabaseMention::Connection {
            connection_id: "nonexistent_xyz".to_string(),
        };
        assert!(mention.resolve().is_err());
    }
}
