use super::*;
use crate::extension::postgres::*;

impl OperLeftAssocDecider for PostgresQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        let common_answer = common_well_known_left_associative(op);
        let pg_specific_answer = matches!(op, BinOper::PgOperator(PgBinOper::Concatenate));
        common_answer || pg_specific_answer
    }
}

impl PrecedenceDecider for PostgresQueryBuilder {
    fn inner_expr_well_known_greater_precedence(
        &self,
        inner: &SimpleExpr,
        outer_oper: &Oper,
    ) -> bool {
        let common_answer = common_inner_expr_well_known_greater_precedence(inner, outer_oper);
        let pg_specific_answer = match inner {
            SimpleExpr::Binary(_, inner_bin_oper, _) => {
                let inner_oper: Oper = (*inner_bin_oper).into();
                if inner_oper.is_arithmetic() || inner_oper.is_shift() {
                    is_ilike(inner_bin_oper)
                } else if is_pg_comparison(inner_bin_oper) {
                    outer_oper.is_logical()
                } else {
                    false
                }
            }
            _ => false,
        };
        common_answer || pg_specific_answer
    }
}

impl QueryBuilder for PostgresQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut dyn SqlWriter) {
        match simple_expr {
            SimpleExpr::AsEnum(type_name, expr) => {
                write!(sql, "CAST(").unwrap();
                self.prepare_simple_expr_common(expr, sql);
                let q = self.quote();
                let type_name = type_name.to_string();
                let (ty, sfx) = if type_name.ends_with("[]") {
                    (&type_name[..type_name.len() - 2], "[]")
                } else {
                    (type_name.as_str(), "")
                };
                write!(sql, " AS {}{}{}{})", q.left(), ty, q.right(), sfx).unwrap();
            }
            _ => QueryBuilder::prepare_simple_expr_common(self, simple_expr, sql),
        }
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut dyn SqlWriter) {
        match select_distinct {
            SelectDistinct::All => write!(sql, "ALL").unwrap(),
            SelectDistinct::Distinct => write!(sql, "DISTINCT").unwrap(),
            SelectDistinct::DistinctOn(cols) => {
                write!(sql, "DISTINCT ON (").unwrap();
                cols.iter().fold(true, |first, column_ref| {
                    if !first {
                        write!(sql, ", ").unwrap();
                    }
                    self.prepare_column_ref(column_ref, sql);
                    false
                });
                write!(sql, ")").unwrap();
            }
            _ => {}
        };
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut dyn SqlWriter) {
        match bin_oper {
            BinOper::PgOperator(oper) => write!(
                sql,
                "{}",
                match oper {
                    PgBinOper::ILike => "ILIKE",
                    PgBinOper::NotILike => "NOT ILIKE",
                    PgBinOper::Matches => "@@",
                    PgBinOper::Contains => "@>",
                    PgBinOper::Contained => "<@",
                    PgBinOper::Concatenate => "||",
                    PgBinOper::Overlap => "&&",
                    PgBinOper::Similarity => "%",
                    PgBinOper::WordSimilarity => "<%",
                    PgBinOper::StrictWordSimilarity => "<<%",
                    PgBinOper::SimilarityDistance => "<->",
                    PgBinOper::WordSimilarityDistance => "<<->",
                    PgBinOper::StrictWordSimilarityDistance => "<<<->",
                    PgBinOper::GetJsonField => "->",
                    PgBinOper::CastJsonField => "->>",
                    PgBinOper::Regex => "~",
                    PgBinOper::RegexCaseInsensitive => "~*",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::EuclideanDistance => "<->",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::NegativeInnerProduct => "<#>",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::CosineDistance => "<=>",
                }
            )
            .unwrap(),
            _ => self.prepare_bin_oper_common(bin_oper, sql),
        }
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_function_name(&self, function: &Function, sql: &mut dyn SqlWriter) {
        match function {
            Function::PgFunction(function) => write!(
                sql,
                "{}",
                match function {
                    PgFunction::ToTsquery => "TO_TSQUERY",
                    PgFunction::ToTsvector => "TO_TSVECTOR",
                    PgFunction::PhrasetoTsquery => "PHRASETO_TSQUERY",
                    PgFunction::PlaintoTsquery => "PLAINTO_TSQUERY",
                    PgFunction::WebsearchToTsquery => "WEBSEARCH_TO_TSQUERY",
                    PgFunction::TsRank => "TS_RANK",
                    PgFunction::TsRankCd => "TS_RANK_CD",
                    PgFunction::StartsWith => "STARTS_WITH",
                    PgFunction::GenRandomUUID => "GEN_RANDOM_UUID",
                    PgFunction::JsonBuildObject => "JSON_BUILD_OBJECT",
                    PgFunction::JsonAgg => "JSON_AGG",
                    PgFunction::ArrayAgg => "ARRAY_AGG",
                    PgFunction::DateTrunc => "DATE_TRUNC",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::Any => "ANY",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::Some => "SOME",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::All => "ALL",
                }
            )
            .unwrap(),
            _ => self.prepare_function_name_common(function, sql),
        }
    }

    fn prepare_table_sample(&self, select: &SelectStatement, sql: &mut dyn SqlWriter) {
        let Some(table_sample) = select.table_sample else {
            return;
        };

        match table_sample.method {
            SampleMethod::BERNOULLI => write!(sql, " TABLESAMPLE BERNOULLI").unwrap(),
            SampleMethod::SYSTEM => write!(sql, " TABLESAMPLE SYSTEM").unwrap(),
        }
        write!(sql, " ({})", table_sample.percentage).unwrap();
        if let Some(repeatable) = table_sample.repeatable {
            write!(sql, " REPEATABLE ({repeatable})").unwrap();
        }
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

    fn write_string_quoted(&self, string: &str, buffer: &mut String) {
        let escaped = self.escape_string(string);
        let string = if escaped.find('\\').is_some() {
            "E'".to_owned() + &escaped + "'"
        } else {
            "'".to_owned() + &escaped + "'"
        };
        write!(buffer, "{string}").unwrap()
    }

    fn write_bytes(&self, bytes: &[u8], buffer: &mut String) {
        write!(buffer, "'\\x").unwrap();
        for b in bytes {
            write!(buffer, "{b:02X}").unwrap();
        }
        write!(buffer, "'").unwrap();
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }
}

fn is_pg_comparison(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::Contained)
            | BinOper::PgOperator(PgBinOper::Contains)
            | BinOper::PgOperator(PgBinOper::Similarity)
            | BinOper::PgOperator(PgBinOper::WordSimilarity)
            | BinOper::PgOperator(PgBinOper::StrictWordSimilarity)
            | BinOper::PgOperator(PgBinOper::Matches)
    )
}

fn is_ilike(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::ILike) | BinOper::PgOperator(PgBinOper::NotILike)
    )
}
