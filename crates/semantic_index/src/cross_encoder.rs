use ndarray::{Array1, Array2, Axis, CowArray};
use ort::{Environment, ExecutionProvider, GraphOptimizationLevel, Session, SessionBuilder, Value};
use tokenizers::Tokenizer;
use util::paths::MODELS_DIR;

struct CrossEncoder {
    session: Session,
    tokenizer: Tokenizer,
}

fn sigmoid(val: f32) -> f32 {
    1.0 / (1.0 + (-val).exp())
}

impl CrossEncoder {
    pub fn load() -> anyhow::Result<Self> {
        let model_path = MODELS_DIR.join("cross-encoder").join("model.onnx");
        let tokenizer_path = MODELS_DIR.join("cross-encoder").join("tokenizer.json");

        let environment = Environment::builder()
            .with_name("cross-encoder")
            .with_execution_providers([ExecutionProvider::CoreML(Default::default())])
            .build()?
            .into_arc();

        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Level1)?
            .with_intra_threads(1)?
            .with_model_from_file(model_path)?;

        let tokenizer = Tokenizer::from_file(tokenizer_path).unwrap();

        Ok(Self { session, tokenizer })
    }

    pub fn score(&self, query: &str, candidates: Vec<&str>) -> anyhow::Result<Vec<f32>> {
        let spans = candidates
            .into_iter()
            .map(|candidate| format!("{}. {}", query, candidate))
            .collect::<Vec<_>>();

        let encodings = self.tokenizer.encode_batch(spans, true).unwrap();

        let mut results = Vec::new();
        for encoding in encodings {
            // Get Input Variables Individually
            let input_ids = encoding.get_ids();
            let attention_mask = encoding.get_attention_mask();
            let token_type_ids = encoding.get_type_ids();
            let length = input_ids.len();

            // Convert to Arrays
            let inputs_ids_array = CowArray::from(ndarray::Array::from_shape_vec(
                (1, length),
                input_ids.iter().map(|&x| x as i64).collect(),
            )?);

            let attention_mask_array = CowArray::from(ndarray::Array::from_shape_vec(
                (1, length),
                attention_mask.iter().map(|&x| x as i64).collect(),
            )?)
            .into_dyn();

            let token_type_ids_array = CowArray::from(ndarray::Array::from_shape_vec(
                (1, length),
                token_type_ids.iter().map(|&x| x as i64).collect(),
            )?)
            .into_dyn();

            let outputs = self.session.run(vec![
                Value::from_array(self.session.allocator(), &inputs_ids_array.into_dyn())?,
                Value::from_array(self.session.allocator(), &attention_mask_array)?,
                Value::from_array(self.session.allocator(), &token_type_ids_array)?,
            ]);

            let output = outputs.unwrap()[0].try_extract::<f32>().unwrap();
            let value = output.view().to_owned();

            let val = value.as_slice().unwrap()[0];
            results.push(sigmoid(val))
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_encoder() {
        let cross_encoder = CrossEncoder::load().unwrap();

        let sample_candidates = vec!["I love you.", "I hate you."];
        let results = cross_encoder.score("I like you", sample_candidates.clone());
        assert_eq!(results.unwrap().len(), sample_candidates.len());
    }
}
