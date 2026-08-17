//! Schema definition & alternations statements

use crate::{backend::SchemaBuilder, ForeignKeyStatement, IndexStatement, TableStatement};

#[derive(Debug, Clone)]
pub enum SchemaStatement {
    TableStatement(TableStatement),
    IndexStatement(IndexStatement),
    ForeignKeyStatement(ForeignKeyStatement),
}

pub trait SchemaStatementBuilder {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String;

    /// Build corresponding SQL statement for certain database backend and return SQL string
    fn to_string<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        self.build(schema_builder)
    }
}
