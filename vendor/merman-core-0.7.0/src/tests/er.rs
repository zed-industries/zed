use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_er_allows_standalone_entities() {
    let engine = Engine::new();
    let text = "erDiagram\nISLAND\nMAINLAND\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "er");
    assert_eq!(res.model["relationships"].as_array().unwrap().len(), 0);
    assert_eq!(res.model["entities"].as_object().unwrap().len(), 2);
    assert!(res.model["entities"].get("ISLAND").is_some());
    assert!(res.model["entities"].get("MAINLAND").is_some());
}

#[test]
fn parse_diagram_er_parses_alias_and_attributes() {
    let engine = Engine::new();
    let text = r#"erDiagram
foo["bar"] {
  string title PK, FK "comment"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let e = &res.model["entities"]["foo"];
    assert_eq!(e["alias"], json!("bar"));
    assert_eq!(e["attributes"][0]["type"], json!("string"));
    assert_eq!(e["attributes"][0]["name"], json!("title"));
    assert_eq!(e["attributes"][0]["keys"], json!(["PK", "FK"]));
    assert_eq!(e["attributes"][0]["comment"], json!("comment"));
}

#[test]
fn parse_diagram_er_empty_quoted_entity_name_is_error() {
    let engine = Engine::new();
    let err = block_on(engine.parse_diagram("erDiagram\n\"\"\n", ParseOptions::default()))
        .unwrap_err()
        .to_string();
    assert!(err.contains("DiagramParse") || err.contains("Unsupported") || !err.is_empty());
}

#[test]
fn parse_diagram_er_rejects_percent_and_backslash_in_quoted_entity_name() {
    let engine = Engine::new();
    assert!(
        block_on(engine.parse_diagram("erDiagram\n\"Blo%rf\"\n", ParseOptions::default())).is_err()
    );
    assert!(
        block_on(engine.parse_diagram("erDiagram\n\"Blo\\\\rf\"\n", ParseOptions::default()))
            .is_err()
    );
}

#[test]
fn parse_diagram_er_supports_empty_attribute_blocks_and_multiple_blocks() {
    let engine = Engine::new();
    let text = r#"erDiagram
BOOK {}
BOOK {
  string title
}
BOOK{
  string author
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let e = &res.model["entities"]["BOOK"];
    assert_eq!(e["attributes"].as_array().unwrap().len(), 2);
    assert_eq!(e["attributes"][0]["name"], json!("title"));
    assert_eq!(e["attributes"][1]["name"], json!("author"));
}

#[test]
fn parse_diagram_er_alias_applies_even_when_relationship_is_defined_first() {
    let engine = Engine::new();
    let text = r#"erDiagram
foo ||--o{ bar : rel
foo["batman"]
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["entities"]["foo"]["alias"], json!("batman"));
    assert_eq!(res.model["entities"]["bar"]["alias"], json!(""));
}

#[test]
fn parse_diagram_er_allows_multiple_statements_without_newlines_like_upstream_jison() {
    let engine = Engine::new();
    let text = r#"erDiagram
foo ||--o{ bar : rel
buzz foo["batman"]
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["entities"]["foo"]["alias"], json!("batman"));
    assert_eq!(res.model["entities"]["bar"]["alias"], json!(""));
    assert!(res.model["entities"].get("buzz").is_some());
}

#[test]
fn parse_diagram_er_self_relationship_does_not_duplicate_entity() {
    let engine = Engine::new();
    let text = r#"erDiagram
NODE ||--o{ NODE : "leads to"
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["entities"].as_object().unwrap().len(), 1);
    let rels = res.model["relationships"].as_array().unwrap();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0]["entityA"], rels[0]["entityB"]);
}

#[test]
fn parse_diagram_er_inline_class_assignment_applies_css_classes() {
    let engine = Engine::new();
    let text = r#"erDiagram
FOO:::pink
classDef pink fill:#f9f
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let e = &res.model["entities"]["FOO"];
    assert_eq!(e["cssClasses"], json!("default pink"));
    assert_eq!(res.model["classes"]["pink"]["styles"], json!(["fill:#f9f"]));
}

