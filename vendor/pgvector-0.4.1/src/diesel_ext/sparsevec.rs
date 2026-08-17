use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::query_builder::QueryId;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::SqlType;
use std::convert::TryFrom;
use std::io::Write;

use crate::SparseVector;

#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "sparsevec"))]
pub struct SparseVectorType;

impl ToSql<SparseVectorType, Pg> for SparseVector {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let dim = self.dim;
        let nnz = self.indices.len();
        out.write_all(&dim.to_be_bytes())?;
        out.write_all(&i32::try_from(nnz)?.to_be_bytes())?;
        out.write_all(&0_i32.to_be_bytes())?;

        for v in &self.indices {
            out.write_all(&v.to_be_bytes())?;
        }

        for v in &self.values {
            out.write_all(&v.to_be_bytes())?;
        }

        Ok(IsNull::No)
    }
}

impl FromSql<SparseVectorType, Pg> for SparseVector {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        SparseVector::from_sql(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{SparseVector, VectorExpressionMethods};
    use diesel::prelude::*;

    table! {
        use diesel::sql_types::*;

        diesel_sparse_items (id) {
            id -> Int4,
            embedding -> Nullable<crate::sql_types::SparseVector>,
        }
    }

    use diesel_sparse_items as items;

    #[derive(Queryable)]
    #[diesel(table_name = items)]
    struct Item {
        pub id: i32,
        pub embedding: Option<SparseVector>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = items)]
    struct NewItem {
        pub embedding: Option<SparseVector>,
    }

    #[test]
    fn it_works() -> Result<(), diesel::result::Error> {
        let mut conn = PgConnection::establish("postgres://localhost/pgvector_rust_test").unwrap();
        diesel::sql_query("CREATE EXTENSION IF NOT EXISTS vector").execute(&mut conn)?;
        diesel::sql_query("DROP TABLE IF EXISTS diesel_sparse_items").execute(&mut conn)?;
        diesel::sql_query(
            "CREATE TABLE diesel_sparse_items (id serial PRIMARY KEY, embedding sparsevec(3))",
        )
        .execute(&mut conn)?;

        let new_items = vec![
            NewItem {
                embedding: Some(SparseVector::from_dense(&[1.0, 1.0, 1.0])),
            },
            NewItem {
                embedding: Some(SparseVector::from_dense(&[2.0, 2.0, 2.0])),
            },
            NewItem {
                embedding: Some(SparseVector::from_dense(&[1.0, 1.0, 2.0])),
            },
            NewItem { embedding: None },
        ];

        diesel::insert_into(items::table)
            .values(&new_items)
            .get_results::<Item>(&mut conn)?;

        let all = items::table.load::<Item>(&mut conn)?;
        assert_eq!(4, all.len());

        let neighbors = items::table
            .order(items::embedding.l2_distance(SparseVector::from_dense(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 3, 2, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );
        assert_eq!(
            Some(SparseVector::from_dense(&[1.0, 1.0, 1.0])),
            neighbors.first().unwrap().embedding
        );

        let neighbors = items::table
            .order(items::embedding.max_inner_product(SparseVector::from_dense(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![2, 3, 1, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let neighbors = items::table
            .order(items::embedding.cosine_distance(SparseVector::from_dense(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 2, 3, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let neighbors = items::table
            .order(items::embedding.l1_distance(SparseVector::from_dense(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 3, 2, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let distances = items::table
            .select(items::embedding.max_inner_product(SparseVector::from_dense(&[1.0, 1.0, 1.0])))
            .order(items::id)
            .load::<Option<f64>>(&mut conn)?;
        assert_eq!(vec![Some(-3.0), Some(-6.0), Some(-4.0), None], distances);

        Ok(())
    }
}
