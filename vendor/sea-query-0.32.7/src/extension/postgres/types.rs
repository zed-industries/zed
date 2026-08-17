use crate::{prepare::*, types::*, QueryBuilder, QuotedBuilder};

/// Helper for constructing any type statement
#[derive(Debug)]
pub struct Type;

#[derive(Clone, Debug)]
pub enum TypeRef {
    Type(DynIden),
    SchemaType(DynIden, DynIden),
    DatabaseSchemaType(DynIden, DynIden, DynIden),
}

pub trait IntoTypeRef {
    fn into_type_ref(self) -> TypeRef;
}

impl IntoTypeRef for TypeRef {
    fn into_type_ref(self) -> TypeRef {
        self
    }
}

impl<I> IntoTypeRef for I
where
    I: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::Type(self.into_iden())
    }
}

impl<A, B> IntoTypeRef for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::SchemaType(self.0.into_iden(), self.1.into_iden())
    }
}

impl<A, B, C> IntoTypeRef for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    fn into_type_ref(self) -> TypeRef {
        TypeRef::DatabaseSchemaType(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TypeCreateStatement {
    pub(crate) name: Option<TypeRef>,
    pub(crate) as_type: Option<TypeAs>,
    pub(crate) values: Vec<DynIden>,
}

#[derive(Debug, Clone)]
pub enum TypeAs {
    // Composite,
    Enum,
    /* Range,
     * Base,
     * Array, */
}

#[derive(Debug, Clone, Default)]
pub struct TypeDropStatement {
    pub(crate) names: Vec<TypeRef>,
    pub(crate) option: Option<TypeDropOpt>,
    pub(crate) if_exists: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TypeAlterStatement {
    pub(crate) name: Option<TypeRef>,
    pub(crate) option: Option<TypeAlterOpt>,
}

#[derive(Debug, Clone)]
pub enum TypeDropOpt {
    Cascade,
    Restrict,
}

#[derive(Debug, Clone)]
pub enum TypeAlterOpt {
    Add {
        value: DynIden,
        placement: Option<TypeAlterAddOpt>,
        if_not_exists: bool,
    },
    Rename(DynIden),
    RenameValue(DynIden, DynIden),
}

#[derive(Debug, Clone)]
pub enum TypeAlterAddOpt {
    Before(DynIden),
    After(DynIden),
}

pub trait TypeBuilder: QuotedBuilder {
    /// Translate [`TypeCreateStatement`] into database specific SQL statement.
    fn prepare_type_create_statement(&self, create: &TypeCreateStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeDropStatement`] into database specific SQL statement.
    fn prepare_type_drop_statement(&self, drop: &TypeDropStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeAlterStatement`] into database specific SQL statement.
    fn prepare_type_alter_statement(&self, alter: &TypeAlterStatement, sql: &mut dyn SqlWriter);

    /// Translate [`TypeRef`] into SQL statement.
    fn prepare_type_ref(&self, type_ref: &TypeRef, sql: &mut dyn SqlWriter) {
        match type_ref {
            TypeRef::Type(name) => {
                name.prepare(sql.as_writer(), self.quote());
            }
            TypeRef::SchemaType(schema, name) => {
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                name.prepare(sql.as_writer(), self.quote());
            }
            TypeRef::DatabaseSchemaType(database, schema, name) => {
                database.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                schema.prepare(sql.as_writer(), self.quote());
                write!(sql, ".").unwrap();
                name.prepare(sql.as_writer(), self.quote());
            }
        }
    }
}

impl Type {
    /// Construct type [`TypeCreateStatement`]
    pub fn create() -> TypeCreateStatement {
        TypeCreateStatement::new()
    }

    /// Construct type [`TypeDropStatement`]
    pub fn drop() -> TypeDropStatement {
        TypeDropStatement::new()
    }

    /// Construct type [`TypeAlterStatement`]
    pub fn alter() -> TypeAlterStatement {
        TypeAlterStatement::new()
    }
}

impl TypeCreateStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create enum as custom type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// enum FontFamily {
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn Write) {
    ///         write!(
    ///             s,
    ///             "{}",
    ///             match self {
    ///                 Self::Type => "font_family",
    ///                 Self::Serif => "serif",
    ///                 Self::Sans => "sans",
    ///                 Self::Monospace => "monospace",
    ///             }
    ///         )
    ///         .unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::create()
    ///         .as_enum(FontFamily::Type)
    ///         .values([FontFamily::Serif, FontFamily::Sans, FontFamily::Monospace])
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"CREATE TYPE "font_family" AS ENUM ('serif', 'sans', 'monospace')"#
    /// );
    /// ```
    pub fn as_enum<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoTypeRef,
    {
        self.name = Some(name.into_type_ref());
        self.as_type = Some(TypeAs::Enum);
        self
    }

    pub fn values<T, I>(&mut self, values: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = T>,
    {
        for v in values.into_iter() {
            self.values.push(v.into_iden());
        }
        self
    }
}

impl TypeDropStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop a type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// struct FontFamily;
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn Write) {
    ///         write!(s, "{}", "font_family").unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::drop()
    ///         .if_exists()
    ///         .name(FontFamily)
    ///         .restrict()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"DROP TYPE IF EXISTS "font_family" RESTRICT"#
    /// );
    /// ```
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: IntoTypeRef,
    {
        self.names.push(name.into_type_ref());
        self
    }

    /// Drop multiple types
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// #[derive(Iden)]
    /// enum KycStatus {
    ///     #[iden = "kyc_status"]
    ///     Type,
    ///     Pending,
    ///     Approved,
    /// }
    ///
    /// #[derive(Iden)]
    /// enum FontFamily {
    ///     #[iden = "font_family"]
    ///     Type,
    ///     Aerial,
    ///     Forte,
    /// }
    ///
    /// assert_eq!(
    ///     Type::drop()
    ///         .if_exists()
    ///         .names([
    ///             SeaRc::new(KycStatus::Type) as DynIden,
    ///             SeaRc::new(FontFamily::Type) as DynIden,
    ///         ])
    ///         .cascade()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"DROP TYPE IF EXISTS "kyc_status", "font_family" CASCADE"#
    /// );
    /// ```
    pub fn names<T, I>(&mut self, names: I) -> &mut Self
    where
        T: IntoTypeRef,
        I: IntoIterator<Item = T>,
    {
        for n in names.into_iter() {
            self.names.push(n.into_type_ref());
        }
        self
    }

    /// Set `IF EXISTS`
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Set `CASCADE`
    pub fn cascade(&mut self) -> &mut Self {
        self.option = Some(TypeDropOpt::Cascade);
        self
    }

    /// Set `RESTRICT`
    pub fn restrict(&mut self) -> &mut Self {
        self.option = Some(TypeDropOpt::Restrict);
        self
    }
}

impl TypeAlterStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Change the definition of a type
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, *};
    ///
    /// enum FontFamily {
    ///     Type,
    ///     Serif,
    ///     Sans,
    ///     Monospace,
    /// }
    ///
    /// impl Iden for FontFamily {
    ///     fn unquoted(&self, s: &mut dyn Write) {
    ///         write!(
    ///             s,
    ///             "{}",
    ///             match self {
    ///                 Self::Type => "font_family",
    ///                 Self::Serif => "serif",
    ///                 Self::Sans => "sans",
    ///                 Self::Monospace => "monospace",
    ///             }
    ///         )
    ///         .unwrap();
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(FontFamily::Type)
    ///         .add_value("cursive")
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font_family" ADD VALUE 'cursive'"#
    /// );
    /// ```
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: IntoTypeRef,
    {
        self.name = Some(name.into_type_ref());
        self
    }

