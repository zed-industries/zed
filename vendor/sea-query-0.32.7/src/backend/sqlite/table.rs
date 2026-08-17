use super::*;

impl TableBuilder for SqliteQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        column_def.name.prepare(sql.as_writer(), self.quote());

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(&column_def.spec, column_type, sql);
        }

        let mut is_primary_key = false;
        let mut is_auto_increment = false;

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::PrimaryKey = column_spec {
                is_primary_key = true;
                continue;
            }
            if let ColumnSpec::AutoIncrement = column_spec {
                is_auto_increment = true;
                continue;
            }
            if let ColumnSpec::Comment(_) = column_spec {
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }

        if is_primary_key {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::PrimaryKey, sql);
        }
        if is_auto_increment {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::AutoIncrement, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        self.prepare_column_type(&[], column_type, sql)
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        "AUTOINCREMENT"
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
        // Sqlite does not support table drop options
    }

    fn prepare_table_truncate_statement(
        &self,
        _truncate: &TableTruncateStatement,
        _sql: &mut dyn SqlWriter,
    ) {
        panic!("Sqlite doesn't support TRUNCATE statement")
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        }
        if alter.options.len() > 1 {
            panic!("Sqlite doesn't support multiple alter options")
        }
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }
        match &alter.options[0] {
            TableAlterOption::AddColumn(AddColumnOption {
                column,
                if_not_exists: _,
            }) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column, sql);
            }
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql.as_writer(), self.quote());
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropColumn(col_name) => {
                write!(sql, "DROP COLUMN ").unwrap();
                col_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
            TableAlterOption::AddForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
        }
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}

impl SqliteQueryBuilder {
    fn prepare_column_type(
        &self,
        column_specs: &[ColumnSpec],
        column_type: &ColumnType,
        sql: &mut dyn SqlWriter,
    ) {
        let is_auto_increment = column_specs
            .iter()
            .any(|s| matches!(s, ColumnSpec::AutoIncrement));
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("char({length})"),
                    None => "char".into(),
                },
                ColumnType::String(length) => match length {
                    StringLen::N(length) => format!("varchar({length})"),
                    _ => "varchar".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger | ColumnType::TinyUnsigned => integer("tinyint").into(),
                ColumnType::SmallInteger | ColumnType::SmallUnsigned => integer("smallint").into(),
                ColumnType::Integer | ColumnType::Unsigned => "integer".into(),
                #[allow(clippy::if_same_then_else)]
                ColumnType::BigInteger | ColumnType::BigUnsigned => if is_auto_increment {
                    "integer"
                } else {
                    integer("bigint")
                }
                .into(),
                ColumnType::Float => "float".into(),
                ColumnType::Double => "double".into(),
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => {
                        if precision > &16 {
                            panic!("precision cannot be larger than 16");
                        }
                        format!("real({precision}, {scale})")
                    }
                    None => "real".into(),
                },
                ColumnType::DateTime => "datetime_text".into(),
                ColumnType::Timestamp => "timestamp_text".into(),
                ColumnType::TimestampWithTimeZone => "timestamp_with_timezone_text".into(),
                ColumnType::Time => "time_text".into(),
                ColumnType::Date => "date_text".into(),
                ColumnType::Interval(_, _) =>
                    unimplemented!("Interval is not available in Sqlite."),
                ColumnType::Binary(length) => format!("blob({length})"),
                ColumnType::VarBinary(length) => match length {
                    StringLen::N(length) => format!("varbinary_blob({length})"),
                    _ => "varbinary_blob".into(),
                },
                ColumnType::Blob => "blob".into(),
                ColumnType::Boolean => "boolean".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("real_money({precision}, {scale})"),
                    None => "real_money".into(),
                },
                ColumnType::Json => "json_text".into(),
                ColumnType::JsonBinary => "jsonb_text".into(),
                ColumnType::Uuid => "uuid_text".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { .. } => "enum_text".into(),
                ColumnType::Array(_) => unimplemented!("Array is not available in Sqlite."),
                ColumnType::Vector(_) => unimplemented!("Vector is not available in Sqlite."),
                ColumnType::Cidr => unimplemented!("Cidr is not available in Sqlite."),
                ColumnType::Inet => unimplemented!("Inet is not available in Sqlite."),
                ColumnType::MacAddr => unimplemented!("MacAddr is not available in Sqlite."),
                ColumnType::Year => unimplemented!("Year is not available in Sqlite."),
                ColumnType::Bit(_) => unimplemented!("Bit is not available in Sqlite."),
                ColumnType::VarBit(_) => unimplemented!("VarBit is not available in Sqlite."),
                ColumnType::LTree => unimplemented!("LTree is not available in Sqlite."),
            }
        )
        .unwrap()
    }
}

fn integer(ty: &str) -> &str {
    if cfg!(feature = "option-sqlite-exact-column-type") {
        "integer"
    } else {
        ty
    }
}
