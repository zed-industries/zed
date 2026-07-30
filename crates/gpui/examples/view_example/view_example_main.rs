#![cfg_attr(target_family = "wasm", no_main)]

//! View example — composing a text input from the `View` primitives.
//!
//! The whole point: a text input is deceptively complicated, and `View` makes it
//! easy to compose one. Three pieces, each shown in its own section:
//!
//!   * `Editor`  — the workhorse entity: cursor, blink, focus, keyboard, and a
//!                 specialized text renderer. All the hard parts live here.
//!   * `Projection<String>` — the data plane. One `Profile` entity holds every
//!                 field; each component gets a projection of the one field it
//!                 touches, so nothing below needs to know the form exists.
//!   * `Input` / `TextArea` — the shaping layer. Each takes a projected string
//!                 (and grows the editor internally) OR an `Editor` (so you can
//!                 read the cursor).
//!
//! The projections are built inline in the element tree with `project!`, which
//! is the intended shape: a component is handed the field it edits, not the
//! struct that contains it.
//!
//! Run: `cargo run -p gpui --example view_example`

mod example_editor;
mod example_input;
mod example_text_area;

#[cfg(test)]
mod example_tests;

use example_editor::Editor;
use example_input::Input;
use example_text_area::TextArea;

use gpui::{
    App, Bounds, Context, Div, Entity, EntityId, IntoElement, KeyBinding, Projection,
    ProjectionMut, Render, SharedString, Window, WindowBounds, WindowOptions, actions, div, hsla,
    prelude::*, project, px, rgb, size,
};
use gpui_platform::application;

actions!(
    view_example,
    [Backspace, Delete, Left, Right, Home, End, Enter, Quit]
);

/// The whole form, in one entity. No component below ever receives this — they
/// get [`Projection`]s of individual fields, so a component that edits a name
/// works the same whether the name is a field here or a standalone entity.
struct Profile {
    primary: Person,
    emergency_contact: Person,
    bio: String,
}

struct Person {
    name: String,
    email: String,
}

/// A subform over *a* person, wherever that person lives. It projects `name` and
/// `email` out of a `ProjectionMut<Person>`, so the projections it builds are
/// two lenses deep — `Profile` to `Person` to `String` — without this component
/// knowing that, and without copying anything along the way.
#[derive(IntoElement)]
struct PersonForm {
    person: ProjectionMut<Person>,
}

impl gpui::View for PersonForm {
    fn entity_id(&self) -> Option<EntityId> {
        // Deliberately *not* the projection's id. A view's identity becomes the
        // notify target for state allocated inside it, so identifying a view by
        // a projection it also reads from feeds that state's notifications back
        // into the projection graph and spins forever. Positional identity is
        // enough here: the two subforms sit at different places in the tree.
        None
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(Input::new(project!(window, cx, &self.person, mut name)).width(px(280.)))
            .child(
                Input::new(project!(window, cx, &self.person, mut email))
                    .width(px(280.))
                    .color(hsla(0., 0., 0.3, 1.)),
            )
    }
}

/// A stateless readout of a projected string, rendered far from the input that
/// writes it: a read-only `Projection<String>` in, no subscription, no wiring.
#[derive(IntoElement)]
struct FieldReadout {
    label: &'static str,
    value: Projection<String>,
}

impl gpui::RenderOnce for FieldReadout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = self.value.read(cx);
        div()
            .text_sm()
            .text_color(hsla(0., 0., 0.45, 1.))
            .child(SharedString::from(format!("{}: {value}", self.label)))
    }
}

/// A tiny stateless view that reads an editor's cursor and is composed *beside*
/// the thing editing it — two views over one entity, zero wiring.
#[derive(IntoElement)]
struct CursorReadout {
    editor: Entity<Editor>,
}

impl CursorReadout {
    fn new(editor: Entity<Editor>) -> Self {
        Self { editor }
    }
}

impl gpui::RenderOnce for CursorReadout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let cursor = self.editor.read(cx).cursor;
        div()
            .text_sm()
            .text_color(hsla(0., 0., 0.45, 1.))
            .child(SharedString::from(format!("cursor @ {cursor}")))
    }
}

struct ViewExample;

impl ViewExample {
    fn new() -> Self {
        Self
    }
}

impl Render for ViewExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The data plane: one entity for the whole form. Fields are handed out
        // below as projections, built inline where they're used.
        let profile = window.use_state(cx, |_, _| Profile {
            primary: Person {
                name: String::new(),
                email: String::from("me@example.com"),
            },
            emergency_contact: Person {
                name: String::new(),
                email: String::new(),
            },
            bio: String::new(),
        });
        // Editors that own their own string internally — no extra wiring up top.
        let notes = window.use_state(cx, |window, cx| Editor::new("multi\nline", window, cx));
        let owned = window.use_state(cx, |window, cx| Editor::new("editable", window, cx));

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf0f0f0))
            .p(px(24.))
            .gap(px(24.))
            // One component, two people, one entity. Each subform is handed a
            // projected `Person` and projects further from there.
            .child(
                section("Subforms — the same component over two projected people").child(
                    div()
                        .flex()
                        .gap(px(16.))
                        .child(PersonForm {
                            person: project!(window, cx, &profile, mut primary),
                        })
                        .child(PersonForm {
                            person: project!(window, cx, &profile, mut emergency_contact),
                        }),
                ),
            )
            // Read-only projections of the very same fields, reached by path
            // from the root instead of through the subform. Type above and these
            // update, because a projection read during render subscribes the
            // reader to its source.
            .child(
                section("Read-only projections — the same fields, somewhere else")
                    .child(FieldReadout {
                        label: "name",
                        value: project!(window, cx, &profile, primary.name),
                    })
                    .child(FieldReadout {
                        label: "contact",
                        value: project!(window, cx, &profile, emergency_contact.name),
                    }),
            )
            .child(
                section("Input — from an Editor (read its cursor beside it)").child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .child(Input::editor(owned.clone()).width(px(320.)))
                        .child(CursorReadout::new(owned)),
                ),
            )
            .child(
                section("Text areas — from a projected field, or from an Editor")
                    .child(TextArea::new(project!(window, cx, &profile, mut bio), 3))
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap(px(12.))
                            .child(TextArea::editor(notes.clone(), 3).color(hsla(
                                250. / 360.,
                                0.7,
                                0.4,
                                1.,
                            )))
                            .child(CursorReadout::new(notes)),
                    ),
            )
    }
}

/// A labeled vertical section.
fn section(title: &str) -> Div {
    div().flex().flex_col().gap(px(8.)).child(
        div()
            .text_sm()
            .text_color(hsla(0., 0., 0.3, 1.))
            .child(SharedString::from(title.to_string())),
    )
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(560.0), px(480.0)), cx);
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("enter", Enter, None),
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ViewExample::new()),
        )
        .unwrap();

        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
