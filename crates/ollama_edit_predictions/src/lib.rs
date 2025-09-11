mod ollama_edit_prediction_provider;

pub use ollama_edit_prediction_provider::{
    OllamaEditPredictionProvider,
    State as OllamaEditPredictionState,
    OLLAMA_DEBOUNCE_TIMEOUT,
};

// Re-export core ollama types that might be needed
pub use ollama::{AvailableModel, OLLAMA_API_URL};
