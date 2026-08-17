# pgvector-rust

[pgvector](https://github.com/pgvector/pgvector) support for Rust

Supports [Rust-Postgres](https://github.com/sfackler/rust-postgres), [SQLx](https://github.com/launchbadge/sqlx), and [Diesel](https://github.com/diesel-rs/diesel)

[![Build Status](https://github.com/pgvector/pgvector-rust/actions/workflows/build.yml/badge.svg)](https://github.com/pgvector/pgvector-rust/actions)

## Getting Started

Follow the instructions for your database library:

- [Rust-Postgres](#rust-postgres)
- [SQLx](#sqlx)
- [Diesel](#diesel)

Or check out some examples:

- [Embeddings](https://github.com/pgvector/pgvector-rust/blob/master/examples/openai/src/main.rs) with OpenAI
- [Binary embeddings](https://github.com/pgvector/pgvector-rust/blob/master/examples/cohere/src/main.rs) with Cohere
- [Sentence embeddings](https://github.com/pgvector/pgvector-rust/blob/master/examples/candle/src/main.rs) with Candle
- [Hybrid search](https://github.com/pgvector/pgvector-rust/blob/master/examples/hybrid_search/src/main.rs) with Candle (Reciprocal Rank Fusion)
- [Recommendations](https://github.com/pgvector/pgvector-rust/blob/master/examples/disco/src/main.rs) with Disco
- [Horizontal scaling](https://github.com/pgvector/pgvector-rust/blob/master/examples/citus/src/main.rs) with Citus
- [Bulk loading](https://github.com/pgvector/pgvector-rust/blob/master/examples/loading/src/main.rs) with `COPY`

## Rust-Postgres

Add this line to your application’s `Cargo.toml` under `[dependencies]`:

```toml
pgvector = { version = "0.4", features = ["postgres"] }
```

Enable the extension

```rust
client.execute("CREATE EXTENSION IF NOT EXISTS vector", &[])?;
```

Create a table

```rust
client.execute("CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(3))", &[])?;
```

Create a vector from a `Vec<f32>`

```rust
use pgvector::Vector;

let embedding = Vector::from(vec![1.0, 2.0, 3.0]);
```

Insert a vector

```rust
client.execute("INSERT INTO items (embedding) VALUES ($1)", &[&embedding])?;
```

Get the nearest neighbor

```rust
let row = client.query_one(
    "SELECT * FROM items ORDER BY embedding <-> $1 LIMIT 1",
    &[&embedding],
)?;
```

Retrieve a vector

```rust
let row = client.query_one("SELECT embedding FROM items LIMIT 1", &[])?;
let embedding: Vector = row.get(0);
```

Use `Option` if the value could be `NULL`

```rust
let embedding: Option<Vector> = row.get(0);
```

## SQLx

Add this line to your application’s `Cargo.toml` under `[dependencies]`:

```toml
pgvector = { version = "0.4", features = ["sqlx"] }
```

For SQLx < 0.8, use `version = "0.3"` and [this readme](https://github.com/pgvector/pgvector-rust/blob/v0.3.4/README.md).

Enable the extension

```rust
sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
    .execute(&pool)
    .await?;
```

Create a table

```rust
sqlx::query("CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(3))")
    .execute(&pool)
    .await?;
```

Create a vector from a `Vec<f32>`

```rust
use pgvector::Vector;

let embedding = Vector::from(vec![1.0, 2.0, 3.0]);
```

Insert a vector

```rust
sqlx::query("INSERT INTO items (embedding) VALUES ($1)")
    .bind(embedding)
    .execute(&pool)
    .await?;
```

Get the nearest neighbors

```rust
let rows = sqlx::query("SELECT * FROM items ORDER BY embedding <-> $1 LIMIT 1")
    .bind(embedding)
    .fetch_all(&pool)
    .await?;
```

Retrieve a vector

```rust
let row = sqlx::query("SELECT embedding FROM items LIMIT 1").fetch_one(&pool).await?;
let embedding: Vector = row.try_get("embedding")?;
```

## Diesel

Add this line to your application’s `Cargo.toml` under `[dependencies]`:

```toml
pgvector = { version = "0.4", features = ["diesel"] }
```

And update your application’s `diesel.toml` under `[print_schema]`:

```toml
import_types = ["diesel::sql_types::*", "pgvector::sql_types::*"]
generate_missing_sql_type_definitions = false
```

Create a migration

```sh
diesel migration generate create_vector_extension
```

with `up.sql`:

```sql
CREATE EXTENSION vector
```

and `down.sql`:

```sql
DROP EXTENSION vector
```

Run the migration

```sql
diesel migration run
```

You can now use the `vector` type in future migrations

```sql
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  embedding VECTOR(3)
)
```

For models, use:

```rust
use pgvector::Vector;

#[derive(Queryable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub embedding: Option<Vector>,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub embedding: Option<Vector>,
}
```

Create a vector from a `Vec<f32>`

```rust
let embedding = Vector::from(vec![1.0, 2.0, 3.0]);
```

Insert a vector

```rust
let new_item = NewItem {
    embedding: Some(embedding)
};

diesel::insert_into(items::table)
    .values(&new_item)
    .get_result::<Item>(&mut conn)?;
```

Get the nearest neighbors

```rust
use pgvector::VectorExpressionMethods;

let neighbors = items::table
    .order(items::embedding.l2_distance(embedding))
    .limit(5)
    .load::<Item>(&mut conn)?;
```

Also supports `max_inner_product`, `cosine_distance`, `l1_distance`, `hamming_distance`, and `jaccard_distance`

Get the distances

```rust
let distances = items::table
    .select(items::embedding.l2_distance(embedding))
    .load::<Option<f64>>(&mut conn)?;
```

Add an approximate index in a migration

```sql
CREATE INDEX my_index ON items USING hnsw (embedding vector_l2_ops)
-- or
CREATE INDEX my_index ON items USING ivfflat (embedding vector_l2_ops) WITH (lists = 100)
```

Use `vector_ip_ops` for inner product and `vector_cosine_ops` for cosine distance

## Serialization

Use the `serde` feature to enable serialization

## Reference

### Vectors

Create a vector

```rust
use pgvector::Vector;

let vec = Vector::from(vec![1.0, 2.0, 3.0]);
```

Convert to a `Vec<f32>`

```rust
let f32_vec: Vec<f32> = vec.into();
```

Get a slice

```rust
let slice = vec.as_slice();
```

### Half Vectors

Note: Use the `halfvec` feature to enable half vectors

Create a half vector from a `Vec<f16>`

```rust
use half::f16;
use pgvector::HalfVector;

let vec = HalfVector::from(vec![f16::from_f32(1.0), f16::from_f32(2.0), f16::from_f32(3.0)]);
```

Or a `f32` slice

```rust
let vec = HalfVector::from_f32_slice(&[1.0, 2.0, 3.0]);
```

Convert to a `Vec<f16>`

```rust
let f16_vec: Vec<f16> = vec.into();
```

Get a slice

```rust
let slice = vec.as_slice();
```

### Binary Vectors

Create a binary vector from a slice of bits

```rust
use pgvector::Bit;

let vec = Bit::new(&[true, false, true]);
```

Or a slice of bytes

```rust
let vec = Bit::from_bytes(&[0b00000000, 0b11111111]);
```

Get the number of bits

```rust
let len = vec.len();
```

Get a slice of bytes

```rust
let bytes = vec.as_bytes();
```

### Sparse Vectors

Create a sparse vector from a dense vector

```rust
use pgvector::SparseVector;

let vec = SparseVector::from_dense(vec![1.0, 0.0, 2.0, 0.0, 3.0, 0.0]);
```

Or a map of non-zero elements

```rust
let map = HashMap::from([(0, 1.0), (2, 2.0), (4, 3.0)]);
let vec = SparseVector::from_map(&map, 6);
```

Note: Indices start at 0

Get the number of dimensions

```rust
let dim = vec.dimensions();
```

Get the indices of non-zero elements

```rust
let indices = vec.indices();
```

Get the values of non-zero elements

```rust
let values = vec.values();
```

Get a dense vector

```rust
let f32_vec = vec.to_vec();
```

## History

View the [changelog](https://github.com/pgvector/pgvector-rust/blob/master/CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/pgvector/pgvector-rust/issues)
- Fix bugs and [submit pull requests](https://github.com/pgvector/pgvector-rust/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/pgvector/pgvector-rust.git
cd pgvector-rust
createdb pgvector_rust_test
cargo test --all-features
```

To run an example:

```sh
cd examples/loading
createdb pgvector_example
cargo run
```
