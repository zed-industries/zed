use gpui::{
    App, AppContext, Application, Bounds, Context, CustomShader, ParentElement, Render, Styled,
    Window, WindowBounds, WindowOptions, custom_shader, div, px, rgb, size,
};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct UserData {
    pub blue: f32,
}

struct ShaderExample {}

impl Render for ShaderExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let fs = CustomShader::new_fragment(
            "return vec4<f32>((input.position.x - input.origin.x) / input.size.x, (input.position.y - input.origin.y) / input.size.y, user_data.blue, 1.0);",
            "blue: f32",
        );

        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x202060))
            .gap_2()
            .child(custom_shader(fs.clone(), UserData { blue: 0.0 }).size_full())
            .child(custom_shader(fs, UserData { blue: 1.0 }).size_full())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ShaderExample {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
