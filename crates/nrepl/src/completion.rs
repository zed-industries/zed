//! Completions from the live REPL (cider-nrepl's `complete` op), merged
//! with whatever the editor already had (clojure-lsp, buffer words).
//!
//! [`NreplCompletionProvider`] wraps the editor's existing provider: it
//! always forwards to it, and when the workspace is connected to an nREPL
//! server it appends the REPL's candidates. REPL candidates know about
//! everything loaded at runtime — vars created by `def` at the REPL,
//! Java classes and members, keywords — which static analysis can't see.

use std::{cell::RefCell, ops::Range, rc::Rc};

use anyhow::Result;
use editor::{CompletionProvider, Editor};
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity, Window};
use language::{Buffer, CodeLabel, ToOffset as _};
use project::{
    Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource,
    lsp_store::CompletionDocumentation,
};
use workspace::Workspace;

use crate::form_at_cursor::parse_namespace;
use crate::nrepl_store::connected_client;
use crate::ops::{self, CompletionCandidate};

/// Resolves the namespace completions are requested in.
pub type NamespaceFn = Rc<dyn Fn(&Entity<Buffer>, &App) -> String>;

pub struct NreplCompletionProvider {
    inner: Option<Rc<dyn CompletionProvider>>,
    workspace: WeakEntity<Workspace>,
    namespace: NamespaceFn,
    /// When false, REPL completions are only offered in Clojure buffers.
    any_language: bool,
}

impl NreplCompletionProvider {
    /// Wraps `inner` for a regular Clojure source editor. The namespace is
    /// taken from the buffer's `(ns ...)` form.
    pub fn for_source_editor(
        inner: Option<Rc<dyn CompletionProvider>>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        Self {
            inner,
            workspace,
            namespace: Rc::new(|buffer, cx| {
                parse_namespace(&buffer.read(cx).snapshot()).unwrap_or_else(|| "user".into())
            }),
            any_language: false,
        }
    }

    /// Provider for an input with no other completion source (the REPL
    /// console), completing in whatever namespace `namespace` reports.
    pub fn standalone(workspace: WeakEntity<Workspace>, namespace: NamespaceFn) -> Self {
        Self {
            inner: None,
            workspace,
            namespace,
            any_language: true,
        }
    }

    fn applies_to(&self, buffer: &Entity<Buffer>, cx: &App) -> bool {
        self.any_language || is_clojure_buffer(buffer, cx)
    }
}

pub fn is_clojure_buffer(buffer: &Entity<Buffer>, cx: &App) -> bool {
    buffer.read(cx).language().is_some_and(|language| {
        let name = language.name();
        let name = name.as_ref();
        name.eq_ignore_ascii_case("clojure")
            || name.eq_ignore_ascii_case("clojurescript")
            || name.eq_ignore_ascii_case("edn")
    })
}

/// Characters that can appear in a Clojure symbol or keyword.
pub fn is_symbol_char(ch: char) -> bool {
    ch.is_alphanumeric() || "-_*+!?<>=/.:$%&'#".contains(ch)
}

/// Returns the byte offset where the symbol ending at `offset` starts.
fn symbol_start(buffer: &Buffer, offset: usize) -> usize {
    let mut start = offset;
    for ch in buffer.reversed_chars_at(offset) {
        if !is_symbol_char(ch) {
            break;
        }
        start -= ch.len_utf8();
    }
    start
}

