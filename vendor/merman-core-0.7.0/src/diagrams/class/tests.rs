use super::*;
use crate::{MermaidConfig, ParseMetadata};

fn meta() -> ParseMetadata {
    ParseMetadata {
        diagram_type: "classDiagram".to_string(),
        config: MermaidConfig::default(),
        effective_config: MermaidConfig::default(),
        title: None,
    }
}

#[test]
fn fast_parser_matches_lalrpop_for_basic_class_diagram() {
    let code = r#"classDiagram
class C1 {
  +String field1
  +method1()
}
C1 <|-- C2 : inherits
"#;
    let meta = meta();
    let slow = parse::parse_class_via_lalrpop(code, &meta).expect("slow parse");
    let fast = fast::parse_class_fast_db(code, &meta)
        .expect("fast parse")
        .expect("fast parser applicable")
        .into_model(&meta);
    assert_eq!(fast, slow);
}

#[test]
fn namespace_qualified_relation_endpoints_create_facade_classes_like_mermaid() {
    let code = r#"classDiagram
namespace Platform["Platform Layer"] {
  namespace FFI {
    class DartBinding
    class PythonBinding
  }
  namespace Core {
    class Renderer
  }
}
Platform.FFI.DartBinding --> Platform.Core.Renderer : calls
Platform.FFI.PythonBinding --> Platform.Core.Renderer : calls
"#;

    let model = parse::parse_class_typed(code, &meta()).expect("class diagram should parse");

    assert_eq!(
        model.classes.keys().cloned().collect::<Vec<_>>(),
        vec![
            "DartBinding",
            "PythonBinding",
            "Renderer",
            "Platform.FFI.DartBinding",
            "Platform.Core.Renderer",
            "Platform.FFI.PythonBinding"
        ]
    );
    assert_eq!(model.relations[0].id1, "Platform.FFI.DartBinding");
    assert_eq!(model.relations[0].id2, "Platform.Core.Renderer");
    assert_eq!(model.relations[1].id1, "Platform.FFI.PythonBinding");
    assert_eq!(model.relations[1].id2, "Platform.Core.Renderer");
    assert_eq!(
        model.namespaces["Platform.FFI"].class_ids,
        vec!["DartBinding", "PythonBinding"]
    );
    assert_eq!(
        model.namespaces["Platform.Core"].class_ids,
        vec!["Renderer"]
    );
}
