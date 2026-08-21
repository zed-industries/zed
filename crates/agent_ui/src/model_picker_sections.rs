use std::sync::Arc;

use collections::HashSet;
use fs::Fs;
use gpui::{App, SharedString};
use settings::update_settings_file;

/// A model picker entry viewed only through its section structure, so that
/// both model pickers can share cursor-restoration policy without sharing
/// entry payloads.
pub trait SectionedEntry {
    /// The title when this entry is a section header of any kind, collapsible
    /// or not.
    fn section_title(&self) -> Option<&SharedString>;

    /// The title when this entry is a collapsible group header. Plain
    /// separators return `None`.
    fn group_header_title(&self) -> Option<&SharedString>;
}

/// The title of the section header (separator or group header) preceding `ix`.
/// Favorited models appear under "Favorite" (and possibly "Recommended") as
/// well as under their provider group; the section disambiguates which copy
/// the cursor is on.
fn section_title_of<E: SectionedEntry>(entries: &[E], ix: usize) -> Option<&SharedString> {
    entries
        .iter()
        .take(ix + 1)
        .rev()
        .find_map(SectionedEntry::section_title)
}

fn position_of_section<E: SectionedEntry>(entries: &[E], section: &SharedString) -> Option<usize> {
    entries
        .iter()
        .position(|entry| entry.section_title() == Some(section))
}

/// What the cursor was resting on before the entry list was rebuilt.
///
/// `Key` is the model identity used to find the entry again. The language
/// model picker needs a composite (provider id, model id) key because the same
/// model id can be offered by more than one provider; the ACP picker's model
/// ids are globally unique.
pub enum PreviousSelection<Key> {
    Header(SharedString),
    Model {
        key: Key,
        section: Option<SharedString>,
    },
}

impl<Key> PreviousSelection<Key> {
    /// Captures the entry at `ix`. `model_key` returns the identity of a
    /// model entry and `None` for anything else; separators are never
    /// captured because the cursor cannot rest on one.
    pub fn capture<E: SectionedEntry>(
        entries: &[E],
        ix: usize,
        model_key: impl FnOnce(&E) -> Option<Key>,
    ) -> Option<Self> {
        let entry = entries.get(ix)?;
        if let Some(title) = entry.group_header_title() {
            return Some(Self::Header(title.clone()));
        }
        Some(Self::Model {
            key: model_key(entry)?,
            section: section_title_of(entries, ix).cloned(),
        })
    }

    /// Restore order: the same model in the same section, the same model in
    /// another section (it moved, e.g. was unfavorited), then the section's
    /// own header (the model is gone but its section remains, e.g. the group
    /// collapsed under the cursor). Returns `None` when nothing survives so
    /// the caller can apply its own final fallback, which differs per picker.
    pub fn restore<E: SectionedEntry>(
        &self,
        entries: &[E],
        is_model: impl Fn(&E, &Key) -> bool,
    ) -> Option<usize> {
        match self {
            Self::Header(section) => position_of_section(entries, section),
            Self::Model { key, section } => entries
                .iter()
                .enumerate()
                .find_map(|(ix, entry)| {
                    (is_model(entry, key) && section_title_of(entries, ix) == section.as_ref())
                        .then_some(ix)
                })
                .or_else(|| entries.iter().position(|entry| is_model(entry, key)))
                .or_else(|| {
                    section
                        .as_ref()
                        .and_then(|section| position_of_section(entries, section))
                }),
        }
    }
}

