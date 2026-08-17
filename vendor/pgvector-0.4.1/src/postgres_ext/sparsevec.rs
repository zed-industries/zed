use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::convert::TryInto;
use std::error::Error;

use crate::SparseVector;

impl<'a> FromSql<'a> for SparseVector {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<SparseVector, Box<dyn Error + Sync + Send>> {
        SparseVector::from_sql(raw)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "sparsevec"
    }
}

impl ToSql for SparseVector {
    fn to_sql(&self, _ty: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let dim = self.dim;
        let nnz = self.indices.len();
        w.put_i32(dim);
        w.put_i32(nnz.try_into()?);
        w.put_i32(0);

        for v in &self.indices {
            w.put_i32(*v);
        }

        for v in &self.values {
            w.put_f32(*v);
        }

        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "sparsevec"
    }

    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use crate::SparseVector;
    use postgres::binary_copy::BinaryCopyInWriter;
    use postgres::types::{Kind, Type};
    use postgres::{Client, NoTls};

    #[test]
    fn it_works() -> Result<(), postgres::Error> {
        let user = std::env::var("USER").unwrap();
        let mut client = Client::configure()
            .host("localhost")
            .dbname("pgvector_rust_test")
            .user(user.as_str())
            .connect(NoTls)?;

        client.execute("CREATE EXTENSION IF NOT EXISTS vector", &[])?;
        client.execute("DROP TABLE IF EXISTS postgres_sparse_items", &[])?;
        client.execute(
            "CREATE TABLE postgres_sparse_items (id bigserial PRIMARY KEY, embedding sparsevec(3))",
            &[],
        )?;

        let vec = SparseVector::from_dense(&[1.0, 2.0, 3.0]);
        let vec2 = SparseVector::from_dense(&[4.0, 5.0, 6.0]);
        client.execute(
            "INSERT INTO postgres_sparse_items (embedding) VALUES ($1), ($2), (NULL)",
            &[&vec, &vec2],
        )?;

        let query_vec = SparseVector::from_dense(&[3.0, 1.0, 2.0]);
        let row = client.query_one(
            "SELECT embedding FROM postgres_sparse_items ORDER BY embedding <-> $1 LIMIT 1",
            &[&query_vec],
        )?;
        let res_vec: SparseVector = row.get(0);
        assert_eq!(vec, res_vec);

        let null_row = client.query_one(
            "SELECT embedding FROM postgres_sparse_items WHERE embedding IS NULL LIMIT 1",
            &[],
        )?;
        let null_res: Option<SparseVector> = null_row.get(0);
        assert!(null_res.is_none());

        // ensures binary format is correct
        let text_row = client.query_one(
            "SELECT embedding::text FROM postgres_sparse_items ORDER BY id LIMIT 1",
            &[],
        )?;
        let text_res: String = text_row.get(0);
        assert_eq!("{1:1,2:2,3:3}/3", text_res);

        // copy
        let sparsevec_type = get_type(&mut client, "sparsevec")?;
        let writer = client
            .copy_in("COPY postgres_sparse_items (embedding) FROM STDIN WITH (FORMAT BINARY)")?;
        let mut writer = BinaryCopyInWriter::new(writer, &[sparsevec_type]);
        writer.write(&[&SparseVector::from_dense(&[1.0, 2.0, 3.0])])?;
        writer.write(&[&SparseVector::from_dense(&[4.0, 5.0, 6.0])])?;
        writer.finish()?;

        Ok(())
    }

    fn get_type(client: &mut Client, name: &str) -> Result<Type, postgres::Error> {
        let row = client.query_one("SELECT pg_type.oid, nspname AS schema FROM pg_type INNER JOIN pg_namespace ON pg_namespace.oid = pg_type.typnamespace WHERE typname = $1", &[&name])?;
        Ok(Type::new(
            name.into(),
            row.get("oid"),
            Kind::Simple,
            row.get("schema"),
        ))
    }
}
