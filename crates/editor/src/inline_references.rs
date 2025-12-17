use super::*;

const INLINE_REFERENCES_HEADER_LINES: u32 = 2;
const INLINE_REFERENCES_MIN_BODY_LINES: u32 = 6;
const INLINE_REFERENCES_MAX_BODY_LINES: u32 = 18;

struct InlineReferencesHighlight;

struct InlineReferencesAddon {
    block_id: CustomBlockId,
    references_editor: Entity<Editor>,
    _subscriptions: Vec<Subscription>,
}

impl Addon for InlineReferencesAddon {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

struct InlineReferencesData {
    host_editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    references_editor: Entity<Editor>,
    locations: Arc<Vec<(Entity<Buffer>, Vec<Range<Point>>)>>,
    title: SharedString,
    summary: SharedString,
    body_rows: u32,
}

impl InlineReferencesData {
    fn close_inline_view(&self, app: &mut App) {
        if let Some(editor) = self.host_editor.upgrade() {
            let _ = editor.update(app, |editor, cx| {
                editor.close_inline_references(cx);
            });
        }
    }

    fn open_in_tab(&self, window: &mut Window, app: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let locations = self.locations_as_map();
        let title = self.title.to_string();
        let _ = workspace.update(app, |workspace, cx| {
            let allow_preview =
                PreviewTabsSettings::get_global(cx).enable_preview_multibuffer_from_code_navigation;
            Editor::open_locations_in_multibuffer(
                workspace,
                locations,
                title,
                false,
                allow_preview,
                MultibufferSelectionMode::First,
                window,
                cx,
            );
        });
    }

    fn locations_as_map(&self) -> std::collections::HashMap<Entity<Buffer>, Vec<Range<Point>>> {
        self.locations.iter().cloned().collect()
    }
}

fn inline_references_render_block(data: Arc<InlineReferencesData>) -> RenderBlock {
    Arc::new(move |cx: &mut BlockContext| {
        let right_margin = cx.margins.right;
        let content_height = cx.line_height * data.body_rows as f32;

        let actions = h_flex()
            .gap_2()
            .child(
                Button::new("inline_refs_open_tab", "Open as Tab")
                    .style(ButtonStyle::Transparent)
                    .size(ButtonSize::Compact)
                    .on_click({
                        let data = data.clone();
                        move |_, window, cx| data.open_in_tab(window, cx)
                    }),
            )
            .child(
                IconButton::new("inline_refs_close", IconName::Close)
                    .icon_size(IconSize::Small)
                    .size(ButtonSize::Compact)
                    .shape(IconButtonShape::Square)
                    .tooltip(Tooltip::text("Close references"))
                    .on_click({
                        let data = data.clone();
                        move |_, _, cx| data.close_inline_view(cx)
                    }),
            );

        let header = h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .gap_2()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(data.title.clone())
                            .color(Color::Default)
                            .line_height_style(LineHeightStyle::UiLabel),
                    )
                    .child(
                        Label::new(data.summary.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .line_height_style(LineHeightStyle::UiLabel),
                    ),
            )
            .child(actions);

        h_flex()
            .w_full()
            .id(cx.block_id)
            .occlude()
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .flex_1()
                    .mr(right_margin)
                    .py(cx.line_height / 2.)
                    .px_2()
                    .gap_2()
                    .child(header)
                    .child(
                        div()
                            .w_full()
                            .h(content_height)
                            .border_t_1()
                            .border_color(cx.theme().colors().border)
                            .child(data.references_editor.clone()),
                    ),
            )
            .into_any_element()
    })
}

