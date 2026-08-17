use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_class_text_label_member_annotation_and_css_classes() {
    let engine = Engine::new();
    let text = r#"classDiagram
class C1["Class 1 with text label"]
<<interface>> C1
C1: member1
cssClass "C1" styleClass
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "classDiagram");

    let c1 = &res.model["classes"]["C1"];
    assert_eq!(c1["label"], json!("Class 1 with text label"));
    assert_eq!(c1["cssClasses"], json!("default styleClass"));
    assert_eq!(c1["annotations"][0], json!("interface"));
    assert_eq!(c1["members"][0]["displayText"], json!("member1"));
}

#[test]
fn parse_diagram_class_css_class_shorthand() {
    let engine = Engine::new();
    let text = r#"classDiagram
class C1["Class 1 with text label"]:::styleClass
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c1 = &res.model["classes"]["C1"];
    assert_eq!(c1["label"], json!("Class 1 with text label"));
    assert_eq!(c1["cssClasses"], json!("default styleClass"));
}

#[test]
fn parse_diagram_class_namespace_and_generic_methods() {
    let engine = Engine::new();
    let text = r#"classDiagram
namespace Company.Project {
  class User {
    +login(username: String, password: String)
    +logout()
  }
}
namespace Company.Project.Module {
  class GenericClass~T~ {
    +addItem(item: T)
    +getItem() T
  }
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let user = &res.model["classes"]["User"];
    assert_eq!(user["parent"], json!("Company.Project"));
    assert_eq!(
        user["methods"][0]["displayText"],
        json!("+login(username: String, password: String)")
    );
    assert_eq!(user["methods"][1]["displayText"], json!("+logout()"));

    let generic = &res.model["classes"]["GenericClass"];
    assert_eq!(generic["type"], json!("T"));
    assert_eq!(generic["parent"], json!("Company.Project.Module"));
    assert_eq!(
        generic["methods"][0]["displayText"],
        json!("+addItem(item: T)")
    );
    assert_eq!(
        generic["methods"][1]["displayText"],
        json!("+getItem() : T")
    );
}

#[test]
fn parse_diagram_class_method_parser_matches_upstream_greedy_regex_boundary() {
    let engine = Engine::new();
    let text = r#"classDiagram
class Parser {
  +outer(inner)(value) Result$
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let method = &res.model["classes"]["Parser"]["methods"][0];
    assert_eq!(
        method["displayText"],
        json!("+outer(inner)(value) : Result")
    );
    assert_eq!(method["id"], json!("outer(inner)"));
    assert_eq!(method["parameters"], json!("value"));
    assert_eq!(method["returnType"], json!("Result"));
    assert_eq!(method["cssStyle"], json!("text-decoration:underline;"));
}

#[test]
fn parse_diagram_class_acc_descr_multiline_collapses_newline_whitespace_without_regex() {
    let engine = Engine::new();
    let text = r#"classDiagram
accTitle: My Title
accDescr {
  This is my multi
  line description
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["accTitle"], json!("My Title"));
    assert_eq!(
        res.model["accDescr"],
        json!("This is my multi\nline description")
    );
}

#[test]
fn parse_diagram_class_hierarchical_dotted_namespace_and_notes() {
    let engine = Engine::new();
    let text = r#"classDiagram
namespace Company.Project.Module {
  class User
  note "Module scoped note"
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let namespaces = res.model["namespaces"].as_object().unwrap();
    assert!(namespaces.contains_key("Company"));
    assert!(namespaces.contains_key("Company.Project"));
    assert!(namespaces.contains_key("Company.Project.Module"));
    assert_eq!(namespaces["Company"]["label"], json!("Company"));
    assert_eq!(namespaces["Company"]["parent"], json!(null));
    assert_eq!(namespaces["Company"]["explicit"], json!(false));
    assert_eq!(namespaces["Company.Project"]["label"], json!("Project"));
    assert_eq!(namespaces["Company.Project"]["parent"], json!("Company"));
    assert_eq!(namespaces["Company.Project"]["explicit"], json!(false));
    assert_eq!(
        namespaces["Company.Project.Module"]["label"],
        json!("Module")
    );
    assert_eq!(
        namespaces["Company.Project.Module"]["parent"],
        json!("Company.Project")
    );
    assert_eq!(
        namespaces["Company.Project.Module"]["classIds"],
        json!(["User"])
    );
    assert_eq!(
        namespaces["Company.Project.Module"]["noteIds"],
        json!(["note0"])
    );

    assert_eq!(
        res.model["classes"]["User"]["parent"],
        json!("Company.Project.Module")
    );
    assert_eq!(
        res.model["notes"][0]["parent"],
        json!("Company.Project.Module")
    );
    assert_eq!(res.model["notes"][0]["class"], json!(null));
    assert_eq!(res.model["notes"][0]["text"], json!("Module scoped note"));
}

#[test]
fn parse_diagram_class_nested_namespace_syntax_builds_qualified_parents() {
    let engine = Engine::new();
    let text = r#"classDiagram
namespace Company {
  namespace Project {
    class User
  }
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["namespaces"]["Company"]["explicit"], json!(true));
    assert_eq!(
        res.model["namespaces"]["Company.Project"]["parent"],
        json!("Company")
    );
    assert_eq!(
        res.model["namespaces"]["Company.Project"]["explicit"],
        json!(true)
    );
    assert_eq!(
        res.model["classes"]["User"]["parent"],
        json!("Company.Project")
    );
}

#[test]
fn parse_diagram_class_hierarchical_namespaces_can_be_disabled() {
    let engine = Engine::new();
    let text = r#"---
config:
  class:
    hierarchicalNamespaces: false
---
classDiagram
namespace Company.Project.Module {
  class User
  note "Module scoped note"
}
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let namespaces = res.model["namespaces"].as_object().unwrap();
    assert_eq!(
        namespaces.keys().cloned().collect::<Vec<_>>(),
        vec!["Company.Project.Module"]
    );
    assert_eq!(
        namespaces["Company.Project.Module"]["label"],
        json!("Company.Project.Module")
    );
    assert_eq!(namespaces["Company.Project.Module"]["parent"], json!(null));
    assert_eq!(
        res.model["classes"]["User"]["parent"],
        json!("Company.Project.Module")
    );
    assert_eq!(
        res.model["notes"][0]["parent"],
        json!("Company.Project.Module")
    );
}

#[test]
fn parse_diagram_class_relation_only_generic_classes_keep_type_params() {
    let engine = Engine::new();
    let text = r#"classDiagram
Class01~T~ <|-- AveryLongClass
Class03~T~ *-- Class04~T~
Class08 <--> C2: Cool label
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let class01 = &res.model["classes"]["Class01"];
    assert_eq!(class01["type"], json!("T"));
    assert_eq!(class01["text"], json!("Class01&lt;T&gt;"));

    let class03 = &res.model["classes"]["Class03"];
    assert_eq!(class03["type"], json!("T"));
    assert_eq!(class03["text"], json!("Class03&lt;T&gt;"));

    let class04 = &res.model["classes"]["Class04"];
    assert_eq!(class04["type"], json!("T"));
    assert_eq!(class04["text"], json!("Class04&lt;T&gt;"));
}

#[test]
fn parse_diagram_class_relation_with_label_and_direction() {
    let engine = Engine::new();
    let text = r#"classDiagram
direction LR
class Admin
class Report
Admin --> Report : generates
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("LR"));

    let rels = res.model["relations"].as_array().unwrap();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0]["id1"], json!("Admin"));
    assert_eq!(rels[0]["id2"], json!("Report"));
    assert_eq!(rels[0]["title"], json!("generates"));
    assert_eq!(rels[0]["relation"]["lineType"], json!(0));
    assert_eq!(rels[0]["relation"]["type1"], json!(-1));
    assert_eq!(rels[0]["relation"]["type2"], json!(3));
}

