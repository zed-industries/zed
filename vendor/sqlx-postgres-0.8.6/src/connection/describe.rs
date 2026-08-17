use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::io::StatementId;
use crate::message::{ParameterDescription, RowDescription};
use crate::query_as::query_as;
use crate::query_scalar::query_scalar;
use crate::statement::PgStatementMetadata;
use crate::type_info::{PgArrayOf, PgCustomType, PgType, PgTypeKind};
use crate::types::Json;
use crate::types::Oid;
use crate::HashMap;
use crate::{PgColumn, PgConnection, PgTypeInfo};
use smallvec::SmallVec;
use sqlx_core::query_builder::QueryBuilder;
use std::sync::Arc;

/// Describes the type of the `pg_type.typtype` column
///
/// See <https://www.postgresql.org/docs/13/catalog-pg-type.html>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TypType {
    Base,
    Composite,
    Domain,
    Enum,
    Pseudo,
    Range,
}

impl TryFrom<i8> for TypType {
    type Error = ();

    fn try_from(t: i8) -> Result<Self, Self::Error> {
        let t = u8::try_from(t).or(Err(()))?;

        let t = match t {
            b'b' => Self::Base,
            b'c' => Self::Composite,
            b'd' => Self::Domain,
            b'e' => Self::Enum,
            b'p' => Self::Pseudo,
            b'r' => Self::Range,
            _ => return Err(()),
        };
        Ok(t)
    }
}

/// Describes the type of the `pg_type.typcategory` column
///
/// See <https://www.postgresql.org/docs/13/catalog-pg-type.html#CATALOG-TYPCATEGORY-TABLE>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TypCategory {
    Array,
    Boolean,
    Composite,
    DateTime,
    Enum,
    Geometric,
    Network,
    Numeric,
    Pseudo,
    Range,
    String,
    Timespan,
    User,
    BitString,
    Unknown,
}

impl TryFrom<i8> for TypCategory {
    type Error = ();

    fn try_from(c: i8) -> Result<Self, Self::Error> {
        let c = u8::try_from(c).or(Err(()))?;

        let c = match c {
            b'A' => Self::Array,
            b'B' => Self::Boolean,
            b'C' => Self::Composite,
            b'D' => Self::DateTime,
            b'E' => Self::Enum,
            b'G' => Self::Geometric,
            b'I' => Self::Network,
            b'N' => Self::Numeric,
            b'P' => Self::Pseudo,
            b'R' => Self::Range,
            b'S' => Self::String,
            b'T' => Self::Timespan,
            b'U' => Self::User,
            b'V' => Self::BitString,
            b'X' => Self::Unknown,
            _ => return Err(()),
        };
        Ok(c)
    }
}

impl PgConnection {
    pub(super) async fn handle_row_description(
        &mut self,
        desc: Option<RowDescription>,
        should_fetch: bool,
    ) -> Result<(Vec<PgColumn>, HashMap<UStr, usize>), Error> {
        let mut columns = Vec::new();
        let mut column_names = HashMap::new();

        let desc = if let Some(desc) = desc {
            desc
        } else {
            // no rows
            return Ok((columns, column_names));
        };

        columns.reserve(desc.fields.len());
        column_names.reserve(desc.fields.len());

        for (index, field) in desc.fields.into_iter().enumerate() {
            let name = UStr::from(field.name);

            let type_info = self
                .maybe_fetch_type_info_by_oid(field.data_type_id, should_fetch)
                .await?;

            let column = PgColumn {
                ordinal: index,
                name: name.clone(),
                type_info,
                relation_id: field.relation_id,
                relation_attribute_no: field.relation_attribute_no,
            };

            columns.push(column);
            column_names.insert(name, index);
        }

        Ok((columns, column_names))
    }

    pub(super) async fn handle_parameter_description(
        &mut self,
        desc: ParameterDescription,
    ) -> Result<Vec<PgTypeInfo>, Error> {
        let mut params = Vec::with_capacity(desc.types.len());

        for ty in desc.types {
            params.push(self.maybe_fetch_type_info_by_oid(ty, true).await?);
        }

        Ok(params)
    }

