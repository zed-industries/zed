use std::{str::FromStr, sync::Arc};

use editor::CompletionProvider;
use fuzzy::{CharBag, StringMatchCandidate};
use gpui::{AppContext, Model, Task};
use language::{CodeLabel, Documentation, LanguageServerId};
use parking_lot::RwLock;
use task::{TaskVariables, VariableName};
use text::{Anchor, ToOffset};
use ui::ViewContext;

pub(crate) struct TaskVariablesCompletionProvider {
    task_variables: Arc<TaskVariables>,
    pub(crate) names: Arc<[StringMatchCandidate]>,
}

impl TaskVariablesCompletionProvider {
    pub(crate) fn new(variables: TaskVariables) -> Self {
        let names = variables
            .keys()
            .enumerate()
            .map(|(index, name)| {
                let name = name.to_string();
                StringMatchCandidate {
                    id: index,
                    char_bag: CharBag::from(name.as_str()),
                    string: name,
                }
            })
            .collect::<Arc<[_]>>();
        Self {
            names,
            task_variables: Arc::new(variables),
        }
    }
    fn current_query(
        buffer: &Model<language::Buffer>,
        position: language::Anchor,
        cx: &AppContext,
    ) -> Option<String> {
        let mut has_trigger_character = false;
        let reversed_query = buffer
            .read(cx)
            .reversed_chars_for_range(Anchor::MIN..position)
            .take_while(|c| {
                let is_trigger = *c == '$';
                if is_trigger {
                    has_trigger_character = true;
                }
                !is_trigger && (*c == '_' || c.is_ascii_alphanumeric())
            })
            .collect::<String>();

        has_trigger_character.then(|| reversed_query.chars().rev().collect())
    }
}

impl CompletionProvider for TaskVariablesCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<language::Buffer>,
        buffer_position: text::Anchor,
        cx: &mut ViewContext<editor::Editor>,
    ) -> gpui::Task<gpui::Result<Vec<project::Completion>>> {
        let Some(current_query) = Self::current_query(buffer, buffer_position, cx) else {
            return Task::ready(Ok(vec![]));
        };
        let buffer = buffer.read(cx);
        let buffer_snapshot = buffer.snapshot();
        let offset = buffer_position.to_offset(&buffer_snapshot);
        let starting_offset = offset - current_query.len();
        let starting_anchor = buffer.anchor_before(starting_offset);
        let executor = cx.background_executor().clone();
        let names = self.names.clone();
        let variables = self.task_variables.clone();
        cx.background_executor().spawn(async move {
            let matches = fuzzy::match_strings(
                &names,
                &current_query,
                true,
                100,
                &Default::default(),
                executor,
            )
            .await;
            // Find all variables starting with this
            Ok(matches
                .into_iter()
                .filter_map(|hit| {
                    let variable_key = VariableName::from_str(&hit.string).ok()?;
                    let value_of_var = variables.get(&variable_key)?.to_owned();
                    Some(project::Completion {
                        old_range: starting_anchor..buffer_position,
                        new_text: hit.string.clone(),
                        label: CodeLabel::plain(hit.string, None),
                        documentation: Some(Documentation::SingleLine(value_of_var)),
                        server_id: LanguageServerId(0), // TODO: Make this optional or something?
                        lsp_completion: Default::default(), // TODO: Make this optional or something?
                    })
                })
                .collect())
        })
    }

    fn resolve_completions(
        &self,
        _buffer: Model<language::Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Arc<RwLock<Box<[project::Completion]>>>,
        _cx: &mut ViewContext<editor::Editor>,
    ) -> gpui::Task<gpui::Result<bool>> {
        Task::ready(Ok(true))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Model<language::Buffer>,
        _completion: project::Completion,
        _push_to_history: bool,
        _cx: &mut ViewContext<editor::Editor>,
    ) -> gpui::Task<gpui::Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Model<language::Buffer>,
        position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        cx: &mut ViewContext<editor::Editor>,
    ) -> bool {
        if text == "$" {
            return true;
        }
        Self::current_query(buffer, position, cx).is_some()
    }
}
