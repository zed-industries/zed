//! Tests for the input composition. Require the `test-support` feature:
//!
//! ```sh
//! cargo test -p gpui --example view_example --features test-support
//! ```

#[cfg(test)]
mod tests {
    use gpui::{Context, Entity, KeyBinding, TestAppContext, Window, prelude::*};

    use crate::example_editor::Editor;
    use crate::example_input::Input;
    use crate::{Backspace, Delete, End, Home, Left, Right};

    /// Two inputs, each backed by an editor we own (so the test can focus and
    /// read them). Proves data flows through the shared `String` and that
    /// sibling inputs stay isolated.
    struct Harness {
        a: Entity<Editor>,
        b: Entity<Editor>,
    }

    impl Render for Harness {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            gpui::div()
                .child(Input::editor(self.a.clone()))
                .child(Input::editor(self.b.clone()))
        }
    }

    fn bind_keys(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new("backspace", Backspace, None),
                KeyBinding::new("delete", Delete, None),
                KeyBinding::new("left", Left, None),
                KeyBinding::new("right", Right, None),
                KeyBinding::new("home", Home, None),
                KeyBinding::new("end", End, None),
            ]);
        });
    }

    fn setup(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Editor>,
        Entity<String>,
        Entity<String>,
        &mut gpui::VisualTestContext,
    ) {
        bind_keys(cx);

        let (harness, cx) = cx.add_window_view(|window, cx| {
            let a_value = cx.new(|_| String::new());
            let b_value = cx.new(|_| String::new());
            let a = cx.new(|cx| Editor::over(a_value, window, cx));
            let b = cx.new(|cx| Editor::over(b_value, window, cx));
            Harness { a, b }
        });

        let a = cx.read_entity(&harness, |h, _| h.a.clone());
        let b = cx.read_entity(&harness, |h, _| h.b.clone());
        let a_value = cx.read_entity(&a, |e, _| e.value.clone());
        let b_value = cx.read_entity(&b, |e, _| e.value.clone());

        // Focus the first input's editor.
        cx.update(|window, cx| {
            let focus_handle = a.read(cx).focus_handle.clone();
            window.focus(&focus_handle, cx);
        });

        (a, a_value, b_value, cx)
    }

    #[gpui::test]
    fn typing_updates_the_shared_string(cx: &mut TestAppContext) {
        let (editor, a_value, _b_value, cx) = setup(cx);

        cx.simulate_input("hello");

        cx.read_entity(&a_value, |value, _| assert_eq!(value, "hello"));
        cx.read_entity(&editor, |editor, _| assert_eq!(editor.cursor, 5));
    }

    #[gpui::test]
    fn sibling_inputs_are_isolated(cx: &mut TestAppContext) {
        let (_editor, a_value, b_value, cx) = setup(cx);

        cx.simulate_input("x");

        cx.read_entity(&a_value, |value, _| assert_eq!(value, "x"));
        cx.read_entity(&b_value, |value, _| {
            assert_eq!(value, "", "typing in input A must not touch input B")
        });
    }

    #[gpui::test]
    fn external_writes_clamp_the_cursor(cx: &mut TestAppContext) {
        let (editor, a_value, _b_value, cx) = setup(cx);

        cx.simulate_input("hello");
        cx.read_entity(&editor, |editor, _| assert_eq!(editor.cursor, 5));

        // Write the shared value from outside the editor. The old cursor (5)
        // now points into the middle of a multi-byte character; the editor's
        // observation must clamp it back onto a boundary.
        cx.update(|_, cx| {
            a_value.update(cx, |value, cx| {
                *value = "日本".to_string();
                cx.notify();
            })
        });

        cx.read_entity(&a_value, |value, _| assert_eq!(value, "日本"));
        cx.read_entity(&editor, |editor, _| {
            assert_eq!(editor.cursor, 3, "cursor must clamp to a char boundary");
        });
    }

    #[gpui::test]
    fn arrows_move_the_cursor(cx: &mut TestAppContext) {
        let (editor, _a_value, _b_value, cx) = setup(cx);

        cx.simulate_input("abc");
        cx.read_entity(&editor, |editor, _| assert_eq!(editor.cursor, 3));

        cx.simulate_keystrokes("left left");
        cx.read_entity(&editor, |editor, _| assert_eq!(editor.cursor, 1));
    }
}
