use crate::{
    ColumnRef, DynIden, IntoIden, QueryStatementBuilder, QueryStatementWriter, SelectExpr,
    SelectStatement, SimpleExpr, SqlWriter, SubQueryStatement, TableRef, Values,
    {Alias, QueryBuilder},
};
use inherent::inherent;

/// A table definition inside a WITH clause ([WithClause]).
///
/// A WITH clause can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// sea-query doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and sea-query
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::table_name] and the [Self::query].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonTableExpression {
    pub(crate) table_name: Option<DynIden>,
    pub(crate) cols: Vec<DynIden>,
    pub(crate) query: Option<Box<SubQueryStatement>>,
    pub(crate) materialized: Option<bool>,
}

impl CommonTableExpression {
    /// Construct a new [`CommonTableExpression`]
    pub fn new() -> CommonTableExpression {
        Self::default()
    }

    /// Sets the CTE table name of the query.
    pub fn table_name<T>(&mut self, table_name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.table_name = Some(table_name.into_iden());
        self
    }

    /// Adds a named column to the CTE table definition.
    pub fn column<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.cols.push(col.into_iden());
        self
    }

    /// Adds a named columns to the CTE table definition.
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = T>,
    {
        self.cols
            .extend(cols.into_iter().map(|col| col.into_iden()));
        self
    }

    /// Some databases allow you to put "MATERIALIZED" or "NOT MATERIALIZED" in the CTE definition.
    /// This will affect how during the execution of [WithQuery] the CTE in the [WithClause] will be
    /// executed. If the database doesn't support this syntax this option specified here will be
    /// ignored and not appear in the generated sql.
    pub fn materialized(&mut self, materialized: bool) -> &mut Self {
        self.materialized = Some(materialized);
        self
    }

    /// Set the query generating the CTE content. The query's result must match the defined
    /// columns.
    pub fn query<Q>(&mut self, query: Q) -> &mut Self
    where
        Q: QueryStatementBuilder,
    {
        self.query = Some(Box::new(query.into_sub_query_statement()));
        self
    }

    /// Create a CTE from a [SelectStatement] if the selections are named columns then this will
    /// return a [CommonTableExpression] that has the column names set. The [Self::table_name] is
    /// set if the [SelectStatement] from clause contains at least one table.
    pub fn from_select(select: SelectStatement) -> Self {
        let mut cte = Self::default();
        cte.try_set_cols_from_selects(&select.selects);
        if let Some(from) = select.from.first() {
            match from {
                TableRef::Table(iden) => cte.set_table_name_from_select(iden),
                TableRef::SchemaTable(_, iden) => cte.set_table_name_from_select(iden),
                TableRef::DatabaseSchemaTable(_, _, iden) => cte.set_table_name_from_select(iden),
                TableRef::TableAlias(_, iden) => cte.set_table_name_from_select(iden),
                TableRef::SchemaTableAlias(_, _, iden) => cte.set_table_name_from_select(iden),
                TableRef::DatabaseSchemaTableAlias(_, _, _, iden) => {
                    cte.set_table_name_from_select(iden)
                }
                _ => {}
            }
        }
        cte.query = Some(Box::new(select.into_sub_query_statement()));
        cte
    }

    fn set_table_name_from_select(&mut self, iden: &DynIden) {
        self.table_name = Some(Alias::new(format!("cte_{}", iden.to_string())).into_iden())
    }

    /// Set up the columns of the CTE to match the given [SelectStatement] selected columns.
    /// This will fail if the select contains non named columns like expressions of wildcards.
    ///
    /// Returns true if the column setup from the select query was successful. If the returned
    /// value is false the columns are untouched.
    pub fn try_set_cols_from_select(&mut self, select: &SelectStatement) -> bool {
        self.try_set_cols_from_selects(&select.selects)
    }

    fn try_set_cols_from_selects(&mut self, selects: &[SelectExpr]) -> bool {
        let vec: Option<Vec<DynIden>> = selects
            .iter()
            .map(|select| {
                if let Some(ident) = &select.alias {
                    Some(ident.clone())
                } else {
                    match &select.expr {
                        SimpleExpr::Column(column) => match column {
                            ColumnRef::Column(iden) => Some(iden.clone()),
                            ColumnRef::TableColumn(table, column) => Some(
                                Alias::new(format!("{}_{}", table.to_string(), column.to_string()))
                                    .into_iden(),
                            ),
                            ColumnRef::SchemaTableColumn(schema, table, column) => Some(
                                Alias::new(format!(
                                    "{}_{}_{}",
                                    schema.to_string(),
                                    table.to_string(),
                                    column.to_string()
                                ))
                                .into_iden(),
                            ),
                            _ => None,
                        },
                        _ => None,
                    }
                }
            })
            .collect();

        if let Some(c) = vec {
            self.cols = c;
            return true;
        }

        false
    }
}

/// For recursive [WithQuery] [WithClause]s the traversing order can be specified in some databases
/// that support this functionality.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchOrder {
    /// Breadth first traversal during the execution of the recursive query.
    BREADTH,
    /// Depth first traversal during the execution of the recursive query.
    DEPTH,
}

