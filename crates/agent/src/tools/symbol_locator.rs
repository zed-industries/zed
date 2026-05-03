use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Identifies a specific symbol (declaration or usage) in the source code.
///
/// Use the file path, line number, and symbol name from file outlines, grep results,
/// or other tool outputs to populate these fields.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SymbolLocator {
    /// The relative path of the file containing the symbol
    /// (e.g. "crates/editor/src/editor.rs").
    pub file_path: String,

    /// The 1-based line number where the symbol appears.
    /// Use the line numbers from file outlines or grep results.
    pub line: u32,

    /// The name of the symbol (function name, type name, variable name, etc.)
    pub symbol_name: String,
}
