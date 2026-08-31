use super::*;
use serde_json::json;

#[test]
fn test_find_binding_prefers_exact_match_over_parameterized() {
    let keymap: KeymapFile = serde_json::from_value(json!([
        {
            "bindings": {
                "ctrl-tab": "agents_sidebar::ToggleThreadSwitcher",
                "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }]
            }
        }
    ]))
    .unwrap();

    let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
    assert_eq!(binding.as_deref(), Some("ctrl-tab"));
}

#[test]
fn test_find_binding_falls_back_to_parameterized_match() {
    let keymap: KeymapFile = serde_json::from_value(json!([
        {
            "bindings": {
                "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }]
            }
        }
    ]))
    .unwrap();

    let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
    assert_eq!(binding.as_deref(), Some("ctrl-shift-tab"));
}

#[test]
fn test_find_binding_prefers_exact_match_regardless_of_order() {
    let keymap: KeymapFile = serde_json::from_value(json!([
        {
            "bindings": {
                "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }],
                "ctrl-tab": "agents_sidebar::ToggleThreadSwitcher"
            }
        }
    ]))
    .unwrap();

    let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
    assert_eq!(binding.as_deref(), Some("ctrl-tab"));
}

#[test]
fn test_find_binding_later_section_overrides_earlier() {
    let keymap: KeymapFile = serde_json::from_value(json!([
        { "bindings": { "ctrl-a": "some::Action" } },
        { "bindings": { "ctrl-b": "some::Action" } }
    ]))
    .unwrap();

    let binding = find_binding_in_keymap(&keymap, "some::Action");
    assert_eq!(binding.as_deref(), Some("ctrl-b"));
}
