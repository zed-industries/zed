use gpui::{
    App, AppContext, Application, Bounds, Context, InteractiveElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, WindowOptions, div, img, point, px, size,
};
use latex_render::{LatexColor, LatexRenderer};
use std::sync::Arc;

struct LatexDemo {
    renderer: Arc<LatexRenderer>,
    formulas: Vec<&'static str>,
}

impl LatexDemo {
    fn new(renderer: Arc<LatexRenderer>) -> Self {
        Self {
            renderer,
            formulas: vec![
                r"x",
                r"X",
                r"x_{1,2}",
                r"x\_{1,2}", // Prettier way of writing underscore, added a special sanitization in renderer
                r"\sum n \pi",
                r"x_{1,2} = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}",
                r"| \vec{A}|=\sqrt{A_x^2 + A_y^2 + A_z^2}",
                r"| \vec{A}|=\sqrt{A_x^2 + A_y^2 + A_z^2} \tag{1}", // tags aren't supported, sanitization test
                r"E = mc^2",
                r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}",
                r"\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}",
                r"\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}",
                r"\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}",
                r"\lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n = e",
                r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}",
            ],
        }
    }
}

impl Render for LatexDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let font_size = 16.0;
        let color = LatexColor::BLACK;

        div()
            .id("latex-demo")
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(gpui::white())
            .size_full()
            .overflow_y_scroll()
            .children(self.formulas.iter().enumerate().map(|(_i, formula)| {
                let result = self.renderer.render(formula, font_size, color, true);

                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .p_2()
                    .border_1()
                    .border_color(gpui::rgb(0xcccccc))
                    .rounded_md()
                    .child(
                        div()
                            .w(px(300.0))
                            .overflow_hidden()
                            .child(formula.to_string()),
                    )
                    .child(
                        div().flex_1().child(match result {
                            Some(Ok((image, (width, height)))) => div().child(
                                img(gpui::ImageSource::Render(image))
                                    .w(px(width as f32))
                                    .h(px(height as f32)),
                            ),
                            Some(Err(e)) => {
                                div().text_color(gpui::red()).child(format!("Error: {}", e))
                            }
                            None => div().text_color(gpui::rgb(0x888888)).child("Rendering..."),
                        }),
                    )
            }))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let window_options = WindowOptions {
            window_bounds: Some(gpui::WindowBounds::Windowed(Bounds {
                origin: point(px(100.0), px(100.0)),
                size: size(px(900.0), px(600.0)),
            })),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("LaTeX Rendering Demo (ReX)".into()),
                ..Default::default()
            }),
            focus: true,
            show: true,
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| {
            let renderer = Arc::new(LatexRenderer::new(cx.background_executor().clone()));
            cx.new(|_| LatexDemo::new(renderer))
        })
        .unwrap();
    });
}
