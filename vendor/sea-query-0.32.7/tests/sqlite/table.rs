use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(
                ColumnDef::new(Glyph::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Glyph::Aspect).double().not_null())
            .col(ColumnDef::new(Glyph::Image).text())
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "glyph" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""aspect" double NOT NULL,"#,
            r#""image" text"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Table::create()
            .table(Font::Table)
            .col(
                ColumnDef::new(Font::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
            )
            .col(ColumnDef::new(Font::Name).string().not_null())
            .col(ColumnDef::new(Font::Variant).string().not_null())
            .col(ColumnDef::new(Font::Language).string().not_null())
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "font" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""name" varchar NOT NULL,"#,
            r#""variant" varchar NOT NULL,"#,
            r#""language" varchar NOT NULL"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Char::FontSize).integer().not_null())
            .col(ColumnDef::new(Char::Character).string().not_null())
            .col(ColumnDef::new(Char::SizeW).unsigned().not_null())
            .col(ColumnDef::new(Char::SizeH).unsigned().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None))
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE IF NOT EXISTS "character" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""font_size" integer NOT NULL,"#,
            r#""character" varchar NOT NULL,"#,
            r#""size_w" integer NOT NULL,"#,
            r#""size_h" integer NOT NULL,"#,
            r#""font_id" integer DEFAULT NULL,"#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Table::create()
            .table(BinaryType::Table)
            .col(ColumnDef::new(BinaryType::BinaryLen).binary_len(32))
            .col(ColumnDef::new(BinaryType::Binary).binary())
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "binary_type" ("#,
            r#""binlen" blob(32),"#,
            r#""bin" blob(1)"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_5() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .col(ColumnDef::new(Char::Character).binary())
            .col(ColumnDef::new(Char::FontSize).binary_len(10))
            .col(ColumnDef::new(Char::SizeW).var_binary(10))
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "character" ("#,
            r#""character" blob(1),"#,
            r#""font_size" blob(10),"#,
            r#""size_w" varbinary_blob(10)"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_6() {
    assert_eq!(
        Table::create()
            .table(Task::Table)
            .col(
                ColumnDef::new(Task::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Task::IsDone).boolean().not_null())
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "task" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""is_done" boolean NOT NULL"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_7() {
    assert_eq!(
        Table::create()
            .table(Task::Table)
            .col(
                ColumnDef::new(Task::Id)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE "task" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_8() {
    assert_eq!(
        Table::create()
            .table(Font::Table)
            .temporary()
            .col(
                ColumnDef::new(Font::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
            )
            .col(ColumnDef::new(Font::Name).string().not_null())
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TEMPORARY TABLE "font" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""name" varchar NOT NULL"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_with_unique_index() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Char::FontSize).integer().not_null())
            .col(ColumnDef::new(Char::Character).string().not_null())
            .col(ColumnDef::new(Char::SizeW).integer().not_null())
            .col(ColumnDef::new(Char::SizeH).integer().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None))
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            )
            .index(Index::create().unique().col(Char::SizeH).col(Char::SizeW))
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE IF NOT EXISTS "character" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""font_size" integer NOT NULL,"#,
            r#""character" varchar NOT NULL,"#,
            r#""size_w" integer NOT NULL,"#,
            r#""size_h" integer NOT NULL,"#,
            r#""font_id" integer DEFAULT NULL,"#,
            r#"UNIQUE ("size_h", "size_w"),"#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_with_primary_unique_index() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
            )
            .col(ColumnDef::new(Char::FontSize).integer().not_null())
            .col(ColumnDef::new(Char::Character).string().not_null())
            .col(ColumnDef::new(Char::SizeW).integer().not_null())
            .col(ColumnDef::new(Char::SizeH).integer().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None))
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            )
            .index(Index::create().unique().primary().col(Char::SizeH).col(Char::SizeW))
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE IF NOT EXISTS "character" ("#,
            r#""id" integer NOT NULL,"#,
            r#""font_size" integer NOT NULL,"#,
            r#""character" varchar NOT NULL,"#,
            r#""size_w" integer NOT NULL,"#,
            r#""size_h" integer NOT NULL,"#,
            r#""font_id" integer DEFAULT NULL,"#,
            r#"PRIMARY KEY ("size_h", "size_w"),"#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_with_unique_index_constraint() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Char::FontSize).integer().not_null())
            .col(ColumnDef::new(Char::Character).string().not_null())
            .col(ColumnDef::new(Char::SizeW).integer().not_null())
            .col(ColumnDef::new(Char::SizeH).integer().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None)),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx-sizehw")
                    .table(Char::Table)
                    .col(Char::SizeH)
                    .col(Char::SizeW)
                    .unique(),
            )
            .to_string(SqliteQueryBuilder),
        [
            r#"CREATE TABLE IF NOT EXISTS "character" ("#,
            r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""font_size" integer NOT NULL,"#,
            r#""character" varchar NOT NULL,"#,
            r#""size_w" integer NOT NULL,"#,
            r#""size_h" integer NOT NULL,"#,
            r#""font_id" integer DEFAULT NULL,"#,
            r#"CONSTRAINT "idx-sizehw" UNIQUE ("size_h", "size_w"),"#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
            r#")"#,
        ].join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Table::drop()
            .table(Glyph::Table)
            .table(Char::Table)
            .cascade()
            .to_string(SqliteQueryBuilder),
        r#"DROP TABLE "glyph", "character""#
    );
}

