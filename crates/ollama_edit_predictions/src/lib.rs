mod ollama_edit_prediction_provider;

pub use ollama_edit_prediction_provider::{OLLAMA_DEBOUNCE_TIMEOUT, OllamaEditPredictionProvider};

// Re-export core ollama types that might be needed
pub use ollama::{AvailableModel, OLLAMA_API_URL};
