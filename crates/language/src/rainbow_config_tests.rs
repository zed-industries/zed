use super::*;

#[test]
fn test_is_valid_identifier_basic() {
    assert!(is_valid_identifier("hello"));
    assert!(is_valid_identifier("my_variable"));
    assert!(is_valid_identifier("_private"));
    assert!(is_valid_identifier("snake_case_name"));
    assert!(is_valid_identifier("CamelCase"));
    assert!(is_valid_identifier("SCREAMING_SNAKE"));
    assert!(is_valid_identifier("var123"));
}

#[test]
fn test_is_valid_identifier_invalid() {
    assert!(!is_valid_identifier(""));
    assert!(!is_valid_identifier(" "));
    assert!(!is_valid_identifier("a"));
    assert!(!is_valid_identifier("_"));
    assert!(!is_valid_identifier("__"));
    assert!(!is_valid_identifier("123abc"));
    assert!(!is_valid_identifier("hello world"));
    assert!(!is_valid_identifier("hello\nworld"));
    assert!(!is_valid_identifier("hello\tworld"));
}

#[test]
fn test_is_valid_identifier_edge_cases() {
    assert!(is_valid_identifier("a1"));
    assert!(is_valid_identifier("_a"));
    assert!(is_valid_identifier("a_"));
    assert!(is_valid_identifier("a_b_c_d_e"));
    assert!(!is_valid_identifier("a-b"));
    assert!(!is_valid_identifier("a.b"));
    assert!(!is_valid_identifier("a::b"));
}

#[test]
fn test_rust_keywords_excluded() {
    let config = RustRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("self"));
    assert!(config.excluded_identifiers().contains("super"));
    assert!(config.excluded_identifiers().contains("crate"));
    assert!(config.excluded_identifiers().contains("let"));
    assert!(config.excluded_identifiers().contains("mut"));
    assert!(config.excluded_identifiers().contains("fn"));
    assert!(config.excluded_identifiers().contains("impl"));
    assert!(config.excluded_identifiers().contains("trait"));
}

#[test]
fn test_rust_should_highlight() {
    let config = RustRainbowConfig::default();
    
    assert!(config.should_highlight("my_variable"));
    assert!(config.should_highlight("user_name"));
    assert!(!config.should_highlight("self"));
    assert!(!config.should_highlight("let"));
    assert!(!config.should_highlight("fn"));
    assert!(!config.should_highlight("_"));
    assert!(!config.should_highlight("a"));
}

#[test]
fn test_python_keywords_excluded() {
    let config = PythonRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("def"));
    assert!(config.excluded_identifiers().contains("class"));
    assert!(config.excluded_identifiers().contains("import"));
    assert!(config.excluded_identifiers().contains("print"));
    assert!(config.excluded_identifiers().contains("len"));
    assert!(config.excluded_identifiers().contains("str"));
    assert!(config.excluded_identifiers().contains("true"));
    assert!(config.excluded_identifiers().contains("false"));
    assert!(config.excluded_identifiers().contains("none"));
}

#[test]
fn test_python_should_highlight() {
    let config = PythonRainbowConfig::default();
    
    assert!(config.should_highlight("my_var"));
    assert!(config.should_highlight("user_data"));
    assert!(!config.should_highlight("def"));
    assert!(!config.should_highlight("class"));
    assert!(!config.should_highlight("print"));
    assert!(!config.should_highlight("len"));
}

#[test]
fn test_typescript_keywords() {
    let config = TypeScriptRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("type"));
    assert!(config.excluded_identifiers().contains("interface"));
    assert!(config.excluded_identifiers().contains("namespace"));
    assert!(config.excluded_identifiers().contains("async"));
    assert!(config.excluded_identifiers().contains("await"));
    assert!(config.excluded_identifiers().contains("Promise"));
}

