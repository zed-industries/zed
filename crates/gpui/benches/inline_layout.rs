use criterion::{Criterion, criterion_group, criterion_main};
use gpui::{
    AppContext, AvailableSpace, Context, IntoElement, ParentElement, Render, Styled,
    TestAppContext, TestDispatcher, Window, div, inline, point, px, size,
};

struct InlineBenchView {
    text_len: usize,
    box_count: usize,
}

impl Render for InlineBenchView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut el = inline();

        let chunk_size = if self.box_count > 0 {
            self.text_len / (self.box_count + 1)
        } else {
            self.text_len
        };

        let text_chunk = "a".repeat(chunk_size);

        el = el.text(text_chunk.clone());

        for _ in 0..self.box_count {
            el = el.child(inline().w(px(20.0)).h(px(20.0)).bg(gpui::red()));
            if chunk_size > 0 {
                el = el.text(text_chunk.clone());
            }
        }

        el
    }
}

fn benchmark_inline_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("inline_layout");

    let cases = [("low", 50, 0), ("med", 500, 5), ("high", 5000, 50)];

    let dispatcher = TestDispatcher::new(1);
    let cx = TestAppContext::build(dispatcher, None);

    for (name, text_len, box_count) in cases {
        group.bench_function(name, |b| {
            let mut cx = cx.clone();
            let cx = cx.add_empty_window();
            let view = cx.update(|_, cx| {
                cx.new(|_| InlineBenchView {
                    text_len,
                    box_count,
                })
            });

            b.iter(|| {
                cx.draw(
                    point(px(0.0), px(0.0)),
                    size(
                        AvailableSpace::Definite(px(800.0)),
                        AvailableSpace::MinContent,
                    ),
                    |_, _| view.clone().into_any_element(),
                );
            });
        });
    }

    group.finish();
}

struct DivBenchView {
    text_len: usize,
    box_count: usize,
}

impl Render for DivBenchView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut el = div();

        let chunk_size = if self.box_count > 0 {
            self.text_len / (self.box_count + 1)
        } else {
            self.text_len
        };
        let text_chunk = "a".repeat(chunk_size);

        el = el.child(text_chunk.clone());

        for _ in 0..self.box_count {
            el = el.child(div().w(px(20.0)).h(px(20.0)).bg(gpui::red()));
            if chunk_size > 0 {
                el = el.child(text_chunk.clone());
            }
        }

        el
    }
}

fn benchmark_div_vs_inline(c: &mut Criterion) {
    let mut group = c.benchmark_group("div_vs_inline");

    // Compare text + boxes: Div (Block/Flex) vs Inline (Inline Flow)
    let cases = [("low", 50, 0), ("med", 500, 5), ("high", 5000, 50)];

    let dispatcher = TestDispatcher::new(1);
    let cx = TestAppContext::build(dispatcher, None);

    for (name, text_len, box_count) in cases {
        // Div
        group.bench_function(format!("div_{}", name), |b| {
            let mut cx = cx.clone();
            let cx = cx.add_empty_window();
            let view = cx.update(|_, cx| {
                cx.new(|_| DivBenchView {
                    text_len,
                    box_count,
                })
            });

            b.iter(|| {
                cx.draw(
                    point(px(0.0), px(0.0)),
                    size(
                        AvailableSpace::Definite(px(800.0)),
                        AvailableSpace::MinContent,
                    ),
                    |_, _| view.clone().into_any_element(),
                );
            });
        });

        // Inline
        group.bench_function(format!("inline_{}", name), |b| {
            let mut cx = cx.clone();
            let cx = cx.add_empty_window();
            let view = cx.update(|_, cx| {
                cx.new(|_| InlineBenchView {
                    text_len,
                    box_count,
                })
            });

            b.iter(|| {
                cx.draw(
                    point(px(0.0), px(0.0)),
                    size(
                        AvailableSpace::Definite(px(800.0)),
                        AvailableSpace::MinContent,
                    ),
                    |_, _| view.clone().into_any_element(),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_inline_layout, benchmark_div_vs_inline);
criterion_main!(benches);
