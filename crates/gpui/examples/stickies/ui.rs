use gpui::{
    AnyWindowHandle, App, Bounds, ClickEvent, Context, CursorStyle, ElementId, Entity, FocusHandle,
    Focusable, Hsla, Pixels, Point, SharedString, Size, Window, WindowBounds, WindowKind,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use std::sync::atomic::Ordering;

use crate::{
    ChangeColorBlue, ChangeColorGray, ChangeColorGreen, ChangeColorPink, ChangeColorPurple,
    ChangeColorYellow, CloseNote, DEFAULT_STICKY_SIZE, NewNote, STICKY_COUNT, TITLEBAR_HEIGHT,
    ZoomWindow, text_area::TextArea,
};

#[derive(Clone, Default, Debug)]
pub enum StickyColor {
    #[default]
    Yellow,
    Blue,
    Green,
    Pink,
    Purple,
    Gray,
}

impl StickyColor {
    fn bg(&self) -> Hsla {
        match self {
            StickyColor::Yellow => rgb(0xFFF48F).into(),
            StickyColor::Blue => rgb(0x98F6FF).into(),
            StickyColor::Green => rgb(0x9BFF88).into(),
            StickyColor::Pink => rgb(0xFFB3E0).into(),
            StickyColor::Purple => rgb(0xD0B3FF).into(),
            StickyColor::Gray => rgb(0xD4D4D4).into(),
        }
    }

    fn titlebar_bg(&self) -> Hsla {
        match self {
            StickyColor::Yellow => rgb(0xFFE900).into(),
            StickyColor::Blue => rgb(0x5DF3FF).into(),
            StickyColor::Green => rgb(0x6FFF52).into(),
            StickyColor::Pink => rgb(0xFF8FD0).into(),
            StickyColor::Purple => rgb(0xB88FFF).into(),
            StickyColor::Gray => rgb(0xB0B0B0).into(),
        }
    }

    // the cmd+number modifier for this color
    // used to bind the color changing shortcuts
    // fn cmd_number(&self) -> u8 {
    //     match self {
    //         StickyColor::Yellow => 1,
    //         StickyColor::Blue => 2,
    //         StickyColor::Green => 3,
    //         StickyColor::Pink => 4,
    //         StickyColor::Purple => 5,
    //         StickyColor::Gray => 6,
    //     }
    // }
}

pub struct Sticky {
    id: ElementId,
    focus_handle: FocusHandle,
    bounds: Bounds<Pixels>,
    color: StickyColor,
    collapsed: bool,
    original_size: Option<Size<Pixels>>,
    content: SharedString,
    window_handle: Option<AnyWindowHandle>,
    text_area: Entity<TextArea>,
}

impl Sticky {
    pub fn new(
        cx: &mut App,
        id: impl Into<ElementId>,
        bounds: Bounds<Pixels>,
        color: StickyColor,
    ) -> Self {
        let text_area = cx.new(|cx| TextArea::new(cx));
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            bounds,
            color,
            collapsed: false,
            original_size: None,
            content: SharedString::new(""),
            window_handle: None,
            text_area,
        }
    }

    pub fn content(mut self, content: impl Into<SharedString>, cx: &mut App) -> Self {
        self.content = content.into();
        // Set initial content in the text area
        self.text_area.update(cx, |area, cx| {
            area.set_content(&self.content, cx);
        });
        self
    }

    // pub fn collapsed(&self) -> bool {
    //     self.collapsed
    // }

    pub fn toggle_collapsed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;

        if self.collapsed {
            // Store original size if this is the first collapse
            if self.original_size.is_none() {
                let current_bounds = window.window_bounds().get_bounds();
                self.original_size = Some(current_bounds.size);
            }
            // Resize to titlebar height only
            window.resize(size(self.bounds.size.width, px(TITLEBAR_HEIGHT)));
        } else {
            // Restore original size
            if let Some(original_size) = self.original_size {
                window.resize(original_size);
            }
        }

        cx.notify();
    }

    pub fn with_window_handle(mut self, handle: AnyWindowHandle) -> Self {
        self.window_handle = Some(handle);
        self
    }

    fn close_note(&mut self, _: &CloseNote, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }

    fn zoom(&mut self, _: &ZoomWindow, window: &mut Window, _cx: &mut Context<Self>) {
        window.zoom_window();
    }

    fn change_color(&mut self, color: StickyColor, _window: &mut Window, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }

    fn new_note(&mut self, _: &NewNote, window: &mut Window, cx: &mut Context<Self>) {
        let current_bounds = window.window_bounds().get_bounds();
        let screen_bounds = window.display(cx).map(|d| d.bounds()).unwrap_or(Bounds {
            origin: Point::default(),
            size: size(px(1920.), px(1080.)),
        });

        let offset = px(24.);
        let new_size = size(px(DEFAULT_STICKY_SIZE.0), px(DEFAULT_STICKY_SIZE.1));

        // Try to place new note below current one
        let mut new_origin = Point {
            x: current_bounds.origin.x + offset,
            y: current_bounds.origin.y + current_bounds.size.height + offset,
        };

        // If it would go off the bottom of the screen, place it above instead
        if new_origin.y + new_size.height > screen_bounds.bottom() {
            new_origin.y = current_bounds.origin.y - new_size.height - offset;
        }

        let new_bounds = Bounds {
            origin: new_origin,
            size: new_size,
        };

        let sticky_id = STICKY_COUNT.fetch_add(1, Ordering::SeqCst) as u64;

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(new_bounds)),
                titlebar: None,
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                ..Default::default()
            },
            |window, cx| {
                let note = cx.new(|cx| {
                    let window_handle = window.window_handle();
                    Sticky::new(
                        cx,
                        ElementId::NamedInteger("sticky".into(), sticky_id),
                        new_bounds,
                        StickyColor::Yellow,
                    )
                    .with_window_handle(window_handle)
                });
                // Focus the TextArea directly instead of the Sticky container
                let text_area_handle = note.read(cx).text_area.read(cx).focus_handle(cx);

                window.activate_window();
                window.focus(&text_area_handle);

                note
            },
        )
        .unwrap();
    }
}

