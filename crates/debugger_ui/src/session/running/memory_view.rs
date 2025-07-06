use std::{sync::LazyLock, time::Duration};

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    AppContext, Empty, Entity, FocusHandle, Focusable, ListState, MouseButton, Stateful, Task,
    TextStyle, list,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Color, Context, Div, Divider, Element, FluentBuilder, InteractiveElement,
    IntoElement, Label, LabelCommon, ParentElement, Render, Scrollbar, ScrollbarState,
    SharedString, StatefulInteractiveElement, Styled, TextSize, Window, div, h_flex, px, v_flex,
};
use util::ResultExt;

pub(crate) struct MemoryView {
    list_state: ListState,
    scroll_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    focus_handle: FocusHandle,
    view_state: ViewState,
    query_editor: Entity<Editor>,
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
    /// To implement the infinite scrolling feature, we track two row indices: base (applicable to current frame) and next frame.
    /// Whenever we render an item at a list boundary, we decrement/increment base index.
    next_row: u64,
    /// How many cells per row do we have?
    line_width: u64,
    selection: Option<SelectedMemoryRange>,
}

impl ViewState {
    fn new(base_row: u64, line_width: u64) -> Self {
        Self {
            base_row,
            next_row: base_row,
            line_width,
            selection: None,
        }
    }
    fn row_count(&self) -> u64 {
        // This was picked fully arbitrarily. There's no incentive for us to care about page sizes other than the fact that it seems to be a good
        // middle ground for data size.
        const PAGE_SIZE: u64 = 4096;
        PAGE_SIZE / self.line_width
    }
    fn schedule_scroll_down(&mut self) {
        self.next_row = self.next_row.saturating_add(1)
    }
    fn schedule_scroll_up(&mut self) {
        self.next_row = self.next_row.saturating_sub(1);
    }
    fn perform_scheduled_scroll(&mut self) {
        self.base_row = self.next_row;
    }
}

static HEX_BYTES_MEMOIZED: LazyLock<[SharedString; 256]> =
    LazyLock::new(|| std::array::from_fn(|byte| SharedString::from(format!("{byte:02X}"))));

