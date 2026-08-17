//! Foreign key definition & alternations statements.
//!
//! # Usage
//!
//! - Table Foreign Key Create, see [`ForeignKeyCreateStatement`]
//! - Table Foreign Key Drop, see [`ForeignKeyDropStatement`]

mod common;
mod create;
mod drop;

pub use common::*;
pub use create::*;
pub use drop::*;

/// Shorthand for constructing any foreign key statement
#[derive(Debug, Clone)]
pub struct ForeignKey;

/// All available types of foreign key statement
#[derive(Debug, Clone)]
pub enum ForeignKeyStatement {
    Create(ForeignKeyCreateStatement),
    Drop(ForeignKeyDropStatement),
}

impl ForeignKey {
    /// Construct foreign key [`ForeignKeyCreateStatement`]
    pub fn create() -> ForeignKeyCreateStatement {
        ForeignKeyCreateStatement::new()
    }

    /// Construct foreign key [`ForeignKeyDropStatement`]
    pub fn drop() -> ForeignKeyDropStatement {
        ForeignKeyDropStatement::new()
    }
}
