//! Input Sandbox - A simple example for testing single-line and multi-line inputs.
//!
//! Run with: `cargo run -p gpui --example input_sandbox`

use gpui::input::bind_input_keys;
use gpui::{
    App, Application, Bounds, Context, Div, Entity, FocusHandle, Focusable, InputState, KeyBinding,
    Stateful, Window, WindowBounds, WindowOptions, div, input, prelude::*, px, rgb, size,
    text_area,
};

struct InputSandbox {
    multiline_input: Entity<InputState>,
    singleline_input: Entity<InputState>,
    use_multiline: bool,
    current_sample: SampleText,
}

impl InputSandbox {
    fn new(cx: &mut Context<Self>) -> Self {
        let initial_sample = SampleText::Typography;

        let multiline_input = cx.new(|cx| {
            let mut input = InputState::new_multiline(cx);
            input.set_content(initial_sample.content(), cx);
            input
        });

        let singleline_input = cx.new(|cx| {
            let mut input = InputState::new_singleline(cx);
            input.set_content("Single-line text input example", cx);
            input
        });

        Self {
            multiline_input,
            singleline_input,
            use_multiline: true,
            current_sample: initial_sample,
        }
    }

    fn toggle_mode(&mut self, _: &ToggleMode, _window: &mut Window, cx: &mut Context<Self>) {
        self.use_multiline = !self.use_multiline;
        cx.notify();
    }

    fn set_sample(&mut self, sample: SampleText, cx: &mut Context<Self>) {
        self.current_sample = sample;
        self.multiline_input.update(cx, |input, cx| {
            input.set_content(sample.content(), cx);
        });
        cx.notify();
    }

    fn active_input(&self) -> &Entity<InputState> {
        if self.use_multiline {
            &self.multiline_input
        } else {
            &self.singleline_input
        }
    }
}

impl Focusable for InputSandbox {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_input().focus_handle(cx)
    }
}

impl Render for InputSandbox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_input = self.active_input().clone();
        let input_state = active_input.read(cx);
        let content = input_state.content().to_string();
        let selected_range = input_state.selected_range().clone();
        let cursor_offset = input_state.cursor_offset();
        let char_count = content.chars().count();
        let line_count = content.lines().count().max(1);

        let focus_handle = active_input.focus_handle(cx);

        let multiline_focus = self.multiline_input.focus_handle(cx);
        let singleline_focus = self.singleline_input.focus_handle(cx);

        div()
            .id("input-sandbox")
            .key_context("InputSandbox")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::toggle_mode))
            .flex()
            .flex_row()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xcccccc))
            .size_full()
            // Left panel - Content area
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_4()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .when(self.use_multiline, |this| {
                                this.child(
                                    text_area(&self.multiline_input)
                                        .size_full()
                                        .bg(rgb(0x1e1e1e))
                                        .text_color(rgb(0xd4d4d4))
                                        .text_base()
                                        .selection_color(gpui::rgba(0x3388ff44))
                                        .cursor_color(rgb(0xffffff)),
                                )
                            })
                            .when(!self.use_multiline, |this| {
                                this.child(
                                    div().flex().items_center().h(px(40.)).child(
                                        input(&self.singleline_input)
                                            .size_full()
                                            .bg(rgb(0x1e1e1e))
                                            .text_color(rgb(0xd4d4d4))
                                            .text_base()
                                            .selection_color(gpui::rgba(0x3388ff44))
                                            .cursor_color(rgb(0xffffff)),
                                    ),
                                )
                            }),
                    ),
            )
            // Right panel - Sidebar
            .child(
                div()
                    .id("sidebar")
                    .w(px(240.))
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .bg(rgb(0x252526))
                    .border_l_1()
                    .border_color(rgb(0x3c3c3c))
                    .overflow_y_scroll()
                    // Mode toggle section
                    .child(
                        sidebar_section("Mode").child(
                            div()
                                .flex()
                                .gap_2()
                                .child(
                                    toggle_button("multi-btn", "Multi-line", self.use_multiline)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            if !this.use_multiline {
                                                this.toggle_mode(&ToggleMode, window, cx);
                                            }
                                        })),
                                )
                                .child(
                                    toggle_button("single-btn", "Single-line", !self.use_multiline)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            if this.use_multiline {
                                                this.toggle_mode(&ToggleMode, window, cx);
                                            }
                                        })),
                                ),
                        ),
                    )
                    // Sample text selector (only in multiline mode)
                    .when(self.use_multiline, |this| {
                        let current_sample = self.current_sample;
                        this.child(
                            sidebar_section("Sample Text").child(
                                div().flex().flex_col().gap_1().children(
                                    SampleText::ALL.iter().map(|sample| {
                                        let sample = *sample;
                                        let is_active = current_sample == sample;
                                        sample_button(sample, is_active).on_click(cx.listener(
                                            move |this, _, _window, cx| {
                                                this.set_sample(sample, cx);
                                            },
                                        ))
                                    }),
                                ),
                            ),
                        )
                    })
                    // Stats section
                    .child(
                        sidebar_section("Statistics")
                            .child(stat_row("Cursor", format!("{}", cursor_offset)))
                            .child(stat_row(
                                "Selection",
                                format!("{}..{}", selected_range.start, selected_range.end),
                            ))
                            .child(stat_row("Characters", format!("{}", char_count)))
                            .child(stat_row("Lines", format!("{}", line_count)))
                            .child(stat_row("Bytes", format!("{}", content.len()))),
                    )
                    // Focus state section
                    .child(
                        sidebar_section("Focus State")
                            .child(stat_row(
                                "Multi-line",
                                if multiline_focus.is_focused(window) {
                                    "focused"
                                } else {
                                    "—"
                                },
                            ))
                            .child(stat_row(
                                "Single-line",
                                if singleline_focus.is_focused(window) {
                                    "focused"
                                } else {
                                    "—"
                                },
                            )),
                    )
                    // Keybindings section
                    .child(
                        sidebar_section("Keybindings")
                            .child(key_row("Ctrl+T", "Toggle mode"))
                            .child(key_row("Cmd+Z", "Undo"))
                            .child(key_row("Cmd+Shift+Z", "Redo"))
                            .child(key_row("Cmd+A", "Select all"))
                            .child(key_row("Cmd+C", "Copy"))
                            .child(key_row("Cmd+X", "Cut"))
                            .child(key_row("Cmd+V", "Paste"))
                            .child(key_row("Alt+←/→", "Word nav"))
                            .child(key_row("Cmd+←/→", "Line start/end"))
                            .child(key_row("Cmd+↑/↓", "Doc start/end")),
                    ),
            )
    }
}