impl CompletionProvider for NreplCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        trigger: editor::CompletionContext,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let inner_task = self
            .inner
            .as_ref()
            .map(|inner| inner.completions(buffer, buffer_position, trigger, window, cx));

        let repl_task = self.repl_completions(buffer, buffer_position, cx);

        cx.background_spawn(async move {
            let mut responses = match inner_task {
                Some(task) => task.await?,
                None => Vec::new(),
            };
            if let Some(repl_task) = repl_task {
                // A REPL that's busy, gone, or lacking cider shouldn't break
                // the LSP completions the user is also waiting for.
                match repl_task.await {
                    Ok(response) => responses.push(response),
                    Err(err) => log::debug!("nrepl: completion failed: {err:#}"),
                }
            }
            Ok(responses)
        })
    }

    fn resolve_completions(
        &self,
        buffer: Entity<Buffer>,
        completion_indices: Vec<usize>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        let Some(inner) = &self.inner else {
            return Task::ready(Ok(false));
        };
        let completion_indices = completion_indices
            .into_iter()
            .filter(|index| {
                !matches!(
                    completions.borrow().get(*index).map(|c| &c.source),
                    Some(CompletionSource::Custom)
                )
            })
            .collect::<Vec<_>>();
        if completion_indices.is_empty() {
            return Task::ready(Ok(false));
        }
        inner.resolve_completions(buffer, completion_indices, completions, cx)
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Entity<Buffer>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
        push_to_history: bool,
        all_commit_ranges: Vec<Range<language::Anchor>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        let is_ours = matches!(
            completions
                .borrow()
                .get(completion_index)
                .map(|c| &c.source),
            Some(CompletionSource::Custom)
        );
        match &self.inner {
            Some(inner) if !is_ours => inner.apply_additional_edits_for_completion(
                buffer,
                completions,
                completion_index,
                push_to_history,
                all_commit_ranges,
                cx,
            ),
            _ => Task::ready(Ok(None)),
        }
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        if let Some(inner) = &self.inner
            && inner.is_completion_trigger(buffer, position, text, trigger_in_words, cx)
        {
            return true;
        }
        let mut chars = text.chars();
        let is_single_symbol_char =
            chars.next().is_some_and(is_symbol_char) && chars.next().is_none();
        is_single_symbol_char && self.applies_to(buffer, cx) && self.is_connected(cx)
    }

    fn sort_completions(&self) -> bool {
        self.inner
            .as_ref()
            .is_none_or(|inner| inner.sort_completions())
    }

    fn filter_completions(&self) -> bool {
        self.inner
            .as_ref()
            .is_none_or(|inner| inner.filter_completions())
    }

    fn show_snippets(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|inner| inner.show_snippets())
    }
}

impl NreplCompletionProvider {
    fn is_connected(&self, cx: &App) -> bool {
        self.workspace
            .upgrade()
            .is_some_and(|workspace| connected_client(workspace.entity_id(), cx).is_ok())
    }

    fn repl_completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut Context<Editor>,
    ) -> Option<Task<Result<CompletionResponse>>> {
        if !self.applies_to(buffer, cx) {
            return None;
        }
        let workspace = self.workspace.upgrade()?;
        let (client, session) = connected_client(workspace.entity_id(), cx).ok()?;

        let buffer_ref = buffer.read(cx);
        let snapshot = buffer_ref.text_snapshot();
        let end = buffer_position.to_offset(&snapshot);
        let start = symbol_start(buffer_ref, end);
        if start == end {
            return None;
        }
        let prefix: String = snapshot.text_for_range(start..end).collect();
        let replace_range = snapshot.anchor_before(start)..buffer_position;
        let namespace = (self.namespace)(buffer, cx);

        Some(cx.background_spawn(async move {
            let candidates = ops::complete(&client, &session, &namespace, &prefix).await?;
            let completions = candidates
                .into_iter()
                .map(|candidate| to_completion(candidate, replace_range.clone()))
                .collect();
            Ok(CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: true,
            })
        }))
    }
}

fn to_completion(
    candidate: CompletionCandidate,
    replace_range: Range<language::Anchor>,
) -> Completion {
    let detail = match (&candidate.kind, &candidate.ns) {
        (Some(kind), Some(ns)) => Some(format!("{kind} · {ns}")),
        (Some(kind), None) => Some(kind.clone()),
        (None, Some(ns)) => Some(ns.clone()),
        (None, None) => None,
    };
    let documentation = match (detail, candidate.doc) {
        (Some(detail), Some(doc)) => Some(format!("{detail}\n\n{doc}")),
        (detail, doc) => detail.or(doc),
    }
    .map(|text| CompletionDocumentation::MultiLineMarkdown(text.into()));

    let label = CodeLabel::plain(candidate.candidate.clone(), None);

    Completion {
        replace_range,
        new_text: candidate.candidate,
        label,
        documentation,
        source: CompletionSource::Custom,
        icon_path: None,
        icon_color: None,
        match_start: None,
        snippet_deduplication_key: None,
        insert_text_mode: None,
        confirm: None,
        group: None,
    }
}

#[cfg(test)]
mod tests {
    use super::is_symbol_char;

    #[test]
    fn symbol_chars() {
        for ch in ['a', '-', '*', '?', '/', '.', ':', '>'] {
            assert!(is_symbol_char(ch), "{ch}");
        }
        for ch in [' ', '(', ')', '[', '"', '{', ',', '@'] {
            assert!(!is_symbol_char(ch), "{ch}");
        }
    }
}
