use super::*;
use pretty_assertions::assert_eq;
use sea_query::{extension::postgres::Type, PostgresQueryBuilder};

#[test]
fn create_1() {
    assert_eq!(
        Type::create()
            .as_enum(Font::Table)
            .values([Font::Name, Font::Variant, Font::Language])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "font" AS ENUM ('name', 'variant', 'language')"#
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Type::create()
            .as_enum(("schema", Font::Table))
            .values([Font::Name, Font::Variant, Font::Language])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "schema"."font" AS ENUM ('name', 'variant', 'language')"#
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Type::create()
            .as_enum(Tea::Enum)
            .values([Tea::EverydayTea, Tea::BreakfastTea])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "tea" AS ENUM ('EverydayTea', 'BreakfastTea')"#
    );

    enum Tea {
        Enum,
        EverydayTea,
        BreakfastTea,
    }

    impl sea_query::Iden for Tea {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            write!(
                s,
                "{}",
                match self {
                    Self::Enum => "tea",
                    Self::EverydayTea => "EverydayTea",
                    Self::BreakfastTea => "BreakfastTea",
                }
            )
            .unwrap();
        }
    }
}

#[test]
fn drop_1() {
    assert_eq!(
        Type::drop()
            .if_exists()
            .name(Font::Table)
            .restrict()
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE IF EXISTS "font" RESTRICT"#
    )
}

#[test]
fn drop_2() {
    assert_eq!(
        Type::drop()
            .name(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE "font""#
    );
}

#[test]
fn drop_3() {
    assert_eq!(
        Type::drop()
            .if_exists()
            .name(Font::Table)
            .cascade()
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE IF EXISTS "font" CASCADE"#
    );
}

#[test]
fn drop_4() {
    assert_eq!(
        Type::drop()
            .name(("schema", Font::Table))
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE "schema"."font""#
    );
}

#[test]
fn alter_1() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight'"#
    )
}
#[test]
fn alter_2() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .before(Font::Variant)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight' BEFORE 'variant'"#
    )
}

#[test]
fn alter_3() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .after(Font::Variant)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight' AFTER 'variant'"#
    )
}

#[test]
fn alter_4() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .rename_to("typeface")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" RENAME TO "typeface""#
    )
}

#[test]
fn alter_5() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .rename_value(Font::Variant, Font::Language)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" RENAME VALUE 'variant' TO 'language'"#
    )
}

#[test]
fn alter_6() {
    assert_eq!(
        Type::alter()
            .name(("schema", Font::Table))
            .rename_to("typeface")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "schema"."font" RENAME TO "typeface""#
    )
}