#[test]
fn parse_diagram_class_style_statement_sets_node_styles() {
    let engine = Engine::new();
    let text = r#"classDiagram
class Class01
style Class01 fill:#f9f,stroke:#333,stroke-width:4px
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c = &res.model["classes"]["Class01"];
    assert_eq!(
        c["styles"],
        json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
    );
}

#[test]
fn parse_diagram_class_classdef_applies_styles_to_css_classes() {
    let engine = Engine::new();
    let text = r#"classDiagram
class Class01
cssClass "Class01" pink
classDef pink fill:#f9f
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c = &res.model["classes"]["Class01"];
    assert_eq!(c["cssClasses"], json!("default pink"));
    assert_eq!(c["styles"], json!(["fill:#f9f"]));
}

#[test]
fn parse_diagram_class_multiple_classdefs_merge_styles() {
    let engine = Engine::new();
    let text = r#"classDiagram
class Class01:::pink
cssClass "Class01" bold
classDef pink fill:#f9f
classDef bold stroke:#333,stroke-width:6px
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c = &res.model["classes"]["Class01"];
    assert_eq!(c["cssClasses"], json!("default pink bold"));
    assert_eq!(
        c["styles"],
        json!(["fill:#f9f", "stroke:#333", "stroke-width:6px"])
    );
}

