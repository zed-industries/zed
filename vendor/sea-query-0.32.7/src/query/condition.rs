use crate::{expr::SimpleExpr, types::LogicalChainOper};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionType {
    Any,
    All,
}

/// Represents the value of an [`Condition::any`] or [`Condition::all`]: a set of disjunctive or conjunctive conditions.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub(crate) negate: bool,
    pub(crate) condition_type: ConditionType,
    pub(crate) conditions: Vec<ConditionExpression>,
}

pub trait IntoCondition {
    fn into_condition(self) -> Condition;
}

pub type Cond = Condition;

/// Represents anything that can be passed to an [`Condition::any`] or [`Condition::all`]'s [`Condition::add`] method.
///
/// The arguments are automatically converted to the right enum.
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionExpression {
    Condition(Condition),
    SimpleExpr(SimpleExpr),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConditionHolderContents {
    #[default]
    Empty,
    Chain(Vec<LogicalChainOper>),
    Condition(Condition),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConditionHolder {
    pub contents: ConditionHolderContents,
}

impl Condition {
    /// Add a condition to the set.
    ///
    /// If it's an [`Condition::any`], it will be separated from the others by an `" OR "` in the query. If it's
    /// an [`Condition::all`], it will be separated by an `" AND "`.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let statement = Query::select()
    ///     .column(Glyph::Id)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(Expr::col(Glyph::Aspect).eq(0).into_condition().not())
    ///             .add(Expr::col(Glyph::Id).eq(0).into_condition().not()),
    ///     )
    ///     .to_string(PostgresQueryBuilder);
    /// assert_eq!(
    ///     statement,
    ///     r#"SELECT "id" FROM "glyph" WHERE (NOT "aspect" = 0) AND (NOT "id" = 0)"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add<C>(mut self, condition: C) -> Self
    where
        C: Into<ConditionExpression>,
    {
        let mut expr: ConditionExpression = condition.into();
        if let ConditionExpression::Condition(ref mut c) = expr {
            // Skip the junction if there is only one.
            if c.conditions.len() == 1 && !c.negate {
                expr = c.conditions.pop().unwrap();
            }
        }
        self.conditions.push(expr);
        self
    }

    /// Add an optional condition to the set.
    ///
    /// Shorthand for `if o.is_some() { self.add(o) }`
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::all()
    ///             .add_option(Some(Expr::col((Glyph::Table, Glyph::Image)).like("A%")))
    ///             .add_option(None::<SimpleExpr>),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add_option<C>(self, other: Option<C>) -> Self
    where
        C: Into<ConditionExpression>,
    {
        if let Some(other) = other {
            self.add(other)
        } else {
            self
        }
    }

    /// Create a condition that is true if any of the conditions is true.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::any()
    ///             .add(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
    ///             .add(Expr::col((Glyph::Table, Glyph::Image)).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn any() -> Condition {
        Condition {
            negate: false,
            condition_type: ConditionType::Any,
            conditions: Vec::new(),
        }
    }

    /// Create a condition that is false if any of the conditions is false.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
    ///             .add(Expr::col((Glyph::Table, Glyph::Image)).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn all() -> Condition {
        Condition {
            negate: false,
            condition_type: ConditionType::All,
            conditions: Vec::new(),
        }
    }

    /// Negates a condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::all()
    ///             .not()
    ///             .add(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
    ///             .add(Expr::col((Glyph::Table, Glyph::Image)).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE NOT (`glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%')"#
    /// );
    /// ```
    ///
    /// # More Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Id)
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(
    ///                 Cond::all()
    ///                     .not()
    ///                     .add(Expr::val(1).eq(1))
    ///                     .add(Expr::val(2).eq(2)),
    ///             )
    ///             .add(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4))),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `id` WHERE (NOT (1 = 1 AND 2 = 2)) AND (3 = 3 OR 4 = 4)"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        self.negate = !self.negate;
        self
    }

    /// Whether or not any condition has been added
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let is_empty = Cond::all().is_empty();
    ///
    /// assert!(is_empty);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// How many conditions were added
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let len = Cond::all().len();
    ///
    /// assert_eq!(len, 0);
    /// ```
    pub fn len(&self) -> usize {
        self.conditions.len()
    }
}

