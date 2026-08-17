use std::error::Error;

use partial_json_fixer::fix_json;

fn fix_json_to_string(partial:&str) -> Result<String, Box<dyn Error>> {
    let json = fix_json(partial);

    Ok(json.to_string())
}

#[test]
fn it_works() {
    let partial = "{\"key\": \"value";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"value\"}");
}

#[test]
fn test_unclosed_object_with_multiple_pairs() {
    let partial = "{\"key1\": \"value1\", \"key2\": 123";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key1\": \"value1\", \"key2\": 123}");
}

#[test]
fn test_unclosed_array_with_mixed_types() {
    let partial = "[1, \"text\", {\"key\": 42}";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "[1, \"text\", {\"key\": 42}]");
}

#[test]
fn test_null() {
    let partial = "{\"key\":null}";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\":null}");
}

#[test]
fn test_incomplete_object() {
    let partial = "{\"key\":";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": null}");
}

#[test]
fn test_incomplete_object_string_start() {
    let partial = "{\"key\":\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\":\"\"}");
}

#[test]
fn test_incomplete_object_trailing_comma() {
    let partial = "{\"key\":\"\",";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\":\"\"}");
}

#[test]
fn test_incomplete_nested_array() {
    let partial = "[[1, 2], [3, 4]";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "[[1, 2], [3, 4]]");
}

#[test]
fn test_incomplete_nested_object() {
    let partial = "{\"outer\": {\"inner\": \"value\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"outer\": {\"inner\": \"value\"}}");
}

#[test]
fn test_trailing_comma_object() {
    let partial = "{\"key\": \"value\",";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"value\"}");
}

#[test]
fn test_trailing_comma_array() {
    let partial = "[1, 2, 3,";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_single_unclosed_quote() {
    let partial = "{\"key\": \"value";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"value\"}");
}

#[test]
fn test_escaped_characters() {
    let partial = "{\"key\": \"A string with \\\"escaped quotes";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"A string with \\\"escaped quotes\"}");
}

#[test]
fn test_deeply_nested_structure() {
    let partial = "{\"level1\": {\"level2\": {\"level3\": {\"key\": \"value\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(
        result,
        "{\"level1\": {\"level2\": {\"level3\": {\"key\": \"value\"}}}}"
    );
}

#[test]
fn test_empty_object() {
    let partial = "{";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{}");
}

#[test]
fn test_empty_array() {
    let partial = "[";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "[]");
}

#[test]
fn test_mixed_structure() {
    let partial = "{\"array\": [1, 2, 3], \"object\": {\"key\": \"value\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(
        result,
        "{\"array\": [1, 2, 3], \"object\": {\"key\": \"value\"}}"
    );
}

#[test]
#[should_panic]
fn test_missing_colon() {
    let partial = "{\"key\" \"value\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"value\"}");
}

#[test]
#[should_panic]
fn test_missing_comma() {
    let partial = "{\"key1\": \"value1\" \"key2\": \"value2\"}";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key1\": \"value1\", \"key2\": \"value2\"}");
}

#[test]
fn test_extra_whitespace() {
    let partial = "{ \"key\"  :    \"value\"   ,   \"array\"  :   [  1 , 2 , 3  ]";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{ \"key\"  :    \"value\"   ,   \"array\"  :   [  1 , 2 , 3  ]}");
}

#[test]
fn test_utf8_characters() {
    let partial = "{\"key\": \"value with emoji \u{1f643}";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": \"value with emoji \u{1f643}\"}");
}

#[test]
fn incomplete_keyword() {
    let partial = "{\"key\": mu";
    let result = fix_json_to_string(partial).unwrap();
    // INTENTIONALLY assert_ne : unsupported case since we only support valid JSON - todo: remove
    // this test
    assert_ne!(result, "{\"key\": null}");
}

#[test]
fn boolean() {
    let partial = "{\"key\": true";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": true}");
}

#[test]
fn multiple_keys() {
    let partial = "{\"key\": true, \"hey\"";
    let result = fix_json_to_string(partial).unwrap();
    assert_eq!(result, "{\"key\": true, \"hey\": null}");
}
