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

#[test]
fn test_settings_reference_placeholder_is_expanded() {
    let mut book = Book::new();
    book.push_item(BookItem::Chapter(Chapter::new(
        "All Settings",
        String::from("# All Settings\n\n{#SETTINGS_REFERENCE#}\n"),
        "reference/all-settings.md",
        Vec::new(),
    )));
    template_settings_reference(&mut book);
    let BookItem::Chapter(chapter) = &book.sections[0] else {
        panic!("expected a chapter");
    };
    assert_eq!(chapter.content.contains("{#SETTINGS_REFERENCE#}"), false);
    assert_eq!(chapter.content.contains("- Setting: `hard_tabs`"), true);
}

#[test]
fn test_settings_reference_contains_sections_for_known_settings() {
    let reference = settings_reference::generate_settings_reference();
    assert_eq!(reference.contains("## Hard Tabs {#hard-tabs}"), true);
    assert_eq!(reference.contains("- Setting: `hard_tabs`"), true);
    assert_eq!(
        reference.contains("## Network Proxy {#network-proxy}"),
        true
    );
    assert_eq!(reference.contains("- Setting: `proxy`"), true);
    assert_eq!(
        reference.contains("### Border Size {#active-pane-modifiers-border-size}"),
        true
    );
    assert_eq!(reference.contains("- `\"contained\"`:"), true);
}

#[test]
fn test_settings_reference_keeps_externally_linked_anchors() {
    let externally_linked_anchors = [
        "auto-indent",
        "auto-install-extensions",
        "calls",
        "colorize-brackets",
        "edit-predictions",
        "enable-language-server",
        "ensure-final-newline-on-save",
        "file-scan-depth",
        "file-scan-exclusions",
        "file-types",
        "format-on-save",
        "formatter",
        "git-worktree-directory",
        "hard-tabs",
        "lsp",
        "modeline-lines",
        "network-proxy",
        "preferred-line-length",
        "remove-trailing-whitespace-on-save",
        "show-completion-documentation",
        "show-completions-on-input",
        "show-whitespaces",
        "soft-wrap",
        "tab-size",
        "terminal",
        "terminal-detect",
    ];
    let reference = settings_reference::generate_settings_reference();
    let mut missing = Vec::new();
    for anchor in externally_linked_anchors {
        if !reference.contains(&format!("{{#{anchor}}}")) {
            missing.push(anchor);
        }
    }
    assert_eq!(missing, Vec::<&str>::new());
}