impl From<Condition> for SimpleExpr {
    fn from(cond: Condition) -> Self {
        let mut inner_exprs = vec![];
        for ce in cond.conditions {
            inner_exprs.push(match ce {
                ConditionExpression::Condition(c) => c.into(),
                ConditionExpression::SimpleExpr(e) => e,
            });
        }
        let mut inner_exprs_into_iter = inner_exprs.into_iter();
        let expr = if let Some(first_expr) = inner_exprs_into_iter.next() {
            let mut out_expr = first_expr;
            for e in inner_exprs_into_iter {
                out_expr = match cond.condition_type {
                    ConditionType::Any => out_expr.or(e),
                    ConditionType::All => out_expr.and(e),
                };
            }
            out_expr
        } else {
            SimpleExpr::Constant(match cond.condition_type {
                ConditionType::Any => false.into(),
                ConditionType::All => true.into(),
            })
        };
        if cond.negate {
            expr.not()
        } else {
            expr
        }
    }
}

impl From<ConditionExpression> for SimpleExpr {
    fn from(ce: ConditionExpression) -> Self {
        match ce {
            ConditionExpression::Condition(c) => c.into(),
            ConditionExpression::SimpleExpr(e) => e,
        }
    }
}

impl From<Condition> for ConditionExpression {
    fn from(condition: Condition) -> Self {
        ConditionExpression::Condition(condition)
    }
}

impl From<SimpleExpr> for ConditionExpression {
    fn from(condition: SimpleExpr) -> Self {
        ConditionExpression::SimpleExpr(condition)
    }
}

