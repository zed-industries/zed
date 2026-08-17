use super::*;

impl TableBuilder for PostgresQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        let f = |column_def: &ColumnDef, sql: &mut dyn SqlWriter| {
            self.prepare_column_type_check_auto_increment(column_def, sql);
        };
        self.prepare_column_def_common(column_def, sql, f);
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
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
                ColumnType::TinyInteger | ColumnType::TinyUnsigned => "smallint".into(),
                ColumnType::SmallInteger | ColumnType::SmallUnsigned => "smallint".into(),
                ColumnType::Integer | ColumnType::Unsigned => "integer".into(),
                ColumnType::BigInteger | ColumnType::BigUnsigned => "bigint".into(),
                ColumnType::Float => "real".into(),
                ColumnType::Double => "double precision".into(),
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("decimal({precision}, {scale})"),
                    None => "decimal".into(),
                },
                ColumnType::DateTime => "timestamp without time zone".into(),
                ColumnType::Timestamp => "timestamp".into(),
                ColumnType::TimestampWithTimeZone => "timestamp with time zone".into(),
                ColumnType::Time => "time".into(),
                ColumnType::Date => "date".into(),
                ColumnType::Interval(fields, precision) => {
                    let mut typ = "interval".to_string();
                    if let Some(fields) = fields {
                        write!(typ, " {fields}").unwrap();
                    }
                    if let Some(precision) = precision {
                        write!(typ, "({precision})").unwrap();
                    }
                    typ
                }
                ColumnType::Binary(_) | ColumnType::VarBinary(_) | ColumnType::Blob =>
                    "bytea".into(),
                ColumnType::Bit(length) => {
                    match length {
                        Some(length) => format!("bit({length})"),
                        None => "bit".into(),
                    }
                }
                ColumnType::VarBit(length) => {
                    format!("varbit({length})")
                }
                ColumnType::Boolean => "bool".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("money({precision}, {scale})"),
                    None => "money".into(),
                },
                ColumnType::Json => "json".into(),
                ColumnType::JsonBinary => "jsonb".into(),
                ColumnType::Uuid => "uuid".into(),
                ColumnType::Array(elem_type) => {
                    let mut sql = String::new();
                    self.prepare_column_type(elem_type, &mut sql);
                    format!("{sql}[]")
                }
                ColumnType::Vector(size) => match size {
                    Some(size) => format!("vector({size})"),
                    None => "vector".into(),
                },
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { name, .. } => name.to_string(),
                ColumnType::Cidr => "cidr".into(),
                ColumnType::Inet => "inet".into(),
                ColumnType::MacAddr => "macaddr".into(),
                ColumnType::Year => unimplemented!("Year is not available in Postgres."),
                ColumnType::LTree => "ltree".into(),
            }
        )
        .unwrap()
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        ""
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }

        alter.options.iter().fold(true, |first, option| {
            if !first {
                write!(sql, ", ").unwrap();
            };
            match option {
                TableAlterOption::AddColumn(AddColumnOption {
                    column,
                    if_not_exists,
                }) => {
                    write!(sql, "ADD COLUMN ").unwrap();
                    if *if_not_exists {
                        write!(sql, "IF NOT EXISTS ").unwrap();
                    }
                    let f = |column_def: &ColumnDef, sql: &mut dyn SqlWriter| {
                        if let Some(column_type) = &column_def.types {
                            write!(sql, " ").unwrap();
                            if column_def
                                .spec
                                .iter()
                                .any(|v| matches!(v, ColumnSpec::AutoIncrement))
                            {
                                self.prepare_column_auto_increment(column_type, sql);
                            } else {
                                self.prepare_column_type(column_type, sql);
                            }
                        }
                    };
                    self.prepare_column_def_common(column, sql, f);
                }
                TableAlterOption::ModifyColumn(column_def) => {
                    if let Some(column_type) = &column_def.types {
                        write!(sql, "ALTER COLUMN ").unwrap();
                        column_def.name.prepare(sql.as_writer(), self.quote());
                        write!(sql, " TYPE ").unwrap();
                        self.prepare_column_type(column_type, sql);
                    }
                    let first = column_def.types.is_none();

                    column_def.spec.iter().fold(first, |first, column_spec| {
                        if !first
                            && !matches!(
                                column_spec,
                                ColumnSpec::AutoIncrement
                                    | ColumnSpec::Generated { .. }
                                    | ColumnSpec::Using(_)
                            )
                        {
                            write!(sql, ", ").unwrap();
                        }
                        match column_spec {
                            ColumnSpec::AutoIncrement => {}
                            ColumnSpec::Null => {
                                write!(sql, "ALTER COLUMN ").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, " DROP NOT NULL").unwrap();
                            }
                            ColumnSpec::NotNull => {
                                write!(sql, "ALTER COLUMN ").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, " SET NOT NULL").unwrap()
                            }
                            ColumnSpec::Default(v) => {
                                write!(sql, "ALTER COLUMN ").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, " SET DEFAULT ").unwrap();
                                QueryBuilder::prepare_simple_expr(self, v, sql);
                            }
                            ColumnSpec::UniqueKey => {
                                write!(sql, "ADD UNIQUE (").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, ")").unwrap();
                            }
                            ColumnSpec::PrimaryKey => {
                                write!(sql, "ADD PRIMARY KEY (").unwrap();
                                column_def.name.prepare(sql.as_writer(), self.quote());
                                write!(sql, ")").unwrap();
                            }
                            ColumnSpec::Check(check) => self.prepare_check_constraint(check, sql),
                            ColumnSpec::Generated { .. } => {}
                            ColumnSpec::Extra(string) => write!(sql, "{string}").unwrap(),
                            ColumnSpec::Comment(_) => {}
                            ColumnSpec::Using(expr) => {
                                write!(sql, " USING ").unwrap();
                                QueryBuilder::prepare_simple_expr(self, expr, sql);
                            }
                        }
                        false
                    });
                }
                TableAlterOption::RenameColumn(from_name, to_name) => {
                    write!(sql, "RENAME COLUMN ").unwrap();
                    from_name.prepare(sql.as_writer(), self.quote());
                    write!(sql, " TO ").unwrap();
                    to_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropColumn(column_name) => {
                    write!(sql, "DROP COLUMN ").unwrap();
                    column_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropForeignKey(name) => {
                    let mut foreign_key = TableForeignKey::new();
                    foreign_key.name(name.to_string());
                    let drop = ForeignKeyDropStatement {
                        foreign_key,
                        table: None,
                    };
                    self.prepare_foreign_key_drop_statement_internal(&drop, sql, Mode::TableAlter);
                }
                TableAlterOption::AddForeignKey(foreign_key) => {
                    let create = ForeignKeyCreateStatement {
                        foreign_key: foreign_key.to_owned(),
                    };
                    self.prepare_foreign_key_create_statement_internal(
                        &create,
                        sql,
                        Mode::TableAlter,
                    );
                }
            }
            false
        });
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

