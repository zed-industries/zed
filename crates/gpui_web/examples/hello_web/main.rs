use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, ElementId, SharedString, Task, Window, WindowBounds, WindowOptions, div,
    px, rgb, size,
};

// ---------------------------------------------------------------------------
// Prime counting (intentionally brute-force so it hammers the CPU)
// ---------------------------------------------------------------------------

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn count_primes_in_range(start: u64, end: u64) -> u64 {
    let mut count = 0;
    for n in start..end {
        if is_prime(n) {
            count += 1;
        }
    }
    count
}

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

const NUM_CHUNKS: u64 = 12;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Preset {
    TenMillion,
    FiftyMillion,
    HundredMillion,
}

impl Preset {
    fn label(self) -> &'static str {
        match self {
            Preset::TenMillion => "10 M",
            Preset::FiftyMillion => "50 M",
            Preset::HundredMillion => "100 M",
        }
    }

    fn value(self) -> u64 {
        match self {
            Preset::TenMillion => 10_000_000,
            Preset::FiftyMillion => 50_000_000,
            Preset::HundredMillion => 100_000_000,
        }
    }

    const ALL: [Preset; 3] = [
        Preset::TenMillion,
        Preset::FiftyMillion,
        Preset::HundredMillion,
    ];
}

struct ChunkResult {
    count: u64,
}

struct Run {
    limit: u64,
    chunks_done: u64,
    chunk_results: Vec<ChunkResult>,
    total: Option<u64>,
    elapsed: Option<f64>,
}

struct HelloWeb {
    selected_preset: Preset,
    current_run: Option<Run>,
    history: Vec<SharedString>,
    _tasks: Vec<Task<()>>,
}

impl HelloWeb {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_preset: Preset::TenMillion,
            current_run: None,
            history: Vec::new(),
            _tasks: Vec::new(),
        }
    }

    fn start_search(&mut self, cx: &mut Context<Self>) {
        let limit = self.selected_preset.value();
        let chunk_size = limit / NUM_CHUNKS;

        self.current_run = Some(Run {
            limit,
            chunks_done: 0,
            chunk_results: Vec::new(),
            total: None,
            elapsed: None,
        });
        self._tasks.clear();
        cx.notify();

        let start_time = web_time::Instant::now();

        for i in 0..NUM_CHUNKS {
            let range_start = i * chunk_size;
            let range_end = if i == NUM_CHUNKS - 1 {
                limit
            } else {
                range_start + chunk_size
            };

            let task = cx.spawn(async move |this, cx| {
                let count = cx
                    .background_spawn(async move { count_primes_in_range(range_start, range_end) })
                    .await;

                this.update(cx, |this, cx| {
                    if let Some(run) = &mut this.current_run {
                        run.chunk_results.push(ChunkResult { count });
                        run.chunks_done += 1;

                        if run.chunks_done == NUM_CHUNKS {
                            let total: u64 = run.chunk_results.iter().map(|r| r.count).sum();
                            let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;
                            run.total = Some(total);
                            run.elapsed = Some(elapsed_ms);
                            this.history.push(
                                format!(
                                    "π({}) = {} ({:.0} ms, {} chunks)",
                                    format_number(run.limit),
                                    format_number(total),
                                    elapsed_ms,
                                    NUM_CHUNKS,
                                )
                                .into(),
                            );
                        }
                        cx.notify();
                    }
                })
                .ok();
            });

            self._tasks.push(task);
        }
    }
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

const BG_BASE: u32 = 0x1e1e2e;
const BG_SURFACE: u32 = 0x313244;
const BG_OVERLAY: u32 = 0x45475a;
const TEXT_PRIMARY: u32 = 0xcdd6f4;
const TEXT_SECONDARY: u32 = 0xa6adc8;
const TEXT_DIM: u32 = 0x6c7086;
const ACCENT_YELLOW: u32 = 0xf9e2af;
const ACCENT_GREEN: u32 = 0xa6e3a1;
const ACCENT_BLUE: u32 = 0x89b4fa;
const ACCENT_MAUVE: u32 = 0xcba6f7;

impl Render for HelloWeb {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_running = self.current_run.as_ref().is_some_and(|r| r.total.is_none());