impl MemoryView {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let view_state = ViewState::new(0, 16);
        let state = ListState::new(
            view_state.row_count() as usize,
            gpui::ListAlignment::Top,
            px(100.),
            {
                let weak = cx.weak_entity();
                move |ix, _, cx| {
                    let ix = ix as u64;
                    let Ok((memory, view_state)) = weak.update(cx, |this, _| {
                        let start = this.view_state.base_row;

                        let row_count = this.view_state.row_count();

                        debug_assert!(row_count > 1);
                        if ix == row_count - 1
                            && (this.scroll_state.is_dragging()
                                || this
                                    .view_state
                                    .selection
                                    .as_ref()
                                    .is_some_and(|selection| selection.is_dragging()))
                        {
                            this.view_state.schedule_scroll_down();
                        } else if ix == 0
                            && (this.scroll_state.is_dragging()
                                || this
                                    .view_state
                                    .selection
                                    .as_ref()
                                    .is_some_and(|selection| selection.is_dragging()))
                        {
                            this.view_state.schedule_scroll_up();
                        }
                        let line_width = this.view_state.line_width;
                        let memory = (0..line_width)
                            .map(|cell_ix| {
                                (((start + ix) * line_width + cell_ix) % (u8::MAX as u64 + 1)) as u8
                            })
                            .collect::<Vec<_>>();
                        (memory, this.view_state.clone())
                    }) else {
                        return div().into_any();
                    };
                    let base_address = (view_state.base_row + ix) * view_state.line_width;

                    h_flex()
                        .id(("memory-view-row-full", ix * view_state.line_width))
                        .size_full()
                        .gap_x_2()
                        .child(
                            div()
                                .child(
                                    Label::new(format!("{:08X}", base_address))
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
                                .id(("memory-view-row-raw-memory", ix * view_state.line_width))
                                .size_full()
                                .px_1()
                                .children(memory.iter().enumerate().map(|(cell_ix, cell)| {
                                    let weak = weak.clone();
                                    div()
                                        .id((
                                            "memory-view-row-raw-memory",
                                            base_address + cell_ix as u64,
                                        ))
                                        .px_0p5()
                                        .when_some(
                                            view_state.selection.as_ref(),
                                            |this, selection| {
                                                this.when(
                                                    selection
                                                        .contains(base_address + cell_ix as u64),
                                                    |this| this.bg(Color::Accent.color(cx)),
                                                )
                                            },
                                        )
                                        .child(
                                            Label::new(HEX_BYTES_MEMOIZED[*cell as usize].clone())
                                                .buffer_font(cx)
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
                                                        this.view_state.selection = Some(
                                                            SelectedMemoryRange::DragUnderway(
                                                                drag.clone(),
                                                            ),
                                                        );
                                                    });

                                                    cx.new(|_| Empty)
                                                }
                                            },
                                        )
                                        .on_drop({
                                            let weak = weak.clone();
                                            move |drag: &Drag, _, cx| {
                                                _ = weak.update(cx, |this, _| {
                                                    this.view_state.selection = Some(
                                                        SelectedMemoryRange::DragComplete(Drag {
                                                            start_address: drag.start_address,
                                                            end_address: base_address
                                                                + cell_ix as u64,
                                                        }),
                                                    );
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

                                                // this.list_state.scroll_by(distance);
                                            });

                                            style
                                        })
                                })),
                        )
                        .child(
                            h_flex()
                                .id(("memory-view-row-ascii-memory", ix * view_state.line_width))
                                .h_full()
                                .px_1()
                                .mr_4()
                                .gap_x_1p5()
                                .border_x_1()
                                .border_color(Color::Muted.color(cx))
                                .children(memory.iter().map(|cell| {
                                    let as_character = char::from(*cell);
                                    let as_visible = if as_character.is_ascii_graphic() {
                                        as_character
                                    } else {
                                        '·'
                                    };
                                    Label::new(format!("{as_visible}"))
                                        .buffer_font(cx)
                                        .size(ui::LabelSize::Small)
                                })),
                        )
                        .overflow_x_scroll()
                        .into_any()
                }
            },
        );

        state.set_scroll_handler({
            let weak = cx.weak_entity();
            move |range, _, cx| {
                _ = weak.update(cx, |this, _| {
                    if range.visible_range.start == 0 {
                        this.view_state.schedule_scroll_up();
                    } else if range.visible_range.end as u64 == this.view_state.row_count() + 1 {
                        this.view_state.schedule_scroll_down();
                    }
                });
            }
        });
        let query_editor = cx.new(|cx| Editor::single_line(window, cx));
        let scroll_state = ScrollbarState::new(state.clone());
        Self {
            scroll_state,
            list_state: state,
            show_scrollbar: false,
            hide_scrollbar_task: None,
            focus_handle: cx.focus_handle(),
            view_state,
            query_editor,
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
                .on_mouse_move(cx.listener(|_, _, _, cx| {
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
    fn render_query_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        EditorElement::new(
            &self.query_editor,
            Self::editor_style(&self.query_editor, cx),
        )
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
}

impl Render for MemoryView {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        self.view_state.perform_scheduled_scroll();
        let this = cx.weak_entity();
        v_flex()
            .id("Memory-view")
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
                    .rounded_md()
                    .border_1()
                    .p_0p5()
                    .mb_0p5()
                    .bg(cx.theme().colors().editor_background)
                    .when_else(
                        self.query_editor
                            .focus_handle(cx)
                            .contains_focused(window, cx),
                        |this| this.border_color(cx.theme().colors().border_focused),
                        |this| this.border_color(cx.theme().colors().border_transparent),
                    )
                    .child(self.render_query_bar(cx)),
            )
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .size_full()
                    .child(list(self.list_state.clone()).size_full())
                    .children(self.render_vertical_scrollbar(cx))
                    .child(
                        div()
                            .absolute()
                            .id("memory-view-top-scroll-area")
                            .h_1_6()
                            .w_full()
                            .opacity(0.5)
                            .top_0()
                            .right_1()
                            .left_1()
                            .on_mouse_move({
                                let this = this.clone();
                                move |_, _, cx| {
                                    _ = this.update(cx, |this, _| {
                                        if this
                                            .view_state
                                            .selection
                                            .as_ref()
                                            .is_some_and(|selection| selection.is_dragging())
                                        {
                                            this.list_state.scroll_by(px(-100.));
                                        }
                                    });
                                    // style
                                }
                            }),
                    )
                    .child(
                        div()
                            .id("memory-view-bottom-scroll-area")
                            .absolute()
                            .h_1_6()
                            .w_full()
                            .opacity(0.5)
                            .bottom_0()
                            .right_1()
                            .left_1()
                            .on_mouse_move(move |_, _, cx| {
                                _ = this.update(cx, |this, _| {
                                    if this
                                        .view_state
                                        .selection
                                        .as_ref()
                                        .is_some_and(|selection| selection.is_dragging())
                                    {
                                        this.list_state.scroll_by(px(100.));
                                    }
                                });
                                // style
                            }),
                    ),
            )
    }
}
