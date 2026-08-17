use super::*;

impl IndexBuilder for MysqlQueryBuilder {
    fn prepare_table_index_expression(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        self.prepare_index_prefix(create, sql);
        write!(sql, "KEY ").unwrap();

        if let Some(name) = &create.index.name {
            write!(
                sql,
                "{}{}{} ",
                self.quote().left(),
                name,
                self.quote().right()
            )
            .unwrap();
        }

        self.prepare_index_type(&create.index_type, sql);
        if matches!(create.index_type, Some(IndexType::FullText)) {
            write!(sql, " ").unwrap();
        }

        self.prepare_index_columns(&create.index.columns, sql);
    }

    fn prepare_index_create_statement(
        &self,
        create: &IndexCreateStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "CREATE ").unwrap();
        self.prepare_index_prefix(create, sql);
        write!(sql, "INDEX ").unwrap();

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
        write!(sql, " ").unwrap();
        self.prepare_index_columns(&create.index.columns, sql);

        self.prepare_index_type(&create.index_type, sql);
    }

    fn prepare_table_ref_index_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(_) => self.prepare_table_ref_iden(table_ref, sql),
            _ => panic!("Not supported"),
        }
    }
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn SqlWriter) {
        write!(sql, "DROP INDEX ").unwrap();

        if drop.if_exists {
            panic!("Mysql does not support IF EXISTS for DROP INDEX")
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

        write!(sql, " ON ").unwrap();
        if let Some(table) = &drop.table {
            self.prepare_table_ref_index_stmt(table, sql);
        }
    }

    fn prepare_index_type(&self, col_index_type: &Option<IndexType>, sql: &mut dyn SqlWriter) {
        if let Some(index_type) = col_index_type {
            if !matches!(index_type, IndexType::FullText) {
                write!(
                    sql,
                    " USING {}",
                    match index_type {
                        IndexType::BTree => "BTREE".to_owned(),
                        IndexType::FullText => unreachable!(),
                        IndexType::Hash => "HASH".to_owned(),
                        IndexType::Custom(custom) => custom.to_string(),
                    }
                )
                .unwrap();
            }
        }
    }

    fn prepare_index_prefix(&self, create: &IndexCreateStatement, sql: &mut dyn SqlWriter) {
        if create.primary {
            write!(sql, "PRIMARY ").unwrap();
        }
        if create.unique {
            write!(sql, "UNIQUE ").unwrap();
        }
        if matches!(create.index_type, Some(IndexType::FullText)) {
            write!(sql, "FULLTEXT ").unwrap();
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
}