#[test]
fn parse_diagram_class_link_and_click_statements_set_clickable_and_metadata() {
    let engine = Engine::new();
    let text = r#"classDiagram
class Class1
link Class1 "google.com" "A tooltip" _self
click Class1 href "example.com" "B tooltip" _blank
click Class1 call functionCall(test, test1, test2) "C tooltip"
callback Class1 "otherCall" "D tooltip"
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let c = &res.model["classes"]["Class1"];
    assert!(c["cssClasses"].as_str().unwrap().contains("clickable"));
    assert_eq!(c["link"], json!("example.com"));
    assert_eq!(c["linkTarget"], json!("_blank"));
    assert_eq!(c["tooltip"], json!("D tooltip"));
    assert_eq!(c["haveCallback"], json!(true));
    assert_eq!(c["callback"]["function"], json!("otherCall"));
    assert_eq!(c["callbackEffective"], json!(false));
}

#[test]
fn parse_diagram_class_href_sanitizes_javascript_urls_when_not_loose() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram(
        r#"classDiagram
class Class1
click Class1 href "javascript:alert(1)" "A tooltip" _self"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    let c = &res.model["classes"]["Class1"];
    assert_eq!(c["link"], json!("about:blank"));
    assert_eq!(c["linkTarget"], json!("_self"));
    assert_eq!(c["tooltip"], json!("A tooltip"));
}

#[test]
fn parse_diagram_class_security_level_sandbox_forces_link_target_top() {
    let engine = Engine::new().with_site_config({
        let mut cfg = MermaidConfig::empty_object();
        cfg.set_value("securityLevel", json!("sandbox"));
        cfg
    });

    let text = r#"classDiagram
class Class1
click Class1 href "google.com" "A tooltip" _self
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c = &res.model["classes"]["Class1"];
    assert_eq!(c["link"], json!("google.com"));
    assert_eq!(c["linkTarget"], json!("_top"));
}

#[test]
fn parse_diagram_class_security_level_loose_marks_callback_effective() {
    let engine = Engine::new().with_site_config({
        let mut cfg = MermaidConfig::empty_object();
        cfg.set_value("securityLevel", json!("loose"));
        cfg
    });

    let text = r#"classDiagram
class Class1
click Class1 call functionCall() "A tooltip"
"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let c = &res.model["classes"]["Class1"];
    assert_eq!(c["haveCallback"], json!(true));
    assert_eq!(c["callback"]["function"], json!("functionCall"));
    assert!(c["callback"].get("args").is_none());
    assert_eq!(c["callbackEffective"], json!(true));
}
