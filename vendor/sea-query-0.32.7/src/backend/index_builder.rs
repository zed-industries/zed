use crate::*;

pub trait IndexBuilder: QuotedBuilder + TableRefBuilder {
    /// Translate [`IndexCreateStatement`] into SQL expression.
    /// This is the default implementation for `PostgresQueryBuilder` and `SqliteQueryBuilder`.
    /// `MysqlQueryBuilder` overrides this default implementation.
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        if let Some(name) = &create.index.name {
            write!(
                sql,
                "CONSTRAINT {}{}{} ",
                self.quote().left(),
                name,
                self.quote().right()
            )
            .unwrap();
        }

        self.prepare_index_prefix(create, sql);

        self.prepare_index_columns(&create.index.columns, sql);

        self.prepare_filter(&create.r#where, sql);
    }

    /// Translate [`IndexCreateStatement`] into SQL statement.
    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    );

    /// Translate [`TableRef`] into SQL statement.
    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter);

    /// Translate [`IndexDropStatement`] into SQL statement.
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter);

    #[doc(hidden)]
    /// Write the index type (Btree, hash, ...).
    fn prepare_index_type(&self, _col_index_type: &Option<IndexType>, _sql: &mut dyn SqlWriter) {}

    #[doc(hidden)]
    /// Write the index prefix (primary, unique, ...).
    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter);

    #[doc(hidden)]
    /// Write the column index prefix.
    fn write_column_index_prefix(&self, col_prefix: &Option<u32>, sql: &mut dyn SqlWriter) {
        if let Some(prefix) = col_prefix {
            write!(sql, " ({prefix})").unwrap();
        }
    }

    #[doc(hidden)]
    /// Write the index column with table column.
    fn prepare_index_column_with_table_column(
        &self,
        column: &IndexColumnTableColumn,
        sql: &mut dyn SqlWriter,
    ) {
        column.name.prepare(sql.as_writer(), self.quote());
        self.write_column_index_prefix(&column.prefix, sql);
        if let Some(order) = &column.order {
            match order {
                IndexOrder::Asc => write!(sql, " ASC").unwrap(),
                IndexOrder::Desc => write!(sql, " DESC").unwrap(),
            }
        }
    }

    #[doc(hidden)]
    /// Write the column index prefix.
    fn prepare_index_columns(&self, columns: &[IndexColumn], sql: &mut dyn SqlWriter) {
        write!(sql, "(").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            match col {
                IndexColumn::TableColumn(column) => {
                    self.prepare_index_column_with_table_column(column, sql);
                }
                IndexColumn::Expr(_) => panic!("Not supported"),
            }
            false
        });
        write!(sql, ")").unwrap();
    }

    #[doc(hidden)]
    // Write WHERE clause for partial index. This function is not available in MySQL.
    fn prepare_filter(&self, _condition: &ConditionHolder, _sql: &mut dyn SqlWriter) {}
}
