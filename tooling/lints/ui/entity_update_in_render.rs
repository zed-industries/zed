// Tests for the `entity_update_in_render` lint.

#![allow(unused)]

extern crate gpui;

use gpui::*;

struct Counter {
    value: i32,
}

// ============================================================
// SHOULD WARN — .update() returning () inside Render::render
// ============================================================

struct MutatingView {
    counter: Entity<Counter>,
}

impl Render for MutatingView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.counter.update(cx, |counter, _cx| {
            counter.value += 1;
        });
        ()
    }
}

// WeakEntity::update returning Result<(), _> inside Render::render
struct WeakMutatingView {
    counter: WeakEntity<Counter>,
}

impl Render for WeakMutatingView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = self.counter.update(cx, |counter, _cx| {
            counter.value += 1;
        });
        ()
    }
}

// Entity::update returning () inside RenderOnce::render
struct OnceMutatingView {
    counter: Entity<Counter>,
}

impl RenderOnce for OnceMutatingView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.counter.update(cx, |counter, _cx| {
            counter.value += 1;
        });
        ()
    }
}

// ============================================================
// SHOULD NOT WARN
// ============================================================

// .update() returning a value (reading, not mutating)
struct ReadingView {
    counter: Entity<Counter>,
}

impl Render for ReadingView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _val: i32 = self.counter.update(cx, |counter, _cx| counter.value);
        ()
    }
}

// .update() inside a closure (e.g. event handler), not directly in render
struct ClosureView {
    counter: Entity<Counter>,
}

impl Render for ClosureView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let counter = &self.counter;
        let _handler = |cx: &mut App| {
            counter.update(cx, |counter, _cx| {
                counter.value += 1;
            });
        };
        ()
    }
}

// .update() outside of render entirely
fn update_outside_render(entity: &Entity<Counter>, cx: &mut App) {
    entity.update(cx, |counter, _cx| {
        counter.value += 1;
    });
}

// .update() on a non-gpui type (unrelated method named "update")
struct FakeEntity;

impl FakeEntity {
    fn update(&self) {}
}

struct FakeView {
    thing: FakeEntity,
}

impl Render for FakeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.thing.update();
        ()
    }
}

fn main() {}