    async fn maybe_fetch_type_info_by_oid(
        &mut self,
        oid: Oid,
        should_fetch: bool,
    ) -> Result<PgTypeInfo, Error> {
        // first we check if this is a built-in type
        // in the average application, the vast majority of checks should flow through this
        if let Some(info) = PgTypeInfo::try_from_oid(oid) {
            return Ok(info);
        }

        // next we check a local cache for user-defined type names <-> object id
        if let Some(info) = self.inner.cache_type_info.get(&oid) {
            return Ok(info.clone());
        }

        // fallback to asking the database directly for a type name
        if should_fetch {
            // we're boxing this future here so we can use async recursion
            let info = Box::pin(async { self.fetch_type_by_oid(oid).await }).await?;

            // cache the type name <-> oid relationship in a paired hashmap
            // so we don't come down this road again
            self.inner.cache_type_info.insert(oid, info.clone());
            self.inner
                .cache_type_oid
                .insert(info.0.name().to_string().into(), oid);

            Ok(info)
        } else {
            // we are not in a place that *can* run a query
            // this generally means we are in the middle of another query
            // this _should_ only happen for complex types sent through the TEXT protocol
            // we're open to ideas to correct this.. but it'd probably be more efficient to figure
            // out a way to "prime" the type cache for connections rather than make this
            // fallback work correctly for complex user-defined types for the TEXT protocol
            Ok(PgTypeInfo(PgType::DeclareWithOid(oid)))
        }
    }

