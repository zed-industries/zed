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
    assert_ne!(submissions[0].submission_id, submissions[1].submission_id);
    let comment = &submissions[0];
    assert_eq!(
        comment.path,
        std::path::PathBuf::from(path!("/root/file.txt"))
    );
    assert_eq!(comment.lines, 0..2);
    assert_eq!(comment.excerpt, "first\nsecond");
    assert_eq!(comment.before, "");
    assert_eq!(comment.after, "\nthird\n");
    assert_eq!(comment.comment, "Explain these lines");
    editor.update_in(cx, |editor, _window, cx| {
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
fn discussion_mouse_controls_target_the_clicked_discussion(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let buffer = cx.new(|cx| Buffer::local("first\nmiddle\nlast\n", cx));
    let window = cx.add_window(|window, cx| Editor::for_buffer(buffer, None, window, cx));
    let editor = window.root(cx).unwrap();
    let cx = &mut VisualTestContext::from_window(*window, cx);
    cx.simulate_resize(gpui::size(px(900.), px(1000.)));
    editor.update_in(cx, |editor, window, cx| {
        for (row, comment) in [(0, "First comment"), (2, "Second comment")] {
            let snapshot = editor.buffer.read(cx).snapshot(cx);
            let range = snapshot.anchor_after(Point::new(row, 0))
                ..snapshot.anchor_before(Point::new(row, snapshot.line_len(MultiBufferRow(row))));
            editor.show_review_overlay_for_anchor_range(
                range.clone(),
                Some(Rc::new(|_, _, _| {})),
                window,
                cx,
            );
            let overlay = editor.diff_review_overlays.last().unwrap();
            let key = overlay.hunk_key.clone();
            let input = overlay.prompt_editor.clone();
            input.update(cx, |input, cx| {
                input.insert("unfinished follow-up", window, cx)
            });
            let id = editor.add_review_comment(key, comment.into(), range, cx);
            editor.set_inline_review_reply(
                id,
                "Pi".into(),
                "A **reply** to this comment".into(),
                cx,
            );
        }
    });
    cx.run_until_parked();

    let close = cx.debug_bounds("ICON-Close").unwrap().center();
    cx.simulate_click(close, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.read_with(cx, |editor, _| {
        assert!(!editor.diff_review_overlays[0].collapsed);
        assert!(editor.diff_review_overlays[1].collapsed);
    });
    let close = cx.debug_bounds("ICON-Close").unwrap().center();
    cx.simulate_click(close, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.read_with(cx, |editor, _| {
        assert!(
            editor
                .diff_review_overlays
                .iter()
                .all(|overlay| overlay.collapsed)
        );
    });

    let reopen =
        cx.debug_bounds("review-discussion").unwrap().origin + gpui::point(px(24.), px(8.));
    cx.simulate_mouse_move(reopen, None, gpui::Modifiers::none());
    editor.update_in(cx, |editor, window, _| {
        assert!(
            !editor
                .last_position_map
                .as_ref()
                .unwrap()
                .text_hitbox
                .is_hovered(window)
        );
    });
    cx.simulate_click(reopen, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.read_with(cx, |editor, cx| {
        assert!(editor.diff_review_overlays[0].collapsed);
        assert!(!editor.diff_review_overlays[1].collapsed);
        assert_eq!(
            editor.diff_review_overlays[1]
                .prompt_editor
                .read(cx)
                .text(cx),
            "unfinished follow-up"
        );
    });

    let input = editor.read_with(cx, |editor, cx| {
        editor.diff_review_overlays[1]
            .prompt_editor
            .read(cx)
            .last_position_map
            .as_ref()
            .unwrap()
            .text_hitbox
            .bounds
            .center()
    });
    cx.simulate_click(input, gpui::Modifiers::none());
    cx.simulate_input(" typed");
    editor.read_with(cx, |editor, cx| {
        assert_eq!(
            editor.diff_review_overlays[1]
                .prompt_editor
                .read(cx)
                .text(cx),
            "unfinished follow-up typed"
        );
        assert_eq!(
            editor.buffer.read(cx).snapshot(cx).text(),
            "first\nmiddle\nlast\n"
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        window.focus(&editor.focus_handle, cx)
    });
    cx.run_until_parked();
    let header = cx.debug_bounds("review-comments-header").unwrap().center();
    cx.simulate_click(header, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.read_with(cx, |editor, _| {
        assert!(editor.diff_review_overlays[0].comments_expanded);
        assert!(!editor.diff_review_overlays[1].comments_expanded);
    });
    let header = cx.debug_bounds("review-comments-header").unwrap().center();
    cx.simulate_click(header, gpui::Modifiers::none());
    cx.run_until_parked();
    let selections = editor.read_with(cx, |editor, _| {
        editor.selections.disjoint_anchors().to_vec()
    });
    let background =
        cx.debug_bounds("review-discussion").unwrap().bottom_right() - gpui::point(px(4.), px(4.));
    cx.simulate_mouse_move(background, None, gpui::Modifiers::none());
    cx.simulate_click(background, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.update_in(cx, |editor, window, _| {
        assert_eq!(editor.selections.disjoint_anchors(), selections);
        assert!(
            !editor
                .last_position_map
                .as_ref()
                .unwrap()
                .text_hitbox
                .is_hovered(window)
        );
    });

    let submit = cx.debug_bounds("ICON-Return").unwrap().center();
    cx.simulate_click(submit, gpui::Modifiers::none());
    cx.run_until_parked();
    editor.read_with(cx, |editor, cx| {
        assert_eq!(editor.stored_review_comments[0].1.len(), 1);
        assert_eq!(editor.stored_review_comments[1].1.len(), 2);
        assert_eq!(
            editor.stored_review_comments[1].1[1].comment,
            "unfinished follow-up typed"
        );
        assert_eq!(
            editor.diff_review_overlays[0]
                .prompt_editor
                .read(cx)
                .text(cx),
            "unfinished follow-up"
        );
        assert!(
            editor.diff_review_overlays[1]
                .prompt_editor
                .read(cx)
                .text(cx)
                .is_empty()
        );
    });
}

#[gpui::test]
async fn saved_reviews_restore_in_a_fresh_editor_and_keep_discussion_identity(
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        serde_json::json!({"file.txt": "before\nfirst\nsecond\nthird\n"}),
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
    let mut review = SavedInlineReview {
        version: 1,
        discussion_id: uuid::Uuid::new_v4().to_string(),
        submission_id: uuid::Uuid::new_v4().to_string(),
        path: path!("/root/file.txt").into(),
        start_line: 1,
        end_line: 2,
        excerpt: "second".into(),
        diff: None,
        before: "first\n".into(),
        after: "\nthird\n".into(),
        comment: "Explain".into(),
        created_at: 1,
        outcome: "pending".into(),
        reply: None,
        error: None,
    };
    let encoded = serde_json::to_string(&review).unwrap();
    let submitted = Rc::new(RefCell::new(Vec::new()));
    let on_submit: ReviewSubmitHandler = Rc::new({
        let submitted = submitted.clone();
        move |comment, _, _| submitted.borrow_mut().push(comment)
    });
    editor.update_in(cx, |editor, window, cx| {
        assert_eq!(editor.total_review_comment_count(), 0);
        editor.fold_ranges(vec![Point::new(1, 0)..Point::new(3, 5)], false, window, cx);
        editor
            .restore_inline_review(
                serde_json::from_str(&encoded).unwrap(),
                on_submit.clone(),
                window,
                cx,
            )
            .unwrap();
        assert_eq!(editor.diff_review_line_range(cx), Some((2, 2)));
        assert!(editor.diff_review_overlays[0].collapsed);
        editor.unfold_ranges(&[Point::new(1, 0)..Point::new(3, 5)], true, false, cx);
        let stored = &editor.stored_review_comments[0].1[0];
        assert!(
            stored
                .submission_status
                .as_ref()
                .unwrap()
                .contains("not resent")
        );
        assert!(stored.on_cancel.is_none());
        review.outcome = "answered".into();
        review.reply = Some("**Pi reply**".into());
        editor
            .restore_inline_review(review.clone(), on_submit.clone(), window, cx)
            .unwrap();
        editor
            .restore_inline_review(review.clone(), on_submit.clone(), window, cx)
            .unwrap();
        assert_eq!(editor.total_review_comment_count(), 1);
        let stored = &editor.stored_review_comments[0].1[0];
        assert_eq!(stored.agent_reply.as_ref().unwrap().0.as_ref(), "Pi");
        assert_eq!(
            stored
                .agent_reply
                .as_ref()
                .unwrap()
                .1
                .read(cx)
                .source()
                .as_ref(),
            "**Pi reply**"
        );
        editor.dismiss_all_diff_review_overlays(cx);
        editor
            .restore_inline_review(review.clone(), on_submit.clone(), window, cx)
            .unwrap();
        assert_eq!(editor.diff_review_overlays.len(), 1);
        select_ranges(editor, "before\nfirst\n«secondˇ»\nthird\n", window, cx);
        editor
            .show_inline_review(on_submit.clone(), window, cx)
            .unwrap();
        let input = editor.diff_review_prompt_editor().unwrap().clone();
        input.update(cx, |input, cx| input.insert("Follow-up", window, cx));
        editor.submit_diff_review_comment(window, cx);
    });
    cx.run_until_parked();
    assert_eq!(submitted.borrow().len(), 1);
    assert_eq!(submitted.borrow()[0].discussion_id, review.discussion_id);
    assert_ne!(submitted.borrow()[0].submission_id, review.submission_id);
    editor.update_in(cx, |editor, window, cx| {
        let buffer = editor.buffer.read(cx).as_singleton().unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "first\nsecond\nthird\n")], None, cx)
        });
        let mut ambiguous = review.clone();
        ambiguous.submission_id = uuid::Uuid::new_v4().to_string();
        assert!(
            editor
                .restore_inline_review(ambiguous, on_submit, window, cx)
                .unwrap_err()
                .to_string()
                .contains("ambiguous")
        );
        assert_eq!(editor.total_review_comment_count(), 2);
    });
}

#[gpui::test]
async fn diff_reviews_capture_and_restore_exact_sides_and_comparisons(cx: &mut TestAppContext) {
    use super::review_source::{ReviewComparison, ReviewSide};
    init_test(cx, |_| {});
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        serde_json::json!({
            ".git": {}, "file.txt": "header\nworking version\nfooter\n",
        }),
    )
    .await;
    fs.set_head_for_repo(
        std::path::Path::new(path!("/project/.git")),
        &[("file.txt", "header\nhead version\nfooter\n".into())],
        "deadbeef",
    );
    fs.set_index_for_repo(
        std::path::Path::new(path!("/project/.git")),
        &[("file.txt", "header\nindex version\nfooter\n".into())],
    );
    let base_oid = fs.set_merge_base_content_for_repo(
        path!("/project/.git").as_ref(),
        &[("file.txt", "header\nbranch base version\nfooter\n".into())],
    )[0];
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/file.txt"), cx)
        })
        .await
        .unwrap();
    for comparison in [
        ReviewComparison::Unstaged,
        ReviewComparison::Staged,
        ReviewComparison::Uncommitted,
        ReviewComparison::Branch,
    ] {
        let (diff, display_buffer, old, new) = match comparison {
            ReviewComparison::Branch => {
                project
                    .update(cx, |project, cx| project.git_scans_complete(cx))
                    .await;
                let repository =
                    project.read_with(cx, |project, cx| project.active_repository(cx).unwrap());
                let diff = project
                    .read_with(cx, |project, _| project.git_store().clone())
                    .update(cx, |store, cx| {
                        store.open_diff_since(Some(base_oid), buffer.clone(), repository, cx)
                    })
                    .await
                    .unwrap();
                (
                    diff,
                    buffer.clone(),
                    "branch base version",
                    "working version",
                )
            }
            ReviewComparison::Unstaged => (
                project
                    .update(cx, |project, cx| {
                        project.open_unstaged_diff(buffer.clone(), cx)
                    })
                    .await
                    .unwrap(),
                buffer.clone(),
                "index version",
                "working version",
            ),
            ReviewComparison::Uncommitted => (
                project
                    .update(cx, |project, cx| {
                        project.open_uncommitted_diff(buffer.clone(), cx)
                    })
                    .await
                    .unwrap(),
                buffer.clone(),
                "head version",
                "working version",
            ),
            ReviewComparison::Staged => {
                let (diff, index) = project
                    .update(cx, |project, cx| {
                        project.open_staged_diff(buffer.clone(), cx)
                    })
                    .await
                    .unwrap();
                (diff, index, "head version", "index version")
            }
        };
        for left in [false, true] {
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = MultiBuffer::new(if left {
                    language::Capability::ReadOnly
                } else {
                    language::Capability::ReadWrite
                });
                let displayed = if left {
                    diff.read(cx).base_text_buffer().clone()
                } else {
                    display_buffer.clone()
                };
                multibuffer.set_excerpts_for_path(
                    multi_buffer::PathKey::sorted(0),
                    displayed.clone(),
                    [Point::zero()..displayed.read(cx).max_point()],
                    0,
                    cx,
                );
                if left {
                    multibuffer.add_inverted_diff(diff.clone(), display_buffer.clone(), cx);
                } else {
                    multibuffer.add_diff(diff.clone(), cx);
                }
                multibuffer.set_all_diff_hunks_expanded(cx);
                if left {
                    multibuffer.set_show_deleted_hunks(false, cx);
                }
                multibuffer
            });
            let window = cx.add_window(|window, cx| {
                let mut editor =
                    Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
                editor.set_diff_hunk_renderer(Some(Arc::new(DefaultDiffHunkRenderer)), cx);
                editor.set_read_only(left);
                editor
            });
            let editor = window.root(cx).unwrap();
            let visual = &mut VisualTestContext::from_window(*window, cx);
            let submitted = Rc::new(RefCell::new(Vec::new()));
            for (side, text) in [(ReviewSide::Old, old), (ReviewSide::New, new)] {
                if left && side == ReviewSide::New {
                    continue;
                }
                editor.update_in(visual, |editor, window, cx| {
                    let snapshot = editor.buffer.read(cx).snapshot(cx);
                    let all_text = snapshot.text();
                    let offset = all_text
                        .find(text)
                        .unwrap_or_else(|| panic!("Missing {text} in {all_text}"));
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections
                                .select_ranges([MultiBufferOffset(offset)
                                    ..MultiBufferOffset(offset + text.len())]);
                        },
                    );
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
                    let input = editor
                        .diff_review_overlays
                        .last()
                        .unwrap()
                        .prompt_editor
                        .clone();
                    input.update(cx, |input, cx| {
                        input.insert("Explain this side", window, cx)
                    });
                    editor.submit_diff_review_comment(window, cx);
                });
                visual.run_until_parked();
                let submissions = submitted.borrow();
                let comment = submissions.last().expect("The diff comment was submitted");
                assert_eq!(
                    comment.path,
                    std::path::PathBuf::from(path!("/project/file.txt"))
                );
                assert_eq!(comment.lines, 1..2);
                assert_eq!(comment.excerpt, text);
                assert_eq!(comment.before, "header\n");
                assert_eq!(comment.after, "\nfooter\n");
                assert_eq!(comment.diff.as_ref().unwrap().side, side);
                assert_eq!(comment.diff.as_ref().unwrap().comparison, comparison);
                let saved = SavedInlineReview {
                    version: 2,
                    discussion_id: comment.discussion_id.clone(),
                    submission_id: comment.submission_id.clone(),
                    path: comment.path.clone(),
                    start_line: 1,
                    end_line: 2,
                    excerpt: comment.excerpt.clone(),
                    diff: comment.diff.clone(),
                    before: comment.before.clone(),
                    after: comment.after.clone(),
                    comment: comment.comment.clone(),
                    created_at: 1,
                    outcome: "answered".into(),
                    reply: Some("The saved reply".into()),
                    error: None,
                };
                let restored = visual.add_window(|window, cx| {
                    let mut editor = Editor::for_multibuffer(
                        multibuffer.clone(),
                        Some(project.clone()),
                        window,
                        cx,
                    );
                    editor.set_diff_hunk_renderer(Some(Arc::new(DefaultDiffHunkRenderer)), cx);
                    editor.set_read_only(left);
                    editor
                });
                let followups = Rc::new(RefCell::new(Vec::new()));
                restored
                    .update(visual, |editor, window, cx| {
                        editor
                            .restore_inline_review(saved.clone(), Rc::new(|_, _, _| {}), window, cx)
                            .unwrap_or_else(|error| {
                                panic!("{comparison:?} {side:?} left={left}: {error:#}")
                            });
                        assert_eq!(editor.total_review_comment_count(), 1);
                        let stored = &editor.stored_review_comments[0].1[0];
                        assert_eq!(stored.diff, saved.diff);
                        assert_eq!(stored.discussion_id.as_ref(), Some(&saved.discussion_id));
                        assert_eq!(
                            stored
                                .agent_reply
                                .as_ref()
                                .unwrap()
                                .1
                                .read(cx)
                                .source()
                                .as_ref(),
                            "The saved reply"
                        );
                        let source = editor
                            .inline_review_source(stored.range.clone(), cx)
                            .unwrap();
                        assert_eq!(source.excerpt, text);
                        assert_eq!(source.diff, saved.diff);
                        let mut changed_base = saved.clone();
                        changed_base.submission_id = uuid::Uuid::new_v4().to_string();
                        changed_base.diff.as_mut().unwrap().base_sha256 = "0".repeat(64);
                        assert!(
                            editor
                                .restore_inline_review(
                                    changed_base,
                                    Rc::new(|_, _, _| {}),
                                    window,
                                    cx
                                )
                                .is_err()
                        );
                        assert_eq!(editor.total_review_comment_count(), 1);
                        let range = editor.stored_review_comments[0].1[0].range.clone();
                        editor.change_selections(
                            SelectionEffects::no_scroll(),
                            window,
                            cx,
                            |selections| selections.select_ranges([range]),
                        );
                        editor
                            .show_inline_review(
                                {
                                    let followups = followups.clone();
                                    Rc::new(move |comment, _, _| {
                                        followups.borrow_mut().push(comment)
                                    })
                                },
                                window,
                                cx,
                            )
                            .unwrap();
                        let input = editor.diff_review_overlays[0].prompt_editor.clone();
                        input.update(cx, |input, cx| {
                            input.insert("Follow-up on this side", window, cx)
                        });
                        editor.submit_diff_review_comment(window, cx);
                    })
                    .unwrap();
                visual.run_until_parked();
                let followups = followups.borrow();
                assert_eq!(followups.len(), 1);
                assert_eq!(followups[0].discussion_id, saved.discussion_id);
                assert_eq!(followups[0].diff, saved.diff);
                assert_ne!(followups[0].submission_id, saved.submission_id);
            }
            if !left {
                editor.update_in(visual, |editor, window, cx| {
                    let text = editor.buffer.read(cx).snapshot(cx).text();
                    let start = text.find(old).unwrap();
                    let end = text.find(new).unwrap() + new.len();
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections
                                .select_ranges([MultiBufferOffset(start)..MultiBufferOffset(end)]);
                        },
                    );
                    assert!(
                        editor
                            .show_inline_review(
                                Rc::new(|_, _, _| panic!("mixed sides must not submit")),
                                window,
                                cx
                            )
                            .is_err()
                    );
                });
            }
        }
    }
}

