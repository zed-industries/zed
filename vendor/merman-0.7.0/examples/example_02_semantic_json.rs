mod support;

use merman::{Engine, ParseOptions};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_02_semantic_json",
        "flowchart TD\n  A[API] --> B[Semantic JSON]\n",
    )?;

    let engine = Engine::new();
    let Some(parsed) = engine.parse_diagram_sync(&input, ParseOptions::strict())? else {
        return Err("no Mermaid diagram detected".into());
    };

    // Keep metadata beside the semantic model so callers can route diagrams without reparsing.
    let output = json!({
        "diagramType": parsed.meta.diagram_type,
        "title": parsed.meta.title,
        "model": parsed.model,
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