/// For recursive [WithQuery] [WithClause]s the traversing order can be specified in some databases
/// that support this functionality.
///
/// The clause contains the type of traversal: [SearchOrder] and the expression that is used to
/// construct the current path.
///
/// A query can have both SEARCH and CYCLE clauses.
///
/// Setting [Self::order] and [Self::expr] is mandatory. The [SelectExpr] used must specify an alias
/// which will be the name that you can use to order the result of the [CommonTableExpression].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Search {
    pub(crate) order: Option<SearchOrder>,
    pub(crate) expr: Option<SelectExpr>,
}

impl Search {
    /// Create a complete [Search] specification from the [SearchOrder] and a [SelectExpr]. The
    /// given [SelectExpr] must have an alias specified.
    pub fn new_from_order_and_expr<EXPR>(order: SearchOrder, expr: EXPR) -> Self
    where
        EXPR: Into<SelectExpr>,
    {
        let expr = expr.into();
        expr.alias.as_ref().unwrap();
        Self {
            order: Some(order),
            expr: Some(expr),
        }
    }

    /// Constructs a new empty [Search].
    pub fn new() -> Self {
        Self::default()
    }

    /// The traversal order to be used.
    pub fn order(&mut self, order: SearchOrder) -> &mut Self {
        self.order = Some(order);
        self
    }

    /// The given [SelectExpr] must have an alias specified.
    ///
    /// The actual expression will be the one used to track the path in the graph.
    ///
    /// The alias of the given [SelectExpr] will be the name of the order column generated by this
    /// clause.
    pub fn expr<EXPR>(&mut self, expr: EXPR) -> &mut Self
    where
        EXPR: Into<SelectExpr>,
    {
        let expr = expr.into();
        expr.alias.as_ref().unwrap();
        self.expr = Some(expr);
        self
    }
}

/// For recursive [WithQuery] [WithClause]s the CYCLE sql clause can be specified to avoid creating
/// an infinite traversals that loops on graph cycles indefinitely. You specify an expression that
/// identifies a node in the graph and that will be used to determine during the iteration of
/// the execution of the query when appending of new values whether the new values are distinct new
/// nodes or are already visited and therefore they should be added again into the result.
///
/// A query can have both SEARCH and CYCLE clauses.
///
/// Setting [Self::set], [Self::expr] and [Self::using] is mandatory.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cycle {
    pub(crate) expr: Option<SimpleExpr>,
    pub(crate) set_as: Option<DynIden>,
    pub(crate) using: Option<DynIden>,
}

impl Cycle {
    /// Create a complete [Search] specification from the [SearchOrder] and a [SelectExpr]. The
    /// given [SelectExpr] must have an alias specified.
    pub fn new_from_expr_set_using<EXPR, ID1, ID2>(expr: EXPR, set: ID1, using: ID2) -> Self
    where
        EXPR: Into<SimpleExpr>,
        ID1: IntoIden,
        ID2: IntoIden,
    {
        Self {
            expr: Some(expr.into()),
            set_as: Some(set.into_iden()),
            using: Some(using.into_iden()),
        }
    }

    /// Constructs a new empty [Cycle].
    pub fn new() -> Self {
        Self::default()
    }

    /// The expression identifying nodes.
    pub fn expr<EXPR>(&mut self, expr: EXPR) -> &mut Self
    where
        EXPR: Into<SimpleExpr>,
    {
        self.expr = Some(expr.into());
        self
    }

    /// The name of the boolean column containing whether we have completed a cycle or not yet
    /// generated by this clause.
    pub fn set<ID>(&mut self, set: ID) -> &mut Self
    where
        ID: IntoIden,
    {
        self.set_as = Some(set.into_iden());
        self
    }

    /// The name of the array typed column that contains the node ids (generated using the
    /// [Self::expr]) that specify the current nodes path that will be generated by this clause.
    pub fn using<ID>(&mut self, using: ID) -> &mut Self
    where
        ID: IntoIden,
    {
        self.using = Some(using.into_iden());
        self
    }
}

