use crate::*;

pub trait TableRefBuilder: QuotedBuilder {
    /// Translate [`TableRef`] that without values into SQL statement.
    fn prepare_table_ref_iden(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(iden) => {
                iden.prepare(sql.as_writer(), self.quote());
            }
            TableRef::SchemaTable(schema, table) => {
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql.as_writer(), self.quote());
            }
            TableRef::DatabaseSchemaTable(database, schema, table) => {
                database.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql.as_writer(), self.quote());
            }
            TableRef::TableAlias(iden, alias) => {
                iden.prepare(sql.as_writer(), self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql.as_writer(), self.quote());
            }
            TableRef::SchemaTableAlias(schema, table, alias) => {
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql.as_writer(), self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql.as_writer(), self.quote());
            }
            TableRef::DatabaseSchemaTableAlias(database, schema, table, alias) => {
                database.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                table.prepare(sql.as_writer(), self.quote());
                write!(sql, " AS ").unwrap();
                alias.prepare(sql.as_writer(), self.quote());
            }
            TableRef::SubQuery(_, _)
            | TableRef::ValuesList(_, _)
            | TableRef::FunctionCall(_, _) => {
                panic!("TableRef with values is not support")
            }
        }
    }
}