#[test]
#[should_panic(expected = "Sqlite doesn't support TRUNCATE statement")]
fn truncate_1() {
    assert_eq!(
        Table::truncate()
            .table(Font::Table)
            .to_string(SqliteQueryBuilder),
        r#"TRUNCATE TABLE "font""#
    );
}

#[test]
fn alter_1() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .add_column(ColumnDef::new("new_col").integer().not_null().default(99))
            .to_string(SqliteQueryBuilder),
        r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 99"#
    );
}

#[test]
#[should_panic(expected = "Sqlite not support modifying table column")]
fn alter_2() {
    Table::alter()
        .table(Font::Table)
        .modify_column(ColumnDef::new("new_col").double())
        .to_string(SqliteQueryBuilder);
}

#[test]
fn alter_3() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .rename_column("new_col", "new_column")
            .to_string(SqliteQueryBuilder),
        r#"ALTER TABLE "font" RENAME COLUMN "new_col" TO "new_column""#
    );
}

#[test]
fn alter_4() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .drop_column("new_column")
            .to_string(SqliteQueryBuilder),
        r#"ALTER TABLE "font" DROP COLUMN "new_column""#
    );
}

#[test]
fn alter_5() {
    assert_eq!(
        Table::rename()
            .table(Font::Table, "font_new")
            .to_string(SqliteQueryBuilder),
        r#"ALTER TABLE "font" RENAME TO "font_new""#
    );
}

#[test]
#[should_panic(expected = "No alter option found")]
fn alter_6() {
    Table::alter().to_string(SqliteQueryBuilder);
}

#[test]
#[should_panic(expected = "Sqlite doesn't support multiple alter options")]
fn alter_7() {
    let _ = Table::alter()
        .table(Font::Table)
        .add_column(ColumnDef::new("new_col").integer())
        .rename_column(Font::Name, "name_new")
        .drop_column("name_new")
        .to_string(SqliteQueryBuilder);
}

#[test]
fn create_with_check_constraint() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(
                ColumnDef::new(Glyph::Id)
                    .integer()
                    .not_null()
                    .check(Expr::col(Glyph::Id).gt(10))
            )
            .check(Expr::col(Glyph::Id).lt(20))
            .check(Expr::col(Glyph::Id).ne(15))
            .to_string(SqliteQueryBuilder),
        r#"CREATE TABLE "glyph" ( "id" integer NOT NULL CHECK ("id" > 10), CHECK ("id" < 20), CHECK ("id" <> 15) )"#,
    );
}

#[test]
fn alter_with_check_constraint() {
    assert_eq!(
        Table::alter()
            .table(Glyph::Table)
            .add_column(
                ColumnDef::new(Glyph::Aspect)
                    .integer()
                    .not_null()
                    .default(101)
                    .check(Expr::col(Glyph::Aspect).gt(100))
            )
            .to_string(SqliteQueryBuilder),
        r#"ALTER TABLE "glyph" ADD COLUMN "aspect" integer NOT NULL DEFAULT 101 CHECK ("aspect" > 100)"#,
    );
}
