use sqlx_mysql::MySql;
use sqlx_test::new;

#[sqlx::test]
async fn test_derive_strong_enum() -> anyhow::Result<()> {
    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "PascalCase")]
    enum PascalCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "camelCase")]
    enum CamelCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "snake_case")]
    enum SnakeCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
    enum ScreamingSnakeCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "kebab-case")]
    enum KebabCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "lowercase")]
    enum LowerCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    #[sqlx(rename_all = "UPPERCASE")]
    enum UpperCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::Type, PartialEq, Eq, Debug)]
    enum DefaultCaseEnum {
        FooFoo,
        BarBar,
        BazBaz,
    }

    #[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
    struct StrongEnumRow {
        pascal_case: PascalCaseEnum,
        camel_case: CamelCaseEnum,
        snake_case: SnakeCaseEnum,
        screaming_snake_case: ScreamingSnakeCaseEnum,
        kebab_case: KebabCaseEnum,
        lowercase: LowerCaseEnum,
        uppercase: UpperCaseEnum,
        default_case: DefaultCaseEnum,
    }

    let mut conn = new::<MySql>().await?;

    sqlx::raw_sql(
        r#"
            CREATE TEMPORARY TABLE strong_enum (
                pascal_case ENUM('FooFoo', 'BarBar', 'BazBaz'),
                camel_case ENUM('fooFoo', 'barBar', 'bazBaz'),
                snake_case ENUM('foo_foo', 'bar_bar', 'baz_baz'),
                screaming_snake_case ENUM('FOO_FOO', 'BAR_BAR', 'BAZ_BAZ'),
                kebab_case ENUM('foo-foo', 'bar-bar', 'baz-baz'),
                lowercase ENUM('foofoo', 'barbar', 'bazbaz'),
                uppercase ENUM('FOOFOO', 'BARBAR', 'BAZBAZ'),
                default_case ENUM('FooFoo', 'BarBar', 'BazBaz')
            );
        "#,
    )
    .execute(&mut conn)
    .await?;

    let input = StrongEnumRow {
        pascal_case: PascalCaseEnum::FooFoo,
        camel_case: CamelCaseEnum::BarBar,
        snake_case: SnakeCaseEnum::BazBaz,
        screaming_snake_case: ScreamingSnakeCaseEnum::FooFoo,
        kebab_case: KebabCaseEnum::BarBar,
        lowercase: LowerCaseEnum::BazBaz,
        uppercase: UpperCaseEnum::FooFoo,
        default_case: DefaultCaseEnum::BarBar,
    };

    sqlx::query(
        r#"
            INSERT INTO strong_enum(
                pascal_case,
                camel_case,
                snake_case,
                screaming_snake_case,
                kebab_case,
                lowercase,
                uppercase,
                default_case
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&input.pascal_case)
    .bind(&input.camel_case)
    .bind(&input.snake_case)
    .bind(&input.screaming_snake_case)
    .bind(&input.kebab_case)
    .bind(&input.lowercase)
    .bind(&input.uppercase)
    .bind(&input.default_case)
    .execute(&mut conn)
    .await?;

    let output: StrongEnumRow = sqlx::query_as("SELECT * FROM strong_enum")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(input, output);

    Ok(())
}

#[sqlx::test]
async fn test_derive_weak_enum() -> anyhow::Result<()> {
    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(i8)]
    enum WeakEnumI8 {
        Foo = i8::MIN,
        Bar = 0,
        Baz = i8::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(i16)]
    enum WeakEnumI16 {
        Foo = i16::MIN,
        Bar = 0,
        Baz = i16::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(i32)]
    enum WeakEnumI32 {
        Foo = i32::MIN,
        Bar = 0,
        Baz = i32::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(i64)]
    enum WeakEnumI64 {
        Foo = i64::MIN,
        Bar = 0,
        Baz = i64::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum WeakEnumU8 {
        Foo = 0,
        Bar = 1,
        Baz = u8::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(u16)]
    enum WeakEnumU16 {
        Foo = 0,
        Bar = 1,
        Baz = u16::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(u32)]
    enum WeakEnumU32 {
        Foo = 0,
        Bar = 1,
        Baz = u32::MAX,
    }

    #[derive(sqlx::Type, Debug, PartialEq, Eq)]
    #[repr(u64)]
    enum WeakEnumU64 {
        Foo = 0,
        Bar = 1,
        Baz = u64::MAX,
    }

    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct WeakEnumRow {
        i8: WeakEnumI8,
        i16: WeakEnumI16,
        i32: WeakEnumI32,
        i64: WeakEnumI64,
        u8: WeakEnumU8,
        u16: WeakEnumU16,
        u32: WeakEnumU32,
        u64: WeakEnumU64,
    }

    let mut conn = new::<MySql>().await?;

    sqlx::raw_sql(
        r#"
            CREATE TEMPORARY TABLE weak_enum (
                i8 TINYINT,
                i16 SMALLINT,
                i32 INT,
                i64 BIGINT,
                u8 TINYINT UNSIGNED,
                u16 SMALLINT UNSIGNED,
                u32 INT UNSIGNED,
                u64 BIGINT UNSIGNED
            )
        "#,
    )
    .execute(&mut conn)
    .await?;

    let rows_in = vec![
        WeakEnumRow {
            i8: WeakEnumI8::Foo,
            i16: WeakEnumI16::Foo,
            i32: WeakEnumI32::Foo,
            i64: WeakEnumI64::Foo,
            u8: WeakEnumU8::Foo,
            u16: WeakEnumU16::Foo,
            u32: WeakEnumU32::Foo,
            u64: WeakEnumU64::Foo,
        },
        WeakEnumRow {
            i8: WeakEnumI8::Bar,
            i16: WeakEnumI16::Bar,
            i32: WeakEnumI32::Bar,
            i64: WeakEnumI64::Bar,
            u8: WeakEnumU8::Bar,
            u16: WeakEnumU16::Bar,
            u32: WeakEnumU32::Bar,
            u64: WeakEnumU64::Bar,
        },
        WeakEnumRow {
            i8: WeakEnumI8::Baz,
            i16: WeakEnumI16::Baz,
            i32: WeakEnumI32::Baz,
            i64: WeakEnumI64::Baz,
            u8: WeakEnumU8::Baz,
            u16: WeakEnumU16::Baz,
            u32: WeakEnumU32::Baz,
            u64: WeakEnumU64::Baz,
        },
    ];

    for row in &rows_in {
        sqlx::query(
            r#"
                INSERT INTO weak_enum(i8, i16, i32, i64, u8, u16, u32, u64)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&row.i8)
        .bind(&row.i16)
        .bind(&row.i32)
        .bind(&row.i64)
        .bind(&row.u8)
        .bind(&row.u16)
        .bind(&row.u32)
        .bind(&row.u64)
        .execute(&mut conn)
        .await?;
    }

    let rows_out: Vec<WeakEnumRow> = sqlx::query_as("SELECT * FROM weak_enum")
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(rows_in, rows_out);

    Ok(())
}
