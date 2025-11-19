use std::time::Duration;

use gpui::{
    AbsoluteLength, Animation, AnimationExt, App, AppContext, Application, Bounds, Context,
    FragmentShader, IntoElement, Length, ParentElement, Radians, Render, RenderOnce, Rgba,
    ShaderUniform, Styled, Window, WindowBounds, WindowOptions, div, px, relative, rgb,
    shader_element, shader_element_with_data, size,
};

#[repr(C)]
#[derive(ShaderUniform, Clone, Copy)]
pub struct WarpShaderInstance {
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub color_c: [f32; 4],
    pub time_a: f32,
    pub time_b: f32,
}

struct ShaderExample {}

impl Render for ShaderExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let warping_shader = FragmentShader::new(
            "
            // Based on https://iquilezles.org/articles/warp/
            let m = (input.size.x + input.size.y) / 5.0;
            let p = vec2<f32>(input.position.x / m, input.position.y / m);

            let q = vec2<f32>(fbm(p, 0.5), fbm(p + vec2<f32>(data.time_a, data.time_b * 0.4), 0.5));
            let r = vec2<f32>(fbm(p * 4.0 * q * 8.0 + vec2<f32>(data.time_b * 1.7, data.time_a * 1.3), 0.5), fbm(p * (data.time_b + 2.0) * (data.time_b + 2.0) / 5.0 * 4.0 * q + vec2<f32>(data.time_a * -0.2, 2.8), 0.5));
            let x = fbm(p + 4.0 * r, 0.5);

            var c = mix(data.color_a, data.color_b, (q.x + r.x) / 2.0);
            c = mix(c, data.color_c, x);

            return vec4<f32>(c.x, c.y, c.z, 1.0);
            ",
        )
        .with_item("fn rand22(n: vec2<f32>) -> f32 { return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453); }")
        .with_item("
            fn noise2(n: vec2<f32>) -> f32 {
                let d = vec2<f32>(0., 1.);
                let b = floor(n);
                let f = smoothstep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
                return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
            }
        ")
        .with_item("
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
        ");

        div().flex().size_full().bg(rgb(0x202060)).with_animation(
            "animation",
            Animation::new(Duration::from_secs(60)).repeat(),
            move |this, t| {
                this.child(
                    shader_element_with_data(
                        warping_shader.clone(),
                        WarpShaderInstance {
                            color_a: [0.15, 0.3, 0.8, 0.0],
                            color_b: [0.9, 0.35, 0.4, 0.0],
                            color_c: [1.0, 0.95, 0.7, 0.0],
                            time_a: (2.0 * 3.0 * std::f32::consts::PI * t + 5.0).sin(),
                            time_b: (2.0 * std::f32::consts::PI * t).sin(),
                        },
                    )
                    .size_full(),
                )
                .child(
                    shader_element_with_data(
                        warping_shader.clone(),
                        WarpShaderInstance {
                            color_a: [0.45, 0.1, 0.1, 0.0],
                            color_b: [0.9, 0.5, 0.0, 0.0],
                            color_c: [1.0, 0.95, 0.7, 0.0],
                            time_a: (2.0 * std::f32::consts::PI * t + 2.0).sin() * 2.0,
                            time_b: (2.0 * 3.0 * std::f32::consts::PI * t + 3.0).sin(),
                        },
                    )
                    .size_full(),
                )
                .child(
                    div()
                        .size_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .child(
                            star()
                                .size(relative(0.9))
                                .rotation(Radians(2.0 * std::f32::consts::PI * t * 10.0)),
                        )
                        .child(
                            shader_element(FragmentShader::new(
                                "
                                let p = (vec2<f32>(input.position.x, input.position.y) - input.origin) / input.size;
                                return vec4<f32>(p.x, p.y, 0.0, 1.0);
                                ",
                            ))
                            .size_20().p_5(),
                        )
                        .child(
                            star()
                                .size(relative(0.3))
                                .border(px(1.0))
                                .border_color(gpui::white())
                                .bg(gpui::green())
                                .rotation(Radians(2.0 * std::f32::consts::PI * t * -30.0)),
                        )
                )
            },
        )
    }
}

