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
            .engine("InnoDB")
            .character_set("utf8mb4")
            .collate("utf8mb4_unicode_ci")
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `glyph` (",
            "`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,",
            "`aspect` double NOT NULL,",
            "`image` text",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
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
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Font::Name).string().not_null())
            .col(ColumnDef::new(Font::Variant).string_len(255).not_null())
            .col(ColumnDef::new(Font::Language).string_len(1024).not_null())
            .engine("InnoDB")
            .character_set("utf8mb4")
            .collate("utf8mb4_unicode_ci")
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `font` (",
            "`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,",
            "`name` varchar(255) NOT NULL,",
            "`variant` varchar(255) NOT NULL,",
            "`language` varchar(1024) NOT NULL",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
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
            .col(ColumnDef::new(Char::Character).string_len(255).not_null())
            .col(ColumnDef::new(Char::SizeW).unsigned().not_null())
            .col(ColumnDef::new(Char::SizeH).unsigned().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None))
            )
            .col(
                ColumnDef::new(Char::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key(
                ForeignKey::create()
                    .name("FK_2e303c3a712662f1fc2a4d0aad6")
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Restrict)
            )
            .engine("InnoDB")
            .character_set("utf8mb4")
            .collate("utf8mb4_unicode_ci")
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE IF NOT EXISTS `character` (",
            "`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,",
            "`font_size` int NOT NULL,",
            "`character` varchar(255) NOT NULL,",
            "`size_w` int UNSIGNED NOT NULL,",
            "`size_h` int UNSIGNED NOT NULL,",
            "`font_id` int DEFAULT NULL,",
            "`created_at` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,",
            "CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`",
            "FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)",
            "ON DELETE CASCADE ON UPDATE RESTRICT",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
        ]
        .join(" ")
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(
                ColumnDef::new(Glyph::Id)
                    .integer()
                    .not_null()
                    .extra("ANYTHING I WANT TO SAY".to_owned())
            )
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `glyph` (",
            "`id` int NOT NULL ANYTHING I WANT TO SAY",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_5() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(ColumnDef::new(Glyph::Id).integer().not_null())
            .index(Index::create().unique().name("idx-glyph-id").col(Glyph::Id))
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `glyph` (",
            "`id` int NOT NULL,",
            "UNIQUE KEY `idx-glyph-id` (`id`)",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_6() {
    assert_eq!(
        Table::create()
            .table(BinaryType::Table)
            .col(ColumnDef::new(BinaryType::BinaryLen).binary_len(32))
            .col(ColumnDef::new(BinaryType::Binary).binary())
            .col(ColumnDef::new(BinaryType::Blob).blob())
            .col(ColumnDef::new(BinaryType::TinyBlob).custom(MySqlType::TinyBlob))
            .col(ColumnDef::new(BinaryType::MediumBlob).custom(MySqlType::MediumBlob))
            .col(ColumnDef::new(BinaryType::LongBlob).custom(MySqlType::LongBlob))
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `binary_type` (",
            "`binlen` binary(32),",
            "`bin` binary(1),",
            "`b` blob,",
            "`tb` tinyblob,",
            "`mb` mediumblob,",
            "`lb` longblob",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_7() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .col(ColumnDef::new(BinaryType::Blob).blob())
            .col(ColumnDef::new(Char::Character).binary())
            .col(ColumnDef::new(Char::FontSize).binary_len(10))
            .col(ColumnDef::new(Char::SizeW).var_binary(10))
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE `character` (",
            "`b` blob,",
            "`character` binary(1),",
            "`font_size` binary(10),",
            "`size_w` varbinary(10)",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_8() {
    assert_eq!(
        Table::create()
            .table(Font::Table)
            .col(ColumnDef::new(Font::Variant).year())
            .to_string(MysqlQueryBuilder),
        "CREATE TABLE `font` ( `variant` year )"
    );
}

#[test]
fn create_9() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .index(Index::create().name("idx-glyph-id").col(Glyph::Id))
            .to_string(MysqlQueryBuilder),
        "CREATE TABLE `glyph` ( KEY `idx-glyph-id` (`id`) )"
    );
}

#[test]
fn create_10() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(ColumnDef::new(Glyph::Id).enumeration("tea", ["EverydayTea", "BreakfastTea"]),)
            .to_string(MysqlQueryBuilder),
        "CREATE TABLE `glyph` ( `id` ENUM('EverydayTea', 'BreakfastTea') )"
    );
}

#[test]
fn create_11() {
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
            .to_string(MysqlQueryBuilder),
        [
            r#"CREATE TEMPORARY TABLE `font` ("#,
            r#"`id` int NOT NULL PRIMARY KEY AUTO_INCREMENT,"#,
            r#"`name` varchar(255) NOT NULL"#,
            r#")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_12() {
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
            .col(ColumnDef::new(Char::Character).string_len(255).not_null())
            .col(ColumnDef::new(Char::SizeW).unsigned().not_null())
            .col(ColumnDef::new(Char::SizeH).unsigned().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Char::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .index(
                Index::create()
                    .name("idx-character-area")
                    .table(Character::Table)
                    .col(Expr::col(Character::SizeH).mul(Expr::col(Character::SizeW))),
            )
            .engine("InnoDB")
            .character_set("utf8mb4")
            .collate("utf8mb4_unicode_ci")
            .to_string(MysqlQueryBuilder),
        [
            "CREATE TABLE IF NOT EXISTS `character` (",
            "`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,",
            "`font_size` int NOT NULL,",
            "`character` varchar(255) NOT NULL,",
            "`size_w` int UNSIGNED NOT NULL,",
            "`size_h` int UNSIGNED NOT NULL,",
            "`font_id` int DEFAULT NULL,",
            "`created_at` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,",
            "KEY `idx-character-area` ((`size_h` * `size_w`))",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
        ]
        .join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Table::drop()
            .table(Glyph::Table)
            .table(Char::Table)
            .cascade()
            .to_string(MysqlQueryBuilder),
        "DROP TABLE `glyph`, `character` CASCADE"
    );
}

#[test]
fn truncate_1() {
    assert_eq!(
        Table::truncate()
            .table(Font::Table)
            .to_string(MysqlQueryBuilder),
        "TRUNCATE TABLE `font`"
    );
}

#[test]
fn alter_1() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .add_column(ColumnDef::new("new_col").integer().not_null().default(100))
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"
    );
}

#[test]
fn alter_2() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .modify_column(ColumnDef::new("new_col").big_integer().default(999))
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `font` MODIFY COLUMN `new_col` bigint DEFAULT 999"
    );
}

#[test]
fn alter_3() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .rename_column("new_col", "new_column")
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"
    );
}

#[test]
fn alter_4() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .drop_column("new_column")
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `font` DROP COLUMN `new_column`"
    );
}

#[test]
fn alter_5() {
    assert_eq!(
        Table::rename()
            .table(Font::Table, "font_new")
            .to_string(MysqlQueryBuilder),
        "RENAME TABLE `font` TO `font_new`"
    );
}

#[test]
#[should_panic(expected = "No alter option found")]
fn alter_6() {
    Table::alter().to_string(MysqlQueryBuilder);
}

#[test]
fn alter_7() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .drop_column("new_column")
            .rename_column(Font::Name, "name_new")
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `font` DROP COLUMN `new_column`, RENAME COLUMN `name` TO `name_new`"
    );
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
            .to_string(MysqlQueryBuilder),
        r"CREATE TABLE `glyph` ( `id` int NOT NULL CHECK (`id` > 10), CHECK (`id` < 20), CHECK (`id` <> 15) )",
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
            .to_string(MysqlQueryBuilder),
        r"ALTER TABLE `glyph` ADD COLUMN `aspect` int NOT NULL DEFAULT 101 CHECK (`aspect` > 100)",
    );
}
