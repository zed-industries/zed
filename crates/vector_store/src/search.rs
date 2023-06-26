use std::{cmp::Ordering, path::PathBuf};

use async_trait::async_trait;
use ndarray::{Array1, Array2};

use crate::db::{DocumentRecord, VectorDatabase};
use anyhow::Result;

#[async_trait]
pub trait VectorSearch {
    // Given a query vector, and a limit to return
    // Return a vector of id, distance tuples.
    async fn top_k_search(&mut self, vec: &Vec<f32>, limit: usize) -> Vec<(usize, f32)>;
}

pub struct BruteForceSearch {
    document_ids: Vec<usize>,
    candidate_array: ndarray::Array2<f32>,
}

impl BruteForceSearch {
    pub fn load(db: &VectorDatabase) -> Result<Self> {
        let documents = db.get_documents()?;
        let embeddings: Vec<&DocumentRecord> = documents.values().into_iter().collect();
        let mut document_ids = vec![];
        for i in documents.keys() {
            document_ids.push(i.to_owned());
        }

        let mut candidate_array = Array2::<f32>::default((documents.len(), 1536));
        for (i, mut row) in candidate_array.axis_iter_mut(ndarray::Axis(0)).enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = embeddings[i].embedding.0[j];
            }
        }

        return Ok(BruteForceSearch {
            document_ids,
            candidate_array,
        });
    }
}

#[async_trait]
impl VectorSearch for BruteForceSearch {
    async fn top_k_search(&mut self, vec: &Vec<f32>, limit: usize) -> Vec<(usize, f32)> {
        let target = Array1::from_vec(vec.to_owned());

        let similarities = self.candidate_array.dot(&target);

        let similarities = similarities.to_vec();

        // construct a tuple vector from the floats, the tuple being (index,float)
        let mut with_indices = similarities
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| (self.document_ids[index], value))
            .collect::<Vec<(usize, f32)>>();

        // sort the tuple vector by float
        with_indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        with_indices.truncate(limit);
        with_indices
    }
}
