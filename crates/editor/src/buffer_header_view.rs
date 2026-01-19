use collections::HashMap;
use gpui::{
    Action, AnyElement, App, Bounds, ClickEvent, ClipboardItem, Element, ElementId, Entity,
    Focusable, GlobalElementId, IntoElement, LayoutId, Length, MouseButton, Pixels, SharedString,
    Window, linear_color_stop, linear_gradient, size, AvailableSpace,
};
use multi_buffer::{Anchor, ExcerptInfo};
use settings::Settings;
use std::path::{self, Path};
use text::BufferId;
use ui::{
    Button, ButtonLike, ButtonStyle, ContextMenu, Icon, IconName, Indicator, KeyBinding, Tooltip,
    prelude::*, right_click_menu, text_for_keystroke,
};

use crate::{
    DisplayRow, Editor, EditorSnapshot, FILE_HEADER_HEIGHT, JumpData,
    MULTI_BUFFER_EXCERPT_HEADER_HEIGHT, OpenExcerpts, ToggleFold, ToggleFoldAll,
    element::header_jump_data,
    scroll::ScrollPixelOffset,
};
use file_icons::FileIcons;
use git::status::FileStatus;
use project::Entry;
use workspace::{ItemSettings, OpenInTerminal, OpenTerminal, RevealInProjectPanel};

pub struct BufferHeadersView {
    editor: Entity<Editor>,
    snapshot: EditorSnapshot,
    line_height: Pixels,
    scroll_position: gpui::Point<crate::scroll::ScrollOffset>,
    visible_row_range: std::ops::Range<DisplayRow>,
    hitbox_origin: gpui::Point<Pixels>,
    hitbox_width: Pixels,
    right_margin: Pixels,
    selected_buffer_ids: Vec<BufferId>,
    latest_selection_anchors: HashMap<BufferId, Anchor>,
}

impl BufferHeadersView {
    pub fn new(
        editor: Entity<Editor>,
        snapshot: EditorSnapshot,
        line_height: Pixels,
        scroll_position: gpui::Point<crate::scroll::ScrollOffset>,
        visible_row_range: std::ops::Range<DisplayRow>,
        hitbox_origin: gpui::Point<Pixels>,
        hitbox_width: Pixels,
        right_margin: Pixels,
        selected_buffer_ids: Vec<BufferId>,
        latest_selection_anchors: HashMap<BufferId, Anchor>,
    ) -> Self {
        Self {
            editor,
            snapshot,
            line_height,
            scroll_position,
            visible_row_range,
            hitbox_origin,
            hitbox_width,
            right_margin,
            selected_buffer_ids,
            latest_selection_anchors,
        }
    }