#[gpui::test]
async fn diff_reviews_support_deleted_files_and_reject_cross_file_selections(
    cx: &mut TestAppContext,
) {
    use super::review_source::ReviewSide;
    init_test(cx, |_| {});
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        serde_json::json!({ ".git": {}, "added.txt": "new file without newline" }),
    )
    .await;
    fs.set_head_for_repo(
        std::path::Path::new(path!("/project/.git")),
        &[("gone.txt", "first\nremoved final line".into())],
        "deadbeef",
    );
    fs.set_index_for_repo(
        std::path::Path::new(path!("/project/.git")),
        &[("gone.txt", "first\nremoved final line".into())],
    );
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let multibuffer = cx.new(|_| MultiBuffer::new(language::Capability::ReadWrite));
    for (index, name) in ["gone.txt", "added.txt"].into_iter().enumerate() {
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(std::path::Path::new(path!("/project")).join(name), cx)
            })
            .await
            .unwrap();
        let diff = project
            .update(cx, |project, cx| {
                project.open_unstaged_diff(buffer.clone(), cx)
            })
            .await
            .unwrap();
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                multi_buffer::PathKey::sorted(index as u64),
                buffer.clone(),
                [Point::zero()..buffer.read(cx).max_point()],
                0,
                cx,
            );
            multibuffer.add_diff(diff, cx);
            multibuffer.set_all_diff_hunks_expanded(cx);
        });
    }
    let window = cx.add_window(|window, cx| {
        let mut editor = Editor::for_multibuffer(multibuffer, Some(project), window, cx);
        editor.set_diff_hunk_renderer(Some(Arc::new(DefaultDiffHunkRenderer)), cx);
        editor
    });
    let editor = window.root(cx).unwrap();
    let cx = &mut VisualTestContext::from_window(*window, cx);
    for (name, text, side, lines) in [
        ("gone.txt", "removed final line", ReviewSide::Old, 1..2),
        (
            "added.txt",
            "new file without newline",
            ReviewSide::New,
            0..1,
        ),
    ] {
        let submitted = Rc::new(RefCell::new(Vec::new()));
        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.buffer.read(cx).snapshot(cx);
            let offset = snapshot.text().find(text).unwrap();
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([
                    MultiBufferOffset(offset)..MultiBufferOffset(offset + text.len())
                ])
            });
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
            let input = editor
                .diff_review_overlays
                .last()
                .unwrap()
                .prompt_editor
                .clone();
            input.update(cx, |input, cx| input.insert("Review this", window, cx));
            editor.submit_diff_review_comment(window, cx);
        });
        cx.run_until_parked();
        let submissions = submitted.borrow();
        let comment = &submissions[0];
        assert_eq!(comment.excerpt, text);
        assert_eq!(comment.lines, lines);
        assert_eq!(
            comment.path,
            std::path::Path::new(path!("/project")).join(name)
        );
        assert_eq!(comment.diff.as_ref().unwrap().side, side);
        editor.update_in(cx, |editor, _, cx| {
            let saved = SavedInlineReview {
                version: 2,
                discussion_id: comment.discussion_id.clone(),
                submission_id: comment.submission_id.clone(),
                path: comment.path.clone(),
                start_line: comment.lines.start,
                end_line: comment.lines.end,
                excerpt: comment.excerpt.clone(),
                diff: comment.diff.clone(),
                before: comment.before.clone(),
                after: comment.after.clone(),
                comment: comment.comment.clone(),
                created_at: 1,
                outcome: "pending".into(),
                reply: None,
                error: None,
            };
            let range = editor.saved_review_range(&saved, cx).unwrap();
            assert_eq!(
                editor.inline_review_source(range, cx).unwrap().excerpt,
                text
            );
        });
    }
    editor.update_in(cx, |editor, window, cx| {
        let snapshot = editor.buffer.read(cx).snapshot(cx);
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
            selections.select_ranges([MultiBufferOffset(0)..snapshot.len()])
        });
        assert!(
            editor
                .show_inline_review(
                    Rc::new(|_, _, _| panic!("must not submit across files")),
                    window,
                    cx
                )
                .is_err()
        );
    });
}

