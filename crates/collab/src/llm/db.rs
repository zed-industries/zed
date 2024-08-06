mod ids;
mod tables;

use sea_orm::DatabaseConnection;

pub use ids::*;
pub use sea_orm::ConnectOptions;

use crate::Result;

/// The database for the LLM service.
pub struct LlmDatabase {
    options: ConnectOptions,
    pool: DatabaseConnection,
}

impl LlmDatabase {
    /// Connects to the database with the given options
    pub async fn new(options: ConnectOptions) -> Result<Self> {
        sqlx::any::install_default_drivers();
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
        })
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }
}
