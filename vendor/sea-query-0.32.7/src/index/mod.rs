//! Index definition & alternations statements.
//!
//! # Usage
//!
//! - Table Index Create, see [`IndexCreateStatement`]
//! - Table Index Drop, see [`IndexDropStatement`]

mod common;
mod create;
mod drop;

pub use common::*;
pub use create::*;
pub use drop::*;

/// Shorthand for constructing any index statement
#[derive(Debug, Clone)]
pub struct Index;

/// All available types of index statement
#[derive(Debug, Clone)]
pub enum IndexStatement {
    Create(IndexCreateStatement),
    Drop(IndexDropStatement),
}

impl Index {
    /// Construct index [`IndexCreateStatement`]
    pub fn create() -> IndexCreateStatement {
        IndexCreateStatement::new()
    }

    /// Construct index [`IndexDropStatement`]
    pub fn drop() -> IndexDropStatement {
        IndexDropStatement::new()
    }
}