#[gpui::test]
async fn branch_reviews_work_when_all_changes_are_already_committed(cx: &mut TestAppContext) {
    use super::review_source::{ReviewComparison, ReviewSide};
    init_test(cx, |_| {});
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), serde_json::json!({
        ".git": {}, "changed.txt": "header\ncommitted change\nfooter\n", "added.txt": "committed addition",
    })).await;
    let head = [
        (
            "changed.txt",
            "header\ncommitted change\nfooter\n".to_owned(),
        ),
        ("added.txt", "committed addition".to_owned()),
    ];
    fs.set_head_for_repo(path!("/project/.git").as_ref(), &head, "deadbeef");
    fs.set_index_for_repo(path!("/project/.git").as_ref(), &head);
    let base_oids = fs.set_merge_base_content_for_repo(
        path!("/project/.git").as_ref(),
        &[
            ("changed.txt", "header\noriginal code\nfooter\n".into()),
            ("deleted.txt", "committed removal".into()),
        ],
    );
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;
    let repository = project.read_with(cx, |project, cx| project.active_repository(cx).unwrap());
    let git_store = project.read_with(cx, |project, _| project.git_store().clone());
    for (name, base, old, new) in [
        (
            "changed.txt",
            Some(base_oids[0]),
            Some("original code"),
            Some("committed change"),
        ),
        ("added.txt", None, None, Some("committed addition")),
        (
            "deleted.txt",
            Some(base_oids[1]),
            Some("committed removal"),
            None,
        ),
    ] {
        let path = std::path::Path::new(path!("/project")).join(name);
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path.clone(), cx)
            })
            .await
            .unwrap();
        let uncommitted = project
            .update(cx, |project, cx| {
                project.open_uncommitted_diff(buffer.clone(), cx)
            })
            .await
            .unwrap();
        let uncommitted = uncommitted.read_with(cx, |diff, cx| diff.snapshot(cx));
        assert_eq!(
            uncommitted.base_text().text(),
            buffer.read_with(cx, |buffer, _| buffer.text())
        );
        if new.is_some() {
            assert!(
                uncommitted.is_empty(),
                "Unexpected uncommitted changes in {name}"
            );
        }
        let diff = git_store
            .update(cx, |store, cx| {
                store.open_diff_since(base, buffer.clone(), repository.clone(), cx)
            })
            .await
            .unwrap();
        assert!(
            !diff.read_with(cx, |diff, cx| diff.snapshot(cx).is_empty()),
            "Missing committed changes in {name}"
        );
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
            multibuffer.set_excerpts_for_path(
                multi_buffer::PathKey::sorted(0),
                buffer.clone(),
                [Point::zero()..buffer.read(cx).max_point()],
                0,
                cx,
            );
            multibuffer.add_diff(diff, cx);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let window = cx.add_window(|window, cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx);
            editor.set_diff_hunk_renderer(Some(Arc::new(DefaultDiffHunkRenderer)), cx);
            editor
        });
        let editor = window.root(cx).unwrap();
        let visual = &mut VisualTestContext::from_window(*window, cx);
        for (side, text) in [(ReviewSide::Old, old), (ReviewSide::New, new)] {
            let Some(text) = text else { continue };
            let posted = Rc::new(RefCell::new(Vec::new()));
            editor.update_in(visual, |editor, window, cx| {
                let snapshot = editor.buffer.read(cx).snapshot(cx);
                let offset = snapshot.text().find(text).unwrap();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([
                        MultiBufferOffset(offset)..MultiBufferOffset(offset + text.len())
                    ])
                });
                editor
                    .show_inline_review(
                        {
                            let posted = posted.clone();
                            Rc::new(move |comment, _, _| posted.borrow_mut().push(comment))
                        },
                        window,
                        cx,
                    )
                    .unwrap();
                let input = editor
                    .diff_review_overlays
                    .last()
                    .unwrap()
                    .prompt_editor
                    .clone();
                input.update(cx, |input, cx| {
                    input.insert("Review this committed change", window, cx)
                });
                editor.submit_diff_review_comment(window, cx);
            });
            visual.run_until_parked();
            let posted = posted.borrow();
            assert_eq!(posted.len(), 1);
            assert_eq!(posted[0].path, path);
            assert_eq!(posted[0].excerpt, text);
            assert_eq!(
                posted[0].diff.as_ref().unwrap().comparison,
                ReviewComparison::Branch
            );
            assert_eq!(posted[0].diff.as_ref().unwrap().side, side);
        }
    }
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