impl PostgresQueryBuilder {
    fn prepare_column_auto_increment(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        match &column_type {
            ColumnType::SmallInteger => write!(sql, "smallserial").unwrap(),
            ColumnType::Integer => write!(sql, "serial").unwrap(),
            ColumnType::BigInteger => write!(sql, "bigserial").unwrap(),
            _ => unimplemented!("{:?} doesn't support auto increment", column_type),
        }
    }

    fn prepare_column_type_check_auto_increment(
        &self,
        column_def: &ColumnDef,
        sql: &mut dyn SqlWriter,
    ) {
        if let Some(column_type) = &column_def.types {
            let is_auto_increment = column_def
                .spec
                .iter()
                .position(|s| matches!(s, ColumnSpec::AutoIncrement));
            if is_auto_increment.is_some() {
                write!(sql, " ").unwrap();
                self.prepare_column_auto_increment(column_type, sql);
            } else {
                write!(sql, " ").unwrap();
                self.prepare_column_type(column_type, sql);
            }
        }
    }

    fn prepare_column_def_common<F>(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter, f: F)
    where
        F: Fn(&ColumnDef, &mut dyn SqlWriter),
    {
        column_def.name.prepare(sql.as_writer(), self.quote());

        f(column_def, sql);

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::AutoIncrement = column_spec {
                continue;
            }
            if let ColumnSpec::Comment(_) = column_spec {
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }
}
