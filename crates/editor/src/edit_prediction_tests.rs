use edit_prediction::EditPredictionProvider;
use gpui::{Entity, KeyBinding, Modifiers, prelude::*};
use indoc::indoc;
use multi_buffer::{Anchor, MultiBufferSnapshot, ToPoint};
use std::{ops::Range, sync::Arc};
use text::{Point, ToOffset};

use crate::{
    AcceptEditPrediction, EditPrediction, MenuEditPredictionsPolicy, editor_tests::init_test,
    test::editor_test_context::EditorTestContext,
};

#[gpui::test]
async fn test_edit_prediction_insert(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let absolute_zero_celsius = ˇ;");

    propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "-273.15");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
}

#[gpui::test]
async fn test_edit_prediction_modification(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let pi = ˇ\"foo\";");

    propose_edits(&provider, vec![(9..14, "3.14159")], &mut cx);
    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    assert_editor_active_edit_completion(&mut cx, |_, edits| {
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].1.as_ref(), "3.14159");
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let pi = 3.14159ˇ;")
}

#[gpui::test]
async fn test_edit_prediction_jump_button(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionProvider::default());
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

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
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

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
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
async fn test_edit_prediction_invalidation_range(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionProvider::default());
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

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
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
        assert!(editor.active_edit_prediction.is_none());
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

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));
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
        assert!(editor.active_edit_prediction.is_none());
    });
}

#[gpui::test]
async fn test_edit_prediction_jump_disabled_for_non_zed_providers(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeNonZedEditPredictionProvider::default());
    assign_editor_completion_provider_non_zed(provider.clone(), &mut cx);

    // Cursor is 2+ lines above the proposed edit
    cx.set_state(indoc! {"
        line 0
        line ˇ1
        line 2
        line 3
        line
    "});

    propose_edits_non_zed(
        &provider,
        vec![(Point::new(4, 3)..Point::new(4, 3), " 4")],
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| editor.update_visible_edit_prediction(window, cx));

    // For non-Zed providers, there should be no move completion (jump functionality disabled)
    cx.editor(|editor, _, _| {
        if let Some(completion_state) = &editor.active_edit_prediction {
            // Should be an Edit prediction, not a Move prediction
            match &completion_state.completion {
                EditPrediction::Edit { .. } => {
                    // This is expected for non-Zed providers
                }
                EditPrediction::MoveWithin { .. } | EditPrediction::MoveOutside { .. } => {
                    panic!(
                        "Non-Zed providers should not show Move predictions (jump functionality)"
                    );
                }
            }
        }
    });
}

#[gpui::test]
async fn test_edit_prediction_preview_cleanup_on_toggle_off(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    // Bind `ctrl-shift-a` to accept the provided edit prediction. The actual key
    // binding here doesn't matter, we simply need to confirm that holding the
    // binding's modifiers triggers the edit prediction preview.
    cx.update(|cx| cx.bind_keys([KeyBinding::new("ctrl-shift-a", AcceptEditPrediction, None)]));

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new(|_| FakeEditPredictionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let x = ˇ;");

    propose_edits(&provider, vec![(8..8, "42")], &mut cx);
    cx.update_editor(|editor, window, cx| {
        editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::ByProvider);
        editor.update_visible_edit_prediction(window, cx)
    });

    cx.editor(|editor, _, _| {
        assert!(editor.has_active_edit_prediction());
    });

    // Simulate pressing the modifiers for `AcceptEditPrediction`, namely
    // `ctrl-shift`, so that we can confirm that the edit prediction preview is
    // activated.
    let modifiers = Modifiers::control_shift();
    cx.simulate_modifiers_change(modifiers);
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(editor.edit_prediction_preview_is_active());
    });

    // Disable showing edit predictions without issuing a new modifiers changed
    // event, to confirm that the edit prediction preview is still active.
    cx.update_editor(|editor, window, cx| {
        editor.set_show_edit_predictions(Some(false), window, cx);
    });

    cx.editor(|editor, _, _| {
        assert!(!editor.has_active_edit_prediction());
        assert!(editor.edit_prediction_preview_is_active());
    });

    // Now release the modifiers
    // Simulate releasing all modifiers, ensuring that even with edit prediction
    // disabled, the edit prediction preview is cleaned up.
    cx.simulate_modifiers_change(Modifiers::none());
    cx.run_until_parked();

    cx.editor(|editor, _, _| {
        assert!(!editor.edit_prediction_preview_is_active());
    });
}