    pub fn add_value<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        self.alter_option(TypeAlterOpt::Add {
            value: value.into_iden(),
            placement: None,
            if_not_exists: false,
        })
    }

    /// Add a enum value before an existing value
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value("weight")
    ///         .before(Font::Variant)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD VALUE 'weight' BEFORE 'variant'"#
    /// )
    /// ```
    pub fn before<T>(mut self, value: T) -> Self
    where
        T: IntoIden,
    {
        if let Some(option) = self.option {
            self.option = Some(option.before(value));
        }
        self
    }

    pub fn after<T>(mut self, value: T) -> Self
    where
        T: IntoIden,
    {
        if let Some(option) = self.option {
            self.option = Some(option.after(value));
        }
        self
    }

    /// Add a enum value if not already exists
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .add_value("weight")
    ///         .if_not_exists()
    ///         .after(Font::Variant)
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" ADD VALUE IF NOT EXISTS 'weight' AFTER 'variant'"#
    /// )
    /// ```
    pub fn if_not_exists(mut self) -> Self {
        if let Some(option) = self.option {
            self.option = Some(option.if_not_exists());
        }
        self
    }

    pub fn rename_to<T>(self, name: T) -> Self
    where
        T: IntoIden,
    {
        self.alter_option(TypeAlterOpt::Rename(name.into_iden()))
    }

    /// Rename a enum value
    ///
    /// ```
    /// use sea_query::{extension::postgres::Type, tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Type::alter()
    ///         .name(Font::Table)
    ///         .rename_value("variant", "language")
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"ALTER TYPE "font" RENAME VALUE 'variant' TO 'language'"#
    /// )
    /// ```
    pub fn rename_value<T, V>(self, existing: T, new_name: V) -> Self
    where
        T: IntoIden,
        V: IntoIden,
    {
        self.alter_option(TypeAlterOpt::RenameValue(
            existing.into_iden(),
            new_name.into_iden(),
        ))
    }

    fn alter_option(mut self, option: TypeAlterOpt) -> Self {
        self.option = Some(option);
        self
    }
}

