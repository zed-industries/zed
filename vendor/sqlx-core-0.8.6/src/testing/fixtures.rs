//! TODO: automatic test fixture capture

use crate::database::Database;

use crate::query_builder::QueryBuilder;

use indexmap::set::IndexSet;
use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::sync::Arc;

pub type Result<T, E = FixtureError> = std::result::Result<T, E>;

/// A snapshot of the current state of the database.
///
/// Can be used to generate an `INSERT` fixture for populating an empty database,
/// or in the future it may be possible to generate a fixture from the difference between
/// two snapshots.
pub struct FixtureSnapshot<DB> {
    tables: BTreeMap<TableName, Table>,
    db: PhantomData<DB>,
}

#[derive(Debug, thiserror::Error)]
#[error("could not create fixture: {0}")]
pub struct FixtureError(String);

pub struct Fixture<DB> {
    ops: Vec<FixtureOp>,
    db: PhantomData<DB>,
}

enum FixtureOp {
    Insert {
        table: TableName,
        columns: Vec<ColumnName>,
        rows: Vec<Vec<Value>>,
    },
    // TODO: handle updates and deletes by diffing two snapshots
}

type TableName = Arc<str>;
type ColumnName = Arc<str>;
type Value = String;

struct Table {
    name: TableName,
    columns: IndexSet<ColumnName>,
    rows: Vec<Vec<Value>>,
    foreign_keys: HashMap<ColumnName, (TableName, ColumnName)>,
}

macro_rules! fixture_assert (
    ($cond:expr, $msg:literal $($arg:tt)*) => {
        if !($cond) {
            return Err(FixtureError(format!($msg $($arg)*)))
        }
    }
);

impl<DB: Database> FixtureSnapshot<DB> {
    /// Generate a fixture to reproduce this snapshot from an empty database using `INSERT`s.
    ///
    /// Note that this doesn't take into account any triggers that might modify the data before
    /// it's stored.
    ///
    /// The `INSERT` statements are ordered on a best-effort basis to satisfy any foreign key
    /// constraints (data from tables with no foreign keys are inserted first, then the tables
    /// that reference those tables, and so on).
    ///
    /// If a cycle in foreign-key constraints is detected, this returns with an error.
    pub fn additive_fixture(&self) -> Result<Fixture<DB>> {
        let visit_order = self.calculate_visit_order()?;

        let mut ops = Vec::new();

        for table_name in visit_order {
            let table = self.tables.get(&table_name).unwrap();

            ops.push(FixtureOp::Insert {
                table: table_name,
                columns: table.columns.iter().cloned().collect(),
                rows: table.rows.clone(),
            });
        }

        Ok(Fixture { ops, db: self.db })
    }

    /// Determine an order for outputting `INSERTS` for each table by calculating the max
    /// length of all its foreign key chains.
    ///
    /// This should hopefully ensure that there are no foreign-key errors.
    fn calculate_visit_order(&self) -> Result<Vec<TableName>> {
        let mut table_depths = HashMap::with_capacity(self.tables.len());
        let mut visited_set = IndexSet::with_capacity(self.tables.len());

        for table in self.tables.values() {
            foreign_key_depth(&self.tables, table, &mut table_depths, &mut visited_set)?;
            visited_set.clear();
        }

        let mut table_names: Vec<TableName> = table_depths.keys().cloned().collect();
        table_names.sort_by_key(|name| table_depths.get(name).unwrap());
        Ok(table_names)
    }
}

/// Implements `ToString` but not `Display` because it uses [`QueryBuilder`] internally,
/// which appends to an internal string.
#[allow(clippy::to_string_trait_impl)]
impl<DB: Database> ToString for Fixture<DB>
where
    for<'a> <DB as Database>::Arguments<'a>: Default,
{
    fn to_string(&self) -> String {
        let mut query = QueryBuilder::<DB>::new("");

        for op in &self.ops {
            match op {
                FixtureOp::Insert {
                    table,
                    columns,
                    rows,
                } => {
                    // Sanity check, empty tables shouldn't appear in snapshots anyway.
                    if columns.is_empty() || rows.is_empty() {
                        continue;
                    }

                    query.push(format_args!("INSERT INTO {table} ("));

                    let mut separated = query.separated(", ");

                    for column in columns {
                        separated.push(column);
                    }

                    query.push(")\n");

                    query.push_values(rows, |mut separated, row| {
                        for value in row {
                            separated.push(value);
                        }
                    });

                    query.push(";\n");
                }
            }
        }

        query.into_sql()
    }
}