    fn render_buffer_header(
        editor: &Entity<Editor>,
        excerpt_info: &ExcerptInfo,
        is_folded: bool,
        is_selected: bool,
        is_sticky: bool,
        jump_data: JumpData,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let editor_read = editor.read(cx);
        let multi_buffer = editor_read.buffer.read(cx);
        let is_read_only = editor_read.read_only(cx);

        let file_status = multi_buffer
            .all_diff_hunks_expanded()
            .then(|| editor_read.status_for_buffer_id(excerpt_info.buffer_id, cx))
            .flatten();
        let indicator = multi_buffer
            .buffer(excerpt_info.buffer_id)
            .and_then(|buffer| {
                let buffer = buffer.read(cx);
                let indicator_color = match (buffer.has_conflict(), buffer.is_dirty()) {
                    (true, _) => Some(Color::Warning),
                    (_, true) => Some(Color::Accent),
                    (false, false) => None,
                };
                indicator_color.map(|indicator_color| Indicator::dot().color(indicator_color))
            });

        let include_root = editor_read
            .project
            .as_ref()
            .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
            .unwrap_or_default();
        let file = excerpt_info.buffer.file();
        let can_open_excerpts = Editor::can_open_excerpts_in_file(file);
        let path_style = file.map(|file| file.path_style(cx));
        let relative_path = excerpt_info.buffer.resolve_file_path(include_root, cx);
        let (parent_path, filename) = if let Some(path) = &relative_path {
            if let Some(path_style) = path_style {
                let (dir, file_name) = path_style.split(path);
                (dir.map(|dir| dir.to_owned()), Some(file_name.to_owned()))
            } else {
                (None, Some(path.clone()))
            }
        } else {
            (None, None)
        };
        let focus_handle = editor_read.focus_handle(cx);
        let colors = cx.theme().colors();

        let buffer_id = excerpt_info.buffer_id;
        let editor_handle = editor.clone();
        let buffer_capability = excerpt_info.buffer.capability;

        let header = div()
            .p_1()
            .w_full()
            .h(FILE_HEADER_HEIGHT as f32 * line_height)
            .child(
                h_flex()
                    .size_full()
                    .flex_basis(Length::Definite(DefiniteLength::Fraction(0.667)))
                    .pl_1()
                    .pr_2()
                    .rounded_sm()
                    .gap_1p5()
                    .when(is_sticky, |el| el.shadow_md())
                    .border_1()
                    .map(|border| {
                        let border_color = if is_selected
                            && is_folded
                            && focus_handle.contains_focused(window, cx)
                        {
                            colors.border_focused
                        } else {
                            colors.border
                        };
                        border.border_color(border_color)
                    })
                    .bg(colors.editor_subheader_background)
                    .hover(|style| style.bg(colors.element_hover))
                    .map(|header| {
                        let editor = editor_handle.clone();
                        let toggle_chevron_icon =
                            FileIcons::get_chevron_icon(!is_folded, cx).map(Icon::from_path);
                        let button_size = rems_from_px(28.);

                        header.child(
                            div()
                                .hover(|style| style.bg(colors.element_selected))
                                .rounded_xs()
                                .child(
                                    ButtonLike::new("toggle-buffer-fold")
                                        .style(ButtonStyle::Transparent)
                                        .height(button_size.into())
                                        .width(button_size)
                                        .children(toggle_chevron_icon)
                                        .tooltip({
                                            let focus_handle = focus_handle.clone();
                                            let is_folded_for_tooltip = is_folded;
                                            move |_window, cx| {
                                                Tooltip::with_meta_in(
                                                    if is_folded_for_tooltip {
                                                        "Unfold Excerpt"
                                                    } else {
                                                        "Fold Excerpt"
                                                    },
                                                    Some(&ToggleFold),
                                                    format!(
                                                        "{} to toggle all",
                                                        text_for_keystroke(
                                                            &gpui::Modifiers::alt(),
                                                            "click",
                                                            cx,
                                                        )
                                                    ),
                                                    &focus_handle,
                                                    cx,
                                                )
                                            }
                                        })
                                        .on_click(move |event, window, cx| {
                                            if event.modifiers().alt {
                                                editor.update(cx, |editor, cx| {
                                                    editor.toggle_fold_all(
                                                        &ToggleFoldAll,
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            } else if is_folded {
                                                editor.update(cx, |editor, cx| {
                                                    editor.unfold_buffer(buffer_id, cx);
                                                });
                                            } else {
                                                editor.update(cx, |editor, cx| {
                                                    editor.fold_buffer(buffer_id, cx);
                                                });
                                            }
                                        }),
                                ),
                        )
                    })
                    .when(!is_read_only, |this| {
                        this.child(
                            h_flex()
                                .size_3()
                                .justify_center()
                                .flex_shrink_0()
                                .children(indicator),
                        )
                    })
                    .child(
                        h_flex()
                            .cursor_pointer()
                            .id("path_header_block")
                            .min_w_0()
                            .size_full()
                            .justify_between()
                            .overflow_hidden()
                            .child(h_flex().min_w_0().flex_1().gap_0p5().map(|path_header| {
                                let filename = filename
                                    .map(SharedString::from)
                                    .unwrap_or_else(|| "untitled".into());

                                path_header
                                    .when(ItemSettings::get_global(cx).file_icons, |el| {
                                        let path = path::Path::new(filename.as_str());
                                        let icon =
                                            FileIcons::get_icon(path, cx).unwrap_or_default();

                                        el.child(Icon::from_path(icon).color(Color::Muted))
                                    })
                                    .child({
                                        let editor = editor_handle.clone();
                                        let jump_data = jump_data.clone();
                                        ButtonLike::new("filename-button")
                                            .child(
                                                Label::new(filename)
                                                    .single_line()
                                                    .color(file_status_label_color(file_status))
                                                    .when(
                                                        file_status.is_some_and(|s| s.is_deleted()),
                                                        |label| label.strikethrough(),
                                                    ),
                                            )
                                            .on_click(window.listener_for(&editor, {
                                                let jump_data = jump_data.clone();
                                                move |editor, e: &ClickEvent, window, cx| {
                                                    editor.open_excerpts_common(
                                                        Some(jump_data.clone()),
                                                        e.modifiers().secondary(),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            }))
                                    })
                                    .when(!buffer_capability.editable(), |el| {
                                        el.child(Icon::new(IconName::FileLock).color(Color::Muted))
                                    })
                                    .when_some(parent_path, |then, path| {
                                        then.child(Label::new(path).truncate().color(
                                            if file_status.is_some_and(FileStatus::is_deleted) {
                                                Color::Custom(colors.text_disabled)
                                            } else {
                                                Color::Custom(colors.text_muted)
                                            },
                                        ))
                                    })
                            }))
                            .when(
                                can_open_excerpts && is_selected && relative_path.is_some(),
                                |el| {
                                    let editor = editor_handle.clone();
                                    let jump_data = jump_data.clone();
                                    el.child(
                                        Button::new("open-file-button", "Open File")
                                            .style(ButtonStyle::OutlinedGhost)
                                            .key_binding(KeyBinding::for_action_in(
                                                &OpenExcerpts,
                                                &focus_handle,
                                                cx,
                                            ))
                                            .on_click(window.listener_for(&editor, {
                                                let jump_data = jump_data.clone();
                                                move |editor, e: &ClickEvent, window, cx| {
                                                    editor.open_excerpts_common(
                                                        Some(jump_data.clone()),
                                                        e.modifiers().secondary(),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            })),
                                    )
                                },
                            )
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(window.listener_for(&editor_handle, {
                                let jump_data = jump_data.clone();
                                move |editor, e: &ClickEvent, window, cx| {
                                    if e.modifiers().alt {
                                        editor.open_excerpts_common(
                                            Some(jump_data.clone()),
                                            e.modifiers().secondary(),
                                            window,
                                            cx,
                                        );
                                        return;
                                    }

                                    if is_folded {
                                        editor.unfold_buffer(buffer_id, cx);
                                    } else {
                                        editor.fold_buffer(buffer_id, cx);
                                    }
                                }
                            })),
                    ),
            );

        let file = excerpt_info.buffer.file().cloned();
        let editor = editor_handle.clone();

        right_click_menu("buffer-header-context-menu")
            .trigger(move |_, _, _| header)
            .menu(move |window, cx| {
                let menu_context = focus_handle.clone();
                let editor = editor.clone();
                let file = file.clone();
                ContextMenu::build(window, cx, move |mut menu, window, cx| {
                    if let Some(file) = file
                        && let Some(project) = editor.read(cx).project()
                        && let Some(worktree) =
                            project.read(cx).worktree_for_id(file.worktree_id(cx), cx)
                    {
                        let path_style = file.path_style(cx);
                        let worktree = worktree.read(cx);
                        let relative_path = file.path();
                        let entry_for_path = worktree.entry_for_path(relative_path);
                        let abs_path = entry_for_path.map(|e| {
                            e.canonical_path.as_deref().map_or_else(
                                || worktree.absolutize(relative_path),
                                Path::to_path_buf,
                            )
                        });
                        let has_relative_path = worktree.root_entry().is_some_and(Entry::is_dir);

                        let parent_abs_path = abs_path
                            .as_ref()
                            .and_then(|abs_path| Some(abs_path.parent()?.to_path_buf()));
                        let relative_path = has_relative_path
                            .then_some(relative_path)
                            .map(ToOwned::to_owned);

                        let visible_in_project_panel =
                            relative_path.is_some() && worktree.is_visible();
                        let reveal_in_project_panel = entry_for_path
                            .filter(|_| visible_in_project_panel)
                            .map(|entry| entry.id);
                        menu = menu
                            .when_some(abs_path, |menu, abs_path| {
                                menu.entry(
                                    "Copy Path",
                                    Some(Box::new(zed_actions::workspace::CopyPath)),
                                    window.handler_for(&editor, move |_, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            abs_path.to_string_lossy().into_owned(),
                                        ));
                                    }),
                                )
                            })
                            .when_some(relative_path, |menu, relative_path| {
                                menu.entry(
                                    "Copy Relative Path",
                                    Some(Box::new(zed_actions::workspace::CopyRelativePath)),
                                    window.handler_for(&editor, move |_, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            relative_path.display(path_style).to_string(),
                                        ));
                                    }),
                                )
                            })
                            .when(
                                reveal_in_project_panel.is_some() || parent_abs_path.is_some(),
                                |menu| menu.separator(),
                            )
                            .when_some(reveal_in_project_panel, |menu, entry_id| {
                                menu.entry(
                                    "Reveal In Project Panel",
                                    Some(Box::new(RevealInProjectPanel::default())),
                                    window.handler_for(&editor, move |editor, _, cx| {
                                        if let Some(project) = &mut editor.project {
                                            project.update(cx, |_, cx| {
                                                cx.emit(project::Event::RevealInProjectPanel(
                                                    entry_id,
                                                ))
                                            });
                                        }
                                    }),
                                )
                            })
                            .when_some(parent_abs_path, |menu, parent_abs_path| {
                                menu.entry(
                                    "Open in Terminal",
                                    Some(Box::new(OpenInTerminal)),
                                    window.handler_for(&editor, move |_, window, cx| {
                                        window.dispatch_action(
                                            OpenTerminal {
                                                working_directory: parent_abs_path.clone(),
                                            }
                                            .boxed_clone(),
                                            cx,
                                        );
                                    }),
                                )
                            });
                    }

                    menu.context(menu_context)
                })
            })
    }

    fn render_sticky_buffer_header(
        editor: &Entity<Editor>,
        excerpt: &ExcerptInfo,
        snapshot: &EditorSnapshot,
        scroll_position: gpui::Point<crate::scroll::ScrollOffset>,
        line_height: Pixels,
        available_width: Pixels,
        selected_buffer_ids: &[BufferId],
        latest_selection_anchors: &HashMap<BufferId, Anchor>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let jump_data = header_jump_data(
            snapshot,
            DisplayRow(scroll_position.y as u32),
            FILE_HEADER_HEIGHT + MULTI_BUFFER_EXCERPT_HEADER_HEIGHT,
            excerpt,
            latest_selection_anchors,
        );

        let editor_bg_color = cx.theme().colors().editor_background;

        let selected = selected_buffer_ids.contains(&excerpt.buffer_id);

        v_flex()
            .w_full()
            .relative()
            .child(
                div()
                    .w(available_width)
                    .h(FILE_HEADER_HEIGHT as f32 * line_height)
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(editor_bg_color.opacity(0.), 0.),
                        linear_color_stop(editor_bg_color, 0.6),
                    ))
                    .absolute()
                    .top_0(),
            )
            .child(
                Self::render_buffer_header(
                    editor,
                    excerpt,
                    false,
                    selected,
                    true,
                    jump_data,
                    line_height,
                    window,
                    cx,
                )
                .into_any_element(),
            )
            .into_any_element()
    }

    fn compute_sticky_header_origin(
        &self,
        scroll_position: gpui::Point<crate::scroll::ScrollOffset>,
    ) -> gpui::Point<Pixels> {
        let mut origin = self.hitbox_origin;
        for (row, block) in self.snapshot.blocks_in_range(self.visible_row_range.clone()) {
            if !block.is_buffer_header() {
                continue;
            }

            if row.0 <= scroll_position.y as u32 {
                continue;
            }

            let max_row = row.0.saturating_sub(FILE_HEADER_HEIGHT);
            let offset = scroll_position.y - max_row as f64;

            if offset > 0.0 {
                origin.y -= Pixels::from(offset * ScrollPixelOffset::from(self.line_height));
            }
            break;
        }
        origin
    }
}

fn file_status_label_color(file_status: Option<FileStatus>) -> Color {
    file_status.map_or(Color::Default, |status| {
        if status.is_conflicted() {
            Color::Conflict
        } else if status.is_modified() {
            Color::Modified
        } else if status.is_deleted() {
            Color::Disabled
        } else if status.is_created() {
            Color::Created
        } else {
            Color::Default
        }
    })
}

pub struct BufferHeadersViewState {
    sticky_header_element: Option<AnyElement>,
    sticky_header_origin: gpui::Point<Pixels>,
    available_width: Pixels,
}

impl Element for BufferHeadersView {
    type RequestLayoutState = BufferHeadersViewState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let sticky_header_excerpt = self
            .snapshot
            .sticky_header_excerpt(self.scroll_position.y.into());

        let available_width = self.hitbox_width - self.right_margin;

        let sticky_header_element = sticky_header_excerpt.map(|sticky_header_excerpt| {
            Self::render_sticky_buffer_header(
                &self.editor,
                sticky_header_excerpt.excerpt,
                &self.snapshot,
                self.scroll_position,
                self.line_height,
                available_width,
                &self.selected_buffer_ids,
                &self.latest_selection_anchors,
                window,
                cx,
            )
        });

        let sticky_header_origin = self.compute_sticky_header_origin(self.scroll_position);

        let layout_id = window.request_layout(gpui::Style::default(), None, cx);

        (
            layout_id,
            BufferHeadersViewState {
                sticky_header_element,
                sticky_header_origin,
                available_width,
            },
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(ref mut header) = request_layout.sticky_header_element {
            let available_size = size(
                AvailableSpace::Definite(request_layout.available_width),
                AvailableSpace::MinContent,
            );
            header.prepaint_as_root(request_layout.sticky_header_origin, available_size, window, cx);
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(ref mut header) = request_layout.sticky_header_element {
            header.paint(window, cx);
        }
    }
}

impl IntoElement for BufferHeadersView {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}