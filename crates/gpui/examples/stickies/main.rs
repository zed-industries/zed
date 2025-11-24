use gpui::{
    App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem, Pixels, Point, SharedString,
    SystemMenuType, WindowBounds, WindowKind, WindowOptions, actions, px, size,
};
use std::sync::atomic::{AtomicUsize, Ordering};

mod text_area;
mod ui;

use text_area::{
    Backspace as TextAreaBackspace, Delete as TextAreaDelete, Down, End as TextAreaEnd, Enter,
    Home as TextAreaHome, Left, Right, SelectDown, SelectLeft, SelectRight, SelectUp, Up,
};
use ui::{Sticky, StickyColor};

actions!(
    stickies,
    [
        AlignLeft,
        AlignRight,
        ArrangeBy,
        Bigger,
        Bold,
        BringAllToFront,
        Center,
        ChangeColorBlue,
        ChangeColorGray,
        ChangeColorGreen,
        ChangeColorPink,
        ChangeColorPurple,
        ChangeColorYellow,
        CloseNote,
        CollapseWindow,
        Copy,
        Cut,
        Delete,
        ExportText,
        Find,
        FindNext,
        FindPrevious,
        FloatOnTop,
        HideOthers,
        HideStickies,
        ImportText,
        Italic,
        Justify,
        MinimizeWindow,
        NewNote,
        PageSetup,
        Paste,
        Print,
        Quit,
        Redo,
        SelectAll,
        ShowAll,
        ShowFonts,
        Smaller,
        SpellingAndGrammar,
        Underline,
        Undo,
        WritingDirection,
        ZoomNote,
        ZoomWindow,
    ]
);

static STICKY_COUNT: AtomicUsize = AtomicUsize::new(0);

// Window setup
// - [x] open two stickies, one blue, one yellow
// - [x] blue behind, yellow in front
// - [x] black border around window

// Titlebar
// - [x] only focused window shows titlebar
// - [x] titlebar is darker version of content area color
// - [ ] double-clicking the titlebar minimizes it (and shows a tiny snippet of the content)
// - [ ] (left) - square closes app, (right) - triangle zooms, - minus icon collapses/expands sticky

// Content area
// - [x] content area is themed by a color, one of 6 options: blue, green, yellow, pink, purple, gray

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

fn setup_menus(cx: &mut App) {
    cx.set_menus(vec![
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New Note", NewNote),
                MenuItem::separator(),
                MenuItem::action("Close", CloseNote),
                MenuItem::separator(),
                MenuItem::action("Import Text...", ImportText),
                MenuItem::action("Export Text...", ExportText),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::action("Undo", Undo),
                MenuItem::action("Redo", Redo),
                MenuItem::separator(),
                MenuItem::action("Cut", Cut),
                MenuItem::action("Copy", Copy),
                MenuItem::action("Paste", Paste),
                MenuItem::action("Delete", Delete),
                MenuItem::action("Select All", SelectAll),
                MenuItem::separator(),
                MenuItem::action("Find", Find),
                MenuItem::action("Find Next", FindNext),
                MenuItem::action("Find Previous", FindPrevious),
                MenuItem::separator(),
                MenuItem::action("Spelling and Grammar", SpellingAndGrammar),
            ],
        },
        Menu {
            name: "Font".into(),
            items: vec![
                MenuItem::action("Show Fonts", ShowFonts),
                MenuItem::separator(),
                MenuItem::action("Bold", Bold),
                MenuItem::action("Italic", Italic),
                MenuItem::action("Underline", Underline),
                MenuItem::separator(),
                MenuItem::action("Bigger", Bigger),
                MenuItem::action("Smaller", Smaller),
                MenuItem::separator(),
                MenuItem::action("Align Left", AlignLeft),
                MenuItem::action("Center", Center),
                MenuItem::action("Justify", Justify),
                MenuItem::action("Align Right", AlignRight),
                MenuItem::separator(),
                MenuItem::action("Writing Direction", WritingDirection),
            ],
        },
        Menu {
            name: "Color".into(),
            items: vec![
                MenuItem::action("Yellow", ChangeColorYellow),
                MenuItem::action("Blue", ChangeColorBlue),
                MenuItem::action("Green", ChangeColorGreen),
                MenuItem::action("Pink", ChangeColorPink),
                MenuItem::action("Purple", ChangeColorPurple),
                MenuItem::action("Gray", ChangeColorGray),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![
                MenuItem::action("Minimize", MinimizeWindow),
                MenuItem::action("Zoom", ZoomWindow),
                MenuItem::action("Float on Top", FloatOnTop),
                MenuItem::separator(),
                MenuItem::action("Collapse", CollapseWindow),
                MenuItem::action("Arrange By", ArrangeBy),
                MenuItem::separator(),
                MenuItem::action("Bring All to Front", BringAllToFront),
                MenuItem::separator(),
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Hide Stickies", HideStickies),
                MenuItem::action("Hide Others", HideOthers),
                MenuItem::action("Show All", ShowAll),
            ],
        },
    ]);
}

fn main() {
    Application::new().run(|cx: &mut App| {
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
                    .content(SharedString::new_static(BLUE_STICKY_CONTENT), cx)
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
                    .content(SharedString::new_static(YELLOW_STICKY_CONTENT), cx)
                    .with_window_handle(window_handle.into())
            })
        })
        .unwrap();

        cx.activate(true);

        // Set up menus
        setup_menus(cx);

        // Register global action handlers
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &BringAllToFront, cx| {
            for window in cx.windows() {
                window
                    .update(cx, |_, window, _| window.activate_window())
                    .ok();
            }
        });

        // Register key bindings
        cx.bind_keys([
            KeyBinding::new("cmd-w", CloseNote, None),
            KeyBinding::new("cmd-n", NewNote, None),
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-1", ChangeColorYellow, None),
            KeyBinding::new("cmd-2", ChangeColorBlue, None),
            KeyBinding::new("cmd-3", ChangeColorGreen, None),
            KeyBinding::new("cmd-4", ChangeColorPink, None),
            KeyBinding::new("cmd-5", ChangeColorPurple, None),
            KeyBinding::new("cmd-6", ChangeColorGray, None),
            KeyBinding::new("cmd-m", MinimizeWindow, None),
            // Text area key bindings
            KeyBinding::new("backspace", TextAreaBackspace, None),
            KeyBinding::new("delete", TextAreaDelete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("up", Up, None),
            KeyBinding::new("down", Down, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("shift-up", SelectUp, None),
            KeyBinding::new("shift-down", SelectDown, None),
            KeyBinding::new("home", TextAreaHome, None),
            KeyBinding::new("end", TextAreaEnd, None),
            KeyBinding::new("enter", Enter, None),
        ]);
    });
}
