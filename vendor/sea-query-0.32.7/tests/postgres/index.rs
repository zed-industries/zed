use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Index::create()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .to_string(PostgresQueryBuilder),
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
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Index::create()
            .full_text()
            .name("idx-glyph-image")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX "idx-glyph-image" ON "glyph" USING GIN ("image")"#
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Index::create()
            .if_not_exists()
            .full_text()
            .name("idx-glyph-image")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX IF NOT EXISTS "idx-glyph-image" ON "glyph" USING GIN ("image")"#
    );
}

#[test]
fn create_5() {
    assert_eq!(
        Index::create()
            .unique()
            .name("idx-glyph-aspect-image")
            .table(("schema", Glyph::Table))
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "schema"."glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_6() {
    assert_eq!(
        Index::create()
            .unique()
            .nulls_not_distinct()
            .name("idx-glyph-aspect-image")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "glyph" ("aspect", "image") NULLS NOT DISTINCT"#
    );
}

#[test]
fn create_7() {
    assert_eq!(
        Index::create()
            .unique()
            .nulls_not_distinct()
            .name("partial-index-glyph-image-not-null")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .and_where(Expr::col(Glyph::Image).is_not_null())
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "partial-index-glyph-image-not-null" ON "glyph" ("image") NULLS NOT DISTINCT WHERE "image" IS NOT NULL"#
    );
}

#[test]
fn create_8() {
    assert_eq!(
        Index::create()
            .name("idx-font-name-include-id-language")
            .table(Font::Table)
            .col(Font::Name)
            .include(Font::Id)
            .include(Font::Language)
            .unique()
            .nulls_not_distinct()
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-font-name-include-id-language" ON "font" ("name") INCLUDE ("id", "language") NULLS NOT DISTINCT"#
    );
}

#[test]
fn create_9() {
    assert_eq!(
        Index::create()
            .name("idx-character-area")
            .table(Character::Table)
            .col(Expr::col(Character::SizeH).mul(Expr::col(Character::SizeW)))
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX "idx-character-area" ON "character" (("size_h" * "size_w"))"#
    )
}

#[test]
fn create_10() {
    assert_eq!(
        Index::create()
            .name("idx-character-character-area-desc-created_at")
            .table(Character::Table)
            .col(Func::upper(Expr::col(Character::Character)))
            .col((
                Expr::col(Character::SizeH).mul(Expr::col(Character::SizeW)),
                IndexOrder::Desc,
            ))
            .col(Character::CreatedAt)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX "idx-character-character-area-desc-created_at" ON "character" ((UPPER("character")), ("size_h" * "size_w") DESC, "created_at")"#
    )
}

#[test]
fn drop_1() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "idx-glyph-aspect""#
    );
}

#[test]
fn drop_2() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table(("schema", Glyph::Table))
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "schema"."idx-glyph-aspect""#
    );
}

#[test]
fn drop_3() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "idx-glyph-aspect""#
    );
}

#[test]
#[should_panic(expected = "Not supported")]
fn drop_4() {
    Index::drop()
        .name("idx-glyph-aspect")
        .table(("database", "schema", Glyph::Table))
        .to_string(PostgresQueryBuilder);
}
