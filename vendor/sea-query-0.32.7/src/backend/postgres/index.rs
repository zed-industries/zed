use super::*;

impl IndexBuilder for PostgresQueryBuilder {
    // Overriden due to different "NULLS NOT UNIQUE" position in table index expression
    // (as opposed to the regular index expression)
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

        if create.nulls_not_distinct {
            write!(sql, "NULLS NOT DISTINCT ").unwrap();
        }

        self.prepare_index_columns(&create.index.columns, sql);

        if !create.include_columns.is_empty() {
            write!(sql, " ").unwrap();
            self.prepare_include_columns(&create.include_columns, sql);
        }
    }

    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

        if create.if_not_exists {
            write!(sql, "IF NOT EXISTS ").unwrap();
        }

        if let Some(name) = &create.index.name {
            write!(
                sql,
                "{}{}{}",
                self.quote().left(),
                name,
                self.quote().right()
            )
            .unwrap();
        }

        write!(sql, " ON ").unwrap();
        if let Some(table) = &create.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }

        self.prepare_index_type(&create.index_type, sql);
        write!(sql, " ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);

        if !create.include_columns.is_empty() {
            write!(sql, " ").unwrap();
            self.prepare_include_columns(&create.include_columns, sql);
        }

        if create.nulls_not_distinct {
            write!(sql, " NULLS NOT DISTINCT").unwrap();
        }
        self.prepare_filter(&create.r#where, sql);
    }

    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(_) | TableRef::SchemaTable(_, _) => {
                self.prepare_table_ref_iden(table_ref, sql)
            }
            _ => panic!("Not supported"),
        }
    }

    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();

        if drop.if_exists {
            write!(sql, "IF EXISTS ").unwrap();
        }

        if let Some(table) = &drop.table {
            match table {
                TableRef::Table(_) => {}
                TableRef::SchemaTable(schema, _) => {
                    schema.prepare(sql.as_writer(), self.quote());
                    write!(sql, ".").unwrap();
                }
                _ => panic!("Not supported"),
            }
        }
        if let Some(name) = &drop.index.name {
            write!(
                sql,
                "{}{}{}",
                self.quote().left(),
                name,
                self.quote().right()
            )
            .unwrap();
        }
    }

    fn prepare_index_type(&self, col_index_type: &Option<IndexType>, sql: &mut dyn SqlWriter) {
        if let Some(index_type) = col_index_type {
            write!(
                sql,
                " USING {}",
                match index_type {
                    IndexType::BTree => "BTREE".to_owned(),
                    IndexType::FullText => "GIN".to_owned(),
                    IndexType::Hash => "HASH".to_owned(),
                    IndexType::Custom(custom) => custom.to_string(),
                }
            )
            .unwrap();
        }
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY KEY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
    }

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
                IndexColumn::Expr(column) => {
                    write!(sql, "(").unwrap();
                    self.prepare_simple_expr(&column.expr, sql);
                    write!(sql, ")").unwrap();
                    if let Some(order) = &column.order {
                        match order {
                            IndexOrder::Asc => write!(sql, " ASC").unwrap(),
                            IndexOrder::Desc => write!(sql, " DESC").unwrap(),
                        }
                    }
                }
            }
            false
        });
        write!(sql, ")").unwrap();
    }

    fn prepare_filter(&self, condition: &ConditionHolder, sql: &mut dyn SqlWriter) {
        self.prepare_condition(condition, "WHERE", sql);
    }
}

impl PostgresQueryBuilder {
    fn prepare_include_columns(&self, columns: &[SeaRc<dyn Iden>], sql: &mut dyn SqlWriter) {
        write!(sql, "INCLUDE (").unwrap();
        columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql.as_writer(), self.quote());
            false
        });
        write!(sql, ")").unwrap();
    }
}