fn foreign_key_depth(
    tables: &BTreeMap<TableName, Table>,
    table: &Table,
    depths: &mut HashMap<TableName, usize>,
    visited_set: &mut IndexSet<TableName>,
) -> Result<usize> {
    if let Some(&depth) = depths.get(&table.name) {
        return Ok(depth);
    }

    // This keeps us from looping forever.
    fixture_assert!(
        visited_set.insert(table.name.clone()),
        "foreign key cycle detected: {:?} -> {:?}",
        visited_set,
        table.name
    );

    let mut refdepth = 0;

    for (colname, (refname, refcol)) in &table.foreign_keys {
        let referenced = tables.get(refname).ok_or_else(|| {
            FixtureError(format!(
                "table {:?} in foreign key `{}.{} references {}.{}` does not exist",
                refname, table.name, colname, refname, refcol
            ))
        })?;

        refdepth = cmp::max(
            refdepth,
            foreign_key_depth(tables, referenced, depths, visited_set)?,
        );
    }

    let depth = refdepth + 1;

    depths.insert(table.name.clone(), depth);

    Ok(depth)
}

#[test]
#[cfg(feature = "any")]
fn test_additive_fixture() -> Result<()> {
    // Just need something that implements `Database`
    use crate::any::Any;

    let mut snapshot = FixtureSnapshot {
        tables: BTreeMap::new(),
        db: PhantomData::<Any>,
    };

    snapshot.tables.insert(
        "foo".into(),
        Table {
            name: "foo".into(),
            columns: ["foo_id", "foo_a", "foo_b"]
                .into_iter()
                .map(Arc::<str>::from)
                .collect(),
            rows: vec![vec!["1".into(), "'asdf'".into(), "true".into()]],
            foreign_keys: HashMap::new(),
        },
    );

    // foreign-keyed to `foo`
    // since `tables` is a `BTreeMap` we would expect a naive algorithm to visit this first.
    snapshot.tables.insert(
        "bar".into(),
        Table {
            name: "bar".into(),
            columns: ["bar_id", "foo_id", "bar_a", "bar_b"]
                .into_iter()
                .map(Arc::<str>::from)
                .collect(),
            rows: vec![vec![
                "1234".into(),
                "1".into(),
                "'2022-07-22 23:27:48.775113301+00:00'".into(),
                "3.14".into(),
            ]],
            foreign_keys: [("foo_id".into(), ("foo".into(), "foo_id".into()))]
                .into_iter()
                .collect(),
        },
    );

    // foreign-keyed to both `foo` and `bar`
    snapshot.tables.insert(
        "baz".into(),
        Table {
            name: "baz".into(),
            columns: ["baz_id", "bar_id", "foo_id", "baz_a", "baz_b"]
                .into_iter()
                .map(Arc::<str>::from)
                .collect(),
            rows: vec![vec![
                "5678".into(),
                "1234".into(),
                "1".into(),
                "'2022-07-22 23:27:48.775113301+00:00'".into(),
                "3.14".into(),
            ]],
            foreign_keys: [
                ("foo_id".into(), ("foo".into(), "foo_id".into())),
                ("bar_id".into(), ("bar".into(), "bar_id".into())),
            ]
            .into_iter()
            .collect(),
        },
    );

    let fixture = snapshot.additive_fixture()?;

    assert_eq!(
        fixture.to_string(),
        "INSERT INTO foo (foo_id, foo_a, foo_b)\n\
         VALUES (1, 'asdf', true);\n\
         INSERT INTO bar (bar_id, foo_id, bar_a, bar_b)\n\
         VALUES (1234, 1, '2022-07-22 23:27:48.775113301+00:00', 3.14);\n\
         INSERT INTO baz (baz_id, bar_id, foo_id, baz_a, baz_b)\n\
         VALUES (5678, 1234, 1, '2022-07-22 23:27:48.775113301+00:00', 3.14);\n"
    );

    Ok(())
}
