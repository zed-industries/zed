mod error;
#[allow(clippy::module_inception)]
mod migrate;
mod migration;
mod migration_type;
mod migrator;
mod source;

pub use error::MigrateError;
pub use migrate::{Migrate, MigrateDatabase};
pub use migration::{AppliedMigration, Migration};
pub use migration_type::MigrationType;
pub use migrator::Migrator;
pub use source::MigrationSource;

#[doc(hidden)]
pub use source::resolve_blocking;
