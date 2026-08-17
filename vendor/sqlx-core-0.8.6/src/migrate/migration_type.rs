use super::Migrator;

/// Migration Type represents the type of migration
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MigrationType {
    /// Simple migration are single file migrations with no up / down queries
    Simple,

    /// ReversibleUp migrations represents the  add or update part of a reversible migrations
    /// It is expected the every migration of this type will have a corresponding down file
    ReversibleUp,

    /// ReversibleDown migrations represents the  delete or downgrade part of a reversible migrations
    /// It is expected the every migration of this type will have a corresponding up file
    ReversibleDown,
}

impl MigrationType {
    pub fn from_filename(filename: &str) -> Self {
        if filename.ends_with(MigrationType::ReversibleUp.suffix()) {
            MigrationType::ReversibleUp
        } else if filename.ends_with(MigrationType::ReversibleDown.suffix()) {
            MigrationType::ReversibleDown
        } else {
            MigrationType::Simple
        }
    }

    pub fn is_reversible(&self) -> bool {
        match self {
            MigrationType::Simple => false,
            MigrationType::ReversibleUp => true,
            MigrationType::ReversibleDown => true,
        }
    }

    pub fn is_up_migration(&self) -> bool {
        match self {
            MigrationType::Simple => true,
            MigrationType::ReversibleUp => true,
            MigrationType::ReversibleDown => false,
        }
    }

    pub fn is_down_migration(&self) -> bool {
        match self {
            MigrationType::Simple => false,
            MigrationType::ReversibleUp => false,
            MigrationType::ReversibleDown => true,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            MigrationType::Simple => "migrate",
            MigrationType::ReversibleUp => "migrate",
            MigrationType::ReversibleDown => "revert",
        }
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            MigrationType::Simple => ".sql",
            MigrationType::ReversibleUp => ".up.sql",
            MigrationType::ReversibleDown => ".down.sql",
        }
    }

    pub fn file_content(&self) -> &'static str {
        match self {
            MigrationType::Simple => "-- Add migration script here\n",
            MigrationType::ReversibleUp => "-- Add up migration script here\n",
            MigrationType::ReversibleDown => "-- Add down migration script here\n",
        }
    }

    pub fn infer(migrator: &Migrator, reversible: bool) -> MigrationType {
        match migrator.iter().next() {
            Some(first_migration) => first_migration.migration_type,
            None => {
                if reversible {
                    MigrationType::ReversibleUp
                } else {
                    MigrationType::Simple
                }
            }
        }
    }
}
