use crate::{ConditionHolder, DynIden, IntoCondition, IntoIden, SimpleExpr};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnConflict {
    pub(crate) targets: Vec<OnConflictTarget>,
    pub(crate) target_where: ConditionHolder,
    pub(crate) action: Option<OnConflictAction>,
    pub(crate) action_where: ConditionHolder,
}

/// Represents ON CONFLICT (upsert) targets
#[derive(Debug, Clone, PartialEq)]
pub enum OnConflictTarget {
    /// A column
    ConflictColumn(DynIden),
    /// An expression `(LOWER(column), ...)`
    ConflictExpr(SimpleExpr),
}

/// Represents ON CONFLICT (upsert) actions
#[derive(Debug, Clone, PartialEq)]
pub enum OnConflictAction {
    /// Do nothing
    DoNothing(Vec<DynIden>),
    /// Update column value of existing row
    Update(Vec<OnConflictUpdate>),
}

/// Represents strategies to update column in ON CONFLICT (upsert) actions
#[derive(Debug, Clone, PartialEq)]
pub enum OnConflictUpdate {
    /// Update column value of existing row with inserting value
    Column(DynIden),
    /// Update column value of existing row with expression
    Expr(DynIden, SimpleExpr),
}

impl OnConflict {
    /// Create a ON CONFLICT expression without target column,
    /// a special method designed for MySQL
    pub fn new() -> Self {
        Default::default()
    }

    /// Set ON CONFLICT target column
    pub fn column<C>(column: C) -> Self
    where
        C: IntoIden,
    {
        Self::columns([column])
    }