fn sidebar_section(title: &str) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .p_3()
        .border_b_1()
        .border_color(rgb(0x3c3c3c))
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(rgb(0x888888))
                .child(title.to_uppercase()),
        )
}

fn toggle_button(id: &'static str, label: &str, active: bool) -> Stateful<Div> {
    div()
        .id(id)
        .px_2()
        .py_1()
        .text_xs()
        .rounded_sm()
        .cursor_pointer()
        .when(active, |this| {
            this.bg(rgb(0x0e639c)).text_color(rgb(0xffffff))
        })
        .when(!active, |this| {
            this.bg(rgb(0x3c3c3c))
                .text_color(rgb(0x888888))
                .hover(|s| s.bg(rgb(0x4c4c4c)))
        })
        .child(label.to_string())
}

fn sample_button(sample: SampleText, active: bool) -> Stateful<Div> {
    div()
        .id(sample.label())
        .px_2()
        .py_1()
        .text_xs()
        .rounded_sm()
        .cursor_pointer()
        .w_full()
        .when(active, |this| {
            this.bg(rgb(0x0e639c)).text_color(rgb(0xffffff))
        })
        .when(!active, |this| {
            this.bg(rgb(0x3c3c3c))
                .text_color(rgb(0x888888))
                .hover(|s| s.bg(rgb(0x4c4c4c)))
        })
        .child(sample.label().to_string())
}

fn stat_row(label: &str, value: impl Into<gpui::SharedString>) -> gpui::Div {
    div()
        .flex()
        .justify_between()
        .text_xs()
        .child(div().text_color(rgb(0x888888)).child(label.to_string()))
        .child(div().text_color(rgb(0xcccccc)).child(value.into()))
}

fn key_row(key: &str, desc: &str) -> gpui::Div {
    div()
        .flex()
        .justify_between()
        .gap_2()
        .text_xs()
        .child(
            div()
                .px_1()
                .bg(rgb(0x3c3c3c))
                .rounded_sm()
                .text_color(rgb(0xaaaaaa))
                .flex_shrink_0()
                .child(key.to_string()),
        )
        .child(
            div()
                .text_color(rgb(0x888888))
                .overflow_hidden()
                .child(desc.to_string()),
        )
}

gpui::actions!(input_sandbox, [ToggleMode]);

