use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from(Char::Table, Char::FontId)
            .to(Font::Table, Font::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(MysqlQueryBuilder),
        [
            "ALTER TABLE `character`",
            "ADD CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`",
            "FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)",
            "ON DELETE CASCADE ON UPDATE CASCADE",
        ]
        .join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        ForeignKey::drop()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .table(Char::Table)
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `character` DROP FOREIGN KEY `FK_2e303c3a712662f1fc2a4d0aad6`"
    );
}
