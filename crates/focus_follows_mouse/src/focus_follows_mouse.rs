use gpui::{Context, Focusable, StatefulInteractiveElement};

pub trait FocusFollowsMouse<E: Focusable>: StatefulInteractiveElement {
    fn focus_follows_mouse(self, enabled: bool, cx: &Context<E>) -> Self {
        if enabled {
            self.on_hover(cx.listener(move |this, enter, window, cx| {
                if *enter {
                    window.focus(&this.focus_handle(cx), cx);
                }
            }))
        } else {
            self
        }
    }
}

impl<E: Focusable, T: StatefulInteractiveElement> FocusFollowsMouse<E> for T {}
