use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;
use ordered_float::OrderedFloat;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;

use crate::auth::CredentialProvider;
use crate::models::LanguageModel;

#[derive(Debug, PartialEq, Clone)]
pub struct Embedding(pub Vec<f32>);

// This is needed for semantic index functionality
// Unfortunately it has to live wherever the "Embedding" struct is created.
// Keeping this in here though, introduces a 'rusqlite' dependency into AI
// which is less than ideal
impl FromSql for Embedding {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let bytes = value.as_blob()?;
        let embedding =
            bincode::deserialize(bytes).map_err(|err| rusqlite::types::FromSqlError::Other(err))?;
        Ok(Embedding(embedding))
    }
}

impl ToSql for Embedding {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let bytes = bincode::serialize(&self.0)
            .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Blob(bytes)))
    }
}
impl From<Vec<f32>> for Embedding {
    fn from(value: Vec<f32>) -> Self {
        Embedding(value)
    }
}

impl Embedding {
    pub fn similarity(&self, other: &Self) -> OrderedFloat<f32> {
        let len = self.0.len();
        assert_eq!(len, other.0.len());

        let mut result = 0.0;
        unsafe {
            matrixmultiply::sgemm(
                1,
                len,
                1,
                1.0,
                self.0.as_ptr(),
                len as isize,
                1,
                other.0.as_ptr(),
                1,
                len as isize,
                0.0,
                &mut result as *mut f32,
                1,
                1,
            );
        }
        OrderedFloat(result)
    }
}

#[async_trait]
pub trait EmbeddingProvider: CredentialProvider {
    fn base_model(&self) -> Box<dyn LanguageModel>;
    async fn embed_batch(&self, spans: Vec<String>) -> Result<Vec<Embedding>>;
    fn max_tokens_per_batch(&self) -> usize;
    fn rate_limit_expiration(&self) -> Option<Instant>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[gpui::test]
    fn test_similarity(mut rng: StdRng) {
        assert_eq!(
            Embedding::from(vec![1., 0., 0., 0., 0.])
                .similarity(&Embedding::from(vec![0., 1., 0., 0., 0.])),
            0.
        );
        assert_eq!(
            Embedding::from(vec![2., 0., 0., 0., 0.])
                .similarity(&Embedding::from(vec![3., 1., 0., 0., 0.])),
            6.
        );

        for _ in 0..100 {
            let size = 1536;
            let mut a = vec![0.; size];
            let mut b = vec![0.; size];
            for (a, b) in a.iter_mut().zip(b.iter_mut()) {
                *a = rng.gen();
                *b = rng.gen();
            }
            let a = Embedding::from(a);
            let b = Embedding::from(b);

            assert_eq!(
                round_to_decimals(a.similarity(&b), 1),
                round_to_decimals(reference_dot(&a.0, &b.0), 1)
            );
        }

        fn round_to_decimals(n: OrderedFloat<f32>, decimal_places: i32) -> f32 {
            let factor = 10.0_f32.powi(decimal_places);
            (n * factor).round() / factor
        }

        fn reference_dot(a: &[f32], b: &[f32]) -> OrderedFloat<f32> {
            OrderedFloat(a.iter().zip(b.iter()).map(|(a, b)| a * b).sum())
        }
    }
}
