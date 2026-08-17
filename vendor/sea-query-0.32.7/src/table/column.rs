use std::fmt;

use crate::{expr::*, types::*};

/// Specification of a table column
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub(crate) table: Option<TableRef>,
    pub(crate) name: DynIden,
    pub(crate) types: Option<ColumnType>,
    pub(crate) spec: Vec<ColumnSpec>,
}

pub trait IntoColumnDef {
    fn into_column_def(self) -> ColumnDef;
}

/// All column types
///
/// | ColumnType            | MySQL data type   | PostgreSQL data type        | SQLite data type             |
/// |-----------------------|-------------------|-----------------------------|------------------------------|
/// | Char                  | char              | char                        | char                         |
/// | String                | varchar           | varchar                     | varchar                      |
/// | Text                  | text              | text                        | text                         |
/// | TinyInteger           | tinyint           | smallint                    | tinyint                      |
/// | SmallInteger          | smallint          | smallint                    | smallint                     |
/// | Integer               | int               | integer                     | integer                      |
/// | BigInteger            | bigint            | bigint                      | integer                      |
/// | TinyUnsigned          | tinyint unsigned  | smallint                    | tinyint                      |
/// | SmallUnsigned         | smallint unsigned | smallint                    | smallint                     |
/// | Unsigned              | int unsigned      | integer                     | integer                      |
/// | BigUnsigned           | bigint unsigned   | bigint                      | integer                      |
/// | Float                 | float             | real                        | float                        |
/// | Double                | double            | double precision            | double                       |
/// | Decimal               | decimal           | decimal                     | real                         |
/// | DateTime              | datetime          | timestamp without time zone | datetime_text                |
/// | Timestamp             | timestamp         | timestamp                   | timestamp_text               |
/// | TimestampWithTimeZone | timestamp         | timestamp with time zone    | timestamp_with_timezone_text |
/// | Time                  | time              | time                        | time_text                    |
/// | Date                  | date              | date                        | date_text                    |
/// | Year                  | year              | N/A                         | N/A                          |
/// | Interval              | N/A               | interval                    | N/A                          |
/// | Blob                  | blob              | bytea                       | blob                         |
/// | Binary                | binary            | bytea                       | blob                         |
/// | VarBinary             | varbinary         | bytea                       | varbinary_blob               |
/// | Bit                   | bit               | bit                         | N/A                          |
/// | VarBit                | bit               | varbit                      | N/A                          |
/// | Boolean               | bool              | bool                        | boolean                      |
/// | Money                 | decimal           | money                       | real_money                   |
/// | Json                  | json              | json                        | json_text                    |
/// | JsonBinary            | json              | jsonb                       | jsonb_text                   |
/// | Uuid                  | binary(16)        | uuid                        | uuid_text                    |
/// | Enum                  | ENUM(...)         | ENUM_NAME                   | enum_text                    |
/// | Array                 | N/A               | DATA_TYPE[]                 | N/A                          |
/// | Vector                | N/A               | vector                      | N/A                          |
/// | Cidr                  | N/A               | cidr                        | N/A                          |
/// | Inet                  | N/A               | inet                        | N/A                          |
/// | MacAddr               | N/A               | macaddr                     | N/A                          |
/// | LTree                 | N/A               | ltree                       | N/A                          |
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ColumnType {
    Char(Option<u32>),
    String(StringLen),
    Text,
    Blob,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    Decimal(Option<(u32, u32)>),
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Time,
    Date,
    Year,
    Interval(Option<PgInterval>, Option<u32>),
    Binary(u32),
    VarBinary(StringLen),
    Bit(Option<u32>),
    VarBit(u32),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Uuid,
    Custom(DynIden),
    Enum {
        name: DynIden,
        variants: Vec<DynIden>,
    },
    Array(RcOrArc<ColumnType>),
    Vector(Option<u32>),
    Cidr,
    Inet,
    MacAddr,
    LTree,
}

/// Length for var-char/binary; default to 255
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StringLen {
    /// String size
    N(u32),
    Max,
    #[default]
    None,
}

