use gpui::{App, BorrowAppContext, Global};
use settings::KeymapLabels;

#[derive(Default)]
struct GlobalKeymapLabels(KeymapLabels);

impl Global for GlobalKeymapLabels {}

pub fn register(cx: &mut App) {
    if !cx.has_global::<GlobalKeymapLabels>() {
        cx.set_global(GlobalKeymapLabels::default());
    }
}

pub fn set_labels(cx: &mut App, labels: KeymapLabels) {
    if cx.has_global::<GlobalKeymapLabels>() {
        cx.update_global(|global: &mut GlobalKeymapLabels, _| global.0 = labels);
    } else {
        cx.set_global(GlobalKeymapLabels(labels));
    }
}

pub fn labels(cx: &mut App) -> KeymapLabels {
    cx.try_global::<GlobalKeymapLabels>()
        .map(|global| global.0.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        DummyKeyboardMapper, KeyBindingContextPredicate, KeyContext, KeybindingKeystroke, Keystroke,
    };
    use ui::SharedString;

    fn parse_keystrokes(keystrokes: &str) -> Vec<KeybindingKeystroke> {
        keystrokes
            .split_whitespace()
            .map(|source| {
                KeybindingKeystroke::new_with_mapper(
                    Keystroke::parse(source).unwrap(),
                    false,
                    &DummyKeyboardMapper,
                )
            })
            .collect()
    }

    fn make_label(
        keystrokes: &str,
        label: impl Into<SharedString>,
        context: Option<&str>,
    ) -> settings::KeymapLabel {
        settings::KeymapLabel {
            keystrokes: parse_keystrokes(keystrokes).into(),
            label: label.into(),
            context_predicate: context
                .map(|ctx| KeyBindingContextPredicate::parse(ctx).unwrap().into()),
            meta: None,
        }
    }

    #[test]
    fn resolves_binding_label_with_precedence() {
        let mut default_labels = KeymapLabels::default();
        default_labels.binding_labels.push(make_label(
            "ctrl-w d",
            "Go to definition (split)",
            Some("VimControl"),
        ));

        let mut user_labels = KeymapLabels::default();
        user_labels.binding_labels.push(make_label(
            "ctrl-w d",
            "Definition in split",
            Some("VimControl"),
        ));

        let mut merged = KeymapLabels::default();
        merged.merge(default_labels);
        merged.merge(user_labels);

        let context_stack = vec![KeyContext::parse("VimControl").unwrap()];
        let keystrokes = parse_keystrokes("ctrl-w d");

        assert_eq!(
            merged
                .resolve_binding_label(&keystrokes, &context_stack)
                .as_ref()
                .map(|s| s.as_ref()),
            Some("Definition in split")
        );
    }

    #[test]
    fn resolves_group_label_for_prefix() {
        let mut labels = KeymapLabels::default();
        labels
            .group_labels
            .push(make_label("ctrl-w", "Window & panes", Some("VimControl")));

        let context_stack = vec![KeyContext::parse("VimControl").unwrap()];
        let keystrokes = parse_keystrokes("ctrl-w");

        assert_eq!(
            labels
                .resolve_group_label(&keystrokes, &context_stack)
                .as_ref()
                .map(|s| s.as_ref()),
            Some("Window & panes")
        );
    }
}
