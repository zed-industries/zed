use crate::{
    App, Bounds, ClipboardItem, Context, Entity, InputHandler, Pixels, TextInputConfiguration,
    UTF16Selection, Window,
};
use std::ops::Range;

/// Implement this trait to allow views to handle textual input when implementing an editor, field, etc.
///
/// Once your view implements this trait, you can use it to construct an [`ElementInputHandler<V>`].
/// This input handler can then be assigned during paint by calling [`Window::handle_input`].
///
/// See [`InputHandler`] for details on how to implement each method.
pub trait EntityInputHandler: 'static + Sized {
    /// See [`InputHandler::text_for_range`] for details
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String>;

    /// See [`InputHandler::selected_text_range`] for details
    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<UTF16Selection>;

    /// See [`InputHandler::marked_text_range`] for details
    fn marked_text_range(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Range<usize>>;

    /// See [`InputHandler::unmark_text`] for details
    fn unmark_text(&mut self, window: &mut Window, cx: &mut Context<Self>);

    /// See [`InputHandler::paste`] for details
    fn paste(&mut self, item: ClipboardItem, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = item.text() {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    /// See [`InputHandler::replace_text_in_range`] for details
    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    );

    /// See [`InputHandler::replace_and_mark_text_in_range`] for details
    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    );

    /// See [`InputHandler::bounds_for_range`] for details
    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>>;

    /// See [`InputHandler::character_index_for_point`] for details
    fn character_index_for_point(
        &mut self,
        point: crate::Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize>;

    /// See [`InputHandler::set_selected_text_range`] for details
    fn set_selected_text_range(
        &mut self,
        _range_utf16: Range<usize>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    /// See [`InputHandler::text_length_utf16`] for details
    fn text_length_utf16(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }

    /// See [`InputHandler::accepts_text_input`] for details
    fn accepts_text_input(&self, _window: &mut Window, _cx: &mut Context<Self>) -> bool {
        true
    }

    /// See [`InputHandler::text_input_configuration`] for details
    fn text_input_configuration(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> TextInputConfiguration {
        TextInputConfiguration::default()
    }
}

/// The canonical implementation of [`crate::PlatformInputHandler`]. Call [`Window::handle_input`]
/// with an instance during your element's paint.
pub struct ElementInputHandler<V> {
    view: Entity<V>,
    element_bounds: Bounds<Pixels>,
}

impl<V: 'static> ElementInputHandler<V> {
    /// Used in [`Element::paint`][element_paint] with the element's bounds, a `Window`, and a `App` context.
    ///
    /// [element_paint]: crate::Element::paint
    pub fn new(element_bounds: Bounds<Pixels>, view: Entity<V>) -> Self {
        ElementInputHandler {
            view,
            element_bounds,
        }
    }
}

impl<V: EntityInputHandler> InputHandler for ElementInputHandler<V> {
    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<UTF16Selection> {
        self.view.update(cx, |view, cx| {
            view.selected_text_range(ignore_disabled_input, window, cx)
        })
    }

    fn marked_text_range(&mut self, window: &mut Window, cx: &mut App) -> Option<Range<usize>> {
        self.view
            .update(cx, |view, cx| view.marked_text_range(window, cx))
    }

    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<String> {
        self.view.update(cx, |view, cx| {
            view.text_for_range(range_utf16, adjusted_range, window, cx)
        })
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.view.update(cx, |view, cx| {
            view.replace_text_in_range(replacement_range, text, window, cx)
        });
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.view.update(cx, |view, cx| {
            view.replace_and_mark_text_in_range(
                range_utf16,
                new_text,
                new_selected_range,
                window,
                cx,
            )
        });
    }

    fn unmark_text(&mut self, window: &mut Window, cx: &mut App) {
        self.view
            .update(cx, |view, cx| view.unmark_text(window, cx));
    }

    fn paste(&mut self, item: ClipboardItem, window: &mut Window, cx: &mut App) {
        self.view
            .update(cx, |view, cx| view.paste(item, window, cx));
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>> {
        self.view.update(cx, |view, cx| {
            view.bounds_for_range(range_utf16, self.element_bounds, window, cx)
        })
    }

    fn character_index_for_point(
        &mut self,
        point: crate::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize> {
        self.view.update(cx, |view, cx| {
            view.character_index_for_point(point, window, cx)
        })
    }

    fn set_selected_text_range(
        &mut self,
        range_utf16: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.view.update(cx, |view, cx| {
            view.set_selected_text_range(range_utf16, window, cx)
        })
    }

    fn element_bounds(&mut self, _window: &mut Window, _cx: &mut App) -> Option<Bounds<Pixels>> {
        Some(self.element_bounds)
    }

    fn text_length_utf16(&mut self, window: &mut Window, cx: &mut App) -> Option<usize> {
        self.view
            .update(cx, |view, cx| view.text_length_utf16(window, cx))
    }

    fn accepts_text_input(&mut self, window: &mut Window, cx: &mut App) -> bool {
        self.view
            .update(cx, |view, cx| view.accepts_text_input(window, cx))
    }

    fn prefers_ime_for_printable_keys(&mut self, window: &mut Window, cx: &mut App) -> bool {
        self.view
            .update(cx, |view, cx| view.accepts_text_input(window, cx))
    }

    fn text_input_configuration(
        &mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> TextInputConfiguration {
        self.view
            .update(cx, |view, cx| view.text_input_configuration(window, cx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnyWindowHandle, AppContext as _, FocusHandle, InteractiveElement as _, IntoElement,
        ParentElement as _, Render, Styled as _, TestAppContext, TextInputAction, canvas, div,
    };

    #[gpui::test]
    fn text_input_configuration_forwarded_only_on_change(cx: &mut TestAppContext) {
        let custom = TextInputConfiguration {
            autocorrect: true,
            input_action: TextInputAction::Send,
            ..Default::default()
        };
        let window = cx.add_window({
            let custom = custom.clone();
            move |_, cx| ConfigurationTestView {
                focus_handle: cx.focus_handle(),
                configuration: custom,
            }
        });
        let view = window.root(cx).unwrap();
        let test_window = cx.test_window(window.into());
        let window = AnyWindowHandle::from(window);
        let draw = |cx: &mut TestAppContext| {
            cx.update_window(window, |_, window, cx| window.draw(cx).clear(cx))
                .unwrap();
        };

        // Nothing is focused, so the platform learns the default configuration.
        draw(cx);
        assert_eq!(
            test_window.text_input_configurations(),
            vec![TextInputConfiguration::default()]
        );

        // Focusing the view routes its configuration to the platform.
        cx.update_window(window, |_, window, cx| {
            let focus_handle = view.read(cx).focus_handle.clone();
            window.focus(&focus_handle, cx);
        })
        .unwrap();
        draw(cx);
        assert_eq!(
            test_window.text_input_configurations(),
            vec![TextInputConfiguration::default(), custom.clone()]
        );

        // Redrawing without a change forwards nothing.
        draw(cx);
        assert_eq!(test_window.text_input_configurations().len(), 2);

        // Changing the configuration forwards the new value.
        let updated = TextInputConfiguration {
            suggestions: true,
            ..custom
        };
        view.update(cx, {
            let updated = updated.clone();
            |view, cx| {
                view.configuration = updated;
                cx.notify();
            }
        });
        draw(cx);
        assert_eq!(
            test_window.text_input_configurations().last(),
            Some(&updated)
        );
        assert_eq!(test_window.text_input_configurations().len(), 3);

        // Losing focus reverts the platform to the default configuration.
        cx.update_window(window, |_, window, _| window.blur())
            .unwrap();
        draw(cx);
        assert_eq!(
            test_window.text_input_configurations().last(),
            Some(&TextInputConfiguration::default())
        );
        assert_eq!(test_window.text_input_configurations().len(), 4);
    }

    struct ConfigurationTestView {
        focus_handle: FocusHandle,
        configuration: TextInputConfiguration,
    }

    impl Render for ConfigurationTestView {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let view = cx.entity();
            let focus_handle = self.focus_handle.clone();
            div().size_full().track_focus(&self.focus_handle).child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, cx| {
                        window.handle_input(
                            &focus_handle,
                            ElementInputHandler::new(bounds, view),
                            cx,
                        );
                    },
                )
                .size_full(),
            )
        }
    }

    impl EntityInputHandler for ConfigurationTestView {
        fn text_for_range(
            &mut self,
            _range: std::ops::Range<usize>,
            _adjusted_range: &mut Option<std::ops::Range<usize>>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<String> {
            None
        }

        fn selected_text_range(
            &mut self,
            _ignore_disabled_input: bool,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<UTF16Selection> {
            None
        }

        fn marked_text_range(
            &self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<std::ops::Range<usize>> {
            None
        }

        fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

        fn replace_text_in_range(
            &mut self,
            _range: Option<std::ops::Range<usize>>,
            _text: &str,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) {
        }

        fn replace_and_mark_text_in_range(
            &mut self,
            _range: Option<std::ops::Range<usize>>,
            _new_text: &str,
            _new_selected_range: Option<std::ops::Range<usize>>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) {
        }

        fn bounds_for_range(
            &mut self,
            _range_utf16: std::ops::Range<usize>,
            _element_bounds: Bounds<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<Bounds<Pixels>> {
            None
        }

        fn character_index_for_point(
            &mut self,
            _point: crate::Point<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<usize> {
            None
        }

        fn text_input_configuration(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> TextInputConfiguration {
            self.configuration.clone()
        }
    }
}