impl PartialEq for ColumnType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            (Self::Interval(l0, l1), Self::Interval(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Binary(l0), Self::Binary(r0)) => l0 == r0,
            (Self::VarBinary(l0), Self::VarBinary(r0)) => l0 == r0,
            (Self::Bit(l0), Self::Bit(r0)) => l0 == r0,
            (Self::VarBit(l0), Self::VarBit(r0)) => l0 == r0,
            (Self::Money(l0), Self::Money(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => l0.to_string() == r0.to_string(),
            (
                Self::Enum {
                    name: l_name,
                    variants: l_variants,
                },
                Self::Enum {
                    name: r_name,
                    variants: r_variants,
                },
            ) => {
                l_name.to_string() == r_name.to_string()
                    && l_variants
                        .iter()
                        .map(|v| v.to_string())
                        .eq(r_variants.iter().map(|v| v.to_string()))
            }
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl ColumnType {
    pub fn custom<T>(ty: T) -> ColumnType
    where
        T: Into<String>,
    {
        ColumnType::Custom(Alias::new(ty).into_iden())
    }

    pub fn string(length: Option<u32>) -> ColumnType {
        match length {
            Some(s) => ColumnType::String(StringLen::N(s)),
            None => ColumnType::String(StringLen::None),
        }
    }

    pub fn var_binary(length: u32) -> ColumnType {
        ColumnType::VarBinary(StringLen::N(length))
    }
}

/// All column specification keywords
#[derive(Debug, Clone)]
pub enum ColumnSpec {
    Null,
    NotNull,
    Default(SimpleExpr),
    AutoIncrement,
    UniqueKey,
    PrimaryKey,
    Check(SimpleExpr),
    Generated { expr: SimpleExpr, stored: bool },
    Extra(String),
    Comment(String),
    Using(SimpleExpr),
}

// All interval fields
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgInterval {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    YearToMonth,
    DayToHour,
    DayToMinute,
    DayToSecond,
    HourToMinute,
    HourToSecond,
    MinuteToSecond,
}

// All possible inputs to DATE_TRUNC (https://www.postgresql.org/docs/current/functions-datetime.html#FUNCTIONS-DATETIME-TRUNC)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgDateTruncUnit {
    Microseconds,
    Milliseconds,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
    Decade,
    Century,
    Millennium,
}

impl fmt::Display for PgDateTruncUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            PgDateTruncUnit::Microseconds => "microseconds",
            PgDateTruncUnit::Milliseconds => "milliseconds",
            PgDateTruncUnit::Second => "second",
            PgDateTruncUnit::Minute => "minute",
            PgDateTruncUnit::Hour => "hour",
            PgDateTruncUnit::Day => "day",
            PgDateTruncUnit::Week => "week",
            PgDateTruncUnit::Month => "month",
            PgDateTruncUnit::Quarter => "quarter",
            PgDateTruncUnit::Year => "year",
            PgDateTruncUnit::Decade => "decade",
            PgDateTruncUnit::Century => "century",
            PgDateTruncUnit::Millennium => "millennium",
        };
        write!(f, "{}", text)
    }
}

impl ColumnDef {
    /// Construct a table column
    pub fn new<T>(name: T) -> Self
    where
        T: IntoIden,
    {
        Self {
            table: None,
            name: name.into_iden(),
            types: None,
            spec: Vec::new(),
        }
    }

    /// Construct a table column with column type
    pub fn new_with_type<T>(name: T, types: ColumnType) -> Self
    where
        T: IntoIden,
    {
        Self {
            table: None,
            name: name.into_iden(),
            types: Some(types),
            spec: Vec::new(),
        }
    }

