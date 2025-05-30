//! Storage layer implementation using RocksDB as foundation

use crate::{
    error::{Result, UideError},
    universal::{RecordId, UniversalRecord},
};
use rocksdb::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use tracing::{debug, info};

/// Column family names for organizing data
pub const CF_RECORDS: &str = "records";
pub const CF_METADATA: &str = "metadata";
pub const CF_INDEXES: &str = "indexes";
pub const CF_RELATIONSHIPS: &str = "relationships";

/// Storage engine configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub path: String,
    pub create_if_missing: bool,
    pub max_open_files: i32,
    pub write_buffer_size: usize,
    pub max_write_buffer_number: i32,
    pub compression_type: CompressionType,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            path: "./uide_data".to_string(),
            create_if_missing: true,
            max_open_files: 1000,
            write_buffer_size: 64 * 1024 * 1024, // 64MB
            max_write_buffer_number: 3,
            compression_type: CompressionType::Lz4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompressionType {
    None,
    Snappy,
    Zlib,
    Lz4,
    Zstd,
}

impl From<CompressionType> for rocksdb::DBCompressionType {
    fn from(ct: CompressionType) -> Self {
        match ct {
            CompressionType::None => rocksdb::DBCompressionType::None,
            CompressionType::Snappy => rocksdb::DBCompressionType::Snappy,
            CompressionType::Zlib => rocksdb::DBCompressionType::Zlib,
            CompressionType::Lz4 => rocksdb::DBCompressionType::Lz4,
            CompressionType::Zstd => rocksdb::DBCompressionType::Zstd,
        }
    }
}

/// Core storage engine using RocksDB
pub struct StorageEngine {
    db: Arc<DB>,
    config: StorageConfig,
}

impl StorageEngine {
    /// Create a new storage engine
    pub async fn new(config: StorageConfig) -> Result<Self> {
        let path = Path::new(&config.path);
        
        // Create directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        // Configure RocksDB options
        let mut db_opts = Options::default();
        db_opts.create_if_missing(config.create_if_missing);
        db_opts.create_missing_column_families(true);
        db_opts.set_max_open_files(config.max_open_files);
        db_opts.set_write_buffer_size(config.write_buffer_size);
        db_opts.set_max_write_buffer_number(config.max_write_buffer_number);
        db_opts.set_compression_type(config.compression_type.clone().into());

        // Configure column families
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(CF_RECORDS, Options::default()),
            ColumnFamilyDescriptor::new(CF_METADATA, Options::default()),
            ColumnFamilyDescriptor::new(CF_INDEXES, Options::default()),
            ColumnFamilyDescriptor::new(CF_RELATIONSHIPS, Options::default()),
        ];

        // Open database
        let db = DB::open_cf_descriptors(&db_opts, &config.path, cf_descriptors)?;
        
        info!("Storage engine initialized at: {}", config.path);
        
