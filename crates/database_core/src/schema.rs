use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub tables: Vec<TableInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
    pub row_count: Option<u64>,
    pub is_virtual: bool,
    #[serde(default)]
    pub ddl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyInfo {
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum IntrospectionLevel {
    Names,
    #[default]
    Metadata,
    FullDdl,
}

pub struct SchemaCache {
    entries: HashMap<String, SchemaCacheEntry>,
    ttl: Duration,
}

struct SchemaCacheEntry {
    schema: DatabaseSchema,
    fetched_at: Instant,
}

impl SchemaCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, connection_name: &str) -> Option<&DatabaseSchema> {
        let entry = self.entries.get(connection_name)?;
        if entry.fetched_at.elapsed() < self.ttl {
            Some(&entry.schema)
        } else {
            None
        }
    }

    pub fn insert(&mut self, connection_name: String, schema: DatabaseSchema) {
        self.entries.insert(
            connection_name,
            SchemaCacheEntry {
                schema,
                fetched_at: Instant::now(),
            },
        );
    }

    pub fn invalidate(&mut self, connection_name: &str) {
        self.entries.remove(connection_name);
    }

    pub fn invalidate_all(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_cache_insert_and_get() {
        let mut cache = SchemaCache::new(Duration::from_secs(60));
        let schema = DatabaseSchema {
            tables: vec![TableInfo {
                name: "users".to_string(),
                columns: vec![],
                indexes: vec![],
                foreign_keys: vec![],
                row_count: Some(10),
                is_virtual: false,
                ddl: None,
            }],
        };

        cache.insert("my_db".to_string(), schema);
        let retrieved = cache.get("my_db");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.expect("just checked").tables.len(), 1);
        assert_eq!(
            retrieved.expect("just checked").tables[0].name,
            "users"
        );
    }

    #[test]
    fn test_schema_cache_ttl_expiry() {
        let mut cache = SchemaCache::new(Duration::from_millis(1));
        let schema = DatabaseSchema {
            tables: vec![],
        };

        cache.insert("my_db".to_string(), schema);
        std::thread::sleep(Duration::from_millis(10));
        assert!(cache.get("my_db").is_none());
    }

    #[test]
    fn test_schema_cache_invalidate() {
        let mut cache = SchemaCache::new(Duration::from_secs(60));
        let schema = DatabaseSchema {
            tables: vec![],
        };

        cache.insert("db_a".to_string(), schema.clone());
        cache.insert("db_b".to_string(), schema);

        assert!(cache.get("db_a").is_some());
        assert!(cache.get("db_b").is_some());

        cache.invalidate("db_a");
        assert!(cache.get("db_a").is_none());
        assert!(cache.get("db_b").is_some());

        cache.invalidate_all();
        assert!(cache.get("db_b").is_none());
    }
}
