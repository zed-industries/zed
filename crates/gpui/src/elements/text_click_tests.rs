use super::*;
use crate::{AppContext, MouseButton, PlatformInput, TestAppContext, point, px, size};

struct ClickView(Rc<Cell<usize>>);
impl crate::Render for ClickView {
    fn render(&mut self, _: &mut Window, _: &mut crate::Context<Self>) -> impl IntoElement {
        let clicks = self.0.clone();
        InteractiveText::new("link", StyledText::new("link plain"))
            .on_click(vec![0..4], move |_, _, _| clicks.set(clicks.get() + 1))
    }
}

fn click_count(
    cx: &mut TestAppContext,
    button: MouseButton,
    release_x: f32,
    repeat: usize,
) -> usize {
    let cx = cx.add_empty_window();
    let clicks = Rc::new(Cell::new(0));
    cx.draw(point(px(0.), px(0.)), size(px(200.), px(60.)), |_, cx| {
        cx.new(|_| ClickView(clicks.clone())).into_any_element()
    });
    cx.update(|window, cx| {
        for _ in 0..repeat {
            window.dispatch_event(
                PlatformInput::MouseDown(MouseDownEvent {
                    position: point(px(2.), px(2.)),
                    button,
                    ..Default::default()
                }),
                cx,
            );
            window.dispatch_event(
                PlatformInput::MouseUp(MouseUpEvent {
                    position: point(px(release_x), px(2.)),
                    button,
                    ..Default::default()
                }),
                cx,
            );
        }
    });
    clicks.get()
}

#[gpui::test]
fn click_does_not_require_a_paint_between_down_and_up(cx: &mut TestAppContext) {
    assert_eq!(click_count(cx, MouseButton::Left, 2., 2), 2);
}

#[gpui::test]
fn releasing_outside_the_link_does_not_activate_it(cx: &mut TestAppContext) {
    assert_eq!(click_count(cx, MouseButton::Left, 120., 1), 0);
}

#[gpui::test]
fn secondary_click_does_not_activate_a_primary_link(cx: &mut TestAppContext) {
    assert_eq!(click_count(cx, MouseButton::Right, 2., 1), 0);
}
