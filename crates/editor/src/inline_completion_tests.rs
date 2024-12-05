use gpui::Model;
use inline_completion::InlineCompletionProvider;
use multi_buffer::MultiBufferSnapshot;
use std::ops::Range;
use ui::Context;

use crate::{
    editor_tests::init_test, test::editor_test_context::EditorTestContext, InlineCompletion,
};

#[gpui::test]
async fn test_inline_completion_insert(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new_model(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let absolute_zero_celsius = ˇ;");

    propose_edits(&provider, vec![(28..28, "-273.15")], &mut cx);
    cx.update_editor(|editor, cx| editor.update_visible_inline_completion(cx));

    assert_editor_active_inline_completion(&mut cx, |_, active_inline_completion| {
        if let InlineCompletion::Edit(edits) =
            active_inline_completion.expect("no active completion")
        {
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].1.as_str(), "-273.15");
        } else {
            panic!("expected edit");
        }
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let absolute_zero_celsius = -273.15ˇ;")
}

#[gpui::test]
async fn test_inline_completion_modification(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let provider = cx.new_model(|_| FakeInlineCompletionProvider::default());
    assign_editor_completion_provider(provider.clone(), &mut cx);
    cx.set_state("let pi = ˇ\"foo\";");

    propose_edits(&provider, vec![(9..14, "3.14159")], &mut cx);
    cx.update_editor(|editor, cx| editor.update_visible_inline_completion(cx));

    assert_editor_active_inline_completion(&mut cx, |_, active_inline_completion| {
        if let InlineCompletion::Edit(edits) =
            active_inline_completion.expect("no active completion")
        {
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].1.as_str(), "3.14159");
        } else {
            panic!("expected edit");
        }
    });

    accept_completion(&mut cx);

    cx.assert_editor_state("let pi = 3.14159ˇ;")
}

fn assert_editor_active_inline_completion(
    cx: &mut EditorTestContext,
    assert: impl FnOnce(MultiBufferSnapshot, Option<&InlineCompletion>),
) {
    cx.editor(|editor, cx| {
        assert(
            editor.buffer().read(cx).snapshot(cx),
            editor
                .active_inline_completion
                .as_ref()
                .map(|state| &state.completion),
        )
    })
}

fn accept_completion(cx: &mut EditorTestContext) {
    cx.update_editor(|editor, cx| {
        editor.accept_inline_completion(&crate::AcceptInlineCompletion, cx)
    })
}

fn propose_edits(
    provider: &Model<FakeInlineCompletionProvider>,
    edits: Vec<(Range<usize>, &str)>,
    cx: &mut EditorTestContext,
) {
    let edits = build_inline_completion(edits, cx);
    cx.update(|cx| {
        provider.update(cx, |provider, _| {
            provider.set_inline_completion(Some(edits))
        })
    });
}

fn build_inline_completion(
    edits: Vec<(Range<usize>, &str)>,
    cx: &mut EditorTestContext,
) -> inline_completion::InlineCompletion {
    let snapshot = cx.buffer_snapshot();
    let edits = edits.into_iter().map(|(range, text)| {
        let range = snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end);
        (range, text.into())
    });
    inline_completion::InlineCompletion {
        edits: edits.collect(),
    }
}

fn assign_editor_completion_provider(
    provider: Model<FakeInlineCompletionProvider>,
    cx: &mut EditorTestContext,
) {
    cx.update_editor(|editor, cx| {
        editor.set_inline_completion_provider(Some(provider), cx);
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

impl InlineCompletionProvider for FakeInlineCompletionProvider {
    fn name() -> &'static str {
        "fake-completion-provider"
    }

    fn is_enabled(
        &self,
        _buffer: &gpui::Model<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &gpui::AppContext,
    ) -> bool {
        true
    }

    fn refresh(
        &mut self,
        _buffer: gpui::Model<language::Buffer>,
        _cursor_position: language::Anchor,
        _debounce: bool,
        _cx: &mut gpui::ModelContext<Self>,
    ) {
    }

    fn cycle(
        &mut self,
        _buffer: gpui::Model<language::Buffer>,
        _cursor_position: language::Anchor,
        _direction: inline_completion::Direction,
        _cx: &mut gpui::ModelContext<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut gpui::ModelContext<Self>) {}

    fn discard(
        &mut self,
        _should_report_inline_completion_event: bool,
        _cx: &mut gpui::ModelContext<Self>,
    ) {
    }

    fn suggest<'a>(
        &mut self,
        _buffer: &gpui::Model<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut gpui::ModelContext<Self>,
    ) -> Option<inline_completion::InlineCompletion> {
        self.completion.clone()
    }
}
