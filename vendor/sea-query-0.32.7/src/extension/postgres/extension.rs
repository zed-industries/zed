use crate::{QueryBuilder, QuotedBuilder, SqlWriter};

/// Creates a new "CREATE or DROP EXTENSION" statement for PostgreSQL
///
/// # Exampl
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Extension;

impl Extension {
    /// Creates a new [`ExtensionCreateStatement`]
    pub fn create() -> ExtensionCreateStatement {
        ExtensionCreateStatement::new()
    }

    /// Creates a new [`ExtensionDropStatement`]
    pub fn drop() -> ExtensionDropStatement {
        ExtensionDropStatement::new()
    }
}

/// Creates a new "CREATE EXTENSION" statement for PostgreSQL
///
/// # Synopsis
///
/// ```ignore
/// CREATE EXTENSION [ IF NOT EXISTS ] extension_name
///     [ WITH ] [ SCHEMA schema_name ]
///              [ VERSION version ]
///              [ CASCADE ]
/// ```
///
/// # Example
///
/// Creates the "ltree" extension if it doesn't exists.
///
/// ```
/// use sea_query::{extension::postgres::Extension, *};
///
/// assert_eq!(
///     Extension::create()
///         .name("ltree")
///         .schema("public")
///         .version("v0.1.0")
///         .cascade()
///         .if_not_exists()
///         .to_string(PostgresQueryBuilder),
///     r#"CREATE EXTENSION IF NOT EXISTS ltree WITH SCHEMA public VERSION v0.1.0 CASCADE"#
/// );
/// ```
///
/// # References
///
/// [Refer to the PostgreSQL Documentation][1]
///
/// [1]: https://www.postgresql.org/docs/current/sql-createextension.html
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExtensionCreateStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,

    /// Conditional to execute query based on existance of the extension.
    pub(crate) if_not_exists: bool,

    /// Determines the presence of the `RESTRICT` statement
    pub(crate) cascade: bool,
}

impl ExtensionCreateStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name of the extension to be created.
    pub fn name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Uses "WITH SCHEMA" on Create Extension Statement.
    pub fn schema<T: Into<String>>(&mut self, schema: T) -> &mut Self {
        self.schema = Some(schema.into());
        self
    }

    /// Uses "VERSION" on Create Extension Statement.
    pub fn version<T: Into<String>>(&mut self, version: T) -> &mut Self {
        self.version = Some(version.into());
        self
    }

    /// Uses "CASCADE" on Create Extension Statement.
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }

    /// Uses "IF NOT EXISTS" on Create Extension Statement.
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }
}

/// Creates a new "DROP EXTENSION" statement for PostgreSQL
///
/// # Synopsis
///
/// ```ignore
/// DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
/// ```
///
/// # Example
///
/// Drops the "ltree" extension if it exists.
///
/// ```
/// use sea_query::{extension::postgres::Extension, *};
///
/// assert_eq!(
///     Extension::drop()
///         .name("ltree")
///         .cascade()
///         .if_exists()
///         .to_string(PostgresQueryBuilder),
///     r#"DROP EXTENSION IF EXISTS ltree CASCADE"#
/// );
/// ```
///
/// # References
///
/// [Refer to the PostgreSQL Documentation][1]
///
/// [1]: https://www.postgresql.org/docs/current/sql-createextension.html
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExtensionDropStatement {
    pub(crate) name: String,
    pub(crate) schema: Option<String>,
    pub(crate) version: Option<String>,

    /// Conditional to execute query based on existance of the extension.
    pub(crate) if_exists: bool,

    /// Determines the presence of the `RESTRICT` statement.
    pub(crate) restrict: bool,

    /// Determines the presence of the `CASCADE` statement
    pub(crate) cascade: bool,
}

impl ExtensionDropStatement {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name of the extension to be dropped.
    pub fn name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Uses "IF EXISTS" on Drop Extension Statement.
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }

    /// Uses "CASCADE" on Drop Extension Statement.
    pub fn cascade(&mut self) -> &mut Self {
        self.cascade = true;
        self
    }

    /// Uses "RESTRICT" on Drop Extension Statement.
    pub fn restrict(&mut self) -> &mut Self {
        self.restrict = true;
        self
    }
}

pub trait ExtensionBuilder: QuotedBuilder {
    /// Translate [`ExtensionCreateStatement`] into database specific SQL statement.
    fn prepare_extension_create_statement(
        &self,
        create: &ExtensionCreateStatement,
        sql: &mut dyn SqlWriter,
    );

    /// Translate [`ExtensionDropStatement`] into database specific SQL statement.
    fn prepare_extension_drop_statement(
        &self,
        drop: &ExtensionDropStatement,
        sql: &mut dyn SqlWriter,
    );
}

macro_rules! impl_extension_statement_builder {
    ( $struct_name: ident, $func_name: ident ) => {
        impl $struct_name {
            pub fn build_ref<T: ExtensionBuilder>(&self, extension_builder: &T) -> String {
                let mut sql = String::with_capacity(256);
                self.build_collect_ref(extension_builder, &mut sql)
            }

            pub fn build_collect<T: ExtensionBuilder>(
                &self,
                extension_builder: T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                self.build_collect_ref(&extension_builder, sql)
            }

            pub fn build_collect_ref<T: ExtensionBuilder>(
                &self,
                extension_builder: &T,
                sql: &mut dyn SqlWriter,
            ) -> String {
                extension_builder.$func_name(self, sql);
                sql.to_string()
            }

            /// Build corresponding SQL statement and return SQL string
            pub fn to_string<T>(&self, extension_builder: T) -> String
            where
                T: ExtensionBuilder + QueryBuilder,
            {
                self.build_ref(&extension_builder)
            }
        }
    };
}

impl_extension_statement_builder!(ExtensionCreateStatement, prepare_extension_create_statement);
impl_extension_statement_builder!(ExtensionDropStatement, prepare_extension_drop_statement);

#[cfg(test)]
mod test {
    use super::super::PgLTree;
    use super::*;

    #[test]
    fn creates_a_stmt_for_create_extension() {
        let create_extension_stmt = Extension::create()
            .name(PgLTree)
            .schema("public")
            .version("v0.1.0")
            .cascade()
            .if_not_exists()
            .to_owned();

        assert_eq!(create_extension_stmt.name, "ltree");
        assert_eq!(create_extension_stmt.schema, Some("public".to_string()));
        assert_eq!(create_extension_stmt.version, Some("v0.1.0".to_string()));
        assert!(create_extension_stmt.cascade);
        assert!(create_extension_stmt.if_not_exists);
    }

    #[test]
    fn creates_a_stmt_for_drop_extension() {
        let drop_extension_stmt = Extension::drop()
            .name(PgLTree)
            .cascade()
            .if_exists()
            .restrict()
            .to_owned();

        assert_eq!(drop_extension_stmt.name, "ltree");
        assert!(drop_extension_stmt.cascade);
        assert!(drop_extension_stmt.if_exists);
        assert!(drop_extension_stmt.restrict);
    }
}
