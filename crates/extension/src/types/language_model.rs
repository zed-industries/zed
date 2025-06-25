pub use language_model::{LanguageModelToolSchemaFormat, request::LanguageModelToolChoice};

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;

    /// Whether this model supports images
    fn supports_images(&self) -> bool;

    /// Whether this model supports tools.
    fn supports_tools(&self) -> bool;

    /// Whether this model supports choosing which tool to use.
    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool;
    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }
}
