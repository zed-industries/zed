use std::ops::Range;

use collections::HashMap;
use editor::display_map::CreaseId;
use editor::{Addon, AnchorRangeExt, Editor};
use gpui::{Entity, Subscription};
use ui::prelude::*;

use crate::{
    context::{AgentContextHandle, AgentContextKey},
    context_picker::crease_for_mention,
    context_store::{ContextStore, ContextStoreEvent},
};

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub icon_path: SharedString,
    pub label: SharedString,
    /// None for a deserialized message, Some otherwise.
    pub context: Option<AgentContextHandle>,
}

#[derive(Default)]
pub struct ContextCreasesAddon {
    creases: HashMap<AgentContextKey, Vec<(CreaseId, SharedString)>>,
    _subscription: Option<Subscription>,
}

impl Addon for ContextCreasesAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

impl ContextCreasesAddon {
    pub fn new() -> Self {
        Self {
            creases: HashMap::default(),
            _subscription: None,
        }
    }

    pub fn add_creases(
        &mut self,
        context_store: &Entity<ContextStore>,
        key: AgentContextKey,
        creases: impl IntoIterator<Item = (CreaseId, SharedString)>,
        cx: &mut Context<Editor>,
    ) {
        self.creases.entry(key).or_default().extend(creases);
        self._subscription = Some(
            cx.subscribe(context_store, |editor, _, event, cx| match event {
                ContextStoreEvent::ContextRemoved(key) => {
                    let Some(this) = editor.addon_mut::<Self>() else {
                        return;
                    };
                    let (crease_ids, replacement_texts): (Vec<_>, Vec<_>) = this
                        .creases
                        .remove(key)
                        .unwrap_or_default()
                        .into_iter()
                        .unzip();
                    let ranges = editor
                        .remove_creases(crease_ids, cx)
                        .into_iter()
                        .map(|(_, range)| range)
                        .collect::<Vec<_>>();
                    editor.unfold_ranges(&ranges, false, false, cx);
                    editor.edit(ranges.into_iter().zip(replacement_texts), cx);
                    cx.notify();
                }
            }),
        )
    }

    pub fn into_inner(self) -> HashMap<AgentContextKey, Vec<(CreaseId, SharedString)>> {
        self.creases
    }
}

pub fn extract_message_creases(
    editor: &mut Editor,
    cx: &mut Context<'_, Editor>,
) -> Vec<MessageCrease> {
    let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
    let mut contexts_by_crease_id = editor
        .addon_mut::<ContextCreasesAddon>()
        .map(std::mem::take)
        .unwrap_or_default()
        .into_inner()
        .into_iter()
        .flat_map(|(key, creases)| {
            let context = key.0;
            creases
                .into_iter()
                .map(move |(id, _)| (id, context.clone()))
        })
        .collect::<HashMap<_, _>>();
    // Filter the addon's list of creases based on what the editor reports,
    // since the addon might have removed creases in it.

    editor.display_map.update(cx, |display_map, cx| {
        display_map
            .snapshot(cx)
            .crease_snapshot
            .creases()
            .filter_map(|(id, crease)| {
                Some((
                    id,
                    (
                        crease.range().to_offset(&buffer_snapshot),
                        crease.metadata()?.clone(),
                    ),
                ))
            })
            .map(|(id, (range, metadata))| {
                let context = contexts_by_crease_id.remove(&id);
                MessageCrease {
                    range,
                    context,
                    label: metadata.label,
                    icon_path: metadata.icon_path,
                }
            })
            .collect()
    })
}

pub fn insert_message_creases(
    editor: &mut Editor,
    message_creases: &[MessageCrease],
    context_store: &Entity<ContextStore>,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) {
    let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
    let creases = message_creases
        .iter()
        .map(|crease| {
            let start = buffer_snapshot.anchor_after(crease.range.start);
            let end = buffer_snapshot.anchor_before(crease.range.end);
            crease_for_mention(
                crease.label.clone(),
                crease.icon_path.clone(),
                start..end,
                cx.weak_entity(),
            )
        })
        .collect::<Vec<_>>();
    let ids = editor.insert_creases(creases.clone(), cx);
    editor.fold_creases(creases, false, window, cx);
    if let Some(addon) = editor.addon_mut::<ContextCreasesAddon>() {
        for (crease, id) in message_creases.iter().zip(ids) {
            if let Some(context) = crease.context.as_ref() {
                let key = AgentContextKey(context.clone());
                addon.add_creases(context_store, key, vec![(id, crease.label.clone())], cx);
            }
        }
    }
}