#[test]
fn parse_diagram_er_direction_statement_sets_direction() {
    let engine = Engine::new();
    let text = r#"erDiagram
direction LR
A ||--o{ B : has
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("LR"));
}

#[test]
fn parse_diagram_er_allows_hyphen_and_underscore_in_unquoted_entity_name() {
    let engine = Engine::new();
    let text = "erDiagram\nDUCK-BILLED_PLATYPUS\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert!(res.model["entities"].get("DUCK-BILLED_PLATYPUS").is_some());
}

#[test]
fn parse_diagram_er_rejects_unquoted_entity_names_with_non_identifier_punctuation() {
    let engine = Engine::new();
    assert!(
        block_on(engine.parse_diagram("erDiagram\nBlo@rf\n", ParseOptions::default())).is_err()
    );
    assert!(
        block_on(engine.parse_diagram("erDiagram\nBlo?rf\n", ParseOptions::default())).is_err()
    );
}

#[test]
fn parse_diagram_er_supports_attribute_name_brackets_and_parens() {
    let engine = Engine::new();
    let text = r#"erDiagram
BOOK {
  string author-ref[name](1)
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let attrs = res.model["entities"]["BOOK"]["attributes"]
        .as_array()
        .unwrap();
    assert_eq!(attrs.len(), 1);
    assert_eq!(attrs[0]["name"], json!("author-ref[name](1)"));
}

#[test]
fn parse_diagram_er_allows_asterisk_at_start_of_attribute_name() {
    let engine = Engine::new();
    let text = r#"erDiagram
BOOK {
  string *title
  id *the_Primary_Key
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let attrs = res.model["entities"]["BOOK"]["attributes"]
        .as_array()
        .unwrap();
    assert_eq!(attrs.len(), 2);
}

#[test]
fn parse_diagram_er_rejects_attribute_names_with_leading_numbers_dashes_or_brackets() {
    let engine = Engine::new();
    for bad in ["0author", "-author", "[author", "(author"] {
        let text = format!("erDiagram\nBOOK {{\n  string {bad}\n}}\n");
        assert!(block_on(engine.parse_diagram(&text, ParseOptions::default())).is_err());
    }
}

#[test]
fn parse_diagram_er_supports_generic_and_array_and_limited_length_types() {
    let engine = Engine::new();
    let text = r#"erDiagram
BOOK {
  type~T~ type
  option~T~ readable "comment"
  string[] readers FK
  character(10) isbn FK
  varchar(5) postal_code "Five digits"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let attrs = res.model["entities"]["BOOK"]["attributes"]
        .as_array()
        .unwrap();
    assert_eq!(attrs.len(), 5);
    assert_eq!(attrs[3]["type"], json!("character(10)"));
    assert_eq!(attrs[4]["type"], json!("varchar(5)"));
    assert_eq!(attrs[4]["comment"], json!("Five digits"));
}

#[test]
fn parse_diagram_er_parses_many_constraints_and_comments() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER {
  int customer_number PK, FK "comment1"
  datetime customer_status_start_datetime PK,UK, FK
  datetime customer_status_end_datetime PK , UK "comment3"
  string customer_firstname
  string customer_lastname "comment5"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let attrs = res.model["entities"]["CUSTOMER"]["attributes"]
        .as_array()
        .unwrap();
    assert_eq!(attrs[0]["keys"], json!(["PK", "FK"]));
    assert_eq!(attrs[0]["comment"], json!("comment1"));
    assert_eq!(attrs[1]["keys"], json!(["PK", "UK", "FK"]));
    assert_eq!(attrs[2]["keys"], json!(["PK", "UK"]));
    assert_eq!(attrs[2]["comment"], json!("comment3"));
    assert_eq!(attrs[3]["keys"], json!([]));
    assert_eq!(attrs[4]["keys"], json!([]));
    assert_eq!(attrs[4]["comment"], json!("comment5"));
}

#[test]
fn parse_diagram_er_allows_multiple_relationships_between_same_two_entities() {
    let engine = Engine::new();
    let text = r#"erDiagram
CAR ||--o{ PERSON : "insured for"
CAR }o--|| PERSON : "owned by"
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["entities"].as_object().unwrap().len(), 2);
    assert_eq!(res.model["relationships"].as_array().unwrap().len(), 2);
}

#[test]
fn parse_diagram_er_supports_one_or_more_cardinality_markers() {
    let engine = Engine::new();
    let text = r#"erDiagram
A ||--|{ B : has
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let rels = res.model["relationships"].as_array().unwrap();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0]["relSpec"]["cardA"], json!("ONE_OR_MORE"));
    assert_eq!(rels[0]["relSpec"]["cardB"], json!("ONLY_ONE"));
}

#[test]
fn parse_diagram_er_relationship_matrix_matches_upstream_spec_minimally() {
    let engine = Engine::new();
    let cases: &[(&str, &str, &str, &str)] = &[
        ("A ||--|{ B : has", "ONE_OR_MORE", "ONLY_ONE", "IDENTIFYING"),
        (
            "A ||..o{ B : has",
            "ZERO_OR_MORE",
            "ONLY_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A |o..o{ B : has",
            "ZERO_OR_MORE",
            "ZERO_OR_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A |o--|{ B : has",
            "ONE_OR_MORE",
            "ZERO_OR_ONE",
            "IDENTIFYING",
        ),
        ("A }|--|| B : has", "ONLY_ONE", "ONE_OR_MORE", "IDENTIFYING"),
        (
            "A }o--|| B : has",
            "ONLY_ONE",
            "ZERO_OR_MORE",
            "IDENTIFYING",
        ),
        (
            "A }o..o| B : has",
            "ZERO_OR_ONE",
            "ZERO_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "A }|..o| B : has",
            "ZERO_OR_ONE",
            "ONE_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "A |o..|| B : has",
            "ONLY_ONE",
            "ZERO_OR_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A ||..|| B : has",
            "ONLY_ONE",
            "ONLY_ONE",
            "NON_IDENTIFYING",
        ),
        ("A ||--o| B : has", "ZERO_OR_ONE", "ONLY_ONE", "IDENTIFYING"),
        (
            "A |o..o| B : has",
            "ZERO_OR_ONE",
            "ZERO_OR_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A }o--o{ B : has",
            "ZERO_OR_MORE",
            "ZERO_OR_MORE",
            "IDENTIFYING",
        ),
        (
            "A }|..|{ B : has",
            "ONE_OR_MORE",
            "ONE_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "A }o--|{ B : has",
            "ONE_OR_MORE",
            "ZERO_OR_MORE",
            "IDENTIFYING",
        ),
        (
            "A }|..o{ B : has",
            "ZERO_OR_MORE",
            "ONE_OR_MORE",
            "NON_IDENTIFYING",
        ),
        // relType variants
        (
            "A ||.-o{ B : has",
            "ZERO_OR_MORE",
            "ONLY_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A ||-.o{ B : has",
            "ZERO_OR_MORE",
            "ONLY_ONE",
            "NON_IDENTIFYING",
        ),
    ];

    for (line, card_a, card_b, rel_type) in cases {
        let text = format!("erDiagram\n{line}\n");
        let res = block_on(engine.parse_diagram(&text, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let rels = res.model["relationships"].as_array().unwrap();
        assert_eq!(rels.len(), 1, "{line}");
        assert_eq!(rels[0]["relSpec"]["cardA"], json!(*card_a), "{line}");
        assert_eq!(rels[0]["relSpec"]["cardB"], json!(*card_b), "{line}");
        assert_eq!(rels[0]["relSpec"]["relType"], json!(*rel_type), "{line}");
    }
}

#[test]
fn parse_diagram_er_relationship_word_aliases_match_upstream_spec_minimally() {
    let engine = Engine::new();
    let cases: &[(&str, &str, &str, &str)] = &[
        (
            "A one or zero to many B : has",
            "ZERO_OR_MORE",
            "ZERO_OR_ONE",
            "IDENTIFYING",
        ),
        (
            "A one or many optionally to zero or one B : has",
            "ZERO_OR_ONE",
            "ONE_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "A zero or more to zero or many B : has",
            "ZERO_OR_MORE",
            "ZERO_OR_MORE",
            "IDENTIFYING",
        ),
        (
            "A many(0) to many(1) B : has",
            "ONE_OR_MORE",
            "ZERO_OR_MORE",
            "IDENTIFYING",
        ),
        (
            "A many optionally to one B : has",
            "ONLY_ONE",
            "ZERO_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "A only one optionally to 1+ B : has",
            "ONE_OR_MORE",
            "ONLY_ONE",
            "NON_IDENTIFYING",
        ),
        (
            "A 0+ optionally to 1 B : has",
            "ONLY_ONE",
            "ZERO_OR_MORE",
            "NON_IDENTIFYING",
        ),
        (
            "HOUSE one to one ROOM : contains",
            "ONLY_ONE",
            "ONLY_ONE",
            "IDENTIFYING",
        ),
    ];

    for (line, card_a, card_b, rel_type) in cases {
        let text = format!("erDiagram\n{line}\n");
        let res = block_on(engine.parse_diagram(&text, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let rels = res.model["relationships"].as_array().unwrap();
        assert_eq!(rels.len(), 1, "{line}");
        assert_eq!(rels[0]["relSpec"]["cardA"], json!(*card_a), "{line}");
        assert_eq!(rels[0]["relSpec"]["cardB"], json!(*card_b), "{line}");
        assert_eq!(rels[0]["relSpec"]["relType"], json!(*rel_type), "{line}");
    }
}

#[test]
fn parse_diagram_er_rejects_invalid_relationship_syntax() {
    let engine = Engine::new();
    assert!(
        block_on(engine.parse_diagram("erDiagram\nA xxx B : has\n", ParseOptions::default()))
            .is_err()
    );
}

#[test]
fn parse_diagram_er_style_statement_applies_styles() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER
style CUSTOMER color:red,stroke:blue,fill:#f9f
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssStyles"],
        json!(["color:red", "stroke:blue", "fill:#f9f"])
    );
}

#[test]
fn parse_diagram_er_style_statements_append_across_multiple_lines() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER
style CUSTOMER color:red
style CUSTOMER fill:#f9f
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssStyles"],
        json!(["color:red", "fill:#f9f"])
    );
}

