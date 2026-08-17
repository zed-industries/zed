//! Table definition & alternations statements.
//!
//! # Usage
//!
//! - Table Create, see [`TableCreateStatement`]
//! - Table Alter, see [`TableAlterStatement`]
//! - Table Drop, see [`TableDropStatement`]
//! - Table Rename, see [`TableRenameStatement`]
//! - Table Truncate, see [`TableTruncateStatement`]

use crate::SchemaBuilder;

mod alter;
mod column;
mod create;
mod drop;
mod rename;
mod truncate;

pub use alter::*;
pub use column::*;
pub use create::*;
pub use drop::*;
pub use rename::*;
pub use truncate::*;

/// Helper for constructing any table statement
#[derive(Debug)]
pub struct Table;

/// All available types of table statement
#[derive(Debug, Clone)]
pub enum TableStatement {
    Create(TableCreateStatement),
    Alter(TableAlterStatement),
    Drop(TableDropStatement),
    Rename(TableRenameStatement),
    Truncate(TableTruncateStatement),
}

impl Table {
    /// Construct table [`TableCreateStatement`]
    pub fn create() -> TableCreateStatement {
        TableCreateStatement::new()
    }

    /// Construct table [`TableAlterStatement`]
    pub fn alter() -> TableAlterStatement {
        TableAlterStatement::new()
    }

    /// Construct table [`TableDropStatement`]
    pub fn drop() -> TableDropStatement {
        TableDropStatement::new()
    }

    /// Construct table [`TableRenameStatement`]
    pub fn rename() -> TableRenameStatement {
        TableRenameStatement::new()
    }

    /// Construct table [`TableTruncateStatement`]
    pub fn truncate() -> TableTruncateStatement {
        TableTruncateStatement::new()
    }
}

impl TableStatement {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build<T: SchemaBuilder>(&self, table_builder: T) -> String {
        match self {
            Self::Create(stat) => stat.build(table_builder),
            Self::Alter(stat) => stat.build(table_builder),
            Self::Drop(stat) => stat.build(table_builder),
            Self::Rename(stat) => stat.build(table_builder),
            Self::Truncate(stat) => stat.build(table_builder),
        }
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn build_any(&self, table_builder: &dyn SchemaBuilder) -> String {
        match self {
            Self::Create(stat) => stat.build_any(table_builder),
            Self::Alter(stat) => stat.build_any(table_builder),
            Self::Drop(stat) => stat.build_any(table_builder),
            Self::Rename(stat) => stat.build_any(table_builder),
            Self::Truncate(stat) => stat.build_any(table_builder),
        }
    }

    /// Build corresponding SQL statement for certain database backend and return SQL string
    pub fn to_string<T: SchemaBuilder>(&self, table_builder: T) -> String {
        match self {
            Self::Create(stat) => stat.to_string(table_builder),
            Self::Alter(stat) => stat.to_string(table_builder),
            Self::Drop(stat) => stat.to_string(table_builder),
            Self::Rename(stat) => stat.to_string(table_builder),
            Self::Truncate(stat) => stat.to_string(table_builder),
        }
    }
}
