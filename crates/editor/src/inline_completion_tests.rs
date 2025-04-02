use gpui::{Entity, prelude::*};
use indoc::indoc;
use inline_completion::EditPredictionProvider;
use multi_buffer::{Anchor, MultiBufferSnapshot, ToPoint};
use project::Project;
use std::ops::Range;
use text::{Point, ToOffset};

use crate::{
    InlineCompletion, editor_tests::init_test, test::editor_test_context::EditorTestContext,
};

#[gpui::test]
async fn test_inline_completion_insert(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let absolute_zero_celsius = ˇ;");

    propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_str(), "-273.15");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
}

#[gpui::test]
async fn test_inline_completion_modification(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let pi = ˇ\"foo\";");

    propose_edits(&provider, vec![(9..14, "3.14159")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_str(), "3.14159");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let pi = 3.14159ˇ;")
}

#[gpui::test]
async fn test_inline_completion_jump_button(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);

    // Cursor is 2+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line
    "});

    propose_edits(
        &provider,
        vec![(Point::new(4, 3)..Point::new(4, 3), " 4")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), Point::new(4, 3));
    });

    // When accepting, cursor is moved to the proposed location
    accept_completion(&mut cx);
    cx.assert_editor_state(indoc! {"
        line 0
        line 1
        line 2
        line 3
        linˇe
    "});

    // Cursor is 2+ lines below the proposed edit
    cx.set_state(indoc! {"
        line 0
        line
        line 2
        line 3
        line ˇ4
    "});

    propose_edits(
        &provider,
        vec![(Point::new(1, 3)..Point::new(1, 3), " 1")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), Point::new(1, 3));
    });

    // When accepting, cursor is moved to the proposed location
    accept_completion(&mut cx);
    cx.assert_editor_state(indoc! {"
        line 0
        linˇe
        line 2
        line 3
        line 4
    "});
}

#[gpui::test]
async fn test_inline_completion_invalidation_range(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);

    // Cursor is 3+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line 4
        line
    "});
    let edit_location = Point::new(5, 3);

    propose_edits(
        &provider,
        vec![(edit_location..edit_location, " 5")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *towards* the completion, it stays active
    cx.set_selections_state(indoc! {"
        line 0
        line 1
        line ˇ2
        line 3
        line 4
        line
    "});
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *away* from the completion, it is discarded
    cx.set_selections_state(indoc! {"
        line ˇ0
        line 1
        line 2
        line 3
        line 4
        line
    "});
    cx.editor(|editor, _, _| {
        assert!(editor.active_inline_completion.is_none());
    });

    // Cursor is 3+ lines below the proposed edit
    cx.set_state(indoc! {"
        line
        line 1
        line 2
        line 3
        line ˇ4
        line 5
    "});
    let edit_location = Point::new(0, 3);

    propose_edits(
        &provider,
        vec![(edit_location..edit_location, " 0")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_inline_completion(window, cx));
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *towards* the completion, it stays active
    cx.set_selections_state(indoc! {"
        line
        line 1
        line 2
        line ˇ3
        line 4
        line 5
    "});
    assert_editor_active_move_completion(&mut cx, |snapshot, move_target| {
        assert_eq!(move_target.to_point(&snapshot), edit_location);
    });

    // If we move *away* from the completion, it is discarded
    cx.set_selections_state(indoc! {"
        line
        line 1
        line 2
        line 3
        line 4
        line ˇ5
    "});
    cx.editor(|editor, _, _| {
        assert!(editor.active_inline_completion.is_none());
    });
}

fn assert_editor_active_edit_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, &Vec<(Range<Anchor>, String)>),
) {
    cx.editor(|editor, _, cx| {
        let completion_state = editor
            .active_inline_completion
            .as_ref()
            .expect("editor has no active completion");

        if let InlineCompletion::Edit { edits, .. } = &completion_state.completion {
            assert(editor.buffer().read(cx).snapshot(cx), edits);
        } else {
            panic!("expected edit completion");
        }
    })
}

fn assert_editor_active_move_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, Anchor),
) {
    cx.editor(|editor, _, cx| {
        let completion_state = editor
            .active_inline_completion
            .as_ref()
            .expect("editor has no active completion");

        if let InlineCompletion::Move { target, .. } = &completion_state.completion {
            assert(editor.buffer().read(cx).snapshot(cx), *target);
        } else {
            panic!("expected move completion");
        }
    })
}

fn accept_completion(cx: &mut EditorTestContext) {
    cx.update_editor(|editor, window, cx| {
        editor.accept_edit_prediction(&crate::AcceptEditPrediction, window, cx)
    })
}

fn propose_edits<T: ToOffset>(
    provider: &Entity<FakeInlineCompletionProvider>,
    edits: Vec<(Range<T>, &str)>,
    cx: &mut EditorTestContext,
) {
    let snapshot = cx.buffer_snapshot();
    let edits = edits.into_iter().map(|(range, text)| {
        let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
        (range, text.into())
    });

    cx.update(|_, cx| {
        provider.update(cx, |provider, _| {
            provider.set_inline_completion(Some(inline_completion::InlineCompletion {
                id: None,
                edits: edits.collect(),
                edit_preview: None,
            }))
        })
    });
}

fn assign_editor_completion_provider(
    provider: Entity<FakeInlineCompletionProvider>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(Some(provider), window, cx);
    })
}

#[derive(Default, Clone)]
struct FakeInlineCompletionProvider {
    completion: Option<inline_completion::InlineCompletion>,
}

impl FakeInlineCompletionProvider {
    pub fn set_inline_completion(
        &mut self,
        completion: Option<inline_completion::InlineCompletion>,
    ) {
        self.completion = completion;
    }
}

impl EditPredictionProvider for FakeInlineCompletionProvider {
    fn name() -> &'static str {
        "fake-completion-provider"
    }

    fn display_name() -> &'static str {
        "Fake Completion Provider"
    }

    fn show_completions_in_menu() -> bool {
        false
    }

    fn is_enabled(
        &self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &gpui::App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self) -> bool {
        false
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        _buffer: gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _debounce: bool,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn cycle(
        &mut self,
        _buffer: gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _direction: inline_completion::Direction,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut gpui::Context<Self>) {}

    fn discard(&mut self, _cx: &mut gpui::Context<Self>) {}

    fn suggest<'a>(
        &mut self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut gpui::Context<Self>,
    ) -> Option<inline_completion::InlineCompletion> {
        self.completion.clone()
    }
}
