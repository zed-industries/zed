use std::{cmp, ops::Range};

use collections::HashMap;
use futures::future::join_all;
use gpui::{Hsla, Rgba};
use itertools::Itertools;
use language::point_from_lsp;
use multi_buffer::Anchor;
use project::{DocumentColor, InlayId};
use settings::Settings as _;
use text::{Bias, BufferId, OffsetRangeExt as _};
use ui::{App, Context, Window};
use util::post_inc;

use crate::{
    DisplayPoint, Editor, EditorSettings, EditorSnapshot, InlaySplice,
    LSP_REQUEST_DEBOUNCE_TIMEOUT, RangeToAnchorExt, editor_settings::DocumentColorsRenderMode,
    inlays::Inlay,
};

#[derive(Debug)]
pub(super) struct LspColorData {
    buffer_colors: HashMap<BufferId, BufferColors>,
    render_mode: DocumentColorsRenderMode,
}

#[derive(Debug, Default)]
struct BufferColors {
    colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>,
    inlay_colors: HashMap<InlayId, usize>,
}

impl LspColorData {
    pub fn new(cx: &App) -> Self {
        Self {
            buffer_colors: HashMap::default(),
            render_mode: EditorSettings::get_global(cx).lsp_document_colors,
        }
    }

    pub fn render_mode_updated(
        &mut self,
        new_render_mode: DocumentColorsRenderMode,
    ) -> Option<InlaySplice> {
        if self.render_mode == new_render_mode {
            return None;
        }
        self.render_mode = new_render_mode;
        match new_render_mode {
            DocumentColorsRenderMode::Inlay => Some(InlaySplice {
                to_remove: Vec::new(),
                to_insert: self
                    .buffer_colors
                    .iter()
                    .flat_map(|(_, buffer_colors)| buffer_colors.colors.iter())
                    .map(|(range, color, id)| {
                        Inlay::color(
                            id.id(),
                            range.start,
                            Rgba {
                                r: color.color.red,
                                g: color.color.green,
                                b: color.color.blue,
                                a: color.color.alpha,
                            },
                        )
                    })
                    .collect(),
            }),
            DocumentColorsRenderMode::None => Some(InlaySplice {
                to_remove: self
                    .buffer_colors
                    .drain()
                    .flat_map(|(_, buffer_colors)| buffer_colors.inlay_colors)
                    .map(|(id, _)| id)
                    .collect(),
                to_insert: Vec::new(),
            }),
            DocumentColorsRenderMode::Border | DocumentColorsRenderMode::Background => {
                Some(InlaySplice {
                    to_remove: self
                        .buffer_colors
                        .iter_mut()
                        .flat_map(|(_, buffer_colors)| buffer_colors.inlay_colors.drain())
                        .map(|(id, _)| id)
                        .collect(),
                    to_insert: Vec::new(),
                })
            }
        }
    }

    fn set_colors(
        &mut self,
        buffer_id: BufferId,
        colors: Vec<(Range<Anchor>, DocumentColor, InlayId)>,
    ) -> bool {
        let buffer_colors = self.buffer_colors.entry(buffer_id).or_default();
        if buffer_colors.colors == colors {
            return false;
        }

        buffer_colors.inlay_colors = colors
            .iter()
            .enumerate()
            .map(|(i, (_, _, id))| (*id, i))
            .collect();
        buffer_colors.colors = colors;
        true
    }

    pub fn editor_display_highlights(
        &self,
        snapshot: &EditorSnapshot,
    ) -> (DocumentColorsRenderMode, Vec<(Range<DisplayPoint>, Hsla)>) {
        let render_mode = self.render_mode;
        let highlights = if render_mode == DocumentColorsRenderMode::None
            || render_mode == DocumentColorsRenderMode::Inlay
        {
            Vec::new()
        } else {
            self.buffer_colors
                .iter()
                .flat_map(|(_, buffer_colors)| &buffer_colors.colors)
                .map(|(range, color, _)| {
                    let display_range = range.clone().to_display_points(snapshot);
                    let color = Hsla::from(Rgba {
                        r: color.color.red,
                        g: color.color.green,
                        b: color.color.blue,
                        a: color.color.alpha,
                    });
                    (display_range, color)
                })
                .collect()
        };
        (render_mode, highlights)
    }
}