/// Macro to easily create an [`Condition::any`].
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///         any![
///             Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]),
///             Expr::col((Glyph::Table, Glyph::Image)).like("A%")
///         ]
///     )
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
/// );
/// ```
#[macro_export]
macro_rules! any {
    ( $( $x:expr ),* $(,)?) => {
        {
            let mut tmp = $crate::Condition::any();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}

/// Macro to easily create an [`Condition::all`].
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let query = Query::select()
///     .column(Glyph::Image)
///     .from(Glyph::Table)
///     .cond_where(
///         all![
///             Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]),
///             Expr::col((Glyph::Table, Glyph::Image)).like("A%")
///         ]
///     )
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
/// );
#[macro_export]
macro_rules! all {
    ( $( $x:expr ),* $(,)?) => {
        {
            let mut tmp = $crate::Condition::all();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}

pub trait ConditionalStatement {
    /// And where condition.
    /// Calling `or_where` after `and_where` will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
    ///     .and_where(Expr::col((Glyph::Table, Glyph::Image)).like("A%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where(&mut self, other: SimpleExpr) -> &mut Self {
        self.cond_where(other)
    }

    /// Optional and where, short hand for `if c.is_some() q.and_where(c)`.
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Aspect).is_in([3, 4]))
    ///     .and_where_option(Some(Expr::col(Glyph::Image).like("A%")))
    ///     .and_where_option(None)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `image` FROM `glyph` WHERE `aspect` IN (3, 4) AND `image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where_option(&mut self, other: Option<SimpleExpr>) -> &mut Self {
        if let Some(other) = other {
            self.and_where(other);
        }
        self
    }

    #[doc(hidden)]
    // Trait implementation.
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self;

    /// Where condition, expressed with `any` and `all`.
    /// Calling `cond_where` multiple times will conjoin them.
    /// Calling `or_where` after `cond_where` will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
    ///             .add(Cond::any()
    ///                 .add(Expr::col((Glyph::Table, Glyph::Image)).like("A%"))
    ///                 .add(Expr::col((Glyph::Table, Glyph::Image)).like("B%"))
    ///             )
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) AND ("glyph"."image" LIKE 'A%' OR "glyph"."image" LIKE 'B%')"#
    /// );
    /// ```
    ///
    /// Using macro
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Image)
    ///     .from(Glyph::Table)
    ///     .cond_where(
    ///         all![
    ///             Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]),
    ///             any![
    ///                 Expr::col((Glyph::Table, Glyph::Image)).like("A%"),
    ///                 Expr::col((Glyph::Table, Glyph::Image)).like("B%"),
    ///             ]
    ///         ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) AND ("glyph"."image" LIKE 'A%' OR "glyph"."image" LIKE 'B%')"#
    /// );
    /// ```
    ///
    /// Calling multiple times; the following two are equivalent:
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(Expr::col(Glyph::Id).eq(1))
    ///         .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 1 AND ("id" = 2 OR "id" = 3)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
    ///         .cond_where(Expr::col(Glyph::Id).eq(1))
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE ("id" = 2 OR "id" = 3) AND "id" = 1"#
    /// );
    /// ```
    ///
    /// Calling multiple times; will be ANDed togother
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(any![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
    ///         .cond_where(any![Expr::col(Glyph::Id).eq(3), Expr::col(Glyph::Id).eq(4)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE ("id" = 1 OR "id" = 2) AND ("id" = 3 OR "id" = 4)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(all![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
    ///         .cond_where(all![Expr::col(Glyph::Id).eq(3), Expr::col(Glyph::Id).eq(4)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 1 AND "id" = 2 AND "id" = 3 AND "id" = 4"#
    /// );
    /// ```
    ///
    /// Some more test cases involving negation
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(
    ///             Cond::all()
    ///                 .not()
    ///                 .add(Expr::col(Glyph::Id).eq(1))
    ///                 .add(Expr::col(Glyph::Id).eq(2)),
    ///         )
    ///         .cond_where(
    ///             Cond::all()
    ///                 .add(Expr::col(Glyph::Id).eq(3))
    ///                 .add(Expr::col(Glyph::Id).eq(4)),
    ///         )
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE (NOT ("id" = 1 AND "id" = 2)) AND ("id" = 3 AND "id" = 4)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column(Glyph::Id)
    ///         .from(Glyph::Table)
    ///         .cond_where(
    ///             Cond::all()
    ///                 .add(Expr::col(Glyph::Id).eq(3))
    ///                 .add(Expr::col(Glyph::Id).eq(4)),
    ///         )
    ///         .cond_where(
    ///             Cond::all()
    ///                 .not()
    ///                 .add(Expr::col(Glyph::Id).eq(1))
    ///                 .add(Expr::col(Glyph::Id).eq(2)),
    ///         )
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 3 AND "id" = 4 AND (NOT ("id" = 1 AND "id" = 2))"#
    /// );
    /// ```
    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition;
}

impl IntoCondition for SimpleExpr {
    fn into_condition(self) -> Condition {
        Condition::all().add(self)
    }
}

impl IntoCondition for Condition {
    fn into_condition(self) -> Condition {
        self
    }
}

impl ConditionHolder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_condition(condition: Condition) -> Self {
        let contents = ConditionHolderContents::Condition(condition);
        Self { contents }
    }

    pub fn is_empty(&self) -> bool {
        match &self.contents {
            ConditionHolderContents::Empty => true,
            ConditionHolderContents::Chain(c) => c.is_empty(),
            ConditionHolderContents::Condition(c) => c.conditions.is_empty(),
        }
    }

    pub fn is_one(&self) -> bool {
        match &self.contents {
            ConditionHolderContents::Empty => true,
            ConditionHolderContents::Chain(c) => c.len() == 1,
            ConditionHolderContents::Condition(c) => c.conditions.len() == 1,
        }
    }

    pub fn add_and_or(&mut self, condition: LogicalChainOper) {
        match &mut self.contents {
            ConditionHolderContents::Empty => {
                self.contents = ConditionHolderContents::Chain(vec![condition])
            }
            ConditionHolderContents::Chain(c) => c.push(condition),
            ConditionHolderContents::Condition(_) => {
                panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
            }
        }
    }

    pub fn add_condition(&mut self, mut addition: Condition) {
        match std::mem::take(&mut self.contents) {
            ConditionHolderContents::Empty => {
                self.contents = ConditionHolderContents::Condition(addition);
            }
            ConditionHolderContents::Condition(mut current) => {
                if current.condition_type == ConditionType::All && !current.negate {
                    if addition.condition_type == ConditionType::All && !addition.negate {
                        current.conditions.append(&mut addition.conditions);
                        self.contents = ConditionHolderContents::Condition(current);
                    } else {
                        self.contents = ConditionHolderContents::Condition(current.add(addition));
                    }
                } else {
                    self.contents = ConditionHolderContents::Condition(
                        Condition::all().add(current).add(addition),
                    );
                }
            }
            ConditionHolderContents::Chain(_) => {
                panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{tests_cfg::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg(feature = "backend-mysql")]
    fn test_blank_condition() {
        let query = Query::select()
            .column(Glyph::Image)
            .from(Glyph::Table)
            .cond_where(Cond::all())
            .cond_where(Expr::val(1).eq(1))
            .cond_where(Expr::val(2).eq(2))
            .cond_where(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4)))
            .to_owned();

        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            "SELECT `image` FROM `glyph` WHERE 1 = 1 AND 2 = 2 AND (3 = 3 OR 4 = 4)"
        );
    }
}
