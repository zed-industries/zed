// use gpui::Model;
// use inline_completion::{CompletionEdit, CompletionProposal, InlineCompletionProvider};
// use multi_buffer::{MultiBufferSnapshot, ToOffset};
// use std::ops::Range;
// use ui::Context;

// use crate::{
//     editor_tests::init_test, test::editor_test_context::EditorTestContext, ComputedCompletionEdit,
// };

// #[gpui::test]
// async fn test_inline_completion_additions_only(cx: &mut gpui::TestAppContext) {
//     init_test(cx, |_| {});

//     let mut cx = EditorTestContext::new(cx).await;
//     let provider = cx.new_model(|_| FakeInlineCompletionProvider::default());
//     assign_editor_completion_provider(provider.clone(), &mut cx);
//     cx.set_state("let absolute_zero_celsius = ˇ;");

//     propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
//     cx.update_editor(|editor, cx| editor.update_visible_prediction(cx));

//     assert_editor_active_inline_completion(&mut cx, |_, active_completion| {
//         if let ComputedCompletionEdit::Insertion { text, .. } =
//             active_completion.expect("No active completion")
//         {
//             assert_eq!(text.to_string().as_str(), "-273.15");
//         } else {
//             panic!("Expected insertion edit");
//         }
//     });

//     accept_completion(&mut cx);

//     cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
// }

// #[gpui::test]
// async fn test_inline_completion_diff(cx: &mut gpui::TestAppContext) {
//     init_test(cx, |_| {});

//     let mut cx = EditorTestContext::new(cx).await;
//     let provider = cx.new_model(|_| FakeInlineCompletionProvider::default());
//     assign_editor_completion_provider(provider.clone(), &mut cx);
//     cx.set_state("let pi = ˇ\"foo\";");

//     propose_edits(&provider, vec![(9..14, "3.14159")], &mut cx);
//     cx.update_editor(|editor, cx| editor.update_visible_prediction(cx));

//     assert_editor_active_inline_completion(&mut cx, |_, active_completion| {
//         if let ComputedCompletionEdit::Diff { text, .. } =
//             active_completion.expect("No active completion")
//         {
//             assert_eq!(text.to_string().as_str(), "3.14159");
//         } else {
//             panic!("Expected diff edit");
//         }
//     });

//     accept_completion(&mut cx);

//     cx.assert_editor_state("let pi = 3.14159ˇ;")
// }

// #[gpui::test]
// async fn test_reusing_completion(cx: &mut gpui::TestAppContext) {
//     init_test(cx, |_| {});

//     let mut cx = EditorTestContext::new(cx).await;
//     let provider = cx.new_model(|_| FakeInlineCompletionProvider::default());
//     assign_editor_completion_provider(provider.clone(), &mut cx);
//     cx.set_state("let absolute_zero_celsius = ˇ;");

//     propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
//     cx.update_editor(|editor, cx| editor.update_visible_prediction(cx));

//     assert_editor_active_inline_completion(&mut cx, |_snapshot, active_completion| {
//         if let ComputedCompletionEdit::Insertion { text, .. } =
//             active_completion.expect("No active completion")
//         {
//             assert_eq!(text.to_string().as_str(), "-273.15");
//         } else {
//             panic!("Expected insertion edit");
//         }
//     });

//     cx.simulate_keystroke("-");
//     cx.update_editor(|editor, cx| editor.update_visible_prediction(cx));
//     assert_editor_active_inline_completion(&mut cx, |snapshot, active_completion| {
//         if let ComputedCompletionEdit::Insertion { text, position, .. } =
//             active_completion.expect("No active completion")
//         {
//             assert_eq!(position.to_offset(&snapshot), 29);
//             assert_eq!(text.to_string().as_str(), "273.15");
//         } else {
//             panic!("Expected insertion edit");
//         }
//     });

//     cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
// }

// fn assert_editor_active_inline_completion(
//     cx: &mut EditorTestContext,
//     assert: impl FnOnce(MultiBufferSnapshot, Option<&ComputedCompletionEdit>),
// ) {
//     cx.editor(|editor, cx| {
//         assert(
//             editor.buffer().read(cx).snapshot(cx),
//             editor
//                 .active_prediction
//                 .as_ref()
//                 .map(|state| state.active_edit()),
//         )
//     })
// }

// fn accept_completion(cx: &mut EditorTestContext) {
//     cx.update_editor(|editor, cx| {
//         editor.accept_inline_completion(&crate::AcceptInlineCompletion, cx)
//     })
// }

// fn propose_edits(
//     provider: &Model<FakeInlineCompletionProvider>,
//     edits: Vec<(Range<usize>, &str)>,
//     cx: &mut EditorTestContext,
// ) {
//     let edits = build_edits(edits, cx);
//     cx.update(|cx| provider.update(cx, |provider, _| provider.set_proposal(Some(edits))));
// }

// fn build_edits(edits: Vec<(Range<usize>, &str)>, cx: &mut EditorTestContext) -> CompletionProposal {
//     let snapshot = cx.buffer_snapshot();
//     let edits = edits.into_iter().map(|(range, text)| {
//         let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
//         CompletionEdit {
//             range,
//             text: text.into(),
//         }
//     });
//     CompletionProposal {
//         edits: edits.into_iter().collect(),
//     }
// }

// fn assign_editor_completion_provider(
//     provider: Model<FakeInlineCompletionProvider>,
//     cx: &mut EditorTestContext,
// ) {
//     cx.update_editor(|editor, cx| {
//         editor.set_inline_completion_provider(Some(provider), cx);
//     })
// }

// #[derive(Default, Clone)]
// struct FakeInlineCompletionProvider {
//     proposal: Option<CompletionProposal>,
// }

// impl FakeInlineCompletionProvider {
//     pub fn set_proposal(&mut self, proposal: Option<CompletionProposal>) {
//         self.proposal = proposal;
//     }
// }

// impl InlineCompletionProvider for FakeInlineCompletionProvider {
//     fn name() -> &'static str {
//         "fake-completion-provider"
//     }

//     fn is_enabled(
//         &self,
//         _buffer: &gpui::Model<language::Buffer>,
//         _cursor_position: language::Anchor,
//         _cx: &gpui::AppContext,
//     ) -> bool {
//         true
//     }

//     fn refresh(
//         &mut self,
//         _buffer: gpui::Model<language::Buffer>,
//         _cursor_position: language::Anchor,
//         _debounce: bool,
//         _cx: &mut gpui::ModelContext<Self>,
//     ) {
//     }

//     fn cycle(
//         &mut self,
//         _buffer: gpui::Model<language::Buffer>,
//         _cursor_position: language::Anchor,
//         _direction: inline_completion::Direction,
//         _cx: &mut gpui::ModelContext<Self>,
//     ) {
//     }

//     fn accept(&mut self, _cx: &mut gpui::ModelContext<Self>) {}

//     fn discard(
//         &mut self,
//         _should_report_inline_completion_event: bool,
//         _cx: &mut gpui::ModelContext<Self>,
//     ) {
//     }

//     fn predict<'a>(
//         &'a self,
//         _buffer: &gpui::Model<language::Buffer>,
//         _cursor_position: language::Anchor,
//         _cx: &'a gpui::AppContext,
//     ) -> Option<inline_completion::CompletionProposal> {
//         self.proposal.clone()
//     }
// }
