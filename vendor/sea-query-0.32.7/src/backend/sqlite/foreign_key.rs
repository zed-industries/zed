use super::*;

impl ForeignKeyBuilder for SqliteQueryBuilder {
    fn prepare_table_ref_fk_stmt(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        match table_ref {
            TableRef::Table(_) => self.prepare_table_ref_iden(table_ref, sql),
            _ => panic!("Not supported"),
        }
    }

    fn prepare_foreign_key_drop_statement_internal(
        &self,
        drop: &ForeignKeyDropStatement,
        sql: &mut dyn SqlWriter,
        mode: Mode,
    ) {
        if mode != Mode::Creation {
            panic!("Sqlite does not support modification of foreign key constraints to existing tables");
        }

        write!(sql, "DROP FOREIGN KEY ").unwrap();
        if let Some(name) = &drop.foreign_key.name {
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

    fn prepare_foreign_key_create_statement_internal(
        &self,
        create: &ForeignKeyCreateStatement,
        sql: &mut dyn SqlWriter,
        mode: Mode,
    ) {
        if mode != Mode::Creation {
            panic!("Sqlite does not support modification of foreign key constraints to existing tables");
        }

        write!(sql, "FOREIGN KEY (").unwrap();
        create.foreign_key.columns.iter().fold(true, |first, col| {
            if !first {
                write!(sql, ", ").unwrap();
            }
            col.prepare(sql.as_writer(), self.quote());
            false
        });
        write!(sql, ")").unwrap();

        write!(sql, " REFERENCES ").unwrap();
        if let Some(ref_table) = &create.foreign_key.ref_table {
            self.prepare_table_ref_fk_stmt(ref_table, sql);
        }
        write!(sql, " (").unwrap();
        create
            .foreign_key
            .ref_columns
            .iter()
            .fold(true, |first, col| {
                if !first {
                    write!(sql, ", ").unwrap();
                }
                col.prepare(sql.as_writer(), self.quote());
                false
            });
        write!(sql, ")").unwrap();

        if let Some(foreign_key_action) = &create.foreign_key.on_delete {
            write!(sql, " ON DELETE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }

        if let Some(foreign_key_action) = &create.foreign_key.on_update {
            write!(sql, " ON UPDATE ").unwrap();
            self.prepare_foreign_key_action(foreign_key_action, sql);
        }
    }
}