impl TypeAlterOpt {
    /// Changes only `ADD VALUE x` options into `ADD VALUE x BEFORE` options, does nothing otherwise
    pub fn before<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add {
                value: iden,
                if_not_exists,
                ..
            } => Self::Add {
                value: iden,
                if_not_exists,
                placement: Some(TypeAlterAddOpt::Before(value.into_iden())),
            },
            _ => self,
        }
    }

    /// Changes only `ADD VALUE x` options into `ADD VALUE x AFTER` options, does nothing otherwise
    pub fn after<T>(self, value: T) -> Self
    where
        T: IntoIden,
    {
        match self {
            TypeAlterOpt::Add {
                value: iden,
                if_not_exists,
                ..
            } => Self::Add {
                value: iden,
                if_not_exists,
                placement: Some(TypeAlterAddOpt::After(value.into_iden())),
            },
            _ => self,
        }
    }

    /// Changes only `ADD VALUE x` options into `ADD VALUE IF NOT EXISTS x` options, does nothing otherwise
    pub fn if_not_exists(self) -> Self {
        match self {
            TypeAlterOpt::Add {
                value, placement, ..
            } => Self::Add {
                value,
                placement,
                if_not_exists: true,
            },
            _ => self,
        }
    }
}

macro_rules! impl_type_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: TypeBuilder>(&self, type_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(type_builder, &mut sql)
            }

            pub fn build_collect<T: TypeBuilder>(
                &self,
                type_builder: T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                self.build_collect_ref(&type_builder, sql)
            }

            pub fn build_collect_ref<T: TypeBuilder>(
                &self,
                type_builder: &T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                type_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, type_builder: T) -> String
            where
                T: TypeBuilder + QueryBuilder,
            {
                self.build_ref(&type_builder)
            }
        }
    };
}

impl_type_statement_builder!(TypeCreateStatement, prepare_type_create_statement);
impl_type_statement_builder!(TypeAlterStatement, prepare_type_alter_statement);
impl_type_statement_builder!(TypeDropStatement, prepare_type_drop_statement);