fn main() {
    Application::new().run(|cx: &mut App| {
        bind_input_keys(cx, None);

        cx.bind_keys([KeyBinding::new("ctrl-t", ToggleMode, None)]);

        let bounds = Bounds::centered(None, size(px(900.), px(700.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(InputSandbox::new);
                let focus_handle = view.read(cx).active_input().focus_handle(cx);
                window.focus(&focus_handle);
                view
            },
        )
        .unwrap();

        cx.activate(true);
    });
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SampleText {
    Typography,
    RtlMixed,
    TrickyText,
}

impl SampleText {
    const ALL: &[SampleText] = &[
        SampleText::Typography,
        SampleText::RtlMixed,
        SampleText::TrickyText,
    ];

    fn label(&self) -> &'static str {
        match self {
            SampleText::Typography => "Typography",
            SampleText::RtlMixed => "RTL/Bidi",
            SampleText::TrickyText => "Tricky Text",
        }
    }

    fn content(&self) -> &'static str {
        match self {
            SampleText::Typography => TYPOGRAPHY_TEXT,
            SampleText::RtlMixed => RTL_MIXED_TEXT,
            SampleText::TrickyText => TRICKY_TEXT,
        }
    }
}

const TYPOGRAPHY_TEXT: &str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZ
abcdefghijklmnopqrstuvwxyz
0123456789!?.

Pixel preview  Resize to fit  zenith zone
Frame  Group  Feedback  Reset
Day day  Month month  Year year
Hour hour  Minute minute  Second second

The quick brown fox jumps over the lazy dog
Pack my box with five dozen liquor jugs
Sphinx of black quartz, judge my vow

jumping far—but not really—over the bar
We found a fix to the ffi problem
Irrational  fi  ffi  fl  ffl

12.4 pt  64%  90px  45 kg  12 o'clock
$64 $7  €64 €64  £7 £7
3° °C °F

#80A6F3  #FFFFFF  #000000
in Drafts • 3 hours ago

• Buy milk?  cc cd ce cq co
• ec ed ee eq eo  oc od oe oq oo"#;

const RTL_MIXED_TEXT: &str = r#"Hebrew:
שלום עולם
מה שלומך היום?

Arabic:
مرحبا بالعالم
كيف حالك اليوم؟

Mixed LTR and RTL:
Hello שלום World עולם
The word مرحبا means hello

Numbers in RTL context:
בשנת 2024 היו 365 ימים
في عام 2024 كان هناك 365 يومًا

Bidirectional with punctuation:
(שלום) "עולם" [מה]!
«مرحبا» "العالم" (كيف)؟

Mixed script sentence:
I learned שלום in Hebrew class
تعلمت "hello" في صف الإنجليزية

Nested direction changes:
Start שלום hello עולם end
Begin مرحبا world العالم finish"#;

const TRICKY_TEXT: &str = r#"═══ EMOJI ═══

Simple: 😀 😎 🎉 ❤️ 🔥 ✨
Skin tones: 👋 👋🏻 👋🏼 👋🏽 👋🏾 👋🏿
ZWJ sequences: 👨‍👩‍👧‍👦  👩‍💻  👨‍🍳  🧑‍🚀
Flags: 🇺🇸 🇬🇧 🇯🇵 🇩🇪 🇫🇷 🏳️‍🌈
Keycaps: 1️⃣ 2️⃣ 3️⃣ #️⃣ *️⃣
Presentation: ☺︎ vs ☺️  ▶︎ vs ▶️
Mixed: Hello 👋 World 🌍! I ❤️ Rust 🦀
Cursor test: →😀← →👨‍👩‍👧‍👦← →🇺🇸← →1️⃣←

═══ MULTIBYTE ═══

Chinese: 你好世界 中文测试
Japanese: こんにちは世界 カタカナ
Korean: 안녕하세요 한국어
Thai: สวัสดีครับ
Hindi: नमस्ते दुनिया
Greek: Γεια σου κόσμε
Russian: Привет мир
Mixed: Hello 你好 こんにちは 안녕 Привет

═══ COMBINING CHARACTERS ═══

Precomposed vs decomposed:
é (precomposed) vs é (e + ́)
ñ vs ñ  •  ü vs ü

Multiple combiners:
ẗ̈ (t + two diacritics)
q̃̃ (q + two tildes)

═══ ZERO-WIDTH & INVISIBLE ═══

Word​Break (ZWJ)
Word‌Break (ZWNJ)
Word⁠Break (word joiner)
Left‎Right (LRM)
Right‏Left (RLM)

Spaces: [ ] (regular) [ ] (NBSP) [] (zero-width)

═══ HOMOGLYPHS ═══

ABCabc (Latin)
АВСавс (Cyrillic - different!)
ΑΒΓαβγ (Greek - different!)

═══ ASTRAL PLANE ═══

𝕳𝖊𝖑𝖑𝖔 (math fraktur)
𝒜𝒷𝒸 (math script)
🜀🜁🜂🜃 (alchemical)

═══ WHITESPACE ═══

Tab:	Column1	Column2
Trailing spaces
  Leading spaces

═══ STRESS TEST ═══

aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

a
b
c"#;
