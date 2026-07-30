//! Tests for the input composition. Require the `test-support` feature:
//!
//! ```sh
//! cargo test -p gpui --example view_example --features test-support
//! ```

#[cfg(test)]
mod tests {
    use gpui::{
        Context, Entity, IntoElement, KeyBinding, ProjectionMut, TestAppContext, Window,
        prelude::*, project,
    };

    use crate::example_editor::Editor;
    use crate::example_input::Input;
    use crate::{Backspace, Delete, End, Home, Left, Right};

    /// Two inputs, each backed by an editor we own (so the test can focus and
    /// read them). Proves data flows through the projected `String` and that
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
        ProjectionMut<String>,
        ProjectionMut<String>,
        &mut gpui::VisualTestContext,
    ) {
        bind_keys(cx);

        let (harness, cx) = cx.add_window_view(|window, cx| {
            // A whole entity projects to itself, so an editor over an entity and
            // an editor over one field of a form are the same thing to `Editor`.
            let a_value = cx.new(|_| String::new()).into();
            let b_value = cx.new(|_| String::new()).into();
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

        cx.update(|_, cx| assert_eq!(a_value.read(cx), "hello"));
        cx.read_entity(&editor, |editor, _| assert_eq!(editor.cursor, 5));
    }

    #[gpui::test]
    fn sibling_inputs_are_isolated(cx: &mut TestAppContext) {
        let (_editor, a_value, b_value, cx) = setup(cx);

        cx.simulate_input("x");

        cx.update(|_, cx| {
            assert_eq!(a_value.read(cx), "x");
            assert_eq!(
                b_value.read(cx),
                "",
                "typing in input A must not touch input B"
            );
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
        cx.update(|_, cx| a_value.update(cx, |value| *value = "日本".to_string()));

        cx.update(|_, cx| assert_eq!(a_value.read(cx), "日本"));
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

    /// Guards a feedback loop: a view's identity is the notify target for state
    /// allocated inside it, so a subform identified by the projection it also
    /// projects from will notify itself forever. This test hangs if that
    /// regresses.
    #[gpui::test]
    fn nested_subforms_do_not_feed_back(cx: &mut TestAppContext) {
        let (root, cx) = cx.add_window_view(|_, cx| SubformHarness {
            profile: cx.new(|_| Profile {
                primary: Person::default(),
                secondary: Person::default(),
            }),
        });

        let profile = cx.read_entity(&root, |root, _| root.profile.clone());

        // Writing the source notifies the projections the subforms read, which
        // in turn notify the editors allocated inside them. If any of those
        // notifications routes back into the projection graph, this never
        // settles.
        cx.update(|_, cx| {
            profile.update(cx, |profile, cx| {
                profile.primary.name = "hi".to_string();
                cx.notify();
            })
        });
        cx.run_until_parked();

        cx.read_entity(&profile, |profile, _| {
            assert_eq!(profile.primary.name, "hi");
            assert_eq!(profile.secondary.name, "", "subforms must stay isolated");
        });
    }

    #[derive(Default)]
    struct Person {
        name: String,
    }

    struct Profile {
        primary: Person,
        secondary: Person,
    }

    /// Two instances of one subform over two projected people, mirroring the
    /// example's `PersonForm`.
    struct SubformHarness {
        profile: Entity<Profile>,
    }

    impl Render for SubformHarness {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let primary = project!(window, cx, &self.profile, mut primary);
            let secondary = project!(window, cx, &self.profile, mut secondary);
            gpui::div()
                .child(Subform { person: primary })
                .child(Subform { person: secondary })
        }
    }

    #[derive(IntoElement)]
    struct Subform {
        person: ProjectionMut<Person>,
    }

    impl gpui::View for Subform {
        fn entity_id(&self) -> Option<gpui::EntityId> {
            None
        }

        fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
            Input::new(project!(window, cx, &self.person, mut name))
        }
    }
}
