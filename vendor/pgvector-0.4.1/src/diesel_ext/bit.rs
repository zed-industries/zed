use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::query_builder::QueryId;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::SqlType;
use std::convert::TryFrom;
use std::io::Write;

use crate::Bit;

#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "bit"))]
pub struct BitType;

impl ToSql<BitType, Pg> for Bit {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let len = self.len;
        out.write_all(&i32::try_from(len)?.to_be_bytes())?;
        out.write_all(&self.data)?;
        Ok(IsNull::No)
    }
}

impl FromSql<BitType, Pg> for Bit {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        Bit::from_sql(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bit, VectorExpressionMethods};
    use diesel::prelude::*;

    table! {
        use diesel::sql_types::*;

        diesel_bit_items (id) {
            id -> Int4,
            embedding -> Nullable<crate::sql_types::Bit>,
        }
    }

    use diesel_bit_items as items;

    #[derive(Queryable)]
    #[diesel(table_name = items)]
    struct Item {
        pub id: i32,
        pub embedding: Option<Bit>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = items)]
    struct NewItem {
        pub embedding: Option<Bit>,
    }

    #[test]
    fn it_works() -> Result<(), diesel::result::Error> {
        let mut conn = PgConnection::establish("postgres://localhost/pgvector_rust_test").unwrap();
        diesel::sql_query("CREATE EXTENSION IF NOT EXISTS vector").execute(&mut conn)?;
        diesel::sql_query("DROP TABLE IF EXISTS diesel_bit_items").execute(&mut conn)?;
        diesel::sql_query(
            "CREATE TABLE diesel_bit_items (id serial PRIMARY KEY, embedding bit(9))",
        )
        .execute(&mut conn)?;

        let new_items = vec![
            NewItem {
                embedding: Some(Bit::new(&[
                    false, false, false, false, false, false, false, false, true,
                ])),
            },
            NewItem {
                embedding: Some(Bit::new(&[
                    false, true, false, true, false, false, false, false, true,
                ])),
            },
            NewItem {
                embedding: Some(Bit::new(&[
                    false, true, true, true, false, false, false, false, true,
                ])),
            },
            NewItem { embedding: None },
        ];

        diesel::insert_into(items::table)
            .values(&new_items)
            .get_results::<Item>(&mut conn)?;

        let all = items::table.load::<Item>(&mut conn)?;
        assert_eq!(4, all.len());

        let neighbors = items::table
            .order(items::embedding.hamming_distance(Bit::new(&[
                false, true, false, true, false, false, false, false, true,
            ])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![2, 3, 1, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );
        assert_eq!(
            Some(Bit::new(&[
                false, true, false, true, false, false, false, false, true
            ])),
            neighbors.first().unwrap().embedding
        );

        let neighbors = items::table
            .order(items::embedding.jaccard_distance(Bit::new(&[
                false, true, false, true, false, false, false, false, true,
            ])))
            .limit(5)
            .load::<Item>(&mut conn)?;
        assert_eq!(
            vec![2, 3, 1, 4],
            neighbors.iter().map(|v| v.id).collect::<Vec<i32>>()
        );

        let distances = items::table
            .select(items::embedding.hamming_distance(Bit::new(&[
                false, true, false, true, false, false, false, false, true,
            ])))
            .order(items::id)
            .load::<Option<f64>>(&mut conn)?;
        assert_eq!(vec![Some(2.0), Some(0.0), Some(1.0), None], distances);

        Ok(())
    }
}
