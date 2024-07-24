//! REPL operations on an [`Editor`].

use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context, Result};
use editor::{Anchor, Editor, RangeToAnchorExt};
use gpui::{prelude::*, AppContext, View, WeakView, WindowContext};
use language::{Language, Point};
use multi_buffer::MultiBufferRow;

use crate::repl_store::ReplStore;
use crate::session::SessionEvent;
use crate::{KernelSpecification, Session};

pub fn run(editor: WeakView<Editor>, cx: &mut WindowContext) -> Result<()> {
    let store = ReplStore::global(cx);
    if !store.read(cx).is_enabled() {
        return Ok(());
    }

    let Some((selected_text, language, anchor_range)) = snippet(editor.clone(), cx) else {
        return Ok(());
    };

    let entity_id = editor.entity_id();

    let kernel_specification = store.update(cx, |store, cx| {
        store
            .kernelspec(&language, cx)
            .with_context(|| format!("No kernel found for language: {}", language.name()))
    })?;

    let fs = store.read(cx).fs().clone();
    let session = if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
        session
    } else {
        let session = cx.new_view(|cx| Session::new(editor.clone(), fs, kernel_specification, cx));

        editor.update(cx, |_editor, cx| {
            cx.notify();

            cx.subscribe(&session, {
                let store = store.clone();
                move |_this, _session, event, cx| match event {
                    SessionEvent::Shutdown(shutdown_event) => {
                        store.update(cx, |store, _cx| {
                            store.remove_session(shutdown_event.entity_id());
                        });
                    }
                }
            })
            .detach();
        })?;

        store.update(cx, |store, _cx| {
            store.insert_session(entity_id, session.clone());
        });

        session
    };

    session.update(cx, |session, cx| {
        session.execute(&selected_text, anchor_range, cx);
    });

    anyhow::Ok(())
}

pub enum SessionSupport {
    ActiveSession(View<Session>),
    Inactive(Box<KernelSpecification>),
    RequiresSetup(Arc<str>),
    Unsupported,
}

pub fn session(editor: WeakView<Editor>, cx: &mut AppContext) -> SessionSupport {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();

    if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
        return SessionSupport::ActiveSession(session);
    };

    let Some(language) = get_language(editor, cx) else {
        return SessionSupport::Unsupported;
    };
    let kernelspec = store.update(cx, |store, cx| store.kernelspec(&language, cx));

    match kernelspec {
        Some(kernelspec) => SessionSupport::Inactive(Box::new(kernelspec)),
        None => match language.name().as_ref() {
            "TypeScript" | "Python" => SessionSupport::RequiresSetup(language.name()),
            _ => SessionSupport::Unsupported,
        },
    }
}

pub fn clear_outputs(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };
    session.update(cx, |session, cx| {
        session.clear_outputs(cx);
        cx.notify();
    });
}

pub fn interrupt(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };

    session.update(cx, |session, cx| {
        session.interrupt(cx);
        cx.notify();
    });
}

pub fn shutdown(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };

    session.update(cx, |session, cx| {
        session.shutdown(cx);
        cx.notify();
    });
}

fn snippet(
    editor: WeakView<Editor>,
    cx: &mut WindowContext,
) -> Option<(String, Arc<Language>, Range<Anchor>)> {
    let selection = editor
        .update(cx, |editor, cx| editor.selections.newest_adjusted(cx))
        .ok()?;

    let editor = editor.upgrade()?;
    let editor = editor.read(cx);

    let buffer = editor.buffer().read(cx).snapshot(cx);
    let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);

    let range = if selection.is_empty() {
        Point::new(selection.start.row, 0)
            ..Point::new(
                selection.start.row,
                multi_buffer_snapshot.line_len(MultiBufferRow(selection.start.row)),
            )
    } else {
        let mut range = selection.range();
        if range.end.column == 0 {
            range.end.row -= 1;
            range.end.column = multi_buffer_snapshot.line_len(MultiBufferRow(range.end.row));
        }
        range
    };

    let anchor_range = range.to_anchors(&multi_buffer_snapshot);

    let selected_text = buffer
        .text_for_range(anchor_range.clone())
        .collect::<String>();

    let start_language = buffer.language_at(anchor_range.start)?;
    let end_language = buffer.language_at(anchor_range.end)?;
    if start_language != end_language {
        return None;
    }

    Some((selected_text, start_language.clone(), anchor_range))
}

fn get_language(editor: WeakView<Editor>, cx: &mut AppContext) -> Option<Arc<Language>> {
    let editor = editor.upgrade()?;
    let selection = editor.read(cx).selections.newest::<usize>(cx);
    let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
    buffer.language_at(selection.head()).cloned()
}
