#![allow(unused, dead_code)]

use gpui::*;

// ============================================================
// Helper types for tests
// ============================================================

struct Editor;

impl Editor {
    fn set_text(&mut self, _text: &str) {}
    fn text(&self) -> String {
        String::new()
    }
}

struct Counter {
    count: u32,
    editor: Entity<Editor>,
}

struct CounterOnce {
    editor: Entity<Editor>,
    weak_editor: WeakEntity<Editor>,
}

// ============================================================
// entity_update_in_render — SHOULD WARN
// ============================================================

// Entity::update with unit-returning closure in Render::render
impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.update(cx, |editor, _cx| {
            editor.set_text("hello");
        });
        ()
    }
}

// Entity::update with unit-returning closure in RenderOnce::render
impl RenderOnce for CounterOnce {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.editor.update(cx, |editor, _cx| {
            editor.set_text("hello");
        });
        ()
    }
}

// WeakEntity::update with unit-returning closure in RenderOnce::render
struct WeakUpdater {
    weak_editor: WeakEntity<Editor>,
}

impl RenderOnce for WeakUpdater {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let _ = self.weak_editor.update(cx, |editor, _cx| {
            editor.set_text("world");
        });
        ()
    }
}

// ============================================================
// entity_update_in_render — SHOULD NOT WARN
// ============================================================

// Entity::update with value-returning closure (reading, not mutating)
struct ReaderView {
    editor: Entity<Editor>,
}

impl RenderOnce for ReaderView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let _text = self.editor.update(cx, |editor, _cx| editor.text());
        ()
    }
}

// Entity::update outside render entirely
fn update_outside_render(editor: &Entity<Editor>, cx: &mut App) {
    editor.update(cx, |editor, _cx| {
        editor.set_text("fine here");
    });
}

// Entity::read in render (not update)
struct ReadOnlyView {
    editor: Entity<Editor>,
}

impl RenderOnce for ReadOnlyView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let _editor = self.editor.read(cx);
        ()
    }
}

// Entity::update inside a closure in render (simulating an event handler
// that executes later, not during render itself)
struct ClosureView {
    editor: Entity<Editor>,
}

impl RenderOnce for ClosureView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let editor = self.editor;
        let _handler = move |cx: &mut App| {
            editor.update(cx, |editor, _cx| {
                editor.set_text("inside closure");
            });
        };
        ()
    }
}

// ============================================================
// notify_in_render — SHOULD WARN
// ============================================================

struct NotifyView {
    count: u32,
}

impl Render for NotifyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.count += 1;
        cx.notify();
        ()
    }
}

// ============================================================
// notify_in_render — SHOULD NOT WARN
// ============================================================

// notify outside render
fn notify_outside_render<T>(cx: &mut Context<'_, T>) {
    cx.notify();
}

// notify inside a closure in render (event handler — runs later)
struct NotifyClosureView;

impl Render for NotifyClosureView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _handler = |cx: &mut Context<'_, Self>| {
            cx.notify();
        };
        ()
    }
}
