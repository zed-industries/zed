use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_info_basic() {
    let engine = Engine::new();
    let text = "info showInfo\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "info");
    assert_eq!(
        res.model,
        json!({
            "type": "info",
            "showInfo": true
        })
    );
}

#[test]
fn parse_diagram_info_rejects_unsupported_grammar_like_upstream() {
    let engine = Engine::new();
    let text = "info unsupported\n";
    let err = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap_err()
        .to_string();
    assert_eq!(
        err,
        "Diagram parse error (info): Parsing failed: unexpected character: ->u<- at offset: 5, skipped 11 characters."
    );
}
