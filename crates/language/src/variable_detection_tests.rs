use super::*;

#[test]
fn test_variable_lsp_token_types_default() {
    let types = Language::default_variable_lsp_token_types();

    assert!(types.contains(&"variable".to_string()));
    assert!(types.contains(&"parameter".to_string()));
    assert!(types.contains(&"const".to_string()));
    assert_eq!(types.len(), 3);
}

#[test]
fn test_variable_parent_kinds_default() {
    let kinds = Language::default_variable_parent_kinds();

    assert!(kinds.contains(&"let_declaration".to_string()));
    assert!(kinds.contains(&"assignment_expression".to_string()));
    assert!(kinds.contains(&"binary_expression".to_string()));
    assert!(kinds.len() > 0);
}

#[test]
fn test_highlights_config_is_variable_lsp_token_type() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![],
        variable_parent_kinds: vec![],
        variable_lsp_token_types: vec![
            "variable".to_string(),
            "parameter".to_string(),
            "const".to_string(),
        ],
    };

    assert!(highlights_config.is_variable_lsp_token_type("variable"));
    assert!(highlights_config.is_variable_lsp_token_type("parameter"));
    assert!(highlights_config.is_variable_lsp_token_type("const"));
    assert!(!highlights_config.is_variable_lsp_token_type("function"));
    assert!(!highlights_config.is_variable_lsp_token_type("class"));
}

#[test]
fn test_highlights_config_is_variable_parent_kind() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![],
        variable_parent_kinds: vec![
            "let_declaration".to_string(),
            "assignment_expression".to_string(),
        ],
        variable_lsp_token_types: vec![],
    };

    assert!(highlights_config.is_variable_parent_kind("let_declaration"));
    assert!(highlights_config.is_variable_parent_kind("assignment_expression"));
    assert!(!highlights_config.is_variable_parent_kind("function_declaration"));
}

#[test]
fn test_highlights_config_is_variable_capture() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![1, 2, 5],
        variable_parent_kinds: vec![],
        variable_lsp_token_types: vec![],
    };

    assert!(highlights_config.is_variable_capture(1));
    assert!(highlights_config.is_variable_capture(2));
    assert!(highlights_config.is_variable_capture(5));
    assert!(!highlights_config.is_variable_capture(0));
    assert!(!highlights_config.is_variable_capture(3));
}

#[test]
fn test_empty_lsp_token_types() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![],
        variable_parent_kinds: vec![],
        variable_lsp_token_types: vec![],
    };

    assert!(!highlights_config.is_variable_lsp_token_type("variable"));
    assert!(!highlights_config.is_variable_lsp_token_type("parameter"));
}

#[test]
fn test_duplicate_lsp_token_types() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![],
        variable_parent_kinds: vec![],
        variable_lsp_token_types: vec![
            "variable".to_string(),
            "variable".to_string(),
            "parameter".to_string(),
        ],
    };

    assert!(highlights_config.is_variable_lsp_token_type("variable"));
}

#[test]
fn test_case_sensitive_lsp_token_types() {
    let highlights_config = HighlightsConfig {
        query: Query::new(&tree_sitter_rust::LANGUAGE.into(), "").unwrap(),
        identifier_capture_indices: vec![],
        variable_capture_indices: vec![],
        variable_parent_kinds: vec![],
        variable_lsp_token_types: vec!["Variable".to_string()],
    };

    assert!(highlights_config.is_variable_lsp_token_type("Variable"));
    assert!(!highlights_config.is_variable_lsp_token_type("variable"));
}