/// A WITH clause can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// You can use this to generate [WithQuery] by calling [WithClause::query].
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// sea-query doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and sea-query
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::cte]. With queries must have at least one CTE.
/// Recursive with query generation will panic if you specify more than one CTE.
///
/// # Examples
///
/// ```
/// use sea_query::{*, IntoCondition, IntoIden, tests_cfg::*};
///
/// let base_query = SelectStatement::new()
///                     .column("id")
///                     .expr(1i32)
///                     .column("next")
///                     .column("value")
///                     .from("table")
///                     .to_owned();
///
/// let cte_referencing = SelectStatement::new()
///                             .column("id")
///                             .expr(Expr::col("depth").add(1i32))
///                             .column("next")
///                             .column("value")
///                             .from("table")
///                             .join(
///                                 JoinType::InnerJoin,
///                                 "cte_traversal",
///                                 Expr::col(("cte_traversal", "next")).equals(("table", "id"))
///                             )
///                             .to_owned();
///
/// let common_table_expression = CommonTableExpression::new()
///             .query(
///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
///             )
///             .column("id")
///             .column("depth")
///             .column("next")
///             .column("value")
///             .table_name("cte_traversal")
///             .to_owned();
///
/// let select = SelectStatement::new()
///         .column(ColumnRef::Asterisk)
///         .from("cte_traversal")
///         .to_owned();
///
/// let with_clause = WithClause::new()
///         .recursive(true)
///         .cte(common_table_expression)
///         .cycle(Cycle::new_from_expr_set_using(SimpleExpr::Column(ColumnRef::Column("id".into_iden())), "looped", "traversal_path"))
///         .to_owned();
///
/// let query = select.with(with_clause).to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder),
///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `table` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `table` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `table`.`id`)) SELECT * FROM `cte_traversal`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder),
///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id")) CYCLE "id" SET "looped" USING "traversal_path" SELECT * FROM "cte_traversal""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder),
///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id") SELECT * FROM "cte_traversal""#
/// );
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WithClause {
    pub(crate) recursive: bool,
    pub(crate) search: Option<Search>,
    pub(crate) cycle: Option<Cycle>,
    pub(crate) cte_expressions: Vec<CommonTableExpression>,
}

impl WithClause {
    /// Constructs a new [WithClause].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether this clause is a recursive with clause of not.
    /// If set to true it will generate a 'WITH RECURSIVE' query.
    ///
    /// You can only specify a single [CommonTableExpression] containing a union query
    /// if this is set to true.
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

    /// For recursive WITH queries you can specify the [Search] clause.
    ///
    /// This setting is not meaningful if the query is not recursive.
    ///
    /// Some databases don't support this clause. In that case this option will be silently ignored.
    pub fn search(&mut self, search: Search) -> &mut Self {
        self.search = Some(search);
        self
    }

    /// For recursive WITH queries you can specify the [Cycle] clause.
    ///
    /// This setting is not meaningful if the query is not recursive.
    ///
    /// Some databases don't support this clause. In that case this option will be silently ignored.
    pub fn cycle(&mut self, cycle: Cycle) -> &mut Self {
        self.cycle = Some(cycle);
        self
    }

    /// Add a [CommonTableExpression] to this with clause.
    pub fn cte(&mut self, cte: CommonTableExpression) -> &mut Self {
        self.cte_expressions.push(cte);
        self
    }

    /// You can turn this into a [WithQuery] using this function. The resulting WITH query will
    /// execute the argument query with this WITH clause.
    pub fn query<T>(self, query: T) -> WithQuery
    where
        T: QueryStatementBuilder + 'static,
    {
        WithQuery::new().with_clause(self).query(query).to_owned()
    }
}

impl From<CommonTableExpression> for WithClause {
    fn from(cte: CommonTableExpression) -> WithClause {
        WithClause::new().cte(cte).to_owned()
    }
}

/// A WITH query. A simple SQL query that has a WITH clause ([WithClause]).
///
/// The [WithClause] can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// sea-query doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and sea-query
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::cte] and the [Self::query].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WithQuery {
    pub(crate) with_clause: WithClause,
    pub(crate) query: Option<Box<SubQueryStatement>>,
}

impl WithQuery {
    /// Constructs a new empty [WithQuery].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the whole [WithClause].
    pub fn with_clause(&mut self, with_clause: WithClause) -> &mut Self {
        self.with_clause = with_clause;
        self
    }

    /// Set the [WithClause::recursive]. See that method for more information.
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.with_clause.recursive = recursive;
        self
    }

    /// Add the [WithClause::search]. See that method for more information.
    pub fn search(&mut self, search: Search) -> &mut Self {
        self.with_clause.search = Some(search);
        self
    }

    /// Set the [WithClause::cycle]. See that method for more information.
    pub fn cycle(&mut self, cycle: Cycle) -> &mut Self {
        self.with_clause.cycle = Some(cycle);
        self
    }

    /// Add a [CommonTableExpression] to the with clause. See [WithClause::cte].
    pub fn cte(&mut self, cte: CommonTableExpression) -> &mut Self {
        self.with_clause.cte_expressions.push(cte);
        self
    }

    /// Set the query that you execute with the [WithClause].
    pub fn query<T>(&mut self, query: T) -> &mut Self
    where
        T: QueryStatementBuilder,
    {
        self.query = Some(Box::new(query.into_sub_query_statement()));
        self
    }
}

impl QueryStatementBuilder for WithQuery {
    fn build_collect_any_into(&self, query_builder: &dyn QueryBuilder, sql: &mut dyn SqlWriter) {
        query_builder.prepare_with_query(self, sql);
    }

    fn into_sub_query_statement(self) -> SubQueryStatement {
        SubQueryStatement::WithStatement(self)
    }
}

#[inherent]
impl QueryStatementWriter for WithQuery {
    pub fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut dyn SqlWriter) {
        query_builder.prepare_with_query(self, sql);
    }

    pub fn build_collect<T: QueryBuilder>(
        &self,
        query_builder: T,
        sql: &mut dyn SqlWriter,
    ) -> String;
    pub fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values);
    pub fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String;
}
