use std::sync::OnceLock;

use editor::{Editor, HighlightKey, MultiBufferOffset};
use gpui::{App, Context, Entity, HighlightStyle};
use language::Buffer;
use settings::Settings as _;
use theme_settings::ThemeSettings;

pub fn init(cx: &mut App) {
    cx.observe_new(|editor: &mut Editor, _window, cx| {
        let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
            return;
        };
        if !is_settings_buffer(&buffer, cx) {
            return;
        }

        cx.subscribe(editor.buffer(), |editor, _buffer, event, cx| {
            if matches!(
                event,
                multi_buffer::Event::Edited { .. } | multi_buffer::Event::Reloaded
            ) {
                refresh(editor, cx);
            }
        })
        .detach();
        refresh(editor, cx);
    })
    .detach();
}

fn is_settings_buffer(buffer: &Entity<Buffer>, cx: &App) -> bool {
    let Some(local) = buffer.read(cx).file().and_then(|file| file.as_local()) else {
        return false;
    };
    let path = local.abs_path(cx);
    path.as_path() == paths::settings_file().as_path()
        || path.ends_with(paths::local_settings_file_relative_path().as_std_path())
}

fn default_settings() -> &'static serde_json::Value {
    static DEFAULTS: OnceLock<serde_json::Value> = OnceLock::new();
    DEFAULTS.get_or_init(|| {
        settings_json::parse_json_with_comments::<serde_json::Value>(&settings::default_settings())
            .unwrap_or(serde_json::Value::Null)
    })
}

fn value_at<'a>(value: &'a serde_json::Value, path: &[String]) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for key in path {
        current = current.as_object()?.get(key)?;
    }
    Some(current)
}

fn is_redundant(user: &serde_json::Value, default: &serde_json::Value) -> bool {
    match (user, default) {
        (serde_json::Value::Object(user), serde_json::Value::Object(default)) => user
            .iter()
            .all(|(key, value)| default.get(key).is_some_and(|d| is_redundant(value, d))),
        _ => user == default,
    }
}

fn refresh(editor: &mut Editor, cx: &mut Context<Editor>) {
    let text = editor.text(cx);

    let user = match settings_json::parse_json_with_comments::<serde_json::Value>(&text) {
        Ok(value) if value.is_object() => value,
        // Invalid mid-edit JSON: clear rather than leave stale highlights.
        _ => {
            editor.clear_highlights(HighlightKey::RedundantSetting, cx);
            return;
        }
    };

    let defaults = default_settings();
    let mut redundant_ranges: Vec<std::ops::Range<usize>> = Vec::new();
    for (path, range) in settings_json::all_pairs(&text) {
        // Skip keys already inside a faded parent block.
        if redundant_ranges
            .iter()
            .any(|covered| covered.start <= range.start && range.end <= covered.end)
        {
            continue;
        }
        if let (Some(user_value), Some(default_value)) =
            (value_at(&user, &path), value_at(defaults, &path))
            && is_redundant(user_value, default_value)
        {
            redundant_ranges.push(range);
        }
    }

    if redundant_ranges.is_empty() {
        editor.clear_highlights(HighlightKey::RedundantSetting, cx);
        return;
    }

    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let anchor_ranges = redundant_ranges
        .into_iter()
        .map(|range| {
            snapshot.anchor_before(MultiBufferOffset(range.start))
                ..snapshot.anchor_after(MultiBufferOffset(range.end))
        })
        .collect();

    let fade = ThemeSettings::get_global(cx).unnecessary_code_fade;
    editor.highlight_text(
        HighlightKey::RedundantSetting,
        anchor_ranges,
        HighlightStyle {
            fade_out: Some(fade),
            ..Default::default()
        },
        cx,
    );
}

#[cfg(test)]
mod tests {
    use super::is_redundant;
    use serde_json::json;

    #[test]
    fn redundancy_matches_subset_of_default() {
        assert!(is_redundant(&json!(false), &json!(false)));
        assert!(!is_redundant(&json!(true), &json!(false)));

        let default = json!({
            "show": "auto",
            "selected_symbol": true,
            "axes": { "horizontal": true, "vertical": true },
        });

        // Subset of the default (default has extra keys) is still redundant.
        assert!(is_redundant(
            &json!({ "selected_symbol": true, "show": "auto" }),
            &default
        ));
        assert!(is_redundant(&json!({ "axes": { "vertical": true } }), &default));

        assert!(!is_redundant(
            &json!({ "selected_symbol": false, "show": "auto" }),
            &default
        ));
        assert!(!is_redundant(&json!({ "nonexistent": 1 }), &default));
    }
}
