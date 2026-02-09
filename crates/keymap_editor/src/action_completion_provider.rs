use collections::HashMap;
use command_palette;
use editor::{CompletionProvider, Editor};
use fuzzy::StringMatchCandidate;
use gpui::{Context, Entity, SharedString, Window};
use language::{self, ToOffset};
use project::{self, CompletionDisplayOptions};

pub struct ActionCompletionProvider {
    action_names: Vec<&'static str>,
    humanized_names: HashMap<&'static str, SharedString>,
}

impl ActionCompletionProvider {
    pub fn new(
        action_names: Vec<&'static str>,
        humanized_names: HashMap<&'static str, SharedString>,
    ) -> Self {
        Self {
            action_names,
            humanized_names,
        }
    }
}

impl CompletionProvider for ActionCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: editor::ExcerptId,
        buffer: &Entity<language::Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> gpui::Task<anyhow::Result<Vec<project::CompletionResponse>>> {
        let buffer = buffer.read(cx);
        let mut count_back = 0;

        for char in buffer.reversed_chars_at(buffer_position) {
            if char.is_ascii_alphanumeric() || char == '_' || char == ':' {
                count_back += 1;
            } else {
                break;
            }
        }

        let start_anchor = buffer.anchor_before(
            buffer_position
                .to_offset(&buffer)
                .saturating_sub(count_back),
        );

        let replace_range = start_anchor..buffer_position;
        let snapshot = buffer.text_snapshot();
        let query: String = snapshot.text_for_range(replace_range.clone()).collect();
        let normalized_query = command_palette::normalize_action_query(&query);

        let candidates: Vec<StringMatchCandidate> = self
            .action_names
            .iter()
            .enumerate()
            .map(|(ix, &name)| {
                let humanized = self
                    .humanized_names
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| name.into());
                StringMatchCandidate::new(ix, &humanized)
            })
            .collect();

        let executor = cx.background_executor().clone();
        let executor_for_fuzzy = executor.clone();
        let action_names = self.action_names.clone();
        let humanized_names = self.humanized_names.clone();

        executor.spawn(async move {
            let matches = fuzzy::match_strings(
                &candidates,
                &normalized_query,
                true,
                true,
                action_names.len(),
                &Default::default(),
                executor_for_fuzzy,
            )
            .await;

            let completions: Vec<project::Completion> = matches
                .iter()
                .take(50)
                .map(|m| {
                    let action_name = action_names[m.candidate_id];
                    let humanized = humanized_names
                        .get(action_name)
                        .cloned()
                        .unwrap_or_else(|| action_name.into());

                    project::Completion {
                        replace_range: replace_range.clone(),
                        label: language::CodeLabel::plain(humanized.to_string(), None),
                        new_text: action_name.to_string(),
                        documentation: None,
                        source: project::CompletionSource::Custom,
                        icon_path: None,
                        match_start: None,
                        snippet_deduplication_key: None,
                        insert_text_mode: None,
                        confirm: None,
                    }
                })
                .collect();

            Ok(vec![project::CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions {
                    dynamic_width: true,
                },
                is_incomplete: false,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<language::Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text.chars().last().is_some_and(|last_char| {
            last_char.is_ascii_alphanumeric() || last_char == '_' || last_char == ':'
        })
    }
}
