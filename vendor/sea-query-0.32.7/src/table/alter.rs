use crate::{
    backend::SchemaBuilder, types::*, ColumnDef, IntoColumnDef, SchemaStatementBuilder,
    TableForeignKey,
};
use inherent::inherent;

/// Alter a table
///
/// # Examples
///
/// ```
/// use sea_query::{tests_cfg::*, *};
///
/// let table = Table::alter()
///     .table(Font::Table)
///     .add_column(ColumnDef::new("new_col").integer().not_null().default(100))
///     .to_owned();
///
/// assert_eq!(
///     table.to_string(MysqlQueryBuilder),
///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(PostgresQueryBuilder),
///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
/// );
/// assert_eq!(
///     table.to_string(SqliteQueryBuilder),
///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
/// );
/// ```
#[derive(Default, Debug, Clone)]
pub struct TableAlterStatement {
    pub(crate) table: Option<TableRef>,
    pub(crate) options: Vec<TableAlterOption>,
}

/// table alter add column options
#[derive(Debug, Clone)]
pub struct AddColumnOption {
    pub(crate) column: ColumnDef,
    pub(crate) if_not_exists: bool,
}

/// All available table alter options
#[derive(Debug, Clone)]
pub enum TableAlterOption {
    AddColumn(AddColumnOption),
    ModifyColumn(ColumnDef),
    RenameColumn(DynIden, DynIden),
    DropColumn(DynIden),
    AddForeignKey(TableForeignKey),
    DropForeignKey(DynIden),
}

impl TableAlterStatement {
    /// Construct alter table statement
    pub fn new() -> Self {
        Self::default()
    }

    /// Set table name
    pub fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Add a column to an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .add_column(ColumnDef::new("new_col").integer().not_null().default(100))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
    /// );
    /// ```
    pub fn add_column<C: IntoColumnDef>(&mut self, column_def: C) -> &mut Self {
        self.options
            .push(TableAlterOption::AddColumn(AddColumnOption {
                column: column_def.into_column_def(),
                if_not_exists: false,
            }));
        self
    }

    /// Try add a column to an existing table if it does not exists
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .add_column_if_not_exists(ColumnDef::new("new_col").integer().not_null().default(100))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` ADD COLUMN IF NOT EXISTS `new_col` int NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN IF NOT EXISTS "new_col" integer NOT NULL DEFAULT 100"#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
    /// );
    /// ```
    pub fn add_column_if_not_exists<C: IntoColumnDef>(&mut self, column_def: C) -> &mut Self {
        self.options
            .push(TableAlterOption::AddColumn(AddColumnOption {
                column: column_def.into_column_def(),
                if_not_exists: true,
            }));
        self
    }

    /// Modify a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .modify_column(ColumnDef::new("new_col").big_integer().default(999))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` MODIFY COLUMN `new_col` bigint DEFAULT 999"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE "font""#,
    ///         r#"ALTER COLUMN "new_col" TYPE bigint,"#,
    ///         r#"ALTER COLUMN "new_col" SET DEFAULT 999"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// // Sqlite not support modifying table column
    /// ```
    pub fn modify_column<C: IntoColumnDef>(&mut self, column_def: C) -> &mut Self {
        self.add_alter_option(TableAlterOption::ModifyColumn(column_def.into_column_def()))
    }

    /// Rename a column in an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .rename_column("new_col", "new_column")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    /// );
    /// ```
    pub fn rename_column<T, R>(&mut self, from_name: T, to_name: R) -> &mut Self
    where
        T: IntoIden,
        R: IntoIden,
    {
        self.add_alter_option(TableAlterOption::RenameColumn(
            from_name.into_iden(),
            to_name.into_iden(),
        ))
    }

