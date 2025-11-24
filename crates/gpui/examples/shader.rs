use std::{f32::consts::PI, time::Duration};

use gpui::{
    AbsoluteLength, Animation, AnimationExt, App, AppContext, Application, Bounds, Context,
    FragmentShader, IntoElement, Length, ParentElement, Radians, Render, RenderOnce, Rgba,
    ShaderUniform, Styled, Window, WindowBounds, WindowOptions, div, px, radians, relative, rgb,
    shader_element, shader_element_with_data, size,
};

#[repr(C)]
#[derive(ShaderUniform, Clone, Copy)]
pub struct WarpShaderInstance {
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub time: f32,
    pub hurst: f32,
}

struct ShaderExample {}

impl Render for ShaderExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let warping_shader = FragmentShader::new(
            "
            // Based on https://iquilezles.org/articles/warp/
            let p = input.position.xy / 500.0;

            let a = vec2<f32>(fbm(p + vec2<f32>(0.0, 0.0), data.hurst),
                              fbm(p + vec2<f32>(5.2, 2.3), data.hurst));
            let b = vec2<f32>(fbm(p + data.time * a + vec2<f32>(1.7, 9.2), data.hurst),
                              fbm(p + data.time * a + vec2<f32>(8.3, 2.8), data.hurst));
            let c = fbm(p + 4.0 * b, data.hurst);

            let color = mix(data.color_a, data.color_b, clamp(b.y - 0.1, 0.0, 1.0));
            return mix(vec4<f32>(0.0, 0.0, 0.0, 1.0), color, clamp(c * 1.5 - 0.5, 0.0, 1.0));
            ",
        )
        .with_item("
            // https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
            fn rand22(n: vec2<f32>) -> f32 {
                return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
            }

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
                var max: f32 = 0.0;

                for (var idx: i32 = 0; idx < 5; idx = idx + 1) {
                    sum = sum + amplitude * noise2(position * frequency);
                    max = max + amplitude;
                    frequency = frequency * 2.0;
                    amplitude = amplitude * gain;
                }

                return sum / max;
            }
        ");

        div().flex().size_full().bg(rgb(0x202060)).with_animation(
            "animation",
            Animation::new(Duration::from_secs(30)).repeat(),
            move |this, t| {
                this.child(
                    shader_element_with_data(
                        warping_shader.clone(),
                        WarpShaderInstance {
                            color_a: [0.0, 0.5, 1.0, 1.0],
                            color_b: [1.0, 0.0, 0.0, 1.0],
                            time: (2.0 * PI * t).sin() * 0.25 + 4.0,
                            hurst: 0.95,
                        },
                    )
                    .size_full()
                    .absolute(),
                )
                .child(star().size(relative(0.5)).border(px(20.0)).border_color(gpui::green()).rotation(radians(6.0 * PI * t)))
                .child(
                    star()
                        .size(relative(0.2))
                        .bg(gpui::blue())
                        .border_color(gpui::white())
                        .border(px(2.0))
                        .rotation(radians(-24.0 * PI * t)),
                )
                .child(shader_element(FragmentShader::new(
                    "return vec4<f32>((input.position.xy - input.origin) / input.size, 0.0, 1.0);"
                )).size_40().cursor_crosshair())
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
struct StarInstanceData {
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
            let radius = min(input.size.x, input.size.y) / 2.0;
            let pos = mat2x2<f32>(data.cosine, -data.sine, data.sine, data.cosine) * (input.position.xy - input.origin - input.size / 2.0);
            let signed_dst = sd_pentagram(pos, radius * 0.90) - radius * 0.1;

            let border = clamp(signed_dst + data.border, 0.0, 1.0);
            var color = mix(data.bg, data.border_color, border);
            color.w = 1.0 - clamp(signed_dst, 0.0, 1.0);

            return color;"
        ).with_item("
            // ported from https://iquilezles.org/articles/distfunctions2d/
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
        StarInstanceData {
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
