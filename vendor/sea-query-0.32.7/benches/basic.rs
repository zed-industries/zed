use criterion::{criterion_group, criterion_main, Criterion};
use sea_query::*;

#[derive(Debug, Iden)]
pub enum Char {
    Table,
    Id,
    Character,
}

fn vanilla() -> String {
    format!(
        "SELECT `{}` from `{}` where `character` = {}",
        "character",
        "character".to_owned(),
        123
    )
}

fn select() -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(Expr::col(Char::Character).eq(123))
        .to_owned()
}

fn select_and_build() {
    select().build(MysqlQueryBuilder);
}

fn select_and_to_string() {
    select().to_string(MysqlQueryBuilder);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vanilla", |b| b.iter(vanilla));
    c.bench_function("select", |b| b.iter(select));
    c.bench_function("select_and_build", |b| b.iter(select_and_build));
    c.bench_function("select_and_to_string", |b| b.iter(select_and_to_string));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
