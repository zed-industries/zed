use cloud_llm_client::predict_edits_v3::{self, Signature};
use edit_prediction::{DataCollectionState, Direction, EditPrediction, EditPredictionProvider};
use edit_prediction_context::{
    DeclarationId, EditPredictionContext, EditPredictionExcerptOptions, SyntaxIndex,
    SyntaxIndexState,
};
use gpui::{App, Entity, EntityId, Task, prelude::*};
use language::{Anchor, ToPoint};
use language::{BufferSnapshot, Point};
use std::collections::HashMap;
use std::{ops::Range, sync::Arc};

pub struct Zeta2EditPredictionProvider {
    current: Option<CurrentEditPrediction>,
    pending: Option<Task<()>>,
}

impl Zeta2EditPredictionProvider {
    pub fn new() -> Self {
        Self {
            current: None,
            pending: None,
        }
    }
}

#[derive(Clone)]
struct CurrentEditPrediction {
    buffer_id: EntityId,
    prediction: EditPrediction,
}

impl EditPredictionProvider for Zeta2EditPredictionProvider {
    fn name() -> &'static str {
        // TODO [zeta2]
        "zed-predict2"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions 2"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, _cx: &App) -> DataCollectionState {
        // TODO [zeta2]
        DataCollectionState::Unsupported
    }

    fn toggle_data_collection(&mut self, _cx: &mut App) {
        // TODO [zeta2]
    }

    fn usage(&self, _cx: &App) -> Option<client::EditPredictionUsage> {
        // TODO [zeta2]
        None
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self) -> bool {
        self.pending.is_some()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<project::Project>>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        // TODO [zeta2] check account
        // TODO [zeta2] actually request completion / interpolate

        let snapshot = buffer.read(cx).snapshot();
        let point = cursor_position.to_point(&snapshot);
        let end_anchor = snapshot.anchor_before(language::Point::new(
            point.row,
            snapshot.line_len(point.row),
        ));

        let edits: Arc<[(Range<Anchor>, String)]> =
            vec![(cursor_position..end_anchor, "👻".to_string())].into();
        let edits_preview_task = buffer.read(cx).preview_edits(edits.clone(), cx);

        // TODO [zeta2] throttle
        // TODO [zeta2] keep 2 requests
        self.pending = Some(cx.spawn(async move |this, cx| {
            let edits_preview = edits_preview_task.await;

            this.update(cx, |this, cx| {
                this.current = Some(CurrentEditPrediction {
                    buffer_id: buffer.entity_id(),
                    prediction: EditPrediction {
                        // TODO! [zeta2] request id?
                        id: None,
                        edits: edits.to_vec(),
                        edit_preview: Some(edits_preview),
                    },
                });
                this.pending.take();
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn cycle(
        &mut self,
        _buffer: Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        // TODO [zeta2] report accept
        self.current.take();
        self.pending.take();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.current.take();
        self.pending.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_prediction = self.current.take()?;

        if current_prediction.buffer_id != buffer.entity_id() {
            return None;
        }

        // TODO [zeta2] interpolate

        Some(current_prediction.prediction)
    }
}

pub fn make_cloud_request_in_background(
    cursor_point: Point,
    buffer: BufferSnapshot,
    events: Vec<predict_edits_v3::Event>,
    can_collect_data: bool,
    diagnostic_groups: Vec<predict_edits_v3::DiagnosticGroup>,
    git_info: Option<cloud_llm_client::PredictEditsGitInfo>,
    excerpt_options: EditPredictionExcerptOptions,
    syntax_index: Entity<SyntaxIndex>,
    cx: &mut App,
) -> Task<Option<predict_edits_v3::PredictEditsRequest>> {
    let index_state = syntax_index.read_with(cx, |index, _cx| index.state().clone());
    cx.background_spawn(async move {
        let index_state = index_state.lock().await;
        EditPredictionContext::gather_context(cursor_point, &buffer, &excerpt_options, &index_state)
            .map(|context| {
                make_cloud_request(
                    context,
                    events,
                    can_collect_data,
                    diagnostic_groups,
                    git_info,
                    &index_state,
                )
            })
    })
}

pub fn make_cloud_request(
    context: EditPredictionContext,
    events: Vec<predict_edits_v3::Event>,
    can_collect_data: bool,
    diagnostic_groups: Vec<predict_edits_v3::DiagnosticGroup>,
    git_info: Option<cloud_llm_client::PredictEditsGitInfo>,
    index_state: &SyntaxIndexState,
) -> predict_edits_v3::PredictEditsRequest {
    let mut signatures = Vec::new();
    let mut declaration_to_signature_index = HashMap::default();
    let mut referenced_declarations = Vec::new();
    for snippet in context.snippets {
        let parent_index = snippet.declaration.parent().and_then(|parent| {
            add_signature(
                parent,
                &mut declaration_to_signature_index,
                &mut signatures,
                index_state,
            )
        });
        let (text, text_is_truncated) = snippet.declaration.item_text();
        referenced_declarations.push(predict_edits_v3::ReferencedDeclaration {
            text: text.into(),
            text_is_truncated,
            signature_range: snippet.declaration.signature_range_in_item_text(),
            parent_index,
            score_components: snippet.score_components,
            signature_score: snippet.scores.signature,
            declaration_score: snippet.scores.declaration,
        });
    }

    let excerpt_parent = context
        .excerpt
        .parent_declarations
        .last()
        .and_then(|(parent, _)| {
            add_signature(
                *parent,
                &mut declaration_to_signature_index,
                &mut signatures,
                index_state,
            )
        });

    predict_edits_v3::PredictEditsRequest {
        excerpt: context.excerpt_text.body,
        referenced_declarations,
        signatures,
        excerpt_parent,
        // todo!
        events,
        can_collect_data,
        diagnostic_groups,
        git_info,
    }
}

fn add_signature(
    declaration_id: DeclarationId,
    declaration_to_signature_index: &mut HashMap<DeclarationId, usize>,
    signatures: &mut Vec<Signature>,
    index: &SyntaxIndexState,
) -> Option<usize> {
    if let Some(signature_index) = declaration_to_signature_index.get(&declaration_id) {
        return Some(*signature_index);
    }
    let Some(parent_declaration) = index.declaration(declaration_id) else {
        log::error!("bug: missing parent declaration");
        return None;
    };
    let parent_index = parent_declaration.parent().and_then(|parent| {
        add_signature(parent, declaration_to_signature_index, signatures, index)
    });
    let (text, text_is_truncated) = parent_declaration.signature_text();
    let signature_index = signatures.len();
    signatures.push(Signature {
        text: text.into(),
        text_is_truncated,
        parent_index,
    });
    declaration_to_signature_index.insert(declaration_id, signature_index);
    Some(signature_index)
}