#[test]
fn test_go_keywords() {
    let config = GoRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("func"));
    assert!(config.excluded_identifiers().contains("chan"));
    assert!(config.excluded_identifiers().contains("defer"));
    assert!(config.excluded_identifiers().contains("go"));
    assert!(config.excluded_identifiers().contains("nil"));
    assert!(config.excluded_identifiers().contains("make"));
    assert!(config.excluded_identifiers().contains("append"));
}

#[test]
fn test_cpp_keywords() {
    let config = CppRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("const"));
    assert!(config.excluded_identifiers().contains("constexpr"));
    assert!(config.excluded_identifiers().contains("namespace"));
    assert!(config.excluded_identifiers().contains("template"));
    assert!(config.excluded_identifiers().contains("nullptr"));
    assert!(config.excluded_identifiers().contains("std"));
}

#[test]
fn test_java_keywords() {
    let config = JavaRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("public"));
    assert!(config.excluded_identifiers().contains("private"));
    assert!(config.excluded_identifiers().contains("protected"));
    assert!(config.excluded_identifiers().contains("static"));
    assert!(config.excluded_identifiers().contains("synchronized"));
    assert!(config.excluded_identifiers().contains("String"));
    assert!(config.excluded_identifiers().contains("System"));
}

#[test]
fn test_all_configs_have_common_keywords() {
    let configs: Vec<Box<dyn RainbowConfig>> = vec![
        Box::new(RustRainbowConfig::default()),
        Box::new(PythonRainbowConfig::default()),
        Box::new(TypeScriptRainbowConfig::default()),
        Box::new(JavaScriptRainbowConfig::default()),
        Box::new(GoRainbowConfig::default()),
        Box::new(CppRainbowConfig::default()),
        Box::new(JavaRainbowConfig::default()),
    ];
    
    for config in configs {
        assert!(config.excluded_identifiers().contains("true") || 
                config.excluded_identifiers().contains("false"),
                "Config should exclude boolean literals");
    }
}

#[test]
fn test_identifier_with_numbers() {
    assert!(is_valid_identifier("var123"));
    assert!(is_valid_identifier("my_var_2"));
    assert!(is_valid_identifier("x1y2z3"));
    assert!(!is_valid_identifier("123var"));
    assert!(!is_valid_identifier("1"));
}

#[test]
fn test_underscore_only_invalid() {
    assert!(!is_valid_identifier("_"));
    assert!(!is_valid_identifier("__"));
    assert!(!is_valid_identifier("___"));
    assert!(!is_valid_identifier("____"));
}

#[test]
fn test_whitespace_invalid() {
    assert!(!is_valid_identifier("hello world"));
    assert!(!is_valid_identifier(" hello"));
    assert!(!is_valid_identifier("hello "));
    assert!(!is_valid_identifier("hello\nworld"));
    assert!(!is_valid_identifier("hello\tworld"));
    assert!(!is_valid_identifier("hello\rworld"));
}

#[test]
fn test_control_characters_invalid() {
    assert!(!is_valid_identifier("hello\x00world"));
    assert!(!is_valid_identifier("\x01hello"));
    assert!(!is_valid_identifier("hello\x1F"));
}

#[test]
fn test_default_config_common_keywords() {
    let config = DefaultRainbowConfig::default();
    
    assert!(config.excluded_identifiers().contains("self"));
    assert!(config.excluded_identifiers().contains("super"));
    assert!(config.excluded_identifiers().contains("true"));
    assert!(config.excluded_identifiers().contains("false"));
    assert!(config.excluded_identifiers().contains("null"));
    assert!(config.excluded_identifiers().contains("undefined"));
}

#[test]
fn test_should_highlight_respects_both_validation_and_keywords() {
    let config = PythonRainbowConfig::default();
    
    assert!(!config.should_highlight("_"));
    assert!(!config.should_highlight("a"));
    assert!(!config.should_highlight("def"));
    
    assert!(config.should_highlight("my_var"));
    assert!(config.should_highlight("user_name"));
}
