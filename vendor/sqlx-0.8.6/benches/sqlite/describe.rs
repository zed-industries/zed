use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use sqlx::sqlite::{Sqlite, SqliteConnection};
use sqlx::{Connection, Executor};
use sqlx_test::new;

// Here we have an async function to benchmark
async fn do_describe_trivial(db: &std::cell::RefCell<SqliteConnection>) {
    db.borrow_mut().describe("select 1").await.unwrap();
}

async fn do_describe_recursive(db: &std::cell::RefCell<SqliteConnection>) {
    db.borrow_mut()
        .describe(
            r#"
            WITH RECURSIVE schedule(begin_date) AS MATERIALIZED (
                SELECT datetime('2022-10-01')
                WHERE datetime('2022-10-01') < datetime('2022-11-03')
                UNION ALL
                SELECT datetime(begin_date,'+1 day')
                FROM schedule
                WHERE datetime(begin_date) < datetime(?2)
            )
            SELECT
            begin_date
            FROM schedule
            GROUP BY begin_date
            "#,
        )
        .await
        .unwrap();
}

async fn do_describe_insert(db: &std::cell::RefCell<SqliteConnection>) {
    db.borrow_mut()
        .describe("INSERT INTO tweet (id, text) VALUES (2, 'Hello') RETURNING *")
        .await
        .unwrap();
}

async fn do_describe_insert_fks(db: &std::cell::RefCell<SqliteConnection>) {
    db.borrow_mut()
        .describe("insert into statements (text) values ('a') returning id")
        .await
        .unwrap();
}

async fn init_connection() -> SqliteConnection {
    let mut conn = new::<Sqlite>().await.unwrap();

    conn.execute(
        r#"
        CREATE TEMPORARY TABLE statements (
          id integer not null primary key,
          text text not null
        );

        CREATE TEMPORARY TABLE votes1 (statement_id integer not null references statements(id));
        CREATE TEMPORARY TABLE votes2 (statement_id integer not null references statements(id));
        CREATE TEMPORARY TABLE votes3 (statement_id integer not null references statements(id));
        CREATE TEMPORARY TABLE votes4 (statement_id integer not null references statements(id));
        CREATE TEMPORARY TABLE votes5 (statement_id integer not null references statements(id));
        CREATE TEMPORARY TABLE votes6 (statement_id integer not null references statements(id));
        --CREATE TEMPORARY TABLE votes7 (statement_id integer not null references statements(id));
        --CREATE TEMPORARY TABLE votes8 (statement_id integer not null references statements(id));
        --CREATE TEMPORARY TABLE votes9 (statement_id integer not null references statements(id));
        --CREATE TEMPORARY TABLE votes10 (statement_id integer not null references statements(id));
        --CREATE TEMPORARY TABLE votes11 (statement_id integer not null references statements(id));
    "#,
    )
    .await
    .unwrap();
    conn
}

fn describe_trivial(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(init_connection()));

    c.bench_with_input(
        BenchmarkId::new("select", "trivial"),
        &db,
        move |b, db_ref| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&runtime).iter(|| do_describe_trivial(db_ref));
        },
    );
}

fn describe_recursive(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(init_connection()));

    c.bench_with_input(
        BenchmarkId::new("select", "recursive"),
        &db,
        move |b, db_ref| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&runtime).iter(|| do_describe_recursive(db_ref));
        },
    );
}

fn describe_insert(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(init_connection()));

    c.bench_with_input(
        BenchmarkId::new("insert", "returning"),
        &db,
        move |b, db_ref| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&runtime).iter(|| do_describe_insert(db_ref));
        },
    );
}

fn describe_insert_fks(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = std::cell::RefCell::new(runtime.block_on(init_connection()));

    c.bench_with_input(BenchmarkId::new("insert", "fks"), &db, move |b, db_ref| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&runtime).iter(|| do_describe_insert_fks(db_ref));
    });
}

criterion_group!(
    benches,
    describe_trivial,
    describe_recursive,
    describe_insert,
    describe_insert_fks
);
criterion_main!(benches);
