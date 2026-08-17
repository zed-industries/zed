use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::query_builder::QueryId;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::SqlType;
use std::convert::TryFrom;
use std::io::Write;

use crate::HalfVector;

#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "halfvec"))]
pub struct HalfVectorType;

impl ToSql<HalfVectorType, Pg> for HalfVector {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let dim = self.0.len();
        out.write_all(&u16::try_from(dim)?.to_be_bytes())?;
        out.write_all(&0_u16.to_be_bytes())?;

        for v in &self.0 {
            out.write_all(&v.to_be_bytes())?;
        }

        Ok(IsNull::No)
    }
}

impl FromSql<HalfVectorType, Pg> for HalfVector {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        HalfVector::from_sql(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{HalfVector, VectorExpressionMethods};
    use diesel::prelude::*;

    table! {
        use diesel::sql_types::*;

        diesel_half_items (id) {
            id -> Int4,
            embedding -> Nullable<crate::sql_types::HalfVector>,
        }
    }

    use diesel_half_items as items;

    #[derive(Queryable)]
    #[diesel(table_name = items)]
    struct Item {
        pub id: i32,
        pub embedding: Option<HalfVector>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = items)]
    struct NewItem {
        pub embedding: Option<HalfVector>,
    }

    #[test]
    fn it_works() -> Result<(), diesel::result::Error> {
        let mut conn = PgConnection::establish("postgres://localhost/pgvector_rust_test").unwrap();
        diesel::sql_query("CREATE EXTENSION IF NOT EXISTS vector").execute(&mut conn)?;
        diesel::sql_query("DROP TABLE IF EXISTS diesel_half_items").execute(&mut conn)?;
        diesel::sql_query(
            "CREATE TABLE diesel_half_items (id serial PRIMARY KEY, embedding halfvec(3))",
        )
        .execute(&mut conn)?;

        let new_items = vec![
            NewItem {
                embedding: Some(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])),
            },
            NewItem {
                embedding: Some(HalfVector::from_f32_slice(&[2.0, 2.0, 2.0])),
            },
            NewItem {
                embedding: Some(HalfVector::from_f32_slice(&[1.0, 1.0, 2.0])),
            },
            NewItem { embedding: None },
        ];

        diesel::insert_into(items::table)
            .values(&new_items)
            .get_results::<Item>(&mut conn)?;

        let all = items::table.load::<Item>(&mut conn)?;
        assert_eq!(4, all.len());

        let neighbors = items::table
            .order(items::embedding.l2_distance(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 3, 2, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );
        assert_eq!(
            Some(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])),
            neighbors.first().unwrap().embedding
        );

        let neighbors = items::table
            .order(items::embedding.max_inner_product(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![2, 3, 1, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let neighbors = items::table
            .order(items::embedding.cosine_distance(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 2, 3, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let neighbors = items::table
            .order(items::embedding.l1_distance(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![1, 3, 2, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let distances = items::table
            .select(
                items::embedding.max_inner_product(HalfVector::from_f32_slice(&[1.0, 1.0, 1.0])),
            )
            .order(items::id)
            .load::<Option<f64>>(&mut conn)?;
        assert_eq!(vec![Some(-3.0), Some(-6.0), Some(-4.0), None], distances);

        Ok(())
    }
}
