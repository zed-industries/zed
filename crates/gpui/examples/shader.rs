use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, AppContext, Application, Bounds, Context, CustomShader,
    ParentElement, Render, Styled, Window, WindowBounds, WindowOptions, custom_shader, div, px,
    rgb, size,
};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct UserData {
    pub time_a: f32,
    pub time_b: f32,
    pub pad0: u32,
    pub pad1: u32,
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub color_c: [f32; 4],
}

struct ShaderExample {}

impl Render for ShaderExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let warping = CustomShader::new_fragment(
            r#"
            // Based on https://iquilezles.org/articles/warp/
            let m = (input.size.x + input.size.y) / 5.0;
            let p = vec2<f32>(input.position.x / m, input.position.y / m);

            let q = vec2<f32>(fbm(p, 0.5), fbm(p + vec2<f32>(user_data.time_a, user_data.time_b * 0.4), 0.5));
            let r = vec2<f32>(fbm(p * 4.0 * q * 8.0 + vec2<f32>(user_data.time_b * 1.7, user_data.time_a * 1.3), 0.5), fbm(p * (user_data.time_b + 2.0) * (user_data.time_b + 2.0) / 5.0 * 4.0 * q + vec2<f32>(user_data.time_a * -0.2, 2.8), 0.5));
            let x = fbm(p + 4.0 * r, 0.5);

            var c = mix(user_data.color_a, user_data.color_b, (q.x + r.x) / 2.0);
            c = mix(c, user_data.color_c, x);

            return vec4<f32>(c.x, c.y, c.z, 1.0);"#,
            "time_a: f32, time_b: f32, color_a: vec3<f32>, pad0: u32, color_b: vec3<f32>, pad1: u32, color_c: vec3<f32>, pad2: u32",
            r#"

            fn rand22(n: vec2<f32>) -> f32 { return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453); }

            // https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
            fn noise2(n: vec2<f32>) -> f32 {
                let d = vec2<f32>(0., 1.);
                let b = floor(n);
                let f = smoothstep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
                return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
            }

            fn fbm(position: vec2<f32>, hurst: f32) -> f32 {
                let gain = exp2(-hurst);

                var frequency: f32 = 1.0;
                var amplitude: f32 = 1.0;
                var sum: f32 = 0.0;

                for (var idx: i32 = 0; idx < 5; idx = idx + 1) {
                    sum = sum + amplitude * noise2(position * frequency);
                    frequency = frequency * 2.0;
                    amplitude = amplitude * gain;
                }

                return sum / 5.0;
            }
            "#,
        );

        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x202060))
            .with_animation(
                "animation",
                Animation::new(Duration::from_secs(60)).repeat(),
                move |this, t| {
                    this.child(
                        custom_shader(
                            warping.clone(),
                            UserData {
                                time_a: (2.0 * 3.0 * std::f32::consts::PI * t + 5.0).sin(),
                                time_b: (2.0 * std::f32::consts::PI * t).sin(),
                                color_a: [0.15, 0.3, 0.8, 0.0],
                                color_b: [0.9, 0.35, 0.4, 0.0],
                                color_c: [1.0, 0.95, 0.7, 0.0],
                                pad0: 0,
                                pad1: 0,
                            },
                        )
                        .size_full(),
                    )
                    .child(
                        custom_shader(
                            warping.clone(),
                            UserData {
                                time_a: (2.0 * std::f32::consts::PI * t + 2.0).sin() * 2.0,
                                time_b: (2.0 * 3.0 * std::f32::consts::PI * t + 3.0).sin(),
                                color_a: [0.45, 0.1, 0.1, 0.0],
                                color_b: [0.9, 0.5, 0.0, 0.0],
                                color_c: [1.0, 0.95, 0.7, 0.0],
                                pad0: 0,
                                pad1: 0,
                            },
                        )
                        .size_full(),
                    )
                },
            )
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
