use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};
use std::convert::TryFrom;

use crate::SparseVector;

impl Type<Postgres> for SparseVector {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("sparsevec")
    }
}

impl Encode<'_, Postgres> for SparseVector {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let dim = self.dim;
        let nnz = self.indices.len();
        buf.extend(dim.to_be_bytes());
        buf.extend(&i32::try_from(nnz)?.to_be_bytes());
        buf.extend(&0_i32.to_be_bytes());

        for v in &self.indices {
            buf.extend(&v.to_be_bytes());
        }

        for v in &self.values {
            buf.extend(&v.to_be_bytes());
        }

        Ok(IsNull::No)
    }
}

impl Decode<'_, Postgres> for SparseVector {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let buf = <&[u8] as Decode<Postgres>>::decode(value)?;
        SparseVector::from_sql(buf)
    }
}

impl PgHasArrayType for SparseVector {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_sparsevec")
    }
}

#[cfg(test)]
mod tests {
    use crate::SparseVector;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Row;

    #[async_std::test]
    async fn it_works() -> Result<(), sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://localhost/pgvector_rust_test")
            .await?;

        sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
            .execute(&pool)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS sqlx_sparse_items")
            .execute(&pool)
            .await?;
        sqlx::query(
            "CREATE TABLE sqlx_sparse_items (id bigserial PRIMARY KEY, embedding sparsevec(3))",
        )
        .execute(&pool)
        .await?;

        let vec = SparseVector::from_dense(&[1.0, 2.0, 3.0]);
        let vec2 = SparseVector::from_dense(&[4.0, 5.0, 6.0]);
        sqlx::query("INSERT INTO sqlx_sparse_items (embedding) VALUES ($1), ($2), (NULL)")
            .bind(&vec)
            .bind(&vec2)
            .execute(&pool)
            .await?;

        let query_vec = SparseVector::from_dense(&[3.0, 1.0, 2.0]);
        let row = sqlx::query(
            "SELECT embedding FROM sqlx_sparse_items ORDER BY embedding <-> $1 LIMIT 1",
        )
        .bind(query_vec)
        .fetch_one(&pool)
        .await?;
        let res_vec: SparseVector = row.try_get("embedding").unwrap();
        assert_eq!(vec, res_vec);
        assert_eq!(vec![1.0, 2.0, 3.0], res_vec.to_vec());

        let empty_vec = SparseVector::from_dense(&[]);
        let empty_res = sqlx::query("INSERT INTO sqlx_sparse_items (embedding) VALUES ($1)")
            .bind(&empty_vec)
            .execute(&pool)
            .await;
        assert!(empty_res.is_err());
        assert!(empty_res
            .unwrap_err()
            .to_string()
            .contains("sparsevec must have at least 1 dimension"));

        let null_row =
            sqlx::query("SELECT embedding FROM sqlx_sparse_items WHERE embedding IS NULL LIMIT 1")
                .fetch_one(&pool)
                .await?;
        let null_res: Option<SparseVector> = null_row.try_get("embedding").unwrap();
        assert!(null_res.is_none());

        // ensures binary format is correct
        let text_row =
            sqlx::query("SELECT embedding::text FROM sqlx_sparse_items ORDER BY id LIMIT 1")
                .fetch_one(&pool)
                .await?;
        let text_res: String = text_row.try_get("embedding").unwrap();
        assert_eq!("{1:1,2:2,3:3}/3", text_res);

        sqlx::query("ALTER TABLE sqlx_sparse_items ADD COLUMN factors sparsevec[]")
            .execute(&pool)
            .await?;

        let vecs = &[vec, vec2];
        sqlx::query("INSERT INTO sqlx_sparse_items (factors) VALUES ($1)")
            .bind(vecs)
            .execute(&pool)
            .await?;

        Ok(())
    }
}
