//! Fades top-level keys in the user `settings.json` whose value matches the
//! built-in default, the same way unused code is faded, so redundant settings
//! are easy to spot and remove.

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
        if !is_user_settings_buffer(&buffer, cx) {
            return;
        }

        // The user settings buffer is owned by this editor, so detaching is safe:
        // the subscription is torn down when the editor (and its buffer) drops.
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

fn is_user_settings_buffer(buffer: &Entity<Buffer>, cx: &App) -> bool {
    buffer
        .read(cx)
        .file()
        .and_then(|file| file.as_local())
        .is_some_and(|local| local.abs_path(cx).as_path() == paths::settings_file().as_path())
}

fn default_settings() -> &'static serde_json::Map<String, serde_json::Value> {
    static DEFAULTS: OnceLock<serde_json::Map<String, serde_json::Value>> = OnceLock::new();
    DEFAULTS.get_or_init(|| {
        settings_json::parse_json_with_comments::<serde_json::Value>(&settings::default_settings())
            .ok()
            .and_then(|value| match value {
                serde_json::Value::Object(map) => Some(map),
                _ => None,
            })
            .unwrap_or_default()
    })
}

fn refresh(editor: &mut Editor, cx: &mut Context<Editor>) {
    let text = editor.text(cx);

    // ponytail: top-level keys only. A nested key set to its default inside an
    // otherwise-customized block won't fade. Recurse per-path if that matters.
    let user = match settings_json::parse_json_with_comments::<serde_json::Value>(&text) {
        Ok(serde_json::Value::Object(map)) => map,
        // Mid-edit / invalid JSON: drop the highlights rather than show stale ones.
        _ => {
            editor.clear_highlights(HighlightKey::RedundantSetting, cx);
            return;
        }
    };

    let defaults = default_settings();
    let redundant_ranges: Vec<_> = settings_json::top_level_pairs(&text)
        .into_iter()
        .filter(|(key, _)| user.get(key) == defaults.get(key))
        .map(|(_, range)| range)
        .collect();

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