#[test]
fn parse_diagram_er_class_statement_assigns_classes() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER
class CUSTOMER firstClass, secondClass, thirdClass
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssClasses"],
        json!("default firstClass secondClass thirdClass")
    );
}

#[test]
fn parse_diagram_er_class_statement_appends_across_multiple_lines() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER
class CUSTOMER firstClass
class CUSTOMER secondClass
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssClasses"],
        json!("default firstClass secondClass")
    );
}

#[test]
fn parse_diagram_er_classdef_defines_styles_and_text_styles() {
    let engine = Engine::new();
    let text = r#"erDiagram
classDef myClass fill:#f9f, stroke: red, color: pink
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["classes"]["myClass"],
        json!({
            "id": "myClass",
            "styles": ["fill:#f9f", "stroke:red", "color:pink"],
            "textStyles": ["color:pink"]
        })
    );
}

#[test]
fn parse_diagram_er_classdef_supports_multiple_classes_in_one_statement() {
    let engine = Engine::new();
    let text = r#"erDiagram
classDef firstClass,secondClass fill:#f9f, stroke: red, color: pink
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["classes"]["firstClass"]["styles"][1],
        json!("stroke:red")
    );
    assert_eq!(
        res.model["classes"]["secondClass"]["styles"][2],
        json!("color:pink")
    );
}

