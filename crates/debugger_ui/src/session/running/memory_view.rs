use std::{sync::LazyLock, time::Duration};

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    AppContext, Empty, Entity, FocusHandle, Focusable, MouseButton, MouseMoveEvent, ScrollStrategy,
    ScrollWheelEvent, Stateful, Task, TextStyle, UniformList, UniformListScrollHandle, bounds,
    point, size, uniform_list,
};
use project::debugger::{MemoryCell, session::Session};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, Color, Context, ContextMenu, Div, Divider, DropdownMenu, Element,
    FluentBuilder, InteractiveElement, IntoElement, Label, LabelCommon, ParentElement,
    PopoverMenuHandle, Render, Scrollbar, ScrollbarState, SharedString, StatefulInteractiveElement,
    Styled, TextSize, Window, div, h_flex, px, v_flex,
};
use util::ResultExt;

pub(crate) struct MemoryView {
    scroll_handle: UniformListScrollHandle,
    scroll_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    focus_handle: FocusHandle,
    view_state: ViewState,
    query_editor: Entity<Editor>,
    session: Entity<Session>,
    width_picker_handle: PopoverMenuHandle<ContextMenu>,
}

impl Focusable for MemoryView {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
#[derive(Clone, Debug)]
struct Drag {
    start_address: u64,
    end_address: u64,
}

impl Drag {
    fn contains(&self, address: u64) -> bool {
        let range = if self.start_address < self.end_address {
            self.start_address..=self.end_address
        } else {
            self.end_address..=self.start_address
        };
        range.contains(&address)
    }
}
#[derive(Clone, Debug)]
enum SelectedMemoryRange {
    DragUnderway(Drag),
    DragComplete(Drag),
}

impl SelectedMemoryRange {
    fn contains(&self, address: u64) -> bool {
        match self {
            SelectedMemoryRange::DragUnderway(drag) => drag.contains(address),
            SelectedMemoryRange::DragComplete(drag) => drag.contains(address),
        }
    }
    fn is_dragging(&self) -> bool {
        matches!(self, SelectedMemoryRange::DragUnderway(_))
    }
}

#[derive(Clone)]
struct ViewState {
    /// Uppermost row index
    base_row: u64,
    /// How many cells per row do we have?
    line_width: ViewWidth,
    selection: Option<SelectedMemoryRange>,
}

impl ViewState {
    fn new(base_row: u64, line_width: ViewWidth) -> Self {
        Self {
            base_row,
            line_width,
            selection: None,
        }
    }
    fn row_count(&self) -> u64 {
        // This was picked fully arbitrarily. There's no incentive for us to care about page sizes other than the fact that it seems to be a good
        // middle ground for data size.
        const PAGE_SIZE: u64 = 4096;
        PAGE_SIZE / self.line_width.width as u64
    }
    fn schedule_scroll_down(&mut self) {
        self.base_row = self.base_row.saturating_add(1)
    }
    fn schedule_scroll_up(&mut self) {
        self.base_row = self.base_row.saturating_sub(1);
    }
}

static HEX_BYTES_MEMOIZED: LazyLock<[SharedString; 256]> =
    LazyLock::new(|| std::array::from_fn(|byte| SharedString::from(format!("{byte:02X}"))));
static UNKNOWN_BYTE: SharedString = SharedString::new_static("??");
impl MemoryView {
    pub(crate) fn new(
        session: Entity<Session>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let view_state = ViewState::new(0, WIDTHS[4].clone());
        let scroll_handle = UniformListScrollHandle::default();

        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Memory Address or Expression", cx);
            editor
        });