    async fn fetch_type_by_oid(&mut self, oid: Oid) -> Result<PgTypeInfo, Error> {
        let (name, typ_type, category, relation_id, element, base_type): (
            String,
            i8,
            i8,
            Oid,
            Oid,
            Oid,
        ) = query_as(
            // Converting the OID to `regtype` and then `text` will give us the name that
            // the type will need to be found at by search_path.
            "SELECT oid::regtype::text, \
                     typtype, \
                     typcategory, \
                     typrelid, \
                     typelem, \
                     typbasetype \
                     FROM pg_catalog.pg_type \
                     WHERE oid = $1",
        )
        .bind(oid)
        .fetch_one(&mut *self)
        .await?;

        let typ_type = TypType::try_from(typ_type);
        let category = TypCategory::try_from(category);

        match (typ_type, category) {
            (Ok(TypType::Domain), _) => self.fetch_domain_by_oid(oid, base_type, name).await,

            (Ok(TypType::Base), Ok(TypCategory::Array)) => {
                Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                    kind: PgTypeKind::Array(
                        self.maybe_fetch_type_info_by_oid(element, true).await?,
                    ),
                    name: name.into(),
                    oid,
                }))))
            }

            (Ok(TypType::Pseudo), Ok(TypCategory::Pseudo)) => {
                Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                    kind: PgTypeKind::Pseudo,
                    name: name.into(),
                    oid,
                }))))
            }

            (Ok(TypType::Range), Ok(TypCategory::Range)) => {
                self.fetch_range_by_oid(oid, name).await
            }

            (Ok(TypType::Enum), Ok(TypCategory::Enum)) => self.fetch_enum_by_oid(oid, name).await,

            (Ok(TypType::Composite), Ok(TypCategory::Composite)) => {
                self.fetch_composite_by_oid(oid, relation_id, name).await
            }

            _ => Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                kind: PgTypeKind::Simple,
                name: name.into(),
                oid,
            })))),
        }
    }

    async fn fetch_enum_by_oid(&mut self, oid: Oid, name: String) -> Result<PgTypeInfo, Error> {
        let variants: Vec<String> = query_scalar(
            r#"
SELECT enumlabel
FROM pg_catalog.pg_enum
WHERE enumtypid = $1
ORDER BY enumsortorder
            "#,
        )
        .bind(oid)
        .fetch_all(self)
        .await?;

        Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            oid,
            name: name.into(),
            kind: PgTypeKind::Enum(Arc::from(variants)),
        }))))
    }

    async fn fetch_composite_by_oid(
        &mut self,
        oid: Oid,
        relation_id: Oid,
        name: String,
    ) -> Result<PgTypeInfo, Error> {
        let raw_fields: Vec<(String, Oid)> = query_as(
            r#"
SELECT attname, atttypid
FROM pg_catalog.pg_attribute
WHERE attrelid = $1
AND NOT attisdropped
AND attnum > 0
ORDER BY attnum
                "#,
        )
        .bind(relation_id)
        .fetch_all(&mut *self)
        .await?;

        let mut fields = Vec::new();

        for (field_name, field_oid) in raw_fields.into_iter() {
            let field_type = self.maybe_fetch_type_info_by_oid(field_oid, true).await?;

            fields.push((field_name, field_type));
        }

        Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            oid,
            name: name.into(),
            kind: PgTypeKind::Composite(Arc::from(fields)),
        }))))
    }

    async fn fetch_domain_by_oid(
        &mut self,
        oid: Oid,
        base_type: Oid,
        name: String,
    ) -> Result<PgTypeInfo, Error> {
        let base_type = self.maybe_fetch_type_info_by_oid(base_type, true).await?;

        Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            oid,
            name: name.into(),
            kind: PgTypeKind::Domain(base_type),
        }))))
    }

    async fn fetch_range_by_oid(&mut self, oid: Oid, name: String) -> Result<PgTypeInfo, Error> {
        let element_oid: Oid = query_scalar(
            r#"
SELECT rngsubtype
FROM pg_catalog.pg_range
WHERE rngtypid = $1
                "#,
        )
        .bind(oid)
        .fetch_one(&mut *self)
        .await?;

        let element = self.maybe_fetch_type_info_by_oid(element_oid, true).await?;

        Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            kind: PgTypeKind::Range(element),
            name: name.into(),
            oid,
        }))))
    }

    pub(crate) async fn resolve_type_id(&mut self, ty: &PgType) -> Result<Oid, Error> {
        if let Some(oid) = ty.try_oid() {
            return Ok(oid);
        }

        match ty {
            PgType::DeclareWithName(name) => self.fetch_type_id_by_name(name).await,
            PgType::DeclareArrayOf(array) => self.fetch_array_type_id(array).await,
            // `.try_oid()` should return `Some()` or it should be covered here
            _ => unreachable!("(bug) OID should be resolvable for type {ty:?}"),
        }
    }

    pub(crate) async fn fetch_type_id_by_name(&mut self, name: &str) -> Result<Oid, Error> {
        if let Some(oid) = self.inner.cache_type_oid.get(name) {
            return Ok(*oid);
        }

        // language=SQL
        let (oid,): (Oid,) = query_as("SELECT $1::regtype::oid")
            .bind(name)
            .fetch_optional(&mut *self)
            .await?
            .ok_or_else(|| Error::TypeNotFound {
                type_name: name.into(),
            })?;

        self.inner
            .cache_type_oid
            .insert(name.to_string().into(), oid);
        Ok(oid)
    }

    pub(crate) async fn fetch_array_type_id(&mut self, array: &PgArrayOf) -> Result<Oid, Error> {
        if let Some(oid) = self
            .inner
            .cache_type_oid
            .get(&array.elem_name)
            .and_then(|elem_oid| self.inner.cache_elem_type_to_array.get(elem_oid))
        {
            return Ok(*oid);
        }

        // language=SQL
        let (elem_oid, array_oid): (Oid, Oid) =
            query_as("SELECT oid, typarray FROM pg_catalog.pg_type WHERE oid = $1::regtype::oid")
                .bind(&*array.elem_name)
                .fetch_optional(&mut *self)
                .await?
                .ok_or_else(|| Error::TypeNotFound {
                    type_name: array.name.to_string(),
                })?;

        // Avoids copying `elem_name` until necessary
        self.inner
            .cache_type_oid
            .entry_ref(&array.elem_name)
            .insert(elem_oid);
        self.inner
            .cache_elem_type_to_array
            .insert(elem_oid, array_oid);

        Ok(array_oid)
    }

    /// Check whether EXPLAIN statements are supported by the current connection
    fn is_explain_available(&self) -> bool {
        let parameter_statuses = &self.inner.stream.parameter_statuses;
        let is_cockroachdb = parameter_statuses.contains_key("crdb_version");
        let is_materialize = parameter_statuses.contains_key("mz_version");
        let is_questdb = parameter_statuses.contains_key("questdb_version");
        !is_cockroachdb && !is_materialize && !is_questdb
    }

    pub(crate) async fn get_nullable_for_columns(
        &mut self,
        stmt_id: StatementId,
        meta: &PgStatementMetadata,
    ) -> Result<Vec<Option<bool>>, Error> {
        if meta.columns.is_empty() {
            return Ok(vec![]);
        }

        if meta.columns.len() * 3 > 65535 {
            tracing::debug!(
                ?stmt_id,
                num_columns = meta.columns.len(),
                "number of columns in query is too large to pull nullability for"
            );
        }

        // Query for NOT NULL constraints for each column in the query.
        //
        // This will include columns that don't have a `relation_id` (are not from a table);
        // assuming those are a minority of columns, it's less code to _not_ work around it
        // and just let Postgres return `NULL`.
        //
        // Use `UNION ALL` syntax instead of `VALUES` due to frequent lack of
        // support for `VALUES` in pgwire supported databases.
        let mut nullable_query = QueryBuilder::new("SELECT NOT attnotnull FROM ( ");
        let mut separated = nullable_query.separated("UNION ALL ");

        let mut column_iter = meta.columns.iter().zip(0i32..);
        if let Some((column, i)) = column_iter.next() {
            separated.push("( SELECT ");
            separated
                .push_bind_unseparated(i)
                .push_unseparated("::int4 AS idx, ");
            separated
                .push_bind_unseparated(column.relation_id)
                .push_unseparated("::int4 AS table_id, ");
            separated
                .push_bind_unseparated(column.relation_attribute_no)
                .push_unseparated("::int2 AS col_idx ) ");
        }

        for (column, i) in column_iter {
            separated.push("( SELECT ");
            separated
                .push_bind_unseparated(i)
                .push_unseparated("::int4, ");
            separated
                .push_bind_unseparated(column.relation_id)
                .push_unseparated("::int4, ");
            separated
                .push_bind_unseparated(column.relation_attribute_no)
                .push_unseparated("::int2 ) ");
        }

        nullable_query.push(
            ") AS col LEFT JOIN pg_catalog.pg_attribute \
                ON table_id IS NOT NULL \
               AND attrelid = table_id \
               AND attnum = col_idx \
            ORDER BY idx",
        );

        let mut nullables: Vec<Option<bool>> = nullable_query
            .build_query_scalar()
            .fetch_all(&mut *self)
            .await
            .map_err(|e| {
                err_protocol!(
                    "error from nullables query: {e}; query: {:?}",
                    nullable_query.sql()
                )
            })?;

        // If the server doesn't support EXPLAIN statements, skip this step (#1248).
        if self.is_explain_available() {
            // patch up our null inference with data from EXPLAIN
            let nullable_patch = self
                .nullables_from_explain(stmt_id, meta.parameters.len())
                .await?;

            for (nullable, patch) in nullables.iter_mut().zip(nullable_patch) {
                *nullable = patch.or(*nullable);
            }
        }

        Ok(nullables)
    }

    /// Infer nullability for columns of this statement using EXPLAIN VERBOSE.
    ///
    /// This currently only marks columns that are on the inner half of an outer join
    /// and returns `None` for all others.
    async fn nullables_from_explain(
        &mut self,
        stmt_id: StatementId,
        params_len: usize,
    ) -> Result<Vec<Option<bool>>, Error> {
        let stmt_id_display = stmt_id
            .display()
            .ok_or_else(|| err_protocol!("cannot EXPLAIN unnamed statement: {stmt_id:?}"))?;

        let mut explain = format!("EXPLAIN (VERBOSE, FORMAT JSON) EXECUTE {stmt_id_display}");
        let mut comma = false;

        if params_len > 0 {
            explain += "(";

            // fill the arguments list with NULL, which should theoretically be valid
            for _ in 0..params_len {
                if comma {
                    explain += ", ";
                }

                explain += "NULL";
                comma = true;
            }

            explain += ")";
        }

        let (Json(explains),): (Json<SmallVec<[Explain; 1]>>,) =
            query_as(&explain).fetch_one(self).await?;

        let mut nullables = Vec::new();

        if let Some(Explain::Plan {
            plan:
                plan @ Plan {
                    output: Some(ref outputs),
                    ..
                },
        }) = explains.first()
        {
            nullables.resize(outputs.len(), None);
            visit_plan(plan, outputs, &mut nullables);
        }

        Ok(nullables)
    }
}