#[test]
fn parse_diagram_er_shorthand_class_assignment_variants_work() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER:::myClass
CUSTOMER2:::myClass {}
CUSTOMER3:::myClass {
  string name
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssClasses"],
        json!("default myClass")
    );
    assert_eq!(
        res.model["entities"]["CUSTOMER2"]["cssClasses"],
        json!("default myClass")
    );
    assert_eq!(
        res.model["entities"]["CUSTOMER3"]["cssClasses"],
        json!("default myClass")
    );
}

#[test]
fn parse_diagram_er_shorthand_assignment_supports_multiple_classes_and_alias() {
    let engine = Engine::new();
    let text = r#"erDiagram
c[CUSTOMER]:::firstClass,secondClass
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["entities"]["c"]["alias"], json!("CUSTOMER"));
    assert_eq!(
        res.model["entities"]["c"]["cssClasses"],
        json!("default firstClass secondClass")
    );
}

#[test]
fn parse_diagram_er_shorthand_assignment_works_in_relationships() {
    let engine = Engine::new();
    let text = r#"erDiagram
CUSTOMER:::myClass ||--o{ PERSON:::myClass : allows
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["entities"]["CUSTOMER"]["cssClasses"],
        json!("default myClass")
    );
    assert_eq!(
        res.model["entities"]["PERSON"]["cssClasses"],
        json!("default myClass")
    );
}

