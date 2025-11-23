use gpui::{
    AnyWindowHandle, App, Application, Bounds, Context, ElementId, Entity, FocusHandle, Focusable,
    Hsla, KeyBinding, Pixels, Point, SharedString, Window, WindowBounds, WindowKind, WindowOptions,
    actions, div, prelude::*, px, rgb, size,
};
use std::sync::atomic::{AtomicUsize, Ordering};

actions!(stickies, [NewNote, CloseNote, ZoomNote]);

static STICKY_COUNT: AtomicUsize = AtomicUsize::new(0);

// Window setup
// - [ ] open two stickies, one blue, one yellow
// - [ ] blue behind, yellow in front
// - [ ] black border around window

// Titlebar
// - [ ] only focused window shows titlebar
// - [ ] titlebar is darker version of content area color
// - [ ] double-clicking the titlebar minimizes it (and shows a tiny snippet of the content)
// - [ ] (left) - square closes app, (right) - triangle zooms, - minus icon collapses/expands sticky

// Content area
// - [ ] content area is themed by a color, one of 6 options: blue, green, yellow, pink, purple, gray

// Menubar
// - [ ] has a menubar with:
//   - [ ] File: New, Close
//   - [ ] Edit: Undo, Redo, Cut, Copy, Paste
//   - [ ] Font
//   - [ ] Color: Blue, Green, Yellow, Pink, Purple, Gray
//   - [ ] Window

pub const TITLEBAR_HEIGHT: f32 = 12.;

pub const DEFAULT_STICKY_SIZE: (f32, f32) = (354., 262.);

pub const YELLOW_STICKY_CONTENT: &str = r#"Make a note of it!

Stickies lets you keep notes (like these) on your desktop. Use a Stickies note to jot down reminders, lists, or other information. You can also use notes to store frequently used text or graphics.

• To close this note, click the close button.

• To collapse this note, double click the title bar.

Your current notes appear when you open Stickies.
"#;

pub const BLUE_STICKY_CONTENT: &str = r#"It’s easy to customize your notes.

Make your notes stand out and get noticed.

• Format text using different fonts and font sizes
• Add emphasis with bold and italic text styles or color.
• Include graphics ￼ .

Stickies has lots of other great features, including a spell checker, import and export features, and other ways to arrange and customize your notes. Plus, you’ll find a “Make New Sticky Note” service in many applications.

Look in Help to find out more about using Stickies.
"#;

struct TextArea {}

#[derive(Clone, Default, Debug)]
enum StickyColor {
    #[default]
    Yellow,
    Blue,
    // Green,
    // Pink,
    // Purple,
    // Gray,
}

impl StickyColor {
    fn bg(&self) -> Hsla {
        match self {
            StickyColor::Yellow => rgb(0xFFF48F).into(),
            StickyColor::Blue => rgb(0x98F6FF).into(),
        }
    }

    fn titlebar_bg(&self) -> Hsla {
        match self {
            StickyColor::Yellow => rgb(0xFFE900).into(),
            StickyColor::Blue => rgb(0x5DF3FF).into(),
        }
    }

    /// the cmd+number modifier for this color
    /// used to bind the color changing shortcuts
    fn cmd_number(&self) -> u8 {
        match self {
            StickyColor::Yellow => 1,
            StickyColor::Blue => 2,
        }
    }
}

struct Sticky {
    id: ElementId,
    focus_handle: FocusHandle,
    bounds: Bounds<Pixels>,
    color: StickyColor,
    collapsed: bool,
    zoomed: bool,
    content: SharedString,
    window_handle: Option<AnyWindowHandle>,
    // text_area: Entity<TextArea>,
}

impl Sticky {
    pub fn new(
        cx: &mut App,
        id: impl Into<ElementId>,
        bounds: Bounds<Pixels>,
        color: StickyColor,
    ) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            bounds,
            color,
            collapsed: false,
            zoomed: false,
            content: SharedString::new(""),
            window_handle: None,
        }
    }

    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }

    pub fn with_window_handle(mut self, handle: AnyWindowHandle) -> Self {
        self.window_handle = Some(handle);
        self
    }

    fn close_note(&mut self, _: &CloseNote, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }

    fn zoom(&mut self, _: &ZoomNote, window: &mut Window, _cx: &mut Context<Self>) {
        window.zoom_window();
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
                cx.new(|cx| {
                    let window_handle = window.window_handle();
                    Sticky::new(
                        cx,
                        ElementId::NamedInteger("sticky".into(), sticky_id),
                        new_bounds,
                        StickyColor::Yellow,
                    )
                    .with_window_handle(window_handle.into())
                })
            },
        )
        .unwrap();
    }
}

impl Render for Sticky {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let window_active = window.is_window_active();
        let focus_handle = self.focus_handle.clone();

        div()
            .id(self.id.clone())
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
            .on_click(cx.listener(move |_, _, window, cx| {
                if !window.is_window_active() {
                    window.activate_window();
                }
                window.focus(&focus_handle);
                cx.notify();
            }))
            .on_action(cx.listener(Self::close_note))
            .on_action(cx.listener(Self::new_note))
            .child(Titlebar::new(entity, window_active))
            .child(
                div()
                    .flex_1()
                    .py(px(8.))
                    .px(px(14.))
                    .child(self.content.clone()),
            )
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
        let sticky_state = self.sticky.read(cx);
        let color = sticky_state.color.clone();

        div()
            .absolute()
            .top_0()
            .left_0()
            .h(px(12.))
            // todo: probably needs to get the width from `sticky_state`
            .w_full()
            .when(self.window_active, |this| this.bg(color.titlebar_bg()))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Register key bindings
        cx.bind_keys([
            KeyBinding::new("cmd-w", CloseNote, None),
            KeyBinding::new("cmd-n", NewNote, None),
        ]);

        let offset = px(24.);

        let first_screen = cx.displays().first().unwrap().clone(); // if you don't have at least one display what are you doing here?
        let screen_bounds = first_screen.bounds();

        let window_options = |bounds: Bounds<Pixels>, focus: bool| WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            display_id: Some(first_screen.id()),
            titlebar: None,
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            focus,
            show: true,
            kind: WindowKind::Normal,
            ..Default::default()
        };

        let blue_bounds = Bounds {
            origin: Point {
                x: screen_bounds.origin.x + px(16.),
                y: screen_bounds.bottom() - px(DEFAULT_STICKY_SIZE.1) - px(16.),
            },
            size: size(px(DEFAULT_STICKY_SIZE.0), px(DEFAULT_STICKY_SIZE.1)),
        };

        cx.open_window(window_options(blue_bounds, false), |window, cx| {
            cx.new(|cx| {
                let window_handle = window.window_handle();
                Sticky::new(cx, "sticky-1", blue_bounds, StickyColor::Blue)
                    .content(SharedString::new_static(BLUE_STICKY_CONTENT))
                    .with_window_handle(window_handle.into())
            })
        })
        .unwrap();

        STICKY_COUNT.store(2, Ordering::SeqCst);

        let yellow_bounds = Bounds {
            origin: Point {
                x: blue_bounds.origin.x + offset,
                y: blue_bounds.origin.y - offset,
            },
            size: blue_bounds.size,
        };
        cx.open_window(window_options(yellow_bounds, true), |window, cx| {
            cx.new(|cx| {
                let window_handle = window.window_handle();
                Sticky::new(cx, "sticky-2", yellow_bounds, StickyColor::Yellow)
                    .content(SharedString::new_static(YELLOW_STICKY_CONTENT))
                    .with_window_handle(window_handle.into())
            })
        })
        .unwrap();

        cx.activate(true);
    });
}