impl Editor {
    pub(crate) fn show_inline_references(
        &mut self,
        selection: Range<Anchor>,
        locations: std::collections::HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        title: String,
        workspace: Entity<Workspace>,
        num_locations: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remove_inline_references(cx);
        let host_editor = cx.entity().downgrade();
        let selection_display_row = {
            let snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            selection.end.to_display_point(&snapshot).row()
        };

        let stored_locations: Arc<Vec<_>> = Arc::new(
            locations
                .into_iter()
                .filter(|(_, ranges)| !ranges.is_empty())
                .collect(),
        );
        if stored_locations.is_empty() {
            return;
        }

        let project = workspace.read(cx).project().clone();
        let capability = project.read(cx).capability();
        let title_for_buffer = title.clone();
        let anchor_ranges = Rc::new(RefCell::new(Vec::new()));
        let anchor_ranges_for_buffer = anchor_ranges.clone();
        let locations_for_buffer = stored_locations.clone();
        let excerpt_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(capability);
            for (buffer, ranges_for_buffer) in locations_for_buffer.iter() {
                let (new_ranges, _) = multibuffer.set_excerpts_for_path(
                    PathKey::for_buffer(buffer, cx),
                    buffer.clone(),
                    ranges_for_buffer.clone(),
                    multibuffer_context_lines(cx),
                    cx,
                );
                anchor_ranges_for_buffer.borrow_mut().extend(new_ranges);
            }
            multibuffer.with_title(title_for_buffer.clone())
        });
        let anchor_ranges = Rc::try_unwrap(anchor_ranges)
            .unwrap_or_else(|_| RefCell::new(Vec::new()))
            .into_inner();

        let references_editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(excerpt_buffer, Some(project.clone()), window, cx);
            editor.disable_scrollbars_and_minimap(window, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor
        });

        let inline_editor_handle = references_editor.downgrade();
        let host_editor_for_inline = host_editor.clone();
        let save_subscription = references_editor.update(cx, |references_editor, _cx| {
            references_editor.set_embedded_workspace(workspace.clone());
            references_editor.inline_references_host = Some(host_editor_for_inline.clone());
            let editor_handle = inline_editor_handle.clone();
            references_editor.register_action(move |action: &workspace::Save, window, cx| {
                let editor_handle = editor_handle.clone();
                let intent = action.save_intent.unwrap_or(SaveIntent::Save);
                let should_handle =
                    matches!(intent, SaveIntent::Save | SaveIntent::SaveWithoutFormat);

                if !should_handle {
                    cx.propagate();
                    return;
                }

                let handled = editor_handle
                    .upgrade()
                    .and_then(|editor| {
                        editor.update(cx, |editor, cx| {
                            editor.project().cloned().map(|project| {
                                let options = SaveOptions {
                                    format: intent != SaveIntent::SaveWithoutFormat,
                                    autosave: false,
                                };

                                editor
                                    .save(options, project, window, cx)
                                    .detach_and_log_err(cx);
                            })
                        })
                    })
                    .is_some();

                if handled {
                    cx.stop_propagation();
                } else {
                    cx.propagate();
                }
            })
        });

        references_editor.update(cx, |references_editor, cx| {
            if let Some(first_range) = anchor_ranges.first() {
                references_editor.change_selections(
                    SelectionEffects::no_scroll(),
                    window,
                    cx,
                    |selections| {
                        selections.clear_disjoint();
                        selections.select_anchor_ranges(std::iter::once(first_range.clone()));
                    },
                );
            }
            references_editor.highlight_background::<InlineReferencesHighlight>(
                &anchor_ranges,
                |_, theme| theme.colors().editor_highlighted_line_background,
                cx,
            );
        });

        let mut body_rows = references_editor.update(cx, |references_editor, cx| {
            references_editor.max_point(cx).row().0 + 1
        });
        body_rows = body_rows.clamp(
            INLINE_REFERENCES_MIN_BODY_LINES,
            INLINE_REFERENCES_MAX_BODY_LINES,
        );
        let total_block_lines = INLINE_REFERENCES_HEADER_LINES + body_rows;

        self.ensure_inline_references_space(selection_display_row, total_block_lines, window, cx);

        let summary = {
            let file_count = stored_locations.len();
            let reference_label = if num_locations == 1 {
                "reference"
            } else {
                "references"
            };
            let file_label = if file_count == 1 { "file" } else { "files" };
            format!("{num_locations} {reference_label} in {file_count} {file_label}")
        };

        let data = Arc::new(InlineReferencesData {
            host_editor,
            workspace: workspace.downgrade(),
            references_editor: references_editor.clone(),
            locations: stored_locations,
            title: SharedString::from(title),
            summary: SharedString::from(summary),
            body_rows,
        });

        let render_block = inline_references_render_block(data);
        let block_id = self.insert_blocks(
            [BlockProperties {
                placement: BlockPlacement::Below(selection.end),
                height: Some(total_block_lines),
                style: BlockStyle::Sticky,
                render: render_block,
                priority: 0,
            }],
            None,
            cx,
        )[0];

        self.register_addon(InlineReferencesAddon {
            block_id,
            references_editor: references_editor.clone(),
            _subscriptions: vec![save_subscription],
        });
    }

    pub(crate) fn remove_inline_references(&mut self, cx: &mut Context<Self>) {
        let Some(block_id) = self
            .addon::<InlineReferencesAddon>()
            .map(|addon| addon.block_id)
        else {
            return;
        };

        let mut block_ids = HashSet::default();
        block_ids.insert(block_id);
        self.remove_blocks(block_ids, None, cx);
        self.unregister_addon::<InlineReferencesAddon>();
    }

    pub fn close_inline_references(&mut self, cx: &mut Context<Self>) -> bool {
        if self.addon::<InlineReferencesAddon>().is_some() {
            self.remove_inline_references(cx);
            true
        } else if let Some(host) = self.inline_references_host.take()
            && let Some(host) = host.upgrade()
        {
            let _ = host.update(cx, |host, cx| host.remove_inline_references(cx));
            true
        } else {
            false
        }
    }

    fn ensure_inline_references_space(
        &mut self,
        selection_row: DisplayRow,
        block_lines: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(visible_lines) = self.visible_line_count() else {
            return;
        };
        if visible_lines <= 0.0 {
            return;
        }

        // Account for the selection line itself + the inline references block.
        const INLINE_REFERENCES_SCROLL_PADDING: f64 = 2.0;
        let block_bottom =
            selection_row.0 as f64 + INLINE_REFERENCES_SCROLL_PADDING + block_lines as f64;
        let mut scroll_position = self.scroll_position(cx);
        let bottom_visible = scroll_position.y + visible_lines;
        if block_bottom > bottom_visible {
            scroll_position.y = (block_bottom - visible_lines).max(0.0);
            self.set_scroll_position(scroll_position, window, cx);
        }
    }

    pub(crate) fn inline_references_editor(&self) -> Option<Entity<Editor>> {
        self.addon::<InlineReferencesAddon>()
            .map(|addon| addon.references_editor.clone())
    }

    pub(crate) fn try_handle_inline_references_save(
        &mut self,
        options: &SaveOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if options.autosave {
            return None;
        }

        let inline_editor = self.inline_references_editor()?;
        let format = options.format;
        let autosave = options.autosave;

        inline_editor.update(cx, |inline_editor, cx| {
            if !inline_editor.is_focused(window) {
                None
            } else {
                inline_editor.project().cloned().map(|project| {
                    inline_editor.save(SaveOptions { format, autosave }, project, window, cx)
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor_tests::init_test;
    use gpui::{TestAppContext, VisualTestContext, WindowHandle, point};
    use indoc::indoc;
    use language::Point;
    use project::{FakeFs, Project};
    use serde_json::json;
    use std::{collections::HashMap, ops::Range, path::PathBuf};
    use util::path;
    use workspace::{OpenOptions, Workspace, item::SaveOptions};

    struct InlineReferencesTestHarness {
        cx: VisualTestContext,
        _workspace: WindowHandle<Workspace>,
        workspace_entity: Entity<Workspace>,
        editor: Entity<Editor>,
    }

    impl InlineReferencesTestHarness {
        async fn new(cx: &mut TestAppContext, file_text: &str) -> Self {
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                path!("/inline_refs_project"),
                json!({ "main.rs": file_text }),
            )
            .await;
            let project = Project::test(fs, [path!("/inline_refs_project").as_ref()], cx).await;
            let workspace =
                cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
            let mut visual_cx = VisualTestContext::from_window(*workspace, cx);
            let workspace_entity = workspace.root(&mut visual_cx).unwrap();
            let editor = workspace
                .update(&mut visual_cx, |workspace, window, cx| {
                    workspace.open_abs_path(
                        PathBuf::from(path!("/inline_refs_project/main.rs")),
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

            Self {
                cx: visual_cx,
                _workspace: workspace,
                workspace_entity,
                editor,
            }
        }

        fn open_inline_references(
            &mut self,
            ranges: Vec<Range<Point>>,
            title: &str,
        ) -> Entity<Editor> {
            self.editor.update_in(&mut self.cx, |editor, window, cx| {
                editor.set_visible_line_count(3.0, window, cx);
                editor.set_scroll_position(point(0., 0.), window, cx);
                let selection = editor.selections.newest_anchor().range();
                let buffer = editor
                    .buffer()
                    .read(cx)
                    .all_buffers()
                    .into_iter()
                    .next()
                    .unwrap();
                let num_locations = ranges.len();
                let mut locations = HashMap::new();
                locations.insert(buffer, ranges);
                editor.show_inline_references(
                    selection,
                    locations,
                    title.to_string(),
                    self.workspace_entity.clone(),
                    num_locations,
                    window,
                    cx,
                );
                editor
                    .inline_references_editor()
                    .expect("inline references editor is created")
            })
        }
    }

    #[gpui::test]
    async fn inline_references_not_created_for_empty_locations(app: &mut TestAppContext) {
        init_test(app, |_| {});

        let mut harness = InlineReferencesTestHarness::new(app, "fn main() {}\n").await;
        let workspace = harness.workspace_entity.clone();
        harness
            .editor
            .update_in(&mut harness.cx, |editor, window, cx| {
                editor.set_visible_line_count(3.0, window, cx);
                let selection = editor.selections.newest_anchor().range();
                let buffer = editor
                    .buffer()
                    .read(cx)
                    .all_buffers()
                    .into_iter()
                    .next()
                    .unwrap();
                let mut locations = HashMap::new();
                locations.insert(buffer, Vec::new());
                editor.show_inline_references(
                    selection,
                    locations,
                    "References".into(),
                    workspace.clone(),
                    0,
                    window,
                    cx,
                );
                assert!(editor.inline_references_editor().is_none());
            });
    }

    #[gpui::test]
    async fn inline_references_render_and_close(app: &mut TestAppContext) {
        init_test(app, |_| {});

        let mut harness = InlineReferencesTestHarness::new(
            app,
            indoc! {"
                fn main() {
                    call_target();
                }

                fn helper() {
                    call_target();
                }
            "},
        )
        .await;

        let inline_editor = harness
            .open_inline_references(vec![Point::new(1, 4)..Point::new(1, 16)], "call_target");

        let scroll_y = harness
            .editor
            .update_in(&mut harness.cx, |editor, _, editor_cx| {
                editor.scroll_position(editor_cx).y
            });
        assert!(
            scroll_y > 0.0,
            "inline references block should reserve space by scrolling"
        );

        inline_editor.update_in(&mut harness.cx, |inline_editor, _, cx| {
            assert!(
                inline_editor.inline_references_host.is_some(),
                "inline editor should know its host"
            );
            let text = inline_editor.display_text(cx);
            assert!(
                text.contains("call_target()"),
                "inline references excerpt should include referenced text"
            );
        });

        harness
            .editor
            .update_in(&mut harness.cx, |editor, _, editor_cx| {
                assert!(editor.close_inline_references(editor_cx));
                assert!(editor.inline_references_editor().is_none());
            });
    }

    #[gpui::test]
    async fn inline_references_can_be_closed_from_inline_editor(app: &mut TestAppContext) {
        init_test(app, |_| {});

        let mut harness = InlineReferencesTestHarness::new(
            app,
            indoc! {"
                fn alpha() {
                    call_beta();
                }
            "},
        )
        .await;

        let inline_editor =
            harness.open_inline_references(vec![Point::new(1, 4)..Point::new(1, 14)], "call_beta");

        inline_editor.update_in(&mut harness.cx, |inline_editor, _, cx| {
            assert!(
                inline_editor.close_inline_references(cx),
                "closing from the inline editor should notify the host"
            );
        });

        harness.editor.update_in(&mut harness.cx, |editor, _, _cx| {
            assert!(
                editor.inline_references_editor().is_none(),
                "host editor should remove inline references when child closes"
            );
        });
    }

    #[gpui::test]
    async fn inline_references_save_forwards_focus(app: &mut TestAppContext) {
        init_test(app, |_| {});

        let mut harness = InlineReferencesTestHarness::new(
            app,
            indoc! {"
                fn entry() {
                    referenced();
                }
            "},
        )
        .await;
        let workspace = harness.workspace_entity.clone();

        harness
            .editor
            .update_in(&mut harness.cx, |editor, window, cx| {
                let selection = editor.selections.newest_anchor().range();
                let buffer = editor
                    .buffer()
                    .read(cx)
                    .all_buffers()
                    .into_iter()
                    .next()
                    .unwrap();
                let mut locations = HashMap::new();
                locations.insert(buffer, vec![Point::new(1, 4)..Point::new(1, 15)]);
                editor.show_inline_references(
                    selection,
                    locations,
                    "referenced".to_string(),
                    workspace.clone(),
                    1,
                    window,
                    cx,
                );
                let inline_editor = editor
                    .inline_references_editor()
                    .expect("inline references editor should exist");
                assert!(
                    editor
                        .try_handle_inline_references_save(
                            &SaveOptions {
                                format: true,
                                autosave: true
                            },
                            window,
                            cx
                        )
                        .is_none(),
                    "autosave should never trigger inline reference saves"
                );
                assert!(
                    editor
                        .try_handle_inline_references_save(
                            &SaveOptions {
                                format: true,
                                autosave: false
                            },
                            window,
                            cx
                        )
                        .is_none(),
                    "unfocused inline editor should not handle save"
                );
                inline_editor.update(cx, |inline_editor, cx| {
                    window.focus(&inline_editor.focus_handle(cx));
                });
                let task = editor
                    .try_handle_inline_references_save(
                        &SaveOptions {
                            format: false,
                            autosave: false,
                        },
                        window,
                        cx,
                    )
                    .expect("focused inline editor should intercept save");
                task.detach();
            });
    }
}
