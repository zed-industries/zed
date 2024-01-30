use gpui::*;

#[derive(IntoElement)]
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
                .child(img(self.resource).w(px(512.0)).h(px(512.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: SharedUrl,
    remote_resource: SharedUrl,
}

impl Render for ImageShowcase {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xFFFFFF))
            .child(ImageFromResource {
                text: "Image loaded from a local file".into(),
                resource: self.local_resource.clone(),
            })
            .child(ImageFromResource {
                text: "Image loaded from a remote resource".into(),
                resource: self.remote_resource.clone(),
            })
    }
}

fn main() {
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| ImageShowcase {
                local_resource: SharedUrl::file("../zed/resources/app-icon.png"),
                remote_resource: SharedUrl::network("https://picsum.photos/512/512"),
            })
        });
    });
}