        // -- Preset buttons --
        let preset_row = Preset::ALL.iter().enumerate().fold(
            div().flex().flex_row().gap_2(),
            |row, (index, &preset)| {
                let is_selected = preset == self.selected_preset;
                let (bg, text_color) = if is_selected {
                    (ACCENT_BLUE, BG_BASE)
                } else {
                    (BG_OVERLAY, TEXT_SECONDARY)
                };
                row.child(
                    div()
                        .id(ElementId::NamedInteger("preset".into(), index as u64))
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(bg))
                        .text_color(rgb(text_color))
                        .text_sm()
                        .cursor_pointer()
                        .when(!is_running, |this| {
                            this.on_click(cx.listener(move |this, _event, _window, _cx| {
                                this.selected_preset = preset;
                            }))
                        })
                        .child(preset.label()),
                )
            },
        );

        // -- Go button --
        let (go_bg, go_text, go_label) = if is_running {
            (BG_OVERLAY, TEXT_DIM, "Running…")
        } else {
            (ACCENT_GREEN, BG_BASE, "Count Primes")
        };
        let go_button = div()
            .id("go")
            .px_4()
            .py(px(6.))
            .rounded_md()
            .bg(rgb(go_bg))
            .text_color(rgb(go_text))
            .cursor_pointer()
            .when(!is_running, |this| {
                this.on_click(cx.listener(|this, _event, _window, cx| {
                    this.start_search(cx);
                }))
            })
            .child(go_label);

        // -- Progress / result area --
        let status_area = if let Some(run) = &self.current_run {
            let progress_fraction = run.chunks_done as f32 / NUM_CHUNKS as f32;
            let progress_pct = (progress_fraction * 100.0) as u32;

            let status_text: SharedString = if let Some(total) = run.total {
                format!(
                    "Found {} primes below {} in {:.0} ms",
                    format_number(total),
                    format_number(run.limit),
                    run.elapsed.unwrap_or(0.0),
                )
                .into()
            } else {
                format!(
                    "Searching up to {} … {}/{} chunks  ({}%)",
                    format_number(run.limit),
                    run.chunks_done,
                    NUM_CHUNKS,
                    progress_pct,
                )
                .into()
            };

            let bar_color = if run.total.is_some() {
                ACCENT_GREEN
            } else {
                ACCENT_BLUE
            };

            let chunk_dots =
                (0..NUM_CHUNKS as usize).fold(div().flex().flex_row().gap_1().mt_2(), |row, i| {
                    let done = i < run.chunks_done as usize;
                    let color = if done { ACCENT_MAUVE } else { BG_OVERLAY };
                    row.child(div().size(px(10.)).rounded_sm().bg(rgb(color)))
                });

            div()
                .flex()
                .flex_col()
                .w_full()
                .gap_2()
                .child(div().text_color(rgb(TEXT_PRIMARY)).child(status_text))
                .child(
                    div()
                        .w_full()
                        .h(px(8.))
                        .rounded_sm()
                        .bg(rgb(BG_OVERLAY))
                        .child(
                            div()
                                .h_full()
                                .rounded_sm()
                                .bg(rgb(bar_color))
                                .w(gpui::relative(progress_fraction)),
                        ),
                )
                .child(chunk_dots)
        } else {
            div().flex().flex_col().w_full().child(
                div()
                    .text_color(rgb(TEXT_DIM))
                    .child("Select a range and press Count Primes to begin."),
            )
        };

        // -- History log --
        let history_section = if self.history.is_empty() {
            div()
        } else {
            self.history
                .iter()
                .rev()
                .fold(div().flex().flex_col().gap_1(), |col, entry| {
                    col.child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(entry.clone()),
                    )
                })
        };

        // -- Layout --
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(BG_BASE))
            .justify_center()
            .items_center()
            .gap_4()
            .p_4()
            // Title
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Prime Sieve — GPUI Web"),
            )
            .child(div().text_sm().text_color(rgb(TEXT_DIM)).child(format!(
                "Background threads: {} · Chunks per run: {}",
                std::thread::available_parallelism().map_or(2, |n| n.get().max(2)),
                NUM_CHUNKS,
            )))
            // Controls
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .p_4()
                    .w(px(500.))
                    .rounded_lg()
                    .bg(rgb(BG_SURFACE))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(ACCENT_YELLOW))
                            .child("Count primes below:"),
                    )
                    .child(preset_row)
                    .child(go_button),
            )
            // Status
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(500.))
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(BG_SURFACE))
                    .child(status_area),
            )
            // History
            .when(!self.history.is_empty(), |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .w(px(500.))
                        .p_4()
                        .rounded_lg()
                        .bg(rgb(BG_SURFACE))
                        .gap_2()
                        .child(div().text_sm().text_color(rgb(TEXT_DIM)).child("History"))
                        .child(history_section),
                )
            })
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    gpui_platform::web_init();
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(640.), px(560.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(HelloWeb::new),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