        let scroll_state = ScrollbarState::new(scroll_handle.clone());
        Self {
            scroll_state,
            scroll_handle,
            show_scrollbar: false,
            hide_scrollbar_task: None,
            focus_handle: cx.focus_handle(),
            view_state,
            query_editor,
            session,
            width_picker_handle: Default::default(),
        }
    }
    fn hide_scrollbar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        self.hide_scrollbar_task = Some(cx.spawn_in(window, async move |panel, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !(self.show_scrollbar || self.scroll_state.is_dragging()) {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("memory-view-vertical-scrollbar")
                .on_mouse_move(cx.listener(|this, evt, _, cx| {
                    this.handle_drag(evt);
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|_, _, _, cx| {
                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_0()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(self.scroll_state.clone())),
        )
    }

    fn render_memory(&self, cx: &mut Context<Self>) -> UniformList {
        let weak = cx.weak_entity();
        let session = self.session.clone();
        let view_state = self.view_state.clone();
        uniform_list(
            "debugger-memory-view",
            self.view_state.row_count() as usize,
            move |range, _, cx| {
                let mut line_buffer = Vec::with_capacity(view_state.line_width.width as usize);
                let memory_start =
                    (view_state.base_row + range.start as u64) * view_state.line_width.width as u64;
                let memory_end = (view_state.base_row + range.end as u64)
                    * view_state.line_width.width as u64
                    - 1;
                let mut memory = session.update(cx, |this, cx| {
                    this.read_memory(memory_start..=memory_end, cx)
                });
                let mut rows = Vec::with_capacity(range.end - range.start);
                for ix in range {
                    line_buffer.extend((&mut memory).take(view_state.line_width.width as usize));
                    rows.push(render_single_memory_view_line(
                        &line_buffer,
                        ix as u64,
                        weak.clone(),
                        cx,
                    ));
                    line_buffer.clear();
                }
                rows
            },
        )
        .track_scroll(self.scroll_handle.clone())
        .on_scroll_wheel(cx.listener(|this, evt: &ScrollWheelEvent, window, _| {
            let delta = evt.delta.pixel_delta(window.line_height());
            let scroll_handle = this.scroll_state.scroll_handle();
            let size = scroll_handle.content_size();
            let viewport = scroll_handle.viewport();
            let current_offset = scroll_handle.offset();
            let first_entry_offset_boundary = size.height / this.view_state.row_count() as f32;
            let last_entry_offset_boundary = size.height - first_entry_offset_boundary;
            if first_entry_offset_boundary + viewport.size.height > current_offset.y.abs() {
                // The topmost entry is visible, hence if we're scrolling up, we need to load extra lines.
                this.view_state.schedule_scroll_up();
            } else if last_entry_offset_boundary < current_offset.y.abs() + viewport.size.height {
                this.view_state.schedule_scroll_down();
            }
            scroll_handle.set_offset(current_offset + point(px(0.), delta.y));
        }))
    }
    fn render_query_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        EditorElement::new(
            &self.query_editor,
            Self::editor_style(&self.query_editor, cx),
        )
    }
    pub(super) fn go_to_memory_reference(
        &mut self,
        memory_reference: &str,
        evaluate_name: Option<&str>,
        stack_frame_id: Option<u64>,
        cx: &mut Context<Self>,
    ) {
        use parse_int::parse;
        let Ok(as_address) = parse::<u64>(&memory_reference) else {
            return;
        };
        let access_size = evaluate_name
            .map(|typ| {
                self.session.update(cx, |this, cx| {
                    this.data_access_size(stack_frame_id, typ, cx)
                })
            })
            .unwrap_or_else(|| Task::ready(None));
        cx.spawn(async move |this, cx| {
            let access_size = access_size.await.unwrap_or(1);
            this.update(cx, |this, cx| {
                this.view_state.base_row =
                    (as_address & !0xfff) / this.view_state.line_width.width as u64;
                this.view_state.selection = Some(SelectedMemoryRange::DragComplete(Drag {
                    start_address: as_address,
                    end_address: as_address + access_size - 1,
                }));
                let line_ix = (as_address & 0xfff) / this.view_state.line_width.width as u64;
                this.scroll_handle
                    .scroll_to_item(line_ix as usize, ScrollStrategy::Center);

                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn handle_drag(&mut self, evt: &MouseMoveEvent) {
        if !evt.dragging() {
            return;
        }
        if !self.scroll_state.is_dragging()
            && !self
                .view_state
                .selection
                .as_ref()
                .is_some_and(|selection| selection.is_dragging())
        {
            return;
        }
        let row_count = self.view_state.row_count();
        debug_assert!(row_count > 1);
        let scroll_handle = self.scroll_state.scroll_handle();
        let viewport = scroll_handle.viewport();
        let (top_area, bottom_area) = {
            let size = size(viewport.size.width, viewport.size.height / 3.);
            (
                bounds(viewport.origin, size),
                bounds(
                    point(viewport.origin.x, viewport.origin.y + size.height * 2.),
                    size,
                ),
            )
        };

        if bottom_area.contains(&evt.position) {
            //ix == row_count - 1 {
            self.view_state.schedule_scroll_down();
        } else if top_area.contains(&evt.position) {
            self.view_state.schedule_scroll_up();
        }
    }

    fn editor_style(editor: &Entity<Editor>, cx: &Context<Self>) -> EditorStyle {
        let is_read_only = editor.read(cx).read_only(cx);
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme();
        let text_style = TextStyle {
            color: if is_read_only {
                theme.colors().text_muted
            } else {
                theme.colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: TextSize::Small.rems(cx).into(),
            font_weight: settings.buffer_font.weight,

            ..Default::default()
        };
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text: text_style,
            ..Default::default()
        }
    }

    fn render_width_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        let weak = cx.weak_entity();
        let selected_width = self.view_state.line_width.clone();
        DropdownMenu::new(
            "memory-view-width-picker",
            selected_width.label.clone(),
            ContextMenu::build(window, cx, |mut this, window, cx| {
                for width in &WIDTHS {
                    let weak = weak.clone();
                    let width = width.clone();
                    this = this.entry(width.label.clone(), None, move |_, cx| {
                        _ = weak.update(cx, |this, _| {
                            // Convert base ix between 2 line widths to keep the shown memory address roughly the same.
                            // All widths are powers of 2, so the conversion should be lossless.
                            match this.view_state.line_width.width.cmp(&width.width) {
                                std::cmp::Ordering::Less => {
                                    // We're converting up.
                                    let shift = width.width.trailing_zeros()
                                        - this.view_state.line_width.width.trailing_zeros();
                                    this.view_state.base_row >>= shift;
                                }
                                std::cmp::Ordering::Greater => {
                                    // We're converting down.
                                    let shift = this.view_state.line_width.width.trailing_zeros()
                                        - width.width.trailing_zeros();
                                    this.view_state.base_row <<= shift;
                                }
                                _ => {}
                            }
                            this.view_state.line_width = width.clone();
                        });
                    });
                }
                if let Some(ix) = WIDTHS
                    .iter()
                    .position(|width| width.width == selected_width.width)
                {
                    for _ in 0..=ix {
                        this.select_next(&Default::default(), window, cx);
                    }
                }
                this
            }),
        )
        .handle(self.width_picker_handle.clone())
    }

    fn change_address(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        use parse_int::parse;
        let text = self.query_editor.read(cx).text(cx);

        let Ok(as_address) = parse::<u64>(&text) else {
            return;
        };
        self.view_state.base_row = (as_address & !0xfff) / self.view_state.line_width.width as u64;
        cx.notify();
    }
    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.view_state.selection = None;
        cx.notify();
    }
}

#[derive(Clone)]
struct ViewWidth {
    width: u8,
    label: SharedString,
}

impl ViewWidth {
    const fn new(width: u8, label: &'static str) -> Self {
        Self {
            width,
            label: SharedString::new_static(label),
        }
    }
}

static WIDTHS: [ViewWidth; 7] = [
    ViewWidth::new(1, "1 byte"),
    ViewWidth::new(2, "2 bytes"),
    ViewWidth::new(4, "4 bytes"),
    ViewWidth::new(8, "8 bytes"),
    ViewWidth::new(16, "16 bytes"),
    ViewWidth::new(32, "32 bytes"),
    ViewWidth::new(64, "64 bytes"),
];

fn render_single_memory_view_line(
    memory: &[MemoryCell],
    ix: u64,
    weak: gpui::WeakEntity<MemoryView>,
    cx: &mut App,
) -> AnyElement {
    let Ok(view_state) = weak.update(cx, |this, _| this.view_state.clone()) else {
        return div().into_any();
    };
    let base_address = (view_state.base_row + ix) * view_state.line_width.width as u64;

    h_flex()
        .id((
            "memory-view-row-full",
            ix * view_state.line_width.width as u64,
        ))
        .size_full()
        .gap_x_2()
        .child(
            div()
                .child(
                    Label::new(format!("{:016X}", base_address))
                        .buffer_font(cx)
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                )
                .px_1()
                .border_r_1()
                .border_color(Color::Muted.color(cx)),
        )
        .child(
            h_flex()
                .id((
                    "memory-view-row-raw-memory",
                    ix * view_state.line_width.width as u64,
                ))
                .px_1()
                .children(memory.iter().enumerate().map(|(cell_ix, cell)| {
                    let weak = weak.clone();
                    div()
                        .id(("memory-view-row-raw-memory-cell", cell_ix as u64))
                        .px_0p5()
                        .when_some(view_state.selection.as_ref(), |this, selection| {
                            this.when(selection.contains(base_address + cell_ix as u64), |this| {
                                this.bg(Color::Accent.color(cx))
                            })
                        })
                        .child(
                            Label::new(
                                cell.0
                                    .map(|val| HEX_BYTES_MEMOIZED[val as usize].clone())
                                    .unwrap_or_else(|| UNKNOWN_BYTE.clone()),
                            )
                            .buffer_font(cx)
                            .when(cell.0.is_none(), |this| this.color(Color::Muted))
                            .size(ui::LabelSize::Small),
                        )
                        .on_drag(
                            Drag {
                                start_address: base_address + cell_ix as u64,
                                end_address: base_address + cell_ix as u64,
                            },
                            {
                                let weak = weak.clone();
                                move |drag, _, _, cx| {
                                    _ = weak.update(cx, |this, _| {
                                        this.view_state.selection =
                                            Some(SelectedMemoryRange::DragUnderway(drag.clone()));
                                    });

                                    cx.new(|_| Empty)
                                }
                            },
                        )
                        .on_drop({
                            let weak = weak.clone();
                            move |drag: &Drag, _, cx| {
                                _ = weak.update(cx, |this, _| {
                                    this.view_state.selection =
                                        Some(SelectedMemoryRange::DragComplete(Drag {
                                            start_address: drag.start_address,
                                            end_address: base_address + cell_ix as u64,
                                        }));
                                });
                            }
                        })
                        .drag_over(move |style, drag: &Drag, _, cx| {
                            _ = weak.update(cx, |this, _| {
                                this.view_state.selection =
                                    Some(SelectedMemoryRange::DragUnderway(Drag {
                                        start_address: drag.start_address,
                                        end_address: base_address + cell_ix as u64,
                                    }));
                            });

                            style
                        })
                })),
        )
        .child(
            h_flex()
                .id((
                    "memory-view-row-ascii-memory",
                    ix * view_state.line_width.width as u64,
                ))
                .h_full()
                .px_1()
                .mr_4()
                .gap_x_1p5()
                .border_x_1()
                .border_color(Color::Muted.color(cx))
                .children(memory.iter().map(|cell| {
                    let as_character = char::from(cell.0.unwrap_or(0));
                    let as_visible = if as_character.is_ascii_graphic() {
                        as_character
                    } else {
                        '·'
                    };
                    Label::new(format!("{as_visible}"))
                        .buffer_font(cx)
                        .when(cell.0.is_none(), |this| this.color(Color::Muted))
                        .size(ui::LabelSize::Small)
                })),
        )
        .into_any()
}

impl Render for MemoryView {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        v_flex()
            .id("Memory-view")
            .on_action(cx.listener(Self::cancel))
            .p_1()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_hover(cx.listener(|this, hovered, window, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbar(window, cx);
                }
            }))
            .child(
                h_flex()
                    .w_full()
                    .mb_0p5()
                    .gap_1()
                    .child(
                        h_flex()
                            .w_full()
                            .rounded_md()
                            .border_1()
                            .p_0p5()
                            .mb_0p5()
                            .bg(cx.theme().colors().editor_background)
                            .on_action(cx.listener(Self::change_address))
                            .when_else(
                                self.query_editor
                                    .focus_handle(cx)
                                    .contains_focused(window, cx),
                                |this| this.border_color(cx.theme().colors().border_focused),
                                |this| this.border_color(cx.theme().colors().border_transparent),
                            )
                            .child(self.render_query_bar(cx)),
                    )
                    .child(self.render_width_picker(window, cx)),
            )
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .size_full()
                    .on_mouse_move(cx.listener(|this, evt: &MouseMoveEvent, _, _| {
                        this.handle_drag(evt);
                    }))
                    .child(self.render_memory(cx).size_full())
                    .children(self.render_vertical_scrollbar(cx)),
            )
    }
}
