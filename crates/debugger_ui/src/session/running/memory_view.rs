use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    AppContext, Entity, FocusHandle, Focusable, ListHorizontalSizingBehavior, ListState,
    MouseButton, ScrollHandle, Stateful, Task, TextStyle, list,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Color, Context, Div, Divider, Element, FluentBuilder, InteractiveElement,
    IntoElement, Label, LabelCommon, LineHeightStyle, ParentElement, Render, Scrollbar,
    ScrollbarState, StatefulInteractiveElement, Styled, TextSize, Window, div, h_flex, px,
    relative, v_flex,
};
use util::ResultExt;

pub(crate) struct MemoryView {
    state: ListState,
    line_width: usize,
    scroll_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    focus_handle: FocusHandle,
    base_row_ix: Arc<AtomicUsize>,
    next_row_ix: Arc<AtomicUsize>,
    query_editor: Entity<Editor>,
}

impl MemoryView {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let line_width = 16;
        let scroll_handle = ScrollHandle::new();
        let base_row_ix = Arc::new(AtomicUsize::new(0));
        let next_row_ix = Arc::new(AtomicUsize::new(0));
        let middle = base_row_ix.clone();
        let next = next_row_ix.clone();
        let mut state = ListState::new(
            Self::list_rows(line_width),
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, window, cx| {
                let start = middle.load(std::sync::atomic::Ordering::Relaxed);

                if ix == 255 {
                    next.fetch_update(
                        std::sync::atomic::Ordering::Relaxed,
                        std::sync::atomic::Ordering::Relaxed,
                        |ix| ix.checked_add(1),
                    );
                } else if ix == 0 {
                    next.fetch_update(
                        std::sync::atomic::Ordering::Relaxed,
                        std::sync::atomic::Ordering::Relaxed,
                        |ix| ix.checked_sub(1),
                    );
                }
                h_flex()
                    .size_full()
                    .gap_2()
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
                            .id(("memory-view-row", ix * line_width))
                            .w_full()
                            .px_1()
                            .gap_1p5()
                            .children((0..line_width).map(|cell_ix| {
                                Label::new(format!(
                                    "{:02X}",
                                    ((start + ix) * line_width + cell_ix) % u8::MAX as usize
                                ))
                                .buffer_font(cx)
                                .size(ui::LabelSize::Small)
                                .line_height_style(LineHeightStyle::UiLabel)
                            }))
                            .overflow_x_scroll(),
                    )
                    .into_any()
            },
        );
        let query_editor = cx.new(|cx| Editor::single_line(window, cx));
        Self {
            scroll_state: ScrollbarState::new(state.clone()),
            state,
            line_width,
            base_row_ix,
            show_scrollbar: false,
            hide_scrollbar_task: None,
            focus_handle: cx.focus_handle(),
            next_row_ix,
            query_editor,
        }
    }

    fn list_rows(bytes_per_row: usize) -> usize {
        4096 / bytes_per_row
    }
    fn middle_address_to_range(start: usize) -> Range<usize> {
        let end = start + 256;
        start..end
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
        self.base_row_ix
            .store(self.next_row_ix.load(Ordering::Relaxed), Ordering::Relaxed);
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
            .child(
                v_flex()
                    .size_full()
                    .child(list(self.state.clone()).size_full())
                    .children(self.render_vertical_scrollbar(cx)),
            )
    }
}
