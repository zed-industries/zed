use ort::{Environment, ExecutionProvider, GraphOptimizationLevel};

struct CrossEncoder {}

impl CrossEncoder {
    pub fn load() -> anyhow::Result<Self> {
        let environment = Environment::builder()
            .with_name("cross-encoder")
            .with_execution_providers([ExecutionProvider::CoreML(Default::default())])
            .build()?
            .into_arc();

        let model = "../models/cross-encoder.onnx";
        let mut session = environment
            .new_session_builder()
            .unwrap()
            .with_optimization_level(GraphOptimizationLevel::Basic)
            .unwrap()
            .with_number_threads(1)
            .unwrap()
            .with_model_from_file(model)
            .unwrap();

        Ok(Self {})
    }
}