        Ok(Self {
            db: Arc::new(db),
            config,
        })
    }

    /// Store a record
    pub async fn store_record(&self, record: &UniversalRecord) -> Result<()> {
        let cf = self.get_cf(CF_RECORDS)?;
        let key = record.id.as_bytes();
        let value = bincode::serialize(record)?;

        self.db.put_cf(&cf, key, value)?;
        debug!("Stored record: {}", record.id);
        
        // Store relationships separately for efficient querying
        self.store_relationships(record).await?;
        
        Ok(())
    }

    /// Retrieve a record by ID
    pub async fn get_record(&self, id: RecordId) -> Result<Option<UniversalRecord>> {
        let cf = self.get_cf(CF_RECORDS)?;
        let key = id.as_bytes();

        match self.db.get_cf(&cf, key)? {
            Some(data) => {
                let record = bincode::deserialize(&data)?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    /// Delete a record
    pub async fn delete_record(&self, id: RecordId) -> Result<bool> {
        let cf = self.get_cf(CF_RECORDS)?;
        let key = id.as_bytes();

        // Check if record exists
        let exists = self.db.get_cf(&cf, key)?.is_some();
        
        if exists {
            self.db.delete_cf(&cf, key)?;
            // Also delete relationships
            self.delete_relationships(id).await?;
            debug!("Deleted record: {}", id);
        }

        Ok(exists)
    }

    /// List all records (for debugging/admin)
    pub async fn list_records(&self, limit: Option<usize>) -> Result<Vec<UniversalRecord>> {
        let cf = self.get_cf(CF_RECORDS)?;
        let iter = self.db.iterator_cf(&cf, rocksdb::IteratorMode::Start);
        
        let mut records = Vec::new();
        let max_records = limit.unwrap_or(1000);

        for (i, item) in iter.enumerate() {
            if i >= max_records {
                break;
            }
            
            let (_, value) = item?;
            let record: UniversalRecord = bincode::deserialize(&value)?;
            records.push(record);
        }

        debug!("Listed {} records", records.len());
        Ok(records)
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> Result<StorageStats> {
        let cf_records = self.get_cf(CF_RECORDS)?;
        let cf_relationships = self.get_cf(CF_RELATIONSHIPS)?;

        // Count records
        let mut record_count = 0;
        let iter = self.db.iterator_cf(&cf_records, rocksdb::IteratorMode::Start);
        for _ in iter {
            record_count += 1;
        }

        // Count relationships
        let mut relationship_count = 0;
        let iter = self.db.iterator_cf(&cf_relationships, rocksdb::IteratorMode::Start);
        for _ in iter {
            relationship_count += 1;
        }

        Ok(StorageStats {
            record_count,
            relationship_count,
            database_size: self.estimate_database_size()?,
        })
    }

    /// Store relationships separately for efficient querying
    async fn store_relationships(&self, record: &UniversalRecord) -> Result<()> {
        if record.relationships.is_empty() {
            return Ok(());
        }

        let cf = self.get_cf(CF_RELATIONSHIPS)?;
        
        for relationship in &record.relationships {
            // Store forward relationship: source -> target
            let forward_key = format!("{}:{}", record.id, relationship.target_id);
            let relationship_data = bincode::serialize(relationship)?;
            self.db.put_cf(&cf, forward_key.as_bytes(), &relationship_data)?;

            // Store reverse relationship if bidirectional
            if relationship.bidirectional {
                let reverse_key = format!("{}:{}", relationship.target_id, record.id);
                self.db.put_cf(&cf, reverse_key.as_bytes(), &relationship_data)?;
            }
        }

        Ok(())
    }

    /// Delete relationships for a record
    async fn delete_relationships(&self, record_id: RecordId) -> Result<()> {
        let cf = self.get_cf(CF_RELATIONSHIPS)?;
        let prefix = format!("{}:", record_id);
        
        // Find and delete all relationships starting with this record ID
        let iter = self.db.prefix_iterator_cf(&cf, prefix.as_bytes());
        let mut keys_to_delete = Vec::new();
        
        for item in iter {
            let (key, _) = item?;
            keys_to_delete.push(key.to_vec());
        }

        for key in keys_to_delete {
            self.db.delete_cf(&cf, &key)?;
        }

        Ok(())
    }

    /// Get column family handle
    fn get_cf(&self, name: &str) -> Result<Arc<BoundColumnFamily>> {
        self.db
            .cf_handle(name)
            .ok_or_else(|| UideError::internal(format!("Column family '{}' not found", name)))
    }

    /// Estimate database size (rough calculation)
    fn estimate_database_size(&self) -> Result<u64> {
        // This is a rough estimate based on RocksDB properties
        // In a real implementation, we'd use more accurate RocksDB statistics
        let path = Path::new(&self.config.path);
        let mut total_size = 0;
        
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(total_size)
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub record_count: usize,
    pub relationship_count: usize,
    pub database_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal::{DataType, UniversalContent, VectorEncoding};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_basic_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config = StorageConfig {
            path: temp_dir.path().to_string_lossy().to_string(),
            ..Default::default()
        };

        let storage = StorageEngine::new(config).await.unwrap();

        // Create a test record
        let record = UniversalRecord::new(
            DataType::Vector,
            UniversalContent::Vector {
                dimensions: 3,
                values: vec![1.0, 2.0, 3.0],
                encoding: VectorEncoding::Float32,
            },
        );

        let record_id = record.id;

        // Store record
        storage.store_record(&record).await.unwrap();

        // Retrieve record
        let retrieved = storage.get_record(record_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, record_id);

        // Delete record
        let deleted = storage.delete_record(record_id).await.unwrap();
        assert!(deleted);

        // Verify deletion
        let not_found = storage.get_record(record_id).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_storage_stats() {
        let temp_dir = TempDir::new().unwrap();
        let config = StorageConfig {
            path: temp_dir.path().to_string_lossy().to_string(),
            ..Default::default()
        };

        let storage = StorageEngine::new(config).await.unwrap();

        // Initially empty
        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.record_count, 0);

        // Add a record
        let record = UniversalRecord::new(
            DataType::Document,
            UniversalContent::Document {
                text: "Test document".to_string(),
                tokens: None,
                language: Some("en".to_string()),
            },
        );

        storage.store_record(&record).await.unwrap();

        // Check stats
        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.record_count, 1);
    }
} 