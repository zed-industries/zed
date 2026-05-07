use crate::{
    NextHistoryQuery, PreviousHistoryQuery, SearchOptions, SelectNextMatch, SelectPreviousMatch,
    ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex, ToggleReplace, ToggleWholeWord,
    quick_search::{
        HistoryDirection, LayoutMode, QuickSearch, QuickSearchDrag, ReplaceAll,
        ReplaceNext, ResizeSide, ToggleFilters, ToggleHistory, ToggleLayout, ToggleSplitMenu,
        clear_resize_highlight, handle_resize_mouse_down, resize_hover_handler,
        state::{StackedLayoutState, TelescopeLayoutState},
    },
};
use gpui::{
    Context, DismissEvent, DragMoveEvent, Focusable, KeyContext, MouseButton, ParentElement,
    Render, Styled, Window,
};
use menu;
use theme::ActiveTheme;
use ui::prelude::*;
use zed_actions::editor::{MoveDown, MoveUp};

mod telescope;
mod window_controls;

#[derive(Clone, Copy)]
struct ResizeDrag {
    mouse_start_y: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
}

struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl Render for super::QuickSearch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modal_width = self.modal_width;
        let focus_handle = self.focus_handle.clone();
        let in_replace = self.replacement_editor.focus_handle(cx).is_focused(window);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("QuickSearch");
        if in_replace {
            key_context.add("in_replace");
        }

        div()
            .id("quick-search-backdrop")
            .absolute()
            .size_full()
            .inset_0()
            .occlude()
            .flex()
            .flex_col()
            .items_center()
            .pt_20()
            .on_click(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
            .child(
                v_flex()
                    .when(self.layout_mode == LayoutMode::Stacked, |this| {
                        this.child(self.render_vertical_resize(ResizeSide::Start, window, cx))
                    })
                    .when(self.layout_mode == LayoutMode::Telescope, |this| {
                        this.child(self.render_telescope_height_resize(
                            ResizeSide::Start,
                            window,
                            cx,
                        ))
                    })
                    .child(self.render_horizontal_resize(ResizeSide::Start, window, cx))
                    .child(self.render_horizontal_resize(ResizeSide::End, window, cx))
                    .m_4()
                    .relative()
                    .top(self.offset.y)
                    .left(self.offset.x)
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(
                        v_flex()
                            .key_context(key_context)
                            .id("quick-search")
                            .track_focus(&focus_handle)
                            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                                cx.emit(DismissEvent);
                            }))
                            .on_action(cx.listener(|this, _: &ReplaceNext, window, cx| {
                                this.replace_next(window, cx);
                            }))
                            .on_action(cx.listener(|this, _: &ReplaceAll, window, cx| {
                                this.replace_all(window, cx);
                            }))
                            .on_action(cx.listener(|this, _: &ToggleFilters, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker.delegate.filters_enabled =
                                        !picker.delegate.filters_enabled;
                                    let focus_handle = if picker.delegate.filters_enabled {
                                        picker.delegate.included_files_editor.focus_handle(cx)
                                    } else {
                                        picker.focus_handle(cx)
                                    };
                                    window.focus(&focus_handle, cx);
                                });
                                cx.notify();
                            }))
                            .on_action(cx.listener(|this, _: &NextHistoryQuery, window, cx| {
                                this.navigate_history(HistoryDirection::Next, window, cx);
                            }))
                            .on_action(cx.listener(|this, _: &PreviousHistoryQuery, window, cx| {
                                this.navigate_history(HistoryDirection::Previous, window, cx);
                            }))
                            .on_action(cx.listener(|this, _: &ToggleHistory, window, cx| {
                                let handle = this
                                    .picker
                                    .read(cx)
                                    .delegate
                                    .history_popover_menu_handle
                                    .clone();
                                handle.toggle(window, cx);
                                cx.notify();
                            }))
                            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker
                                        .delegate
                                        .search_options
                                        .toggle(SearchOptions::CASE_SENSITIVE);
                                    picker.refresh(window, cx);
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleWholeWord, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker
                                        .delegate
                                        .search_options
                                        .toggle(SearchOptions::WHOLE_WORD);
                                    picker.refresh(window, cx);
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleIncludeIgnored, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker
                                        .delegate
                                        .search_options
                                        .toggle(SearchOptions::INCLUDE_IGNORED);
                                    picker.refresh(window, cx);
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleRegex, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker.delegate.search_options.toggle(SearchOptions::REGEX);
                                    picker.refresh(window, cx);
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleReplace, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker.delegate.replace_enabled =
                                        !picker.delegate.replace_enabled;
                                    let focus_handle = if picker.delegate.replace_enabled {
                                        picker.delegate.replacement_editor.focus_handle(cx)
                                    } else {
                                        picker.focus_handle(cx)
                                    };
                                    window.focus(&focus_handle, cx);
                                });
                                cx.notify();
                            }))
                            .on_action(cx.listener(|this, _: &SelectNextMatch, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    let match_count = picker.delegate.matches.len();
                                    if match_count > 0 {
                                        let new_index =
                                            (picker.delegate.selected_index + 1) % match_count;
                                        picker
                                            .set_selected_index(new_index, None, true, window, cx);
                                    }
                                });
                            }))
                            .on_action(cx.listener(|this, _: &SelectPreviousMatch, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    let match_count = picker.delegate.matches.len();
                                    if match_count > 0 {
                                        let new_index = if picker.delegate.selected_index == 0 {
                                            match_count - 1
                                        } else {
                                            picker.delegate.selected_index - 1
                                        };
                                        picker
                                            .set_selected_index(new_index, None, true, window, cx);
                                    }
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleSplitMenu, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    let menu_handle = &picker.delegate.split_popover_menu_handle;
                                    if menu_handle.is_deployed() {
                                        menu_handle.hide(cx);
                                    } else {
                                        menu_handle.show(window, cx);
                                    }
                                });
                            }))
                            .on_action(cx.listener(|this, _: &ToggleLayout, window, cx| {
                                this.layout_mode = match this.layout_mode {
                                    LayoutMode::Stacked => LayoutMode::Telescope,
                                    LayoutMode::Telescope => LayoutMode::Stacked,
                                };
                                let default_width_rems = match this.layout_mode {
                                    LayoutMode::Stacked => {
                                        StackedLayoutState::DEFAULT_MODAL_WIDTH_REMS
                                    }
                                    LayoutMode::Telescope => {
                                        TelescopeLayoutState::DEFAULT_MODAL_WIDTH_REMS
                                    }
                                };
                                this.modal_width =
                                    rems(default_width_rems).to_pixels(window.rem_size());
                                cx.notify();
                            }))
                            .on_action(cx.listener(Self::go_to_file_split_left))
                            .on_action(cx.listener(Self::go_to_file_split_right))
                            .on_action(cx.listener(Self::go_to_file_split_up))
                            .on_action(cx.listener(Self::go_to_file_split_down))
                            .on_action(cx.listener(|this, action: &MoveUp, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker.editor_move_up(action, window, cx);
                                });
                            }))
                            .on_action(cx.listener(|this, action: &MoveDown, window, cx| {
                                this.picker.update(cx, |picker, cx| {
                                    picker.editor_move_down(action, window, cx);
                                });
                            }))
                            .w(modal_width)
                            .bg(cx.theme().colors().elevated_surface_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_lg()
                            .shadow_lg()
                            .child(self.render_header(window, cx))
                            .child(self.render_content(window, cx)),
                    )
                    .children([
                        self.render_corner_resize(ResizeSide::Start, ResizeSide::Start, window, cx),
                        self.render_corner_resize(ResizeSide::End, ResizeSide::Start, window, cx),
                        self.render_corner_resize(ResizeSide::Start, ResizeSide::End, window, cx),
                        self.render_corner_resize(ResizeSide::End, ResizeSide::End, window, cx),
                    ]),
            )
    }
}

impl QuickSearch {
    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let delegate = &self.picker.read(cx).delegate;
        let match_count = delegate.matches.len();
        let file_count = delegate.file_count;
        let search_in_progress = delegate.search_in_progress;

        h_flex()
            .id("quick-search-header")
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .justify_between()
            .on_drag(
                QuickSearchDrag {
                    mouse_start: window.mouse_position(),
                    offset_start: self.offset,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<QuickSearchDrag>(cx.listener(
                |this, event: &DragMoveEvent<QuickSearchDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    this.offset = drag.offset_start + (event.event.position - drag.mouse_start);
                    cx.notify();
                },
            ))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new("Quick Search").size(LabelSize::Default))
                    .when(search_in_progress || match_count > 0, |this| {
                        let prefix = if search_in_progress {
                            "Searching... "
                        } else {
                            ""
                        };
                        this.child(
                            Label::new(format!(
                                "{prefix}{match_count} matches in {file_count} files"
                            ))
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                        )
                    }),
            )
            .child(self.render_header_controls(window, cx))
    }

    fn render_preview_header(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let delegate = &self.picker.read(cx).delegate;
        let selected_match = delegate.matches.get(delegate.selected_index);

        if let Some(m) = selected_match {
            let path = &m.path.path;
            let file_name = path
                .file_name()
                .map(|name| name.to_string())
                .unwrap_or_default();
            let directory = path
                .parent()
                .map(|path| path.as_std_path().to_string_lossy().to_string())
                .unwrap_or_default();

            let split_menu_handle = delegate.split_popover_menu_handle.clone();
            let focus_handle = self.focus_handle.clone();

            h_flex()
                .px_2()
                .py_1()
                .gap_2()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().editor_background)
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .min_w(px(0.))
                        .child(Label::new(file_name).size(LabelSize::Small).truncate())
                        .child(
                            Label::new(directory)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate(),
                        ),
                )
                .child(window_controls::render_split_menu(
                    split_menu_handle,
                    focus_handle,
                    window,
                    cx,
                ))
        } else {
            h_flex().h(px(26.0))
        }
    }

    fn render_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(self.render_preview_header(window, cx))
            .child(
                div()
                    .h(self.stacked.preview_height)
                    .overflow_hidden()
                    .child(self.preview_editor.clone()),
            )
    }

    fn render_results_resize(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_highlighted = window.use_state(cx, |_window, _cx| false);
        let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
        let handle_height = px(window_controls::RESIZE_HANDLE_HEIGHT);
        let handle_offset = (handle_height - divider_size) / 2.0;

        div()
            .id("resize-divider")
            .relative()
            .h(divider_size)
            .w_full()
            .bg(cx.theme().colors().border)
            .when(*is_highlighted.read(cx), |this| {
                this.bg(cx.theme().colors().border_focused)
            })
            .child(
                div()
                    .id("resize-handle")
                    .absolute()
                    .top(-handle_offset)
                    .left_0()
                    .right_0()
                    .h(handle_height)
                    .cursor_row_resize()
                    .block_mouse_except_scroll()
                    .on_hover(resize_hover_handler(is_highlighted.clone()))
                    .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
                    .on_drag(
                        ResizeDrag {
                            mouse_start_y: window.mouse_position().y,
                            results_height_start: self.stacked.results_height,
                            preview_height_start: self.stacked.preview_height,
                        },
                        highlighted_drag_preview(is_highlighted.clone()),
                    )
                    .on_drop::<ResizeDrag>(clear_resize_highlight(is_highlighted.clone())),
            )
            .on_drag_move::<ResizeDrag>(cx.listener(
                |this, event: &DragMoveEvent<ResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.y - drag.mouse_start_y;
                    let total_height = drag.results_height_start + drag.preview_height_start;

                    let new_results = (drag.results_height_start + delta)
                        .max(px(StackedLayoutState::MIN_PANEL_HEIGHT))
                        .min(total_height - px(StackedLayoutState::MIN_PANEL_HEIGHT));
                    let new_preview = total_height - new_results;

                    this.stacked.results_height = new_results;
                    this.stacked.preview_height = new_preview;
                    cx.notify();
                },
            ))
    }

    fn render_content(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.layout_mode {
            LayoutMode::Stacked => self.render_stacked_content(window, cx).into_any_element(),
            LayoutMode::Telescope => self.render_telescope_content(window, cx).into_any_element(),
        }
    }

    fn render_stacked_content(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .child(
                div()
                    .h(self.stacked.results_height)
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .child(self.render_results_resize(window, cx))
            .child(self.render_preview(window, cx))
            .child(self.render_vertical_resize(ResizeSide::End, window, cx))
    }
}

fn highlighted_drag_preview<T>(
    is_highlighted: gpui::Entity<bool>,
) -> impl Fn(&T, gpui::Point<Pixels>, &mut Window, &mut App) -> gpui::Entity<DragPreview> {
    move |_, _, _, cx| {
        is_highlighted.write(cx, true);
        cx.new(|_| DragPreview)
    }
}
