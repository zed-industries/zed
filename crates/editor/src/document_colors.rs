//! Colors shown inline in the editor, from two sources: `textDocument/documentColor`
//! for language servers that implement it, and each language's `colors.scm`
//! tree-sitter query for everything else (Bevy's `Color::srgb(..)`, hex literals,
//! and so on).
//!
//! Both sources feed the same inlay pipeline, so a swatch looks and behaves the
//! same however the color was found. Clicking one opens the color picker; see
//! [`crate::color_picker`].

use std::{cmp, ops::Range};

use collections::HashMap;
use futures::future::join_all;
use gpui::{AppContext as _, Hsla, Rgba};
use itertools::Itertools;
use language::point_from_lsp;
use multi_buffer::Anchor;
use project::InlayId;
use settings::Settings as _;
use text::{Bias, BufferId};
use ui::{App, Context, Window};
use util::post_inc;

use crate::{
    DisplayPoint, Editor, EditorSettings, EditorSnapshot, InlaySplice,
    LSP_REQUEST_DEBOUNCE_TIMEOUT, RangeToAnchorExt, editor_settings::DocumentColorsRenderMode,
    inlays::Inlay,
};

/// A color found in a buffer, in the same shape a language server reports.
/// Where it came from does not matter once it is on screen: the picker works
/// out how to rewrite it from the text at its range.
fn rgba(color: lsp::Color) -> Rgba {
    Rgba {
        r: color.red,
        g: color.green,
        b: color.blue,
        a: color.alpha,
    }
}

#[derive(Debug)]
pub(super) struct DocumentColorsData {
    buffer_colors: HashMap<BufferId, BufferColors>,
    pub(super) render_mode: DocumentColorsRenderMode,
}

#[derive(Debug, Default)]
struct BufferColors {
    colors: Vec<(Range<Anchor>, lsp::Color, InlayId)>,
    inlay_colors: HashMap<InlayId, usize>,
}

