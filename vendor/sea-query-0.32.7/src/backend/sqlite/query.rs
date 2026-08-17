use super::*;
use crate::extension::sqlite::SqliteBinOper;

impl QueryBuilder for SqliteQueryBuilder {
    fn prepare_select_lock(&self, _select_lock: &LockClause, _sql: &mut dyn SqlWriter) {
        // SQLite doesn't supports row locking
    }

    fn prepare_sub_query_oper(&self, oper: &SubQueryOper, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match oper {
                SubQueryOper::Exists => "EXISTS",
                SubQueryOper::Any => panic!("Operator 'ANY' doesnot support"),
                SubQueryOper::Some => panic!("Operator 'SOME' doesnot support"),
                SubQueryOper::All => panic!("Operator 'ALL' doesnot support"),
            }
        )
        .unwrap();
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut dyn SqlWriter) {
        match bin_oper {
            BinOper::SqliteOperator(bin_oper) => write!(
                sql,
                "{}",
                match bin_oper {
                    SqliteBinOper::Glob => "GLOB",
                    SqliteBinOper::Match => "MATCH",
                    SqliteBinOper::GetJsonField => "->",
                    SqliteBinOper::CastJsonField => "->>",
                }
            )
            .unwrap(),
            _ => self.prepare_bin_oper_common(bin_oper, sql),
        }
    }

    fn prepare_union_statement(
        &self,
        union_type: UnionType,
        select_statement: &SelectStatement,
        sql: &mut dyn SqlWriter,
    ) {
        match union_type {
            UnionType::Intersect => write!(sql, " INTERSECT ").unwrap(),
            UnionType::Distinct => write!(sql, " UNION ").unwrap(),
            UnionType::Except => write!(sql, " EXCEPT ").unwrap(),
            UnionType::All => write!(sql, " UNION ALL ").unwrap(),
        }
        self.prepare_select_statement(select_statement, sql);
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_with_clause_recursive_options(&self, _: &WithClause, _: &mut dyn SqlWriter) {
        // Sqlite doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
    }

    fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    fn greatest_function(&self) -> &str {
        "MAX"
    }

    fn least_function(&self) -> &str {
        "MIN"
    }

    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn insert_default_values(&self, _: u32, sql: &mut dyn SqlWriter) {
        // SQLite doesn't support inserting multiple rows with default values
        write!(sql, "DEFAULT VALUES").unwrap()
    }
}