    /// Drop a column from an existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Font::Table)
    ///     .drop_column("new_column")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     r#"ALTER TABLE `font` DROP COLUMN `new_column`"#
    /// );
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     r#"ALTER TABLE "font" DROP COLUMN "new_column""#
    /// );
    /// assert_eq!(
    ///     table.to_string(SqliteQueryBuilder),
    ///     r#"ALTER TABLE "font" DROP COLUMN "new_column""#
    /// );
    /// ```
    pub fn drop_column<T>(&mut self, col_name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.add_alter_option(TableAlterOption::DropColumn(col_name.into_iden()))
    }

    /// Add a foreign key to existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let foreign_key_char = TableForeignKey::new()
    ///     .name("FK_character_glyph")
    ///     .from_tbl(Char::Table)
    ///     .from_col(Char::FontId)
    ///     .from_col(Char::Id)
    ///     .to_tbl(Glyph::Table)
    ///     .to_col(Char::FontId)
    ///     .to_col(Char::Id)
    ///     .on_delete(ForeignKeyAction::Cascade)
    ///     .on_update(ForeignKeyAction::Cascade)
    ///     .to_owned();
    ///
    /// let foreign_key_font = TableForeignKey::new()
    ///     .name("FK_character_font")
    ///     .from_tbl(Char::Table)
    ///     .from_col(Char::FontId)
    ///     .to_tbl(Font::Table)
    ///     .to_col(Font::Id)
    ///     .on_delete(ForeignKeyAction::Cascade)
    ///     .on_update(ForeignKeyAction::Cascade)
    ///     .to_owned();
    ///
    /// let table = Table::alter()
    ///     .table(Character::Table)
    ///     .add_foreign_key(&foreign_key_char)
    ///     .add_foreign_key(&foreign_key_font)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE `character`"#,
    ///         r#"ADD CONSTRAINT `FK_character_glyph`"#,
    ///         r#"FOREIGN KEY (`font_id`, `id`) REFERENCES `glyph` (`font_id`, `id`)"#,
    ///         r#"ON DELETE CASCADE ON UPDATE CASCADE,"#,
    ///         r#"ADD CONSTRAINT `FK_character_font`"#,
    ///         r#"FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)"#,
    ///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE "character""#,
    ///         r#"ADD CONSTRAINT "FK_character_glyph""#,
    ///         r#"FOREIGN KEY ("font_id", "id") REFERENCES "glyph" ("font_id", "id")"#,
    ///         r#"ON DELETE CASCADE ON UPDATE CASCADE,"#,
    ///         r#"ADD CONSTRAINT "FK_character_font""#,
    ///         r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
    ///         r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// // Sqlite not support modifying table column
    /// ```
    pub fn add_foreign_key(&mut self, foreign_key: &TableForeignKey) -> &mut Self {
        self.add_alter_option(TableAlterOption::AddForeignKey(foreign_key.to_owned()))
    }

    /// Drop a foreign key from existing table
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::alter()
    ///     .table(Character::Table)
    ///     .drop_foreign_key("FK_character_glyph")
    ///     .drop_foreign_key("FK_character_font")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE `character`"#,
    ///         r#"DROP FOREIGN KEY `FK_character_glyph`,"#,
    ///         r#"DROP FOREIGN KEY `FK_character_font`"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE "character""#,
    ///         r#"DROP CONSTRAINT "FK_character_glyph","#,
    ///         r#"DROP CONSTRAINT "FK_character_font""#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// // Sqlite not support modifying table column
    /// ```
    pub fn drop_foreign_key<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.add_alter_option(TableAlterOption::DropForeignKey(name.into_iden()))
    }

    fn add_alter_option(&mut self, alter_option: TableAlterOption) -> &mut Self {
        self.options.push(alter_option);
        self
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            options: std::mem::take(&mut self.options),
        }
    }
}

#[inherent]
impl SchemaStatementBuilder for TableAlterStatement {
    pub fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_alter_statement(self, &mut sql);
        sql
    }

    pub fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_alter_statement(self, &mut sql);
        sql
    }

    pub fn to_string<T: SchemaBuilder>(&self, schema_builder: T) -> String;
}