    /// Set column not null
    pub fn not_null(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::NotNull);
        self
    }

    /// Set column null
    pub fn null(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::Null);
        self
    }

    /// Set default expression of a column
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let table = Table::create()
    ///     .table(Char::Table)
    ///     .col(ColumnDef::new(Char::FontId).integer().default(12i32))
    ///     .col(
    ///         ColumnDef::new(Char::CreatedAt)
    ///             .timestamp()
    ///             .default(Expr::current_timestamp())
    ///             .not_null(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     table.to_string(MysqlQueryBuilder),
    ///     [
    ///         "CREATE TABLE `character` (",
    ///         "`font_id` int DEFAULT 12,",
    ///         "`created_at` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL",
    ///         ")",
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "character" ("#,
    ///         r#""font_id" integer DEFAULT 12,"#,
    ///         r#""created_at" timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn default<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Default(value.into()));
        self
    }

    /// Set column auto increment
    pub fn auto_increment(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::AutoIncrement);
        self
    }

    /// Set column unique constraint
    pub fn unique_key(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::UniqueKey);
        self
    }

    /// Set column as primary key
    pub fn primary_key(&mut self) -> &mut Self {
        self.spec.push(ColumnSpec::PrimaryKey);
        self
    }

    /// Set column type as char with custom length
    pub fn char_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Char(Some(length)));
        self
    }

    /// Set column type as char
    pub fn char(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Char(None));
        self
    }

    /// Set column type as string with custom length
    pub fn string_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::String(StringLen::N(length)));
        self
    }

    /// Set column type as string
    pub fn string(&mut self) -> &mut Self {
        self.types = Some(ColumnType::String(Default::default()));
        self
    }

    /// Set column type as text
    pub fn text(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Text);
        self
    }

    /// Set column type as tiny_integer
    pub fn tiny_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyInteger);
        self
    }

    /// Set column type as small_integer
    pub fn small_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallInteger);
        self
    }

    /// Set column type as integer
    pub fn integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Integer);
        self
    }

    /// Set column type as big_integer
    pub fn big_integer(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigInteger);
        self
    }

    /// Set column type as tiny_unsigned
    pub fn tiny_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TinyUnsigned);
        self
    }

    /// Set column type as small_unsigned
    pub fn small_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::SmallUnsigned);
        self
    }

    /// Set column type as unsigned
    pub fn unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Unsigned);
        self
    }

    /// Set column type as big_unsigned
    pub fn big_unsigned(&mut self) -> &mut Self {
        self.types = Some(ColumnType::BigUnsigned);
        self
    }

    /// Set column type as float
    pub fn float(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Float);
        self
    }

    /// Set column type as double
    pub fn double(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Double);
        self
    }

    /// Set column type as decimal with custom precision and scale
    pub fn decimal_len(&mut self, precision: u32, scale: u32) -> &mut Self {
        self.types = Some(ColumnType::Decimal(Some((precision, scale))));
        self
    }

    /// Set column type as decimal
    pub fn decimal(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Decimal(None));
        self
    }

    /// Set column type as date_time
    pub fn date_time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::DateTime);
        self
    }

    /// Set column type as interval type with optional fields and precision. Postgres only
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(ColumnDef::new("I1").interval(None, None).not_null())
    ///         .col(
    ///             ColumnDef::new("I2")
    ///                 .interval(Some(PgInterval::YearToMonth), None)
    ///                 .not_null()
    ///         )
    ///         .col(ColumnDef::new("I3").interval(None, Some(42)).not_null())
    ///         .col(
    ///             ColumnDef::new("I4")
    ///                 .interval(Some(PgInterval::Hour), Some(43))
    ///                 .not_null()
    ///         )
    ///         .to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "glyph" ("#,
    ///         r#""I1" interval NOT NULL,"#,
    ///         r#""I2" interval YEAR TO MONTH NOT NULL,"#,
    ///         r#""I3" interval(42) NOT NULL,"#,
    ///         r#""I4" interval HOUR(43) NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    #[cfg(feature = "backend-postgres")]
    pub fn interval(&mut self, fields: Option<PgInterval>, precision: Option<u32>) -> &mut Self {
        self.types = Some(ColumnType::Interval(fields, precision));
        self
    }

    #[cfg(feature = "postgres-vector")]
    pub fn vector(&mut self, size: Option<u32>) -> &mut Self {
        self.types = Some(ColumnType::Vector(size));
        self
    }

    /// Set column type as timestamp
    pub fn timestamp(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Timestamp);
        self
    }

    /// Set column type as timestamp with time zone. Postgres only
    pub fn timestamp_with_time_zone(&mut self) -> &mut Self {
        self.types = Some(ColumnType::TimestampWithTimeZone);
        self
    }

    /// Set column type as time
    pub fn time(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Time);
        self
    }

    /// Set column type as date
    pub fn date(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Date);
        self
    }

    /// Set column type as year
    /// Only MySQL supports year
    pub fn year(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Year);
        self
    }

    /// Set column type as binary with custom length
    pub fn binary_len(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::Binary(length));
        self
    }

    /// Set column type as binary with default length of 1
    pub fn binary(&mut self) -> &mut Self {
        self.binary_len(1)
    }

    /// Set column type as binary with variable length
    pub fn var_binary(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::VarBinary(StringLen::N(length)));
        self
    }

    /// Set column type as bit with variable length
    pub fn bit(&mut self, length: Option<u32>) -> &mut Self {
        self.types = Some(ColumnType::Bit(length));
        self
    }

    /// Set column type as varbit with variable length
    pub fn varbit(&mut self, length: u32) -> &mut Self {
        self.types = Some(ColumnType::VarBit(length));
        self
    }

    /// Set column type as blob
    pub fn blob(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Blob);
        self
    }

    /// Set column type as boolean
    pub fn boolean(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Boolean);
        self
    }

    /// Set column type as money with custom precision and scale
    pub fn money_len(&mut self, precision: u32, scale: u32) -> &mut Self {
        self.types = Some(ColumnType::Money(Some((precision, scale))));
        self
    }

    /// Set column type as money
    pub fn money(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Money(None));
        self
    }

    /// Set column type as json.
    pub fn json(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Json);
        self
    }

    /// Set column type as json binary.
    pub fn json_binary(&mut self) -> &mut Self {
        self.types = Some(ColumnType::JsonBinary);
        self
    }

    /// Set column type as uuid
    pub fn uuid(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Uuid);
        self
    }

    /// Use a custom type on this column.
    pub fn custom<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.types = Some(ColumnType::Custom(name.into_iden()));
        self
    }

    /// Set column type as enum.
    pub fn enumeration<N, S, V>(&mut self, name: N, variants: V) -> &mut Self
    where
        N: IntoIden,
        S: IntoIden,
        V: IntoIterator<Item = S>,
    {
        self.types = Some(ColumnType::Enum {
            name: name.into_iden(),
            variants: variants.into_iter().map(IntoIden::into_iden).collect(),
        });
        self
    }

    /// Set column type as an array with a specified element type.
    /// This is only supported on Postgres.
    pub fn array(&mut self, elem_type: ColumnType) -> &mut Self {
        self.types = Some(ColumnType::Array(RcOrArc::new(elem_type)));
        self
    }

    /// Set columnt type as cidr.
    /// This is only supported on Postgres.
    pub fn cidr(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Cidr);
        self
    }

    /// Set columnt type as inet.
    /// This is only supported on Postgres.
    pub fn inet(&mut self) -> &mut Self {
        self.types = Some(ColumnType::Inet);
        self
    }

    /// Set columnt type as macaddr.
    /// This is only supported on Postgres.
    pub fn mac_address(&mut self) -> &mut Self {
        self.types = Some(ColumnType::MacAddr);
        self
    }

    /// Set column type as `ltree`
    /// This is only supported on Postgres.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(
    ///             ColumnDef::new(Glyph::Id)
    ///                 .integer()
    ///                 .not_null()
    ///                 .auto_increment()
    ///                 .primary_key()
    ///         )
    ///         .col(ColumnDef::new(Glyph::Tokens).ltree())
    ///         .to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "glyph" ("#,
    ///         r#""id" serial NOT NULL PRIMARY KEY,"#,
    ///         r#""tokens" ltree"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn ltree(&mut self) -> &mut Self {
        self.types = Some(ColumnType::LTree);
        self
    }

    /// Set constraints as SimpleExpr
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// assert_eq!(
    ///     Table::create()
    ///         .table(Glyph::Table)
    ///         .col(
    ///             ColumnDef::new(Glyph::Id)
    ///                 .integer()
    ///                 .not_null()
    ///                 .check(Expr::col(Glyph::Id).gt(10))
    ///         )
    ///         .to_string(MysqlQueryBuilder),
    ///     r#"CREATE TABLE `glyph` ( `id` int NOT NULL CHECK (`id` > 10) )"#,
    /// );
    /// ```
    pub fn check<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Check(value.into()));
        self
    }

    /// Sets the column as generated with SimpleExpr
    pub fn generated<T>(&mut self, expr: T, stored: bool) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Generated {
            expr: expr.into(),
            stored,
        });
        self
    }

    /// Some extra options in custom string
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// let table = Table::create()
    ///     .table(Char::Table)
    ///     .col(
    ///         ColumnDef::new(Char::Id)
    ///             .uuid()
    ///             .extra("DEFAULT gen_random_uuid()")
    ///             .primary_key()
    ///             .not_null(),
    ///     )
    ///     .col(
    ///         ColumnDef::new(Char::CreatedAt)
    ///             .timestamp_with_time_zone()
    ///             .extra("DEFAULT NOW()")
    ///             .not_null(),
    ///     )
    ///     .to_owned();
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"CREATE TABLE "character" ("#,
    ///         r#""id" uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,"#,
    ///         r#""created_at" timestamp with time zone DEFAULT NOW() NOT NULL"#,
    ///         r#")"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn extra<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.spec.push(ColumnSpec::Extra(string.into()));
        self
    }

    /// Some extra options in custom string
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    /// let table = Table::alter()
    ///     .table(Char::Table)
    ///     .modify_column(
    ///         ColumnDef::new(Char::Id)
    ///             .integer()
    ///             .using(Expr::col(Char::Id).cast_as("integer")),
    ///     )
    ///     .to_owned();
    /// assert_eq!(
    ///     table.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"ALTER TABLE "character""#,
    ///         r#"ALTER COLUMN "id" TYPE integer USING CAST("id" AS integer)"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn using<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        self.spec.push(ColumnSpec::Using(value.into()));
        self
    }

    /// MySQL only.
    pub fn comment<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.spec.push(ColumnSpec::Comment(string.into()));
        self
    }

    pub fn get_column_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_column_type(&self) -> Option<&ColumnType> {
        self.types.as_ref()
    }

    pub fn get_column_spec(&self) -> &Vec<ColumnSpec> {
        self.spec.as_ref()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            name: std::mem::replace(&mut self.name, SeaRc::new(NullAlias::new())),
            types: self.types.take(),
            spec: std::mem::take(&mut self.spec),
        }
    }
}

impl IntoColumnDef for &mut ColumnDef {
    fn into_column_def(self) -> ColumnDef {
        self.take()
    }
}

impl IntoColumnDef for ColumnDef {
    fn into_column_def(self) -> ColumnDef {
        self
    }
}