fn assert_editor_active_edit_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, &Vec<(Range<Anchor>, Arc<str>)>),
) {
    cx.editor(|editor, _, cx| {
        let completion_state = editor
            .active_edit_prediction
            .as_ref()
            .expect("editor has no active completion");

        if let EditPrediction::Edit { edits, .. } = &completion_state.completion {
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
            .active_edit_prediction
            .as_ref()
            .expect("editor has no active completion");

        if let EditPrediction::MoveWithin { target, .. } = &completion_state.completion {
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
    provider: &Entity<FakeEditPredictionProvider>,
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
            provider.set_edit_prediction(Some(edit_prediction::EditPrediction::Local {
                id: None,
                edits: edits.collect(),
                edit_preview: None,
            }))
        })
    });
}

fn assign_editor_completion_provider(
    provider: Entity<FakeEditPredictionProvider>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(Some(provider), window, cx);
    })
}

fn propose_edits_non_zed<T: ToOffset>(
    provider: &Entity<FakeNonZedEditPredictionProvider>,
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
            provider.set_edit_prediction(Some(edit_prediction::EditPrediction::Local {
                id: None,
                edits: edits.collect(),
                edit_preview: None,
            }))
        })
    });
}

fn assign_editor_completion_provider_non_zed(
    provider: Entity<FakeNonZedEditPredictionProvider>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, window, cx| {
        editor.set_edit_prediction_provider(Some(provider), window, cx);
    })
}

#[derive(Default, Clone)]
pub struct FakeEditPredictionProvider {
    pub completion: Option<edit_prediction::EditPrediction>,
}

impl FakeEditPredictionProvider {
    pub fn set_edit_prediction(&mut self, completion: Option<edit_prediction::EditPrediction>) {
        self.completion = completion;
    }
}

impl EditPredictionProvider for FakeEditPredictionProvider {
    fn name() -> &'static str {
        "fake-completion-provider"
    }

    fn display_name() -> &'static str {
        "Fake Completion Provider"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn supports_jump_to_edit() -> bool {
        true
    }

    fn is_enabled(
        &self,
        _buffer: &gpui::Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &gpui::App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &gpui::App) -> bool {
        false
    }

    fn refresh(
        &mut self,
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
        _direction: edit_prediction::Direction,
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
    ) -> Option<edit_prediction::EditPrediction> {
        self.completion.clone()
    }
}

#[derive(Default, Clone)]
pub struct FakeNonZedEditPredictionProvider {
    pub completion: Option<edit_prediction::EditPrediction>,
}

impl FakeNonZedEditPredictionProvider {
    pub fn set_edit_prediction(&mut self, completion: Option<edit_prediction::EditPrediction>) {
        self.completion = completion;
    }
}

impl EditPredictionProvider for FakeNonZedEditPredictionProvider {
    fn name() -> &'static str {
        "fake-non-zed-provider"
    }

    fn display_name() -> &'static str {
        "Fake Non-Zed Provider"
    }

    fn show_completions_in_menu() -> bool {
        false
    }

    fn supports_jump_to_edit() -> bool {
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

    fn is_refreshing(&self, _cx: &gpui::App) -> bool {
        false
    }

    fn refresh(
        &mut self,
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
        _direction: edit_prediction::Direction,
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
    ) -> Option<edit_prediction::EditPrediction> {
        self.completion.clone()
    }
}