impl Editor {
    pub(super) fn refresh_colors_for_visible_range(
        &mut self,
        buffer_id: Option<BufferId>,
        _: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() {
            return;
        }
        let Some(project) = self.project.as_ref() else {
            return;
        };
        if self
            .colors
            .as_ref()
            .is_none_or(|colors| colors.render_mode == DocumentColorsRenderMode::None)
        {
            return;
        }

        let buffers_to_query = self
            .visible_excerpts(true, cx)
            .into_values()
            .map(|(buffer, ..)| buffer)
            .chain(buffer_id.and_then(|buffer_id| self.buffer.read(cx).buffer(buffer_id)))
            .filter(|editor_buffer| {
                let editor_buffer_id = editor_buffer.read(cx).remote_id();
                buffer_id.is_none_or(|buffer_id| buffer_id == editor_buffer_id)
                    && self.registered_buffers.contains_key(&editor_buffer_id)
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        let project = project.downgrade();
        self.refresh_colors_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                .await;

            let Some(all_colors_task) = project
                .update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        buffers_to_query
                            .into_iter()
                            .filter_map(|buffer| {
                                let buffer_id = buffer.read(cx).remote_id();
                                let colors_task = lsp_store.document_colors(buffer, cx)?;
                                Some(async move { (buffer_id, colors_task.await) })
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .ok()
            else {
                return;
            };

            let all_colors = join_all(all_colors_task).await;
            if all_colors.is_empty() {
                return;
            }
            let Ok((multi_buffer_snapshot, editor_excerpts)) = editor.update(cx, |editor, cx| {
                let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let editor_excerpts = multi_buffer_snapshot.excerpts().fold(
                    HashMap::default(),
                    |mut acc, (excerpt_id, buffer_snapshot, excerpt_range)| {
                        let excerpt_data = acc
                            .entry(buffer_snapshot.remote_id())
                            .or_insert_with(Vec::new);
                        let excerpt_point_range =
                            excerpt_range.context.to_point_utf16(buffer_snapshot);
                        excerpt_data.push((
                            excerpt_id,
                            buffer_snapshot.clone(),
                            excerpt_point_range,
                        ));
                        acc
                    },
                );
                (multi_buffer_snapshot, editor_excerpts)
            }) else {
                return;
            };

            let mut new_editor_colors: HashMap<BufferId, Vec<(Range<Anchor>, DocumentColor)>> =
                HashMap::default();
            for (buffer_id, colors) in all_colors {
                let Some(excerpts) = editor_excerpts.get(&buffer_id) else {
                    continue;
                };
                match colors {
                    Ok(colors) => {
                        if colors.colors.is_empty() {
                            new_editor_colors
                                .entry(buffer_id)
                                .or_insert_with(Vec::new)
                                .clear();
                        } else {
                            for color in colors.colors {
                                let color_start = point_from_lsp(color.lsp_range.start);
                                let color_end = point_from_lsp(color.lsp_range.end);

                                for (excerpt_id, buffer_snapshot, excerpt_range) in excerpts {
                                    if !excerpt_range.contains(&color_start.0)
                                        || !excerpt_range.contains(&color_end.0)
                                    {
                                        continue;
                                    }
                                    let start = buffer_snapshot.anchor_before(
                                        buffer_snapshot.clip_point_utf16(color_start, Bias::Left),
                                    );
                                    let end = buffer_snapshot.anchor_after(
                                        buffer_snapshot.clip_point_utf16(color_end, Bias::Right),
                                    );
                                    let Some(range) = multi_buffer_snapshot
                                        .anchor_range_in_excerpt(*excerpt_id, start..end)
                                    else {
                                        continue;
                                    };

                                    let new_buffer_colors =
                                        new_editor_colors.entry(buffer_id).or_insert_with(Vec::new);

                                    let (Ok(i) | Err(i)) =
                                        new_buffer_colors.binary_search_by(|(probe, _)| {
                                            probe
                                                .start
                                                .cmp(&range.start, &multi_buffer_snapshot)
                                                .then_with(|| {
                                                    probe
                                                        .end
                                                        .cmp(&range.end, &multi_buffer_snapshot)
                                                })
                                        });
                                    new_buffer_colors.insert(i, (range, color));
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => log::error!("Failed to retrieve document colors: {e}"),
                }
            }

            editor
                .update(cx, |editor, cx| {
                    let mut colors_splice = InlaySplice::default();
                    let Some(colors) = &mut editor.colors else {
                        return;
                    };
                    let mut updated = false;
                    for (buffer_id, new_buffer_colors) in new_editor_colors {
                        let mut new_buffer_color_inlays =
                            Vec::with_capacity(new_buffer_colors.len());
                        let mut existing_buffer_colors = colors
                            .buffer_colors
                            .entry(buffer_id)
                            .or_default()
                            .colors
                            .iter()
                            .peekable();
                        for (new_range, new_color) in new_buffer_colors {
                            let rgba_color = Rgba {
                                r: new_color.color.red,
                                g: new_color.color.green,
                                b: new_color.color.blue,
                                a: new_color.color.alpha,
                            };

                            loop {
                                match existing_buffer_colors.peek() {
                                    Some((existing_range, existing_color, existing_inlay_id)) => {
                                        match existing_range
                                            .start
                                            .cmp(&new_range.start, &multi_buffer_snapshot)
                                            .then_with(|| {
                                                existing_range
                                                    .end
                                                    .cmp(&new_range.end, &multi_buffer_snapshot)
                                            }) {
                                            cmp::Ordering::Less => {
                                                colors_splice.to_remove.push(*existing_inlay_id);
                                                existing_buffer_colors.next();
                                                continue;
                                            }
                                            cmp::Ordering::Equal => {
                                                if existing_color == &new_color {
                                                    new_buffer_color_inlays.push((
                                                        new_range,
                                                        new_color,
                                                        *existing_inlay_id,
                                                    ));
                                                } else {
                                                    colors_splice
                                                        .to_remove
                                                        .push(*existing_inlay_id);

                                                    let inlay = Inlay::color(
                                                        post_inc(&mut editor.next_color_inlay_id),
                                                        new_range.start,
                                                        rgba_color,
                                                    );
                                                    let inlay_id = inlay.id;
                                                    colors_splice.to_insert.push(inlay);
                                                    new_buffer_color_inlays
                                                        .push((new_range, new_color, inlay_id));
                                                }
                                                existing_buffer_colors.next();
                                                break;
                                            }
                                            cmp::Ordering::Greater => {
                                                let inlay = Inlay::color(
                                                    post_inc(&mut editor.next_color_inlay_id),
                                                    new_range.start,
                                                    rgba_color,
                                                );
                                                let inlay_id = inlay.id;
                                                colors_splice.to_insert.push(inlay);
                                                new_buffer_color_inlays
                                                    .push((new_range, new_color, inlay_id));
                                                break;
                                            }
                                        }
                                    }
                                    None => {
                                        let inlay = Inlay::color(
                                            post_inc(&mut editor.next_color_inlay_id),
                                            new_range.start,
                                            rgba_color,
                                        );
                                        let inlay_id = inlay.id;
                                        colors_splice.to_insert.push(inlay);
                                        new_buffer_color_inlays
                                            .push((new_range, new_color, inlay_id));
                                        break;
                                    }
                                }
                            }
                        }

                        if existing_buffer_colors.peek().is_some() {
                            colors_splice
                                .to_remove
                                .extend(existing_buffer_colors.map(|(_, _, id)| *id));
                        }
                        updated |= colors.set_colors(buffer_id, new_buffer_color_inlays);
                    }

                    if colors.render_mode == DocumentColorsRenderMode::Inlay
                        && !colors_splice.is_empty()
                    {
                        editor.splice_inlays(&colors_splice.to_remove, colors_splice.to_insert, cx);
                        updated = true;
                    }

                    if updated {
                        cx.notify();
                    }
                })
                .ok();
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        sync::{
            Arc,
            atomic::{self, AtomicUsize},
        },
        time::Duration,
    };

    use futures::StreamExt;
    use gpui::{Rgba, TestAppContext, VisualTestContext};
    use language::FakeLspAdapter;
    use languages::rust_lang;
    use project::{FakeFs, Project};
    use serde_json::json;
    use util::{path, rel_path::rel_path};
    use workspace::{
        CloseActiveItem, MoveItemToPaneInDirection, OpenOptions,
        item::{Item as _, SaveOptions},
    };

    use crate::{
        Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT, actions::MoveToEnd, editor_tests::init_test,
    };

    fn extract_color_inlays(editor: &Editor, cx: &gpui::App) -> Vec<Rgba> {
        editor
            .all_inlays(cx)
            .into_iter()
            .filter_map(|inlay| inlay.get_color())
            .map(Rgba::from)
            .collect()
    }

    #[gpui::test(iterations = 10)]
    async fn test_document_colors(cx: &mut TestAppContext) {
        let expected_color = Rgba {
            r: 0.33,
            g: 0.33,
            b: 0.33,
            a: 0.33,
        };

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/a"),
            json!({
                "first.rs": "fn main() { let a = 5; }",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| workspace::Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    color_provider: Some(lsp::ColorProviderCapability::Simple(true)),
                    ..lsp::ServerCapabilities::default()
                },
                name: "rust-analyzer",
                ..FakeLspAdapter::default()
            },
        );
        let mut fake_servers_without_capabilities = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    color_provider: Some(lsp::ColorProviderCapability::Simple(false)),
                    ..lsp::ServerCapabilities::default()
                },
                name: "not-rust-analyzer",
                ..FakeLspAdapter::default()
            },
        );

        let editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/a/first.rs")),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let fake_language_server = fake_servers.next().await.unwrap();
        let fake_language_server_without_capabilities =
            fake_servers_without_capabilities.next().await.unwrap();
        let requests_made = Arc::new(AtomicUsize::new(0));
        let closure_requests_made = Arc::clone(&requests_made);
        let mut color_request_handle = fake_language_server
            .set_request_handler::<lsp::request::DocumentColor, _, _>(move |params, _| {
                let requests_made = Arc::clone(&closure_requests_made);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Uri::from_file_path(path!("/a/first.rs")).unwrap()
                    );
                    requests_made.fetch_add(1, atomic::Ordering::Release);
                    Ok(vec![
                        lsp::ColorInformation {
                            range: lsp::Range {
                                start: lsp::Position {
                                    line: 0,
                                    character: 0,
                                },
                                end: lsp::Position {
                                    line: 0,
                                    character: 1,
                                },
                            },
                            color: lsp::Color {
                                red: 0.33,
                                green: 0.33,
                                blue: 0.33,
                                alpha: 0.33,
                            },
                        },
                        lsp::ColorInformation {
                            range: lsp::Range {
                                start: lsp::Position {
                                    line: 0,
                                    character: 0,
                                },
                                end: lsp::Position {
                                    line: 0,
                                    character: 1,
                                },
                            },
                            color: lsp::Color {
                                red: 0.33,
                                green: 0.33,
                                blue: 0.33,
                                alpha: 0.33,
                            },
                        },
                    ])
                }
            });

        let _handle = fake_language_server_without_capabilities
            .set_request_handler::<lsp::request::DocumentColor, _, _>(move |_, _| async move {
                panic!("Should not be called");
            });
        cx.executor().advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT);
        color_request_handle.next().await.unwrap();
        cx.run_until_parked();
        assert_eq!(
            1,
            requests_made.load(atomic::Ordering::Acquire),
            "Should query for colors once per editor open"
        );
        editor.update_in(cx, |editor, _, cx| {
            assert_eq!(
                vec![expected_color],
                extract_color_inlays(editor, cx),
                "Should have an initial inlay"
            );
        });

        // opening another file in a split should not influence the LSP query counter
        workspace
            .update(cx, |workspace, window, cx| {
                assert_eq!(
                    workspace.panes().len(),
                    1,
                    "Should have one pane with one editor"
                );
                workspace.move_item_to_pane_in_direction(
                    &MoveItemToPaneInDirection {
                        direction: workspace::SplitDirection::Right,
                        focus: false,
                        clone: true,
                    },
                    window,
                    cx,
                );
            })
            .unwrap();
        cx.run_until_parked();
        workspace
            .update(cx, |workspace, _, cx| {
                let panes = workspace.panes();
                assert_eq!(panes.len(), 2, "Should have two panes after splitting");
                for pane in panes {
                    let editor = pane
                        .read(cx)
                        .active_item()
                        .and_then(|item| item.downcast::<Editor>())
                        .expect("Should have opened an editor in each split");
                    let editor_file = editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .expect("test deals with singleton buffers")
                        .read(cx)
                        .file()
                        .expect("test buffese should have a file")
                        .path();
                    assert_eq!(
                        editor_file.as_ref(),
                        rel_path("first.rs"),
                        "Both editors should be opened for the same file"
                    )
                }
            })
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(500));
        let save = editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.handle_input("dirty", window, cx);
            editor.save(
                SaveOptions {
                    format: true,
                    autosave: true,
                },
                project.clone(),
                window,
                cx,
            )
        });
        save.await.unwrap();

        color_request_handle.next().await.unwrap();
        cx.run_until_parked();
        assert_eq!(
            2,
            requests_made.load(atomic::Ordering::Acquire),
            "Should query for colors once per save (deduplicated) and once per formatting after save"
        );

        drop(editor);
        let close = workspace
            .update(cx, |workspace, window, cx| {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.close_active_item(&CloseActiveItem::default(), window, cx)
                })
            })
            .unwrap();
        close.await.unwrap();
        let close = workspace
            .update(cx, |workspace, window, cx| {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.close_active_item(&CloseActiveItem::default(), window, cx)
                })
            })
            .unwrap();
        close.await.unwrap();
        assert_eq!(
            2,
            requests_made.load(atomic::Ordering::Acquire),
            "After saving and closing all editors, no extra requests should be made"
        );
        workspace
            .update(cx, |workspace, _, cx| {
                assert!(
                    workspace.active_item(cx).is_none(),
                    "Should close all editors"
                )
            })
            .unwrap();

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.navigate_backward(&workspace::GoBack, window, cx);
                })
            })
            .unwrap();
        cx.executor().advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT);
        cx.run_until_parked();
        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .expect("Should have reopened the editor again after navigating back")
                    .downcast::<Editor>()
                    .expect("Should be an editor")
            })
            .unwrap();

        assert_eq!(
            2,
            requests_made.load(atomic::Ordering::Acquire),
            "Cache should be reused on buffer close and reopen"
        );
        editor.update(cx, |editor, cx| {
            assert_eq!(
                vec![expected_color],
                extract_color_inlays(editor, cx),
                "Should have an initial inlay"
            );
        });

        drop(color_request_handle);
        let closure_requests_made = Arc::clone(&requests_made);
        let mut empty_color_request_handle = fake_language_server
            .set_request_handler::<lsp::request::DocumentColor, _, _>(move |params, _| {
                let requests_made = Arc::clone(&closure_requests_made);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Uri::from_file_path(path!("/a/first.rs")).unwrap()
                    );
                    requests_made.fetch_add(1, atomic::Ordering::Release);
                    Ok(Vec::new())
                }
            });
        let save = editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.handle_input("dirty_again", window, cx);
            editor.save(
                SaveOptions {
                    format: false,
                    autosave: true,
                },
                project.clone(),
                window,
                cx,
            )
        });
        save.await.unwrap();

        cx.executor().advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT);
        empty_color_request_handle.next().await.unwrap();
        cx.run_until_parked();
        assert_eq!(
            3,
            requests_made.load(atomic::Ordering::Acquire),
            "Should query for colors once per save only, as formatting was not requested"
        );
        editor.update(cx, |editor, cx| {
            assert_eq!(
                Vec::<Rgba>::new(),
                extract_color_inlays(editor, cx),
                "Should clear all colors when the server returns an empty response"
            );
        });
    }
}
