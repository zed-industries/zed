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
