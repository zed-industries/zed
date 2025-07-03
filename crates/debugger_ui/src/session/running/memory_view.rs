use std::{
    cell::OnceCell,
    rc::{Rc, Weak},
    sync::{Arc, LazyLock, atomic::AtomicUsize},
    time::Duration,
};

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    AppContext, Entity, FocusHandle, Focusable, ListState, MouseButton, Stateful, Task, TextStyle,
    list,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Color, Context, Div, Divider, Element, FluentBuilder, InteractiveElement,
    IntoElement, Label, LabelCommon, LineHeightStyle, ParentElement, Render, Scrollbar,
    ScrollbarState, SharedString, StatefulInteractiveElement, Styled, TextSize, Window, div,
    h_flex, px, v_flex,
};
use util::ResultExt;

pub(crate) struct MemoryView {
    list_state: ListState,
    scroll_state: Rc<ScrollbarState>,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    focus_handle: FocusHandle,
    view_state: Arc<ViewState>,
    query_editor: Entity<Editor>,
}

struct ViewState {
    /// Uppermost row index
    base_row: AtomicUsize,
    /// To implement the infinite scrolling feature, we track two row indices: base (applicable to current frame) and next frame.
    /// Whenever we render an item at a list boundary, we decrement/increment base index.
    next_row: AtomicUsize,
    /// How many cells per row do we have?
    line_width: AtomicUsize,
}

impl ViewState {
    fn new(address: usize, line_width: usize) -> Self {
        Self {
            base_row: address.into(),
            next_row: address.into(),
            line_width: line_width.into(),
        }
    }
    fn row_count(&self) -> usize {
        // This was picked fully arbitrarily. There's no incentive for us to care about page sizes other than the fact that it seems to be a good
        // middle ground for data size.
        const PAGE_SIZE: usize = 4096;
        PAGE_SIZE / self.line_width()
    }
    fn schedule_scroll_down(&self) {
        _ = self.next_row.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |ix| ix.checked_add(1),
        );
    }
    fn schedule_scroll_up(&self) {
        _ = self.next_row.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |ix| ix.checked_sub(1),
        );
    }
    fn perform_scheduled_scroll(&self) {
        self.base_row.store(
            self.next_row.load(std::sync::atomic::Ordering::Relaxed),
            std::sync::atomic::Ordering::Relaxed,
        );
    }
    fn base_row(&self) -> usize {
        self.base_row.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn line_width(&self) -> usize {
        self.line_width.load(std::sync::atomic::Ordering::Relaxed)
    }
}

static HEX_BYTES_MEMOIZED: LazyLock<[SharedString; 256]> =
    LazyLock::new(|| std::array::from_fn(|byte| SharedString::from(format!("{byte:02X}"))));

impl MemoryView {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let view_state = Arc::new(ViewState::new(0, 16));
        let weak_scroll_state: Rc<OnceCell<Weak<ScrollbarState>>> = Rc::new(OnceCell::new());
        let state = ListState::new(
            view_state.row_count(),
            gpui::ListAlignment::Top,
            px(100.),
            {
                let view_state = view_state.clone();
                let weak_scroll_state = weak_scroll_state.clone();
                move |ix, _, cx| {
                    let start = view_state.base_row();

                    let row_count = view_state.row_count();
                    let is_dragging = || {
                        weak_scroll_state.get().map_or(false, |state| {
                            state.upgrade().map_or(false, |state| state.is_dragging())
                        })
                    };
                    debug_assert!(row_count > 1);
                    if ix == row_count - 1 && is_dragging() {
                        view_state.schedule_scroll_down();
                    } else if ix == 0 && is_dragging() {
                        view_state.schedule_scroll_up();
                    }
                    let line_width = view_state.line_width();
                    let memory = (0..line_width)
                        .map(|cell_ix| {
                            (((start + ix) * line_width + cell_ix) % (u8::MAX as usize)) as u8
                        })
                        .collect::<Vec<_>>();
                    h_flex()
                        .id(("memory-view-row-full", ix * line_width))
                        .size_full()
                        .gap_x_2()
                        .child(
                            div()
                                .child(
                                    Label::new(format!("{:08X}", (start + ix) * line_width))
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
                                .id(("memory-view-row-raw-memory", ix * line_width))
                                .w_full()
                                .px_1()
                                .gap_1p5()
                                .children(memory.iter().map(|cell| {
                                    Label::new(HEX_BYTES_MEMOIZED[*cell as usize].clone())
                                        .buffer_font(cx)
                                        .size(ui::LabelSize::Small)
                                        .line_height_style(LineHeightStyle::UiLabel)
                                })),
                        )
                        .child(
                            h_flex()
                                .id(("memory-view-row-ascii-memory", ix * line_width))
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
                                        .line_height_style(LineHeightStyle::UiLabel)
                                })),
                        )
                        .overflow_x_scroll()
                        .into_any()
                }
            },
        );

        state.set_scroll_handler({
            let view_state = view_state.clone();
            move |range, _, _| {
                if range.visible_range.start == 0 {
                    view_state.schedule_scroll_up();
                } else if range.visible_range.end == view_state.row_count() + 1 {
                    view_state.schedule_scroll_down();
                }
            }
        });
        let query_editor = cx.new(|cx| Editor::single_line(window, cx));
        let scroll_state = Rc::new(ScrollbarState::new(state.clone()));
        _ = weak_scroll_state.set(Rc::downgrade(&scroll_state));
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
                .children(Scrollbar::vertical((*self.scroll_state).clone())),
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
                    .children(self.render_vertical_scrollbar(cx)),
            )
    }
}