impl DocumentColorsData {
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
                    .values()
                    .flat_map(|buffer_colors| buffer_colors.colors.iter())
                    .map(|(range, color, id)| {
                        Inlay::color(id.id(), range.start, rgba(*color))
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
        colors: Vec<(Range<Anchor>, lsp::Color, InlayId)>,
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

    /// The color whose swatch owns `inlay_id`, if any.
    pub fn color_for_inlay(&self, inlay_id: InlayId) -> Option<(Range<Anchor>, lsp::Color)> {
        self.buffer_colors.values().find_map(|buffer_colors| {
            let index = *buffer_colors.inlay_colors.get(&inlay_id)?;
            let (range, color, _) = buffer_colors.colors.get(index)?;
            Some((range.clone(), *color))
        })
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
                .values()
                .flat_map(|buffer_colors| &buffer_colors.colors)
                .map(|(range, color, _)| {
                    let display_range = range.clone().to_display_points(snapshot);
                    (display_range, Hsla::from(rgba(*color)))
                })
                .collect()
        };
        (render_mode, highlights)
    }
}

/// Insert a color into the per-buffer list, keeping it sorted by range.
///
/// A language server and the syntax query can report the same color; whichever
/// is inserted first wins, and language server colors are inserted first
/// because `textDocument/colorPresentation` also knows how to rewrite them.
fn insert_color(
    editor_colors: &mut HashMap<BufferId, Vec<(Range<Anchor>, lsp::Color)>>,
    multi_buffer_snapshot: &multi_buffer::MultiBufferSnapshot,
    buffer_id: BufferId,
    range: Range<Anchor>,
    color: lsp::Color,
) {
    let buffer_colors = editor_colors.entry(buffer_id).or_default();
    let (Ok(i) | Err(i)) = buffer_colors.binary_search_by(|(probe, _)| {
        probe
            .start
            .cmp(&range.start, multi_buffer_snapshot)
            .then_with(|| probe.end.cmp(&range.end, multi_buffer_snapshot))
    });
    if buffer_colors
        .get(i)
        .is_some_and(|(existing, _)| *existing == range)
    {
        return;
    }
    buffer_colors.insert(i, (range, color));
}

impl Editor {
    pub(super) fn refresh_document_colors(
        &mut self,
        buffer_id: Option<BufferId>,
        _: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.lsp_data_enabled() {
            return;
        }
        if self
            .colors
            .as_ref()
            .is_none_or(|colors| colors.render_mode == DocumentColorsRenderMode::None)
        {
            return;
        }

        let buffers = self
            .visible_buffers(cx)
            .into_iter()
            .chain(buffer_id.and_then(|buffer_id| self.buffer.read(cx).buffer(buffer_id)))
            .filter(|editor_buffer| {
                buffer_id.is_none_or(|buffer_id| buffer_id == editor_buffer.read(cx).remote_id())
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        // Only buffers that a language server has been told about can answer a
        // document color request; the syntax query works on any of them.
        let lsp_buffers = self
            .project
            .as_ref()
            .map(|_| {
                buffers
                    .iter()
                    .filter(|buffer| {
                        let buffer = buffer.read(cx);
                        self.is_lsp_relevant(buffer.file(), cx)
                            && self.registered_buffers.contains_key(&buffer.remote_id())
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let project = self.project.as_ref().map(|project| project.downgrade());

        self.refresh_colors_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                .await;

            // Snapshot after the debounce, so that the buffers have been parsed
            // and reflect the edit that triggered this refresh.
            let syntax_snapshots = cx.update(|cx| {
                buffers
                    .iter()
                    .map(|buffer| buffer.read(cx).snapshot())
                    .collect::<Vec<_>>()
            });
            let syntax_colors = cx
                .background_spawn(async move {
                    syntax_snapshots
                        .into_iter()
                        .map(|snapshot| {
                            let colors = snapshot
                                .color_matches(0..snapshot.len())
                                .map(|color| (color.range, color.color))
                                .collect::<Vec<_>>();
                            (snapshot, colors)
                        })
                        .collect::<Vec<_>>()
                })
                .await;

            let lsp_colors = match project {
                Some(project) => {
                    let Some(all_colors_task) = project
                        .update(cx, |project, cx| {
                            project.lsp_store().update(cx, |lsp_store, cx| {
                                lsp_buffers
                                    .into_iter()
                                    .filter_map(|buffer| {
                                        let buffer_snapshot = buffer.read(cx).snapshot();
                                        let colors_task = lsp_store.document_colors(buffer, cx)?;
                                        Some(async move { (buffer_snapshot, colors_task.await) })
                                    })
                                    .collect::<Vec<_>>()
                            })
                        })
                        .ok()
                    else {
                        return;
                    };
                    join_all(all_colors_task).await
                }
                None => Vec::new(),
            };

            let Some(multi_buffer_snapshot) = editor
                .update(cx, |editor, cx| editor.buffer.read(cx).snapshot(cx))
                .ok()
            else {
                return;
            };

            let mut new_editor_colors: HashMap<BufferId, Vec<(Range<Anchor>, lsp::Color)>> =
                HashMap::default();
            for (buffer_snapshot, colors) in lsp_colors {
                match colors {
                    Ok(colors) => {
                        if colors.colors.is_empty() {
                            new_editor_colors
                                .entry(buffer_snapshot.remote_id())
                                .or_default()
                                .clear();
                        } else {
                            for color in colors.colors {
                                let color_start = point_from_lsp(color.lsp_range.start);
                                let color_end = point_from_lsp(color.lsp_range.end);

                                let Some(range) = multi_buffer_snapshot
                                    .buffer_anchor_range_to_anchor_range(
                                        buffer_snapshot.anchor_range_outside(
                                            buffer_snapshot
                                                .clip_point_utf16(color_start, Bias::Left)
                                                ..buffer_snapshot
                                                    .clip_point_utf16(color_end, Bias::Right),
                                        ),
                                    )
                                else {
                                    continue;
                                };

                                insert_color(
                                    &mut new_editor_colors,
                                    &multi_buffer_snapshot,
                                    buffer_snapshot.remote_id(),
                                    range,
                                    color.color,
                                );
                            }
                        }
                    }
                    Err(e) => log::error!("Failed to retrieve document colors: {e}"),
                }
            }

            for (buffer_snapshot, colors) in syntax_colors {
                for (color_range, color) in colors {
                    let Some(range) = multi_buffer_snapshot.buffer_anchor_range_to_anchor_range(
                        buffer_snapshot.anchor_range_outside(color_range),
                    ) else {
                        continue;
                    };
                    insert_color(
                        &mut new_editor_colors,
                        &multi_buffer_snapshot,
                        buffer_snapshot.remote_id(),
                        range,
                        color,
                    );
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
                            let rgba_color = rgba(new_color);

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
    use gpui::{Rgba, TestAppContext};
    use language::FakeLspAdapter;
    use languages::rust_lang;
    use project::{FakeFs, Project};
    use serde_json::json;
    use util::{path, rel_path::rel_path};
    use workspace::{
        CloseActiveItem, MoveItemToPaneInDirection, MultiWorkspace, OpenOptions,
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

    #[gpui::test]
    async fn test_syntax_colors_without_a_language_server(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/a"),
            json!({
                "first.rs": "fn draw() { gizmos.line(a, b, Color::srgb(0.2, 0.9, 0.4)); }",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/a/first.rs")),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Parsing the buffer schedules a colors refresh, and each refresh
        // replaces the one before it, so the clock has to be advanced until no
        // further refresh is queued.
        for _ in 0..3 {
            cx.executor().advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT);
            cx.run_until_parked();
        }

        editor.update(cx, |editor, cx| {
            let inlays = extract_color_inlays(editor, cx);
            assert_eq!(
                inlays.len(),
                1,
                "The colors query should produce a swatch with no language server involved, got {inlays:?}"
            );
            // The swatch round-trips through HSLA, so compare the 8-bit values.
            let byte = |value: f32| (value * 255.0).round() as u8;
            assert_eq!(
                [
                    byte(inlays[0].r),
                    byte(inlays[0].g),
                    byte(inlays[0].b),
                    byte(inlays[0].a)
                ],
                [51, 230, 102, 255]
            );
        });

        // Picking a new color rewrites the constructor's arguments in place.
        let range = editor.update(cx, |editor, _| {
            editor
                .colors
                .as_ref()
                .expect("colors are enabled")
                .buffer_colors
                .values()
                .flat_map(|buffer_colors| buffer_colors.colors.iter())
                .map(|(range, _, _)| range.clone())
                .next()
                .expect("a color was found")
        });
        editor.update(cx, |editor, cx| {
            editor.rewrite_color(
                &range,
                lsp::Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.0,
                    alpha: 1.0,
                },
                cx,
            );
            assert_eq!(
                editor.text(cx),
                "fn draw() { gizmos.line(a, b, Color::srgb(1.0, 0.5, 0.0)); }"
            );
        });
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
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

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
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/a/first.rs")),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
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
        workspace.update_in(cx, |workspace, window, cx| {
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
        });
        cx.run_until_parked();
        workspace.update_in(cx, |workspace, _, cx| {
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
        });

        cx.executor().advance_clock(Duration::from_millis(500));
        let save = editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.handle_input("dirty", window, cx);
            editor.save(
                SaveOptions {
                    format: true,
                    force_format: false,
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
        let close = workspace.update_in(cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.close_active_item(&CloseActiveItem::default(), window, cx)
            })
        });
        close.await.unwrap();
        let close = workspace.update_in(cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.close_active_item(&CloseActiveItem::default(), window, cx)
            })
        });
        close.await.unwrap();
        assert_eq!(
            2,
            requests_made.load(atomic::Ordering::Acquire),
            "After saving and closing all editors, no extra requests should be made"
        );
        workspace.update_in(cx, |workspace, _, cx| {
            assert!(
                workspace.active_item(cx).is_none(),
                "Should close all editors"
            )
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.navigate_backward(&workspace::GoBack, window, cx);
            })
        });
        cx.executor().advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT);
        cx.run_until_parked();
        let editor = workspace.update_in(cx, |workspace, _, cx| {
            workspace
                .active_item(cx)
                .expect("Should have reopened the editor again after navigating back")
                .downcast::<Editor>()
                .expect("Should be an editor")
        });

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
                    force_format: false,
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