#[test]
fn parse_diagram_er_relationship_labels_allow_empty_quoted_and_unquoted() {
    let engine = Engine::new();
    let res_empty = block_on(engine.parse_diagram(
        "erDiagram\nCUSTOMER ||--|{ ORDER : \"\"\n",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(res_empty.model["relationships"][0]["roleA"], json!(""));

    let res_unquoted = block_on(engine.parse_diagram(
        "erDiagram\nCUSTOMER ||--|{ ORDER : places\n",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res_unquoted.model["relationships"][0]["roleA"],
        json!("places")
    );
}

#[test]
fn parse_diagram_er_parent_child_relationship_sets_md_parent_cardinality() {
    let engine = Engine::new();
    let text = r#"erDiagram
PROJECT u--o{ TEAM_MEMBER : "parent"
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let rel = &res.model["relationships"][0];
    assert_eq!(rel["relSpec"]["cardB"], json!("MD_PARENT"));
    assert_eq!(rel["relSpec"]["cardA"], json!("ZERO_OR_MORE"));
}

#[test]
fn parse_diagram_er_allows_prototype_like_entity_names() {
    let engine = Engine::new();
    for name in ["__proto__", "constructor", "prototype"] {
        let text = format!("erDiagram\n{name} ||--|{{ ORDER : place\n");
        assert!(block_on(engine.parse_diagram(&text, ParseOptions::default())).is_ok());
    }
}

#[test]
fn parse_diagram_er_parses_relationship_cardinality_and_type() {
    let engine = Engine::new();
    let text = r#"erDiagram
CAR ||--o{ DRIVER : "insured for"
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["entities"].as_object().unwrap().len(), 2);
    let rels = res.model["relationships"].as_array().unwrap();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0]["roleA"], json!("insured for"));
    assert_eq!(rels[0]["relSpec"]["cardA"], json!("ZERO_OR_MORE"));
    assert_eq!(rels[0]["relSpec"]["cardB"], json!("ONLY_ONE"));
    assert_eq!(rels[0]["relSpec"]["relType"], json!("IDENTIFYING"));
}

#[test]
fn parse_diagram_er_supports_numeric_cardinality_shorthands() {
    let engine = Engine::new();
    let text = r#"erDiagram
A 1+--0+ B : has
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let rels = res.model["relationships"].as_array().unwrap();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0]["relSpec"]["cardA"], json!("ZERO_OR_MORE"));
    assert_eq!(rels[0]["relSpec"]["cardB"], json!("ONE_OR_MORE"));
}

#[test]
fn parse_diagram_er_acc_title_and_multiline_description() {
    let engine = Engine::new();
    let text = r#"erDiagram
accTitle: graph title
accDescr { this graph is
  about
  stuff
}
A ||--o{ B : has
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["accTitle"], json!("graph title"));
    assert_eq!(res.model["accDescr"], json!("this graph is\nabout\nstuff"));
}

#[test]
fn parse_diagram_er_multibyte_attribute_does_not_panic() {
    // Attribute block tokens that start with a multibyte UTF-8 character must
    // not be sliced on a fixed byte boundary while probing for PK/FK/UK keys.
    let engine = Engine::new();
    let text = "erDiagram\n顧客 {\n  文字列 名前\n}\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let attr = &res.model["entities"]["顧客"]["attributes"][0];
    assert_eq!(attr["type"], json!("文字列"));
    assert_eq!(attr["name"], json!("名前"));
}
