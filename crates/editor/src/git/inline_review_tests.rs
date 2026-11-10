use super::*;
use crate::{editor_tests::init_test, test::select_ranges};
use gpui::{TestAppContext, VisualTestContext};
use project::FakeFs;
use std::cell::RefCell;
use util::path;
use workspace::{MultiWorkspace, OpenOptions};

#[gpui::test]
async fn inline_review_submits_selected_lines_and_keeps_reply_through_edits(
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        serde_json::json!({ "file.txt": "first\nsecond\nthird\n" }),
    )
    .await;
    let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
    let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace = window
        .read_with(cx, |mw, _| mw.workspace().clone())
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                path!("/root/file.txt").into(),
                OpenOptions::default(),
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let submitted = Rc::new(RefCell::new(Vec::new()));
    editor.update_in(cx, |editor, window, cx| {
        select_ranges(editor, "«first\nsecond\nˇ»third\n", window, cx);
        editor
            .show_inline_review(
                {
                    let submitted = submitted.clone();
                    Rc::new(move |comment, _, _| submitted.borrow_mut().push(comment))
                },
                window,
                cx,
            )
            .unwrap();
        let input = editor.diff_review_prompt_editor().unwrap().clone();
        input.update(cx, |input, cx| {
            input.insert("Explain these lines", window, cx)
        });
        editor.submit_diff_review_comment(window, cx);
        input.update(cx, |input, cx| input.insert("Why?", window, cx));
        editor.submit_diff_review_comment(window, cx);
    });
    cx.run_until_parked();
    let submissions = submitted.borrow();
    assert_eq!(submissions.len(), 2);
    assert_eq!(submissions[0].discussion_id, submissions[1].discussion_id);
    assert_ne!(submissions[0].id, submissions[1].id);
    let comment = &submissions[0];
    assert_eq!(
        comment.path,
        std::path::PathBuf::from(path!("/root/file.txt"))
    );
    assert_eq!(comment.lines, 0..2);
    assert_eq!(comment.excerpt, "first\nsecond");
    assert_eq!(comment.comment, "Explain these lines");
    editor.update_in(cx, |editor, window, cx| {
        editor.set_inline_review_reply(comment.id, "Pi".into(), "The final reply".into(), cx);
        let buffer = editor.buffer.read(cx).as_singleton().unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "before\n")], None, cx));
        let snapshot = editor.buffer.read(cx).snapshot(cx);
        let stored = &editor.stored_review_comments[0].1[0];
        assert!(!stored.is_outdated(&snapshot));
        let (author, reply) = stored.agent_reply.as_ref().unwrap();
        assert_eq!(author.as_ref(), "Pi");
        assert_eq!(reply.read(cx).source().as_ref(), "The final reply");
        buffer.update(cx, |buffer, cx| buffer.edit([(7..12, "changed")], None, cx));
        let snapshot = editor.buffer.read(cx).snapshot(cx);
        assert!(editor.stored_review_comments[0].1[0].is_outdated(&snapshot));
        assert_eq!(
            editor.stored_review_comments[0].1[0]
                .original_excerpt
                .as_deref(),
            Some("first\nsecond")
        );
    });
}

#[gpui::test]
fn hiding_one_discussion_preserves_others_drafts_and_inflight_reply(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let buffer = cx.new(|cx| Buffer::local("first\nsecond\nthird\n", cx));
    let editor = cx.add_window(|window, cx| Editor::for_buffer(buffer, None, window, cx));
    editor
        .update(cx, |editor, window, cx| {
            editor.show_review_overlay(
                DisplayRow(0)..DisplayRow(0),
                Some(Rc::new(|_, _, _| {})),
                window,
                cx,
            );
            let first_key = editor.diff_review_overlays[0].hunk_key.clone();
            let first_range = editor.diff_review_overlays[0].anchor_range.clone();
            let first_id = editor.add_review_comment(
                first_key.clone(),
                "First comment".into(),
                first_range,
                cx,
            );
            let cancelled = Rc::new(std::cell::Cell::new(false));
            editor.set_inline_review_status(
                first_id,
                "Queued".into(),
                Some({
                    let cancelled = cancelled.clone();
                    Rc::new(move |_| cancelled.set(true))
                }),
                cx,
            );
            let first_input = editor.diff_review_overlays[0].prompt_editor.clone();
            first_input.update(cx, |input, cx| {
                input.insert("unfinished follow-up", window, cx)
            });

            editor.show_review_overlay(
                DisplayRow(1)..DisplayRow(1),
                Some(Rc::new(|_, _, _| {})),
                window,
                cx,
            );
            let second_key = editor.diff_review_overlays[1].hunk_key.clone();
            let second_range = editor.diff_review_overlays[1].anchor_range.clone();
            editor.add_review_comment(
                second_key.clone(),
                "Second comment".into(),
                second_range,
                cx,
            );

            editor.set_review_overlay_collapsed(&first_key, true, window, cx);
            assert_eq!(editor.diff_review_overlays.len(), 2);
            assert!(editor.diff_review_overlays[0].collapsed);
            assert!(!editor.diff_review_overlays[1].collapsed);
            assert_eq!(editor.total_review_comment_count(), 2);
            assert!(!cancelled.get());
            editor.set_inline_review_reply(
                first_id,
                "Pi".into(),
                "Reply received while hidden".into(),
                cx,
            );
            assert!(editor.diff_review_overlays[0].collapsed);

            editor.set_review_overlay_collapsed(&first_key, false, window, cx);
            assert!(!editor.diff_review_overlays[0].collapsed);
            assert_eq!(first_input.read(cx).text(cx), "unfinished follow-up");
            let snapshot = editor.buffer.read(cx).snapshot(cx);
            let first = &editor.comments_for_hunk(&first_key, &snapshot)[0];
            assert_eq!(first.id, first_id);
            assert_eq!(first.comment, "First comment");
            assert_eq!(
                first
                    .agent_reply
                    .as_ref()
                    .unwrap()
                    .1
                    .read(cx)
                    .source()
                    .as_ref(),
                "Reply received while hidden"
            );
            assert_eq!(
                editor.comments_for_hunk(&second_key, &snapshot)[0].comment,
                "Second comment"
            );
        })
        .unwrap();
}

#[gpui::test]
fn inline_review_rejects_unsaved_buffers(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let editor = cx.add_window(|window, cx| Editor::single_line(window, cx));
    editor
        .update(cx, |editor, window, cx| {
            assert!(
                editor
                    .show_inline_review(Rc::new(|_, _, _| panic!("must not submit")), window, cx)
                    .is_err()
            );
            assert_eq!(editor.total_review_comment_count(), 0);
        })
        .unwrap();
}
