pub mod engine;
pub mod storage;
pub mod query;
pub mod index;
pub mod universal;
pub mod error;
pub mod semantic_schema;

// Core types
pub use engine::UnifiedDataEngine;
pub use universal::{UniversalRecord, UniversalContent, Relationship};
pub use query::{UniversalQuery, QueryTarget, SearchResults};
pub use error::{UideError, Result}; 