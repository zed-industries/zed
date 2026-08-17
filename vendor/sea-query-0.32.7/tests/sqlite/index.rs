use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Index::create()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .to_string(SqliteQueryBuilder),
        r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Index::create()
            .unique()
            .name("idx-glyph-aspect-image")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(SqliteQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Index::create()
            .if_not_exists()
            .unique()
            .name("idx-glyph-aspect-image")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(SqliteQueryBuilder),
        r#"CREATE UNIQUE INDEX IF NOT EXISTS "idx-glyph-aspect-image" ON "glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Index::create()
            .if_not_exists()
            .unique()
            .name("partial-index-glyph-image-not-null")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .and_where(Expr::col(Glyph::Image).is_not_null())
            .to_string(SqliteQueryBuilder),
        r#"CREATE UNIQUE INDEX IF NOT EXISTS "partial-index-glyph-image-not-null" ON "glyph" ("image") WHERE "image" IS NOT NULL"#
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .to_string(SqliteQueryBuilder),
        r#"DROP INDEX "idx-glyph-aspect""#
    );
}
