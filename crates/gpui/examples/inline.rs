use gpui::prelude::*;
use gpui::{
    App, Application, Context, InteractiveElement, StatefulInteractiveElement, Window,
    WindowOptions, div, inline, px, rems, rgb,
};

struct InlineLayoutDemo;

impl gpui::Render for InlineLayoutDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::white())
            .id("main-scroll")
            .overflow_y_scroll()
            .p(px(16.0))
            .gap(px(12.0))
            .child("Inline layout tests (pixels only)")
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x555555))
                    .child("Each case tweaks width/height/padding/margin/border on inline() and its inner box."),
            )


            // ---- Test 1: minimal box model (only widths/heights + borders) ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 1: inline has only 1px border; inner box 80×40 with 1px red border")
                    .child({
                        let box_elem = div()
                            .w(px(80.0))
                            .h(px(40.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("inline box");

                        inline()
                            .border_1()
                            .border_color(gpui::black())
                            // .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text.")
                    }),
            )

            // ---- Test 2: inline width + padding, box simple ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 2: inline w=200, p=16; box 80×40 with red border")
                    .child({
                        let box_elem = div()
                            .w(px(80.0))
                            .h(px(40.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("inline box");

                        inline()
                            .w(px(200.0))
                            .p(px(16.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text inside padded inline().")
                    }),
            )

            // ---- Test 3: inline margin, box padding ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 3: inline m=12; box 80×40 with p=6, 1px red border")
                    .child({
                        let box_elem = div()
                            .w(px(80.0))
                            .h(px(40.0))
                            .p(px(6.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("padded box");

                        inline()
                            .m(px(12.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text with margin around inline().")
                    }),
            )

            // ---- Test 4: narrow inline width (wrap), wide inner box ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 4: inline w=160; inner box w=140, tests wrapping inside inline width")
                    .child({
                        let box_elem = div()
                            .w(px(140.0))
                            .h(px(40.0))
                            .p(px(4.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("wide box");

                        inline()
                            .w(px(160.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text that should wrap inside the constrained inline().")
                    }),
            )

            // ---- Test 5: explicit inline height vs tall box ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 5: inline h=40; inner box 80×40 with border")
                    .child({
                        let box_elem = div()
                            .w(px(80.0))
                            .h(px(40.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("tall box");

                        inline()
                            .h(px(40.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Short ")
                            .child(box_elem)
                            .text(" text.")
                    }),
            )

            // ---- Test 6: big inline padding + box margin ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 6: inline p=12; box m=10")
                    .child({
                        let box_elem = div()
                            .w(px(80.0))
                            .h(px(40.0))
                            .m(px(10.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("box margin");

                        inline()
                            .p(px(12.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text with margins around the inner box.")
                    }),
            )

            // ---- Test 7: full stress – w/h + p/m + borders on both ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child("Test 7: inline w=360, h=80, p=10, m=8; box 140×40, p=10, m=10")
                    .child({
                        let box_elem = div()
                            .w(px(140.0))
                            .h(px(40.0))
                            .p(px(10.0))
                            .m(px(10.0))
                            .border_1()
                            .border_color(gpui::red())
                            .child("heavy box");

                        inline()
                            .w(px(80.0))
                            .h(px(80.0))
                            .p(px(10.0))
                            .m(px(8.0))
                            .border_1()
                            .border_color(gpui::black())
                            .font_size(px(16.0))
                            .text("Hello ")
                            .child(box_elem)
                            .text(" tail text in full box-model stress.")
                    }),
            )

            // ---- Test 8: Interactive inline with hover, click, tooltip ----
            .child(
                div()
                    .border_1()
                    .border_color(gpui::black())
                    .p(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child("Test 8: Interactive inline with hover and click")
                    .child({
                        inline()
                            .text("Here is some text with a ")
                            .border_1()
                            .border_color(gpui::black())
                            .child(
                                inline()
                                    .h(px(40.0))
                                    .border_1()
                                    .border_color(gpui::blue())
                                    .text_color(gpui::blue())
                                    .text("clickable link")
                                    .truncate()
                                    .into_element()
                                    .id("interactive-link")
                                    .hover(|style| {
                                        style
                                            .bg(gpui::blue().opacity(0.1))
                                            .border_color(gpui::red())
                                            .cursor_pointer()
                                    })
                                    .on_click(|_event, _window, _cx| {
                                        println!("Interactive inline clicked!");
                                    })
                            )
                            .text(" embedded in the flow. Try hovering and clicking!")
                    }),
            )
            // Test 9: Text ellipsis demonstration
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(8.0))
                    .border_1()
                    .border_color(gpui::green())
                    .child("Test 9: Text ellipsis with width constraint (uses truncate())")
                    .child(
                        inline()
                            .border_1()
                            .border_color(gpui::blue())
                            .text("This is a long text that will overflow and be truncated with an ellipsis")
                            .truncate() // = overflow_hidden + whitespace_nowrap + text_ellipsis
                            .into_element()
                            .id("test-9-inline")
                    ),
            )
            // ---- Test 10: Markdown List Item Reproduction (flex_1 + w_0) ----
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(gpui::white())
                    .p(px(16.0))
                    .border_1()
                    .border_color(gpui::red())
                    .child("Reproduction: Markdown List Item (flex_1 + w_0)")
                    .child(
                        // List Item container
                        div().border_1().border_color(gpui::blue())
                            .child(
                                // Content container (The Culprit!)
                                div()
                                    .child(
                                        // Node 2 (Flex Row) - The Paragraph modified by Image
                                        div()
                                            .mb_2()
                                            .line_height(rems(1.3))
                                            .items_center()
                                            .flex()
                                            .flex_row()
                                            .child(
                                                // Node 0 (Image)
                                                div()
                                                    .w(px(36.))
                                                    .h(px(20.))
                                                    .bg(gpui::green())
                                            )
                                            .child(
                                                // Node 1 (Text)
                                                inline()
                                                    .text(" item one") // Short text should wrap if bug is present
                                                    .text_color(gpui::black())
                                            )
                                    )
                            )
                    )
            )
            // ---- Test 11: truncate with inline box ----
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(8.0))
                    .border_1()
                    .border_color(gpui::black())
                    .child("Test 11: truncate with inline box")
                    .child({
                        let box_elem = div().w(px(32.0)).h(px(16.0)).bg(gpui::red());

                        inline()
                            .border_1()
                            .border_color(gpui::blue())
                            .text("Prefix ")
                            .child(box_elem)
                            .text(" tail text that should be truncated after the box")
                            .truncate()
                    }),
            )
            // ---- Test 12: line clamp ellipsis on last visible line ----
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(8.0))
                    .border_1()
                    .border_color(gpui::black())
                    .child("Test 14: line clamp ellipsis on last visible line")
                    .child(
                        inline()
                            .w(px(180.0))
                            .border_1()
                            .border_color(gpui::blue())
                            .line_clamp(2)
                            .text_ellipsis()
                            .text(
                                "This is a longer paragraph that should wrap to multiple lines \
                                 and show an ellipsis on the last visible line.",
                            ),
                    ),
            )
            // ---- Test 13: ellipsis wider than line (clipped) ----
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(8.0))
                    .border_1()
                    .border_color(gpui::black())
                    .child("Test 15: ellipsis wider than line (clipped)")
                    .child(
                        inline()
                            .border_1()
                            .border_color(gpui::blue())
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_overflow(gpui::TextOverflow::Truncate("................................................".into()))
                            .text("Very long text that can be tested for elipis"),
                    ),
            )
            // ---- Test 14: ellipsis style at run boundary (previous-run bias) ----
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(8.0))
                    .border_1()
                    .border_color(gpui::black())
                    .child("Test 17: ellipsis style at run boundary (prev run color)")
                    .child(
                        inline()
                            // .w(px(120.0))
                            .border_1()
                            .border_color(gpui::blue())
                            .text_color(gpui::red())
                            .text("Very very long red text that can be tested for elipis style")
                            .text_color(gpui::blue())
                            .text("Alos very long blue text that can be tested for elipis style")
                            .truncate(),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // You can tweak these window dimensions to see different wrapping/behaviour
        // let bounds = Bounds::centered(None, size(px(900.0), px(700.0)), cx);

        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| InlineLayoutDemo)
        })
        .unwrap();

        cx.activate(true);
    });
}
