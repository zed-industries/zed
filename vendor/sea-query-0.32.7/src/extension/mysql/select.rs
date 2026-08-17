use super::{IndexHint, IndexHintScope, IndexHintType};
use crate::{IntoIden, SelectStatement};

pub trait MySqlSelectStatementExt {
    fn use_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;

    fn force_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;

    fn ignore_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden;
}

impl MySqlSelectStatementExt for SelectStatement {
    /// Use index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .use_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` USE INDEX (`IDX_123456`)"#
    /// );
    /// ```
    fn use_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        self.index_hints.push(IndexHint {
            index: index.into_iden(),
            r#type: IndexHintType::Use,
            scope,
        });
        self
    }

    /// Force index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .force_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` FORCE INDEX (`IDX_123456`)"#
    /// );
    /// ```
    fn force_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        self.index_hints.push(IndexHint {
            index: index.into_iden(),
            r#type: IndexHintType::Force,
            scope,
        });
        self
    }

    /// Ignore index hint for MySQL
    ///
    /// Give the optimizer information about how to choose indexes during query processing.
    /// See [MySQL reference manual for Index Hints](https://dev.mysql.com/doc/refman/8.0/en/index-hints.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::mysql::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .ignore_index(IndexName::new("IDX_123456"), IndexHintScope::All)
    ///     .column(Char::SizeW)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `size_w` FROM `character` IGNORE INDEX (`IDX_123456`)"#
    /// )
    /// ```
    fn ignore_index<I>(&mut self, index: I, scope: IndexHintScope) -> &mut Self
    where
        I: IntoIden,
    {
        self.index_hints.push(IndexHint {
            index: index.into_iden(),
            r#type: IndexHintType::Ignore,
            scope,
        });
        self
    }
}