fn visit_plan(plan: &Plan, outputs: &[String], nullables: &mut Vec<Option<bool>>) {
    if let Some(plan_outputs) = &plan.output {
        // all outputs of a Full Join must be marked nullable
        // otherwise, all outputs of the inner half of an outer join must be marked nullable
        if plan.join_type.as_deref() == Some("Full")
            || plan.parent_relation.as_deref() == Some("Inner")
        {
            for output in plan_outputs {
                if let Some(i) = outputs.iter().position(|o| o == output) {
                    // N.B. this may produce false positives but those don't cause runtime errors
                    nullables[i] = Some(true);
                }
            }
        }
    }

    if let Some(plans) = &plan.plans {
        if let Some("Left") | Some("Right") = plan.join_type.as_deref() {
            for plan in plans {
                visit_plan(plan, outputs, nullables);
            }
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Explain {
    // NOTE: the returned JSON may not contain a `plan` field, for example, with `CALL` statements:
    // https://github.com/launchbadge/sqlx/issues/1449
    //
    // In this case, we should just fall back to assuming all is nullable.
    //
    // It may also contain additional fields we don't care about, which should not break parsing:
    // https://github.com/launchbadge/sqlx/issues/2587
    // https://github.com/launchbadge/sqlx/issues/2622
    Plan {
        #[serde(rename = "Plan")]
        plan: Plan,
    },

    // This ensures that parsing never technically fails.
    //
    // We don't want to specifically expect `"Utility Statement"` because there might be other cases
    // and we don't care unless it contains a query plan anyway.
    Other(serde::de::IgnoredAny),
}

#[derive(serde::Deserialize, Debug)]
struct Plan {
    #[serde(rename = "Join Type")]
    join_type: Option<String>,
    #[serde(rename = "Parent Relationship")]
    parent_relation: Option<String>,
    #[serde(rename = "Output")]
    output: Option<Vec<String>>,
    #[serde(rename = "Plans")]
    plans: Option<Vec<Plan>>,
}

#[test]
fn explain_parsing() {
    let normal_plan = r#"[
   {
     "Plan": {
       "Node Type": "Result",
       "Parallel Aware": false,
       "Async Capable": false,
       "Startup Cost": 0.00,
       "Total Cost": 0.01,
       "Plan Rows": 1,
       "Plan Width": 4,
       "Output": ["1"]
     }
   }
]"#;

    // https://github.com/launchbadge/sqlx/issues/2622
    let extra_field = r#"[
   {                                        
     "Plan": {                              
       "Node Type": "Result",               
       "Parallel Aware": false,             
       "Async Capable": false,              
       "Startup Cost": 0.00,                
       "Total Cost": 0.01,                  
       "Plan Rows": 1,                      
       "Plan Width": 4,                     
       "Output": ["1"]                      
     },                                     
     "Query Identifier": 1147616880456321454
   }                                        
]"#;

    // https://github.com/launchbadge/sqlx/issues/1449
    let utility_statement = r#"["Utility Statement"]"#;

    let normal_plan_parsed = serde_json::from_str::<[Explain; 1]>(normal_plan).unwrap();
    let extra_field_parsed = serde_json::from_str::<[Explain; 1]>(extra_field).unwrap();
    let utility_statement_parsed = serde_json::from_str::<[Explain; 1]>(utility_statement).unwrap();

    assert!(
        matches!(normal_plan_parsed, [Explain::Plan { plan: Plan { .. } }]),
        "unexpected parse from {normal_plan:?}: {normal_plan_parsed:?}"
    );

    assert!(
        matches!(extra_field_parsed, [Explain::Plan { plan: Plan { .. } }]),
        "unexpected parse from {extra_field:?}: {extra_field_parsed:?}"
    );

    assert!(
        matches!(utility_statement_parsed, [Explain::Other(_)]),
        "unexpected parse from {utility_statement:?}: {utility_statement_parsed:?}"
    )
}
