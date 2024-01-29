use gpui::*;

struct ImageFromResource {
    text: SharedString,
    resource: SharedUrl,
}

impl RenderOnce for ImageFromResource {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        div().child(
            div()
                .flex_row()
                .size_full()
                .gap_4()
                .child(self.text)
                .child(img(self.resource).w(px(320.0)).h(px(80.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: SharedUrl,
    remote_resource: SharedUrl,
}

impl Render for ImageShowcase {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xFFFFFF))
            .child(
                ImageFromResource {
                    text: "Image loaded from a local file".into(),
                    resource: self.local_resource.clone(),
                }
                .render(cx),
            )
            .child(
                ImageFromResource {
                    text: "Image loaded from a remote resource".into(),
                    resource: self.remote_resource.clone(),
                }
                .render(cx),
            )
    }
}

fn main() {
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| ImageShowcase {
                local_resource: SharedUrl::File("examples/assets/zed_logo.png".into()),
                remote_resource: SharedUrl::Network("https://picsum.photos/320/72".into()),
            })
        });
    });
}