#[derive(IntoElement)]
pub struct Star {
    size: Length,
    bg: Rgba,
    border_color: Rgba,
    border: AbsoluteLength,
    rotation: Radians,
}

impl Star {
    pub fn size(mut self, length: impl Into<Length>) -> Self {
        self.size = length.into();
        self
    }

    pub fn bg(mut self, color: impl Into<Rgba>) -> Self {
        self.bg = color.into();
        self
    }

    pub fn border_color(mut self, color: impl Into<Rgba>) -> Self {
        self.border_color = color.into();
        self
    }

    pub fn border(mut self, length: impl Into<AbsoluteLength>) -> Self {
        self.border = length.into();
        self
    }

    pub fn rotation(mut self, rotation: impl Into<Radians>) -> Self {
        self.rotation = rotation.into();
        self
    }
}

fn star() -> Star {
    Star {
        size: px(40.0).into(),
        bg: gpui::yellow().into(),
        border_color: gpui::black().into(),
        border: px(10.0).into(),
        rotation: Radians(0.0),
    }
}

#[repr(C)]
#[derive(ShaderUniform, Clone, Copy)]
struct StarUserData {
    bg: [f32; 4],
    border_color: [f32; 4],
    border: f32,
    sine: f32,
    cosine: f32,
}

impl RenderOnce for Star {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl gpui::IntoElement {
        shader_element_with_data(FragmentShader::new(
            "
            let r = min(input.size.x, input.size.y) / 2.0;
            let p = vec2<f32>(input.position.x - input.origin.x, input.position.y - input.origin.y) - input.size / 2.0;
            let p_rot = mat2x2<f32>(data.cosine, -data.sine, data.sine, data.cosine) * p;
            let d = sd_pentagram(p_rot, r * 0.90) - r * 0.1;

            let bg = mix(data.bg, vec4<f32>(data.bg.x, data.bg.y, data.bg.z, 0.0), clamp(d, 0.0, 1.0));
            let border = clamp(abs(d + data.border / 2.0) - data.border / 2.0 - 1.0, 0.0, 1.0);
            let color = mix(data.border_color, bg, border);

            return color;"
        ).with_item("
            // https://iquilezles.org/articles/distfunctions2d/
            fn sd_pentagram(pos: vec2<f32>, r: f32) -> f32 {
                var p = vec2<f32>(pos.x, -pos.y);
                let k1x = 0.809016994; // cos(pi/5)
                let k2x = 0.309016994; // sin(pi/10)
                let k1y = 0.587785252; // sin(pi/5)
                let k2y = 0.951056516; // cos(pi/10)
                let k1z = 0.726542528; // tan(pi/5)
                let v1 = vec2( k1x, -k1y);
                let v2 = vec2(-k1x, -k1y);
                let v3 = vec2( k2x, -k2y);

                p.x = abs(p.x);
                p -= 2.0 * max(dot(v1, p), 0.0) * v1;
                p -= 2.0 * max(dot(v2, p), 0.0) * v2;
                p.x = abs(p.x);
                p.y -= r;
                return length(p - v3 * clamp(dot(p, v3), 0.0, k1z * r)) * sign(p.y * v3.x - p.x * v3.y);
            }
        "),
        StarUserData {
            bg: [self.bg.r, self.bg.g, self.bg.b, self.bg.a],
            border_color: [self.border_color.r, self.border_color.g, self.border_color.b, self.border_color.a],
            border: self.border.to_pixels(window.rem_size()).into(),
            sine: self.rotation.0.sin(),
            cosine: self.rotation.0.cos(),
        }).size(self.size)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.0)), cx);
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
