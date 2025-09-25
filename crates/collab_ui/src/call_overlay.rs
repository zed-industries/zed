use gpui::AppContext;
use ui::{App, Context, IntoElement, Render, Styled, Window, div, px};
use workspace::Workspace;

pub struct CallOverlay {}

impl Render for CallOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().w(px(100.)).h(px(100.)).bg(gpui::blue())
    }
}

pub fn init(cx: &App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        let dock = workspace.dock_at_position(workspace::dock::DockPosition::Left);
        dock.update(cx, |dock, cx| {
            let overlay = cx.new(|_| CallOverlay {});
            dock.add_overlay(
                cx,
                Box::new(move |window, cx| {
                    overlay.update(cx, |overlay, cx| {
                        overlay.render(window, cx).into_any_element()
                    })
                }),
            )
        });
    })
    .detach();
}
