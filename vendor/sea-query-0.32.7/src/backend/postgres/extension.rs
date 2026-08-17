use super::*;
use crate::extension::postgres::*;

impl ExtensionBuilder for PostgresQueryBuilder {
    fn prepare_extension_create_statement(
        &self,
        create: &ExtensionCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE EXTENSION ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap()
        }

        write!(sql, "{}", create.name).unwrap();

        if let Some(schema) = create.schema.as_ref() {
            write!(sql, " WITH SCHEMA {}", schema).unwrap();
        }

        if let Some(version) = create.version.as_ref() {
            write!(sql, " VERSION {}", version).unwrap();
        }

        if create.cascade {
            write!(sql, " CASCADE").unwrap();
        }
    }

    fn prepare_extension_drop_statement(
        &self,
        drop: &ExtensionDropStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "DROP EXTENSION ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        write!(sql, "{}", drop.name).unwrap();

        if drop.cascade {
            write!(sql, " CASCADE").unwrap();
        }

        if drop.restrict {
            write!(sql, " RESTRICT").unwrap();
        }
    }
}
