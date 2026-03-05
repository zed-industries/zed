use crate::mention_provider::DatabaseMention;

#[test]
fn test_parse_connection_mention() {
    let mention = DatabaseMention::parse("db:connection:production").unwrap();
    assert_eq!(
        mention,
        DatabaseMention::Connection {
            connection_id: "production".to_string()
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
fn test_parse_schema_mention_with_schema_name() {
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
fn test_parse_query_mention_with_sql() {
    let mention = DatabaseMention::parse("db:query:mydb:SELECT COUNT(*) FROM users").unwrap();
    assert_eq!(
        mention,
        DatabaseMention::Query {
            connection: "mydb".to_string(),
            sql: "SELECT COUNT(*) FROM users".to_string()
        }
    );
}

#[test]
fn test_parse_unknown_type_returns_none() {
    assert!(DatabaseMention::parse("db:unknown:mydb").is_none());
}

#[test]
fn test_parse_non_db_prefix_returns_none() {
    assert!(DatabaseMention::parse("file:///some/path").is_none());
    assert!(DatabaseMention::parse("https://example.com").is_none());
}

#[test]
fn test_parse_empty_string_returns_none() {
    assert!(DatabaseMention::parse("").is_none());
}

#[test]
fn test_parse_missing_connection_returns_none() {
    assert!(DatabaseMention::parse("db:table:").is_none());
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
fn test_label_schema() {
    let mention = DatabaseMention::Schema {
        connection: "dev".to_string(),
        schema: "public".to_string(),
    };
    assert_eq!(mention.label(), "@schema:dev/public");
}

#[test]
fn test_label_query_short_sql() {
    let mention = DatabaseMention::Query {
        connection: "db".to_string(),
        sql: "SELECT 1".to_string(),
    };
    let label = mention.label();
    assert!(label.contains("SELECT 1"));
    assert!(!label.contains('…'));
}

#[test]
fn test_label_query_truncates_long_sql() {
    let long_sql = "SELECT id, name, email, phone, address FROM customers WHERE active = true";
    let mention = DatabaseMention::Query {
        connection: "db".to_string(),
        sql: long_sql.to_string(),
    };
    let label = mention.label();
    assert!(label.contains('…'));
}

#[test]
fn test_resolve_connection_missing_returns_error() {
    let mention = DatabaseMention::Connection {
        connection_id: "does_not_exist_xyz".to_string(),
    };
    assert!(mention.resolve().is_err());
}

#[test]
fn test_resolve_table_missing_connection_returns_error() {
    let mention = DatabaseMention::Table {
        connection: "does_not_exist_xyz".to_string(),
        table: "users".to_string(),
    };
    assert!(mention.resolve().is_err());
}

#[test]
fn test_resolve_query_missing_connection_returns_error() {
    let mention = DatabaseMention::Query {
        connection: "does_not_exist_xyz".to_string(),
        sql: "SELECT 1".to_string(),
    };
    assert!(mention.resolve().is_err());
}