/// Flips `group` in the local set before persisting, because the pickers'
/// settings observers compare the incoming settings against that local set to
/// avoid refreshing for their own writes.
pub fn toggle_group_collapsed(
    collapsed_groups: &mut HashSet<SharedString>,
    group: SharedString,
    fs: Arc<dyn Fs>,
    cx: &App,
) {
    let collapsed = !collapsed_groups.contains(&group);
    if collapsed {
        collapsed_groups.insert(group.clone());
    } else {
        collapsed_groups.remove(&group);
    }

    update_settings_file(fs, cx, move |settings, _cx| {
        settings
            .agent
            .get_or_insert_default()
            .set_model_group_collapsed(&group, collapsed);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestEntry {
        Separator(SharedString),
        GroupHeader(SharedString),
        Model {
            provider: &'static str,
            id: &'static str,
        },
    }

    impl SectionedEntry for TestEntry {
        fn section_title(&self) -> Option<&SharedString> {
            match self {
                TestEntry::Separator(title) | TestEntry::GroupHeader(title) => Some(title),
                TestEntry::Model { .. } => None,
            }
        }

        fn group_header_title(&self) -> Option<&SharedString> {
            match self {
                TestEntry::GroupHeader(title) => Some(title),
                TestEntry::Separator(_) | TestEntry::Model { .. } => None,
            }
        }
    }

    fn separator(title: &'static str) -> TestEntry {
        TestEntry::Separator(title.into())
    }

    fn header(title: &'static str) -> TestEntry {
        TestEntry::GroupHeader(title.into())
    }

    fn model(provider: &'static str, id: &'static str) -> TestEntry {
        TestEntry::Model { provider, id }
    }

    type TestKey = (&'static str, &'static str);

    fn capture(entries: &[TestEntry], ix: usize) -> Option<PreviousSelection<TestKey>> {
        PreviousSelection::capture(entries, ix, |entry| match entry {
            TestEntry::Model { provider, id } => Some((*provider, *id)),
            _ => None,
        })
    }

    fn restore(previous: &PreviousSelection<TestKey>, entries: &[TestEntry]) -> Option<usize> {
        previous.restore(entries, |entry, (provider, id)| {
            matches!(entry, TestEntry::Model { provider: p, id: i } if p == provider && i == id)
        })
    }

    #[test]
    fn capture_ignores_separators() {
        let entries = [
            separator("Favorite"),
            model("zed", "claude"),
            separator("zed"),
        ];
        assert!(capture(&entries, 0).is_none());
        assert!(capture(&entries, 2).is_none());
        // Index 3 is out of bounds.
        assert!(capture(&entries, 3).is_none());
    }

    #[test]
    fn restores_the_same_model_in_the_same_section() {
        // The favorited model appears both under "Favorite" and under its
        // provider group; the section anchor keeps the cursor on the group
        // copy instead of the first id match.
        let entries = [
            separator("Favorite"),
            model("zed", "claude"),
            header("zed"),
            model("zed", "claude"),
            header("openai"),
            model("openai", "gpt"),
        ];
        let previous = capture(&entries, 3).unwrap();
        assert_eq!(restore(&previous, &entries), Some(3));
    }

    #[test]
    fn follows_a_model_that_moved_to_another_section() {
        let before = [
            separator("Favorite"),
            model("zed", "claude"),
            header("zed"),
            model("zed", "claude"),
        ];
        let previous = capture(&before, 1).unwrap();

        // The model was unfavorited: the Favorite section is gone and only
        // the group copy remains.
        let after = [header("zed"), model("zed", "claude")];
        assert_eq!(restore(&previous, &after), Some(1));
    }

    #[test]
    fn prefers_the_model_anywhere_over_its_old_section_header() {
        let before = [
            separator("Favorite"),
            model("zed", "claude"),
            header("zed"),
            model("zed", "claude"),
        ];
        let previous = capture(&before, 1).unwrap();

        // The model left the Favorite section but the section itself remains:
        // the cursor follows the model rather than staying on the section.
        let after = [
            separator("Favorite"),
            model("zed", "gemini"),
            header("zed"),
            model("zed", "claude"),
        ];
        assert_eq!(restore(&previous, &after), Some(3));
    }

    #[test]
    fn falls_back_to_the_section_header_when_the_model_is_gone() {
        // Two providers offer the same model id, so the composite key must
        // not treat the other provider's copy as a match.
        let before = [
            header("zed"),
            model("zed", "gpt-5"),
            header("openai"),
            model("openai", "gpt-5"),
        ];
        let previous = capture(&before, 3).unwrap();

        // The openai group collapsed under the cursor: its model is gone but
        // its header remains, and zed's same-id model must not win.
        let after = [header("zed"), model("zed", "gpt-5"), header("openai")];
        assert_eq!(restore(&previous, &after), Some(2));
    }

    #[test]
    fn falls_back_to_a_separator_section() {
        // The section anchor can be a plain separator, not just a collapsible
        // group header.
        let before = [
            separator("Favorite"),
            model("zed", "claude"),
            header("zed"),
        ];
        let previous = capture(&before, 1).unwrap();

        let after = [separator("Favorite"), header("zed")];
        assert_eq!(restore(&previous, &after), Some(0));
    }

    #[test]
    fn returns_none_when_model_and_section_are_gone() {
        let before = [header("zed"), model("zed", "claude")];
        let previous = capture(&before, 1).unwrap();

        let after = [header("openai"), model("openai", "gpt")];
        assert_eq!(restore(&previous, &after), None);
    }

    #[test]
    fn header_restores_to_its_section() {
        let before = [
            header("zed"),
            model("zed", "claude"),
            header("openai"),
            model("openai", "gpt"),
        ];
        let previous = capture(&before, 2).unwrap();

        let after = [header("openai"), header("zed")];
        assert_eq!(restore(&previous, &after), Some(0));

        let gone = [header("zed"), model("zed", "claude")];
        assert_eq!(restore(&previous, &gone), None);
    }
}