    /// Set ON CONFLICT target columns
    pub fn columns<I, C>(columns: I) -> Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        Self {
            targets: columns
                .into_iter()
                .map(|c| OnConflictTarget::ConflictColumn(c.into_iden()))
                .collect(),
            target_where: ConditionHolder::new(),
            action: None,
            action_where: ConditionHolder::new(),
        }
    }

    /// Set ON CONFLICT target expression
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(["abcd".into(), 3.1415.into()])
    ///     .on_conflict(
    ///         OnConflict::new()
    ///             .expr(Expr::col(Glyph::Id))
    ///             .update_column(Glyph::Aspect)
    ///             .value(Glyph::Image, Expr::val(1).add(2))
    ///             .to_owned(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"INSERT INTO `glyph` (`aspect`, `image`)"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn expr<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<SimpleExpr>,
    {
        Self::exprs(self, [expr])
    }

    /// Set multiple target expressions for ON CONFLICT. See [`OnConflict::expr`]
    pub fn exprs<I, T>(&mut self, exprs: I) -> &mut Self
    where
        T: Into<SimpleExpr>,
        I: IntoIterator<Item = T>,
    {
        self.targets.append(
            &mut exprs
                .into_iter()
                .map(|e: T| OnConflictTarget::ConflictExpr(e.into()))
                .collect(),
        );
        self
    }

    /// Set ON CONFLICT do nothing.
    ///
    /// Please use [`Self::do_nothing_on()`] and provide primary keys if you are using MySQL.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(["abcd".into(), 3.1415.into()])
    ///     .on_conflict(
    ///         OnConflict::columns([Glyph::Id, Glyph::Aspect])
    ///             .do_nothing()
    ///             .to_owned(),
    ///     )
    ///     .to_owned();
    ///
    /// // Sadly this is not valid today.
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"INSERT INTO `glyph` (`aspect`, `image`)"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON DUPLICATE KEY IGNORE"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn do_nothing(&mut self) -> &mut Self {
        self.action = Some(OnConflictAction::DoNothing(vec![]));
        self
    }

    /// Set ON CONFLICT do nothing, but with MySQL specific polyfill.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic(["abcd".into(), 3.1415.into()])
    ///     .on_conflict(
    ///         OnConflict::columns([Glyph::Id, Glyph::Aspect])
    ///             .do_nothing_on([Glyph::Id])
    ///             .to_owned(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"INSERT INTO `glyph` (`aspect`, `image`)"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON DUPLICATE KEY UPDATE `id` = `id`"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn do_nothing_on<C, I>(&mut self, pk_cols: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.action = Some(OnConflictAction::DoNothing(
            pk_cols.into_iter().map(IntoIden::into_iden).collect(),
        ));
        self
    }

    /// Set ON CONFLICT update column
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([
    ///         "abcd".into(),
    ///         3.1415.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::columns([Glyph::Id, Glyph::Aspect])
    ///             .update_column(Glyph::Aspect)
    ///             .value(Glyph::Image, Expr::val(1).add(2))
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     [
    ///         r#"INSERT INTO `glyph` (`aspect`, `image`)"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     [
    ///         r#"INSERT INTO "glyph" ("aspect", "image")"#,
    ///         r#"VALUES ('abcd', 3.1415)"#,
    ///         r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn update_column<C>(&mut self, column: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.update_columns([column])
    }

    /// Set ON CONFLICT update columns
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .update_columns([Glyph::Aspect, Glyph::Image])
    ///             .to_owned(),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = VALUES(`image`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
    /// );
    /// ```
    pub fn update_columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        let mut update_strats: Vec<OnConflictUpdate> = columns
            .into_iter()
            .map(|x| OnConflictUpdate::Column(IntoIden::into_iden(x)))
            .collect();

        match &mut self.action {
            Some(OnConflictAction::Update(v)) => {
                v.append(&mut update_strats);
            }
            Some(OnConflictAction::DoNothing(_)) | None => {
                self.action = Some(OnConflictAction::Update(update_strats));
            }
        };
        self
    }

    /// Set ON CONFLICT update exprs
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .value(Glyph::Image, Expr::val(1).add(2))
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `image` = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// ```
    pub fn values<C, I>(&mut self, values: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = (C, SimpleExpr)>,
    {
        let mut update_exprs: Vec<OnConflictUpdate> = values
            .into_iter()
            .map(|(c, e)| OnConflictUpdate::Expr(c.into_iden(), e))
            .collect();

        match &mut self.action {
            Some(OnConflictAction::Update(v)) => {
                v.append(&mut update_exprs);
            }
            Some(OnConflictAction::DoNothing(_)) | None => {
                self.action = Some(OnConflictAction::Update(update_exprs));
            }
        };
        self
    }

    /// Set ON CONFLICT update value
    pub fn value<C, T>(&mut self, col: C, value: T) -> &mut Self
    where
        C: IntoIden,
        T: Into<SimpleExpr>,
    {
        self.values([(col, value.into())])
    }

    /// Set target WHERE
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .value(Glyph::Image, Expr::val(1).add(2))
    ///             .target_and_where(Expr::col((Glyph::Table, Glyph::Aspect)).is_null())
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `image` = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") WHERE "glyph"."aspect" IS NULL DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") WHERE "glyph"."aspect" IS NULL DO UPDATE SET "image" = 1 + 2"#
    /// );
    /// ```
    pub fn target_and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.target_cond_where(other)
    }

    /// Set target WHERE
    pub fn target_and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.target_cond_where(other);
        }
        self
    }

    /// Set target WHERE
    pub fn target_cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.target_where.add_condition(condition.into_condition());
        self
    }

    /// Set action WHERE
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::insert()
    ///     .into_table(Glyph::Table)
    ///     .columns([Glyph::Aspect, Glyph::Image])
    ///     .values_panic([
    ///         2.into(),
    ///         3.into(),
    ///     ])
    ///     .on_conflict(
    ///         OnConflict::column(Glyph::Id)
    ///             .value(Glyph::Image, Expr::val(1).add(2))
    ///             .action_and_where(Expr::col((Glyph::Table, Glyph::Aspect)).is_null())
    ///             .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `image` = 1 + 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2 WHERE "glyph"."aspect" IS NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "image" = 1 + 2 WHERE "glyph"."aspect" IS NULL"#
    /// );
    /// ```
    pub fn action_and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.action_cond_where(other)
    }

    /// Set action WHERE
    pub fn action_and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.action_cond_where(other);
        }
        self
    }

    /// Set action WHERE
    pub fn action_cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.action_where.add_condition(condition.into_condition());
        self
    }
}