impl Render for Sticky {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let window_active = window.is_window_active();

        div()
            .id(self.id.clone())
            .key_context("Note")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::close_note))
            .on_action(cx.listener(Self::new_note))
            .on_action(cx.listener(Self::zoom))
            .on_action(cx.listener(|this, _: &ChangeColorYellow, window, cx| {
                this.change_color(StickyColor::Yellow, window, cx)
            }))
            .on_action(cx.listener(|this, _: &ChangeColorBlue, window, cx| {
                this.change_color(StickyColor::Blue, window, cx)
            }))
            .on_action(cx.listener(|this, _: &ChangeColorGreen, window, cx| {
                this.change_color(StickyColor::Green, window, cx)
            }))
            .on_action(cx.listener(|this, _: &ChangeColorPink, window, cx| {
                this.change_color(StickyColor::Pink, window, cx)
            }))
            .on_action(cx.listener(|this, _: &ChangeColorPurple, window, cx| {
                this.change_color(StickyColor::Purple, window, cx)
            }))
            .on_action(cx.listener(|this, _: &ChangeColorGray, window, cx| {
                this.change_color(StickyColor::Gray, window, cx)
            }))
            .relative()
            .bg(self.color.bg())
            .border_1()
            .border_color(gpui::black())
            .w(self.bounds.size.width)
            .h(self.bounds.size.height)
            .flex_col()
            .text_size(px(12.))
            .line_height(px(14.))
            .pt(px(TITLEBAR_HEIGHT)) // reserve space for absolutely positioned titlebar
            .on_click(cx.listener(move |this, _, window, cx| {
                if !window.is_window_active() {
                    window.activate_window();
                }
                // Focus the TextArea instead of the Sticky container
                let text_area_handle = this.text_area.read(cx).focus_handle(cx);
                window.focus(&text_area_handle);
                cx.notify();
            }))
            .when(self.collapsed, |this| {
                this.h(px(TITLEBAR_HEIGHT * 3.0)).overflow_hidden()
            })
            .when(!self.collapsed, |this| {
                this.child(Titlebar::new(entity, window_active))
                    .child(self.text_area.clone())
            })
    }
}

impl Focusable for Sticky {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct Titlebar {
    sticky: Entity<Sticky>,
    window_active: bool,
}

impl Titlebar {
    pub fn new(sticky: Entity<Sticky>, window_active: bool) -> Self {
        Self {
            sticky,
            window_active,
        }
    }
}

impl RenderOnce for Titlebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.sticky.clone();
        let sticky_state = self.sticky.read(cx);
        let note_preview = sticky_state.text_area.read(cx).content();
        let note_preview = if note_preview.len() > 45 {
            format!("{}...", &note_preview[..45])
        } else {
            note_preview.to_string()
        };
        let note_preview = SharedString::new(note_preview);
        let color = sticky_state.color.clone();
        let collapsed = sticky_state.collapsed;

        div()
            .id("titlebar")
            .cursor(CursorStyle::Arrow)
            .active(|style| style.cursor(CursorStyle::ClosedHand))
            .absolute()
            .left_0()
            // this bit prevents some cursor flashing when hovering on
            // the very top pixel of the window
            .top(-px(1.0))
            .h(px(TITLEBAR_HEIGHT) + px(2.0))
            .w_full()
            .text_size(px(10.))
            .on_click(move |click: &ClickEvent, window, cx| {
                if click.click_count() < 2 {
                    return;
                }

                let state = state.clone();

                state.update(cx, |state, cx| {
                    state.toggle_collapsed(window, cx);
                });
            })
            // .on_click(|event, window, cx: &mut App| {
            //     match event {
            //         ClickEvent::Mouse(),
            //         _ => {},
            //     }
            // })
            .when(self.window_active, |this| this.bg(color.titlebar_bg()))
            .when(!collapsed, |this| this.child(note_preview))
    }
}
