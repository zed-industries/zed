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
