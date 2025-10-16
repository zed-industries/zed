use std::{ops::Range, rc::Rc, time::Duration};

use gpui::{
    App, Application, Bounds, Context, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    Render, SharedString, UniformListScrollHandle, Window, WindowBounds, WindowOptions, canvas,
    div, point, prelude::*, px, rgb, size, uniform_list,
};

const TOTAL_ITEMS: usize = 10000;
const SCROLLBAR_THUMB_WIDTH: Pixels = px(8.);
const SCROLLBAR_THUMB_HEIGHT: Pixels = px(100.);

pub struct Quote {
    name: SharedString,
    symbol: SharedString,
    last_done: f64,
    prev_close: f64,
    open: f64,
    high: f64,
    low: f64,
    timestamp: Duration,
    volume: i64,
    turnover: f64,
    ttm: f64,
    market_cap: f64,
    float_cap: f64,
    shares: f64,
    pb: f64,
    pe: f64,
    eps: f64,
    dividend: f64,
    dividend_yield: f64,
    dividend_per_share: f64,
    dividend_date: SharedString,
    dividend_payment: f64,
}

impl Quote {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        // simulate a base price in a realistic range
        let prev_close = rng.random_range(100.0..200.0);
        let change = rng.random_range(-5.0..5.0);
        let last_done = prev_close + change;
        let open = prev_close + rng.random_range(-3.0..3.0);
        let high = (prev_close + rng.random_range::<f64, _>(0.0..10.0)).max(open);
        let low = (prev_close - rng.random_range::<f64, _>(0.0..10.0)).min(open);
        let timestamp = Duration::from_secs(rng.random_range(0..86400));
        let volume = rng.random_range(1_000_000..100_000_000);
        let turnover = last_done * volume as f64;
        let symbol = {
            let mut ticker = String::new();
            if rng.random_bool(0.5) {
                ticker.push_str(&format!(
                    "{:03}.{}",
                    rng.random_range(100..1000),
                    rng.random_range(0..10)
                ));
            } else {
                ticker.push_str(&format!(
                    "{}{}",
                    rng.random_range('A'..='Z'),
                    rng.random_range('A'..='Z')
                ));
            }
            ticker.push_str(&format!(".{}", rng.random_range('A'..='Z')));
            ticker
        };
        let name = format!(
            "{} {} - #{}",
            symbol,
            rng.random_range(1..100),
            rng.random_range(10000..100000)
        );
        let ttm = rng.random_range(0.0..10.0);
        let market_cap = rng.random_range(1_000_000.0..10_000_000.0);
        let float_cap = market_cap + rng.random_range(1_000.0..10_000.0);
        let shares = rng.random_range(100.0..1000.0);
        let pb = market_cap / shares;
        let pe = market_cap / shares;
        let eps = market_cap / shares;
        let dividend = rng.random_range(0.0..10.0);
        let dividend_yield = rng.random_range(0.0..10.0);
        let dividend_per_share = rng.random_range(0.0..10.0);
        let dividend_date = SharedString::new(format!(
            "{}-{}-{}",
            rng.random_range(2000..2023),
            rng.random_range(1..12),
            rng.random_range(1..28)
        ));
        let dividend_payment = rng.random_range(0.0..10.0);

        Self {
            name: name.into(),
            symbol: symbol.into(),
            last_done,
            prev_close,
            open,
            high,
            low,
            timestamp,
            volume,
            turnover,
            pb,
            pe,
            eps,
            ttm,
            market_cap,
            float_cap,
            shares,
            dividend,
            dividend_yield,
            dividend_per_share,
            dividend_date,
            dividend_payment,
        }
    }

    fn change(&self) -> f64 {
        (self.last_done - self.prev_close) / self.prev_close * 100.0
    }

    fn change_color(&self) -> gpui::Hsla {
        if self.change() > 0.0 {
            gpui::green()
        } else {
            gpui::red()
        }
    }

    fn turnover_ratio(&self) -> f64 {
        self.volume as f64 / self.turnover * 100.0
    }
}

#[derive(IntoElement)]
struct TableRow {
    ix: usize,
    quote: Rc<Quote>,
}
impl TableRow {
    fn new(ix: usize, quote: Rc<Quote>) -> Self {
        Self { ix, quote }
    }

    fn render_cell(&self, key: &str, width: Pixels, color: gpui::Hsla) -> impl IntoElement {
        div()
            .whitespace_nowrap()
            .truncate()
            .w(width)
            .px_1()
            .child(match key {
                "id" => div().child(format!("{}", self.ix)),
                "symbol" => div().child(self.quote.symbol.clone()),
                "name" => div().child(self.quote.name.clone()),
                "last_done" => div()
                    .text_color(color)
                    .child(format!("{:.3}", self.quote.last_done)),
                "prev_close" => div()
                    .text_color(color)
                    .child(format!("{:.3}", self.quote.prev_close)),
                "change" => div()
                    .text_color(color)
                    .child(format!("{:.2}%", self.quote.change())),
                "timestamp" => div()
                    .text_color(color)
                    .child(format!("{:?}", self.quote.timestamp.as_secs())),
                "open" => div()
                    .text_color(color)
                    .child(format!("{:.2}", self.quote.open)),
                "low" => div()
                    .text_color(color)
                    .child(format!("{:.2}", self.quote.low)),
                "high" => div()
                    .text_color(color)
                    .child(format!("{:.2}", self.quote.high)),
                "ttm" => div()
                    .text_color(color)
                    .child(format!("{:.2}", self.quote.ttm)),
                "eps" => div()
                    .text_color(color)
                    .child(format!("{:.2}", self.quote.eps)),
                "market_cap" => {
                    div().child(format!("{:.2} M", self.quote.market_cap / 1_000_000.0))
                }
                "float_cap" => div().child(format!("{:.2} M", self.quote.float_cap / 1_000_000.0)),
                "turnover" => div().child(format!("{:.2} M", self.quote.turnover / 1_000_000.0)),
                "volume" => div().child(format!("{:.2} M", self.quote.volume as f64 / 1_000_000.0)),
                "turnover_ratio" => div().child(format!("{:.2}%", self.quote.turnover_ratio())),
                "pe" => div().child(format!("{:.2}", self.quote.pe)),
                "pb" => div().child(format!("{:.2}", self.quote.pb)),
                "shares" => div().child(format!("{:.2}", self.quote.shares)),
                "dividend" => div().child(format!("{:.2}", self.quote.dividend)),
                "yield" => div().child(format!("{:.2}%", self.quote.dividend_yield)),
                "dividend_per_share" => {
                    div().child(format!("{:.2}", self.quote.dividend_per_share))
                }
                "dividend_date" => div().child(format!("{}", self.quote.dividend_date)),
                "dividend_payment" => div().child(format!("{:.2}", self.quote.dividend_payment)),
                _ => div().child("--"),
            })
    }
}

const FIELDS: [(&str, f32); 24] = [
    ("id", 64.),
    ("symbol", 64.),
    ("name", 180.),
    ("last_done", 80.),
    ("prev_close", 80.),
    ("open", 80.),
    ("low", 80.),
    ("high", 80.),
    ("ttm", 50.),
    ("market_cap", 96.),
    ("float_cap", 96.),
    ("turnover", 120.),
    ("volume", 100.),
    ("turnover_ratio", 96.),
    ("pe", 64.),
    ("pb", 64.),
    ("eps", 64.),
    ("shares", 96.),
    ("dividend", 64.),
    ("yield", 64.),
    ("dividend_per_share", 64.),
    ("dividend_date", 96.),
    ("dividend_payment", 64.),
    ("timestamp", 120.),
];

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.quote.change_color();
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .bg(if self.ix.is_multiple_of(2) {
                rgb(0xFFFFFF)
            } else {
                rgb(0xFAFAFA)
            })
            .py_0p5()
            .px_2()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width), color)))
    }
}

struct DataTable {
    /// Use `Rc` to share the same quote data across multiple items, avoid cloning.
    quotes: Vec<Rc<Quote>>,
    visible_range: Range<usize>,
    scroll_handle: UniformListScrollHandle,
    /// The position in thumb bounds when dragging start mouse down.
    drag_position: Option<Point<Pixels>>,
}

impl DataTable {
    fn new() -> Self {
        Self {
            quotes: Vec::new(),
            visible_range: 0..0,
            scroll_handle: UniformListScrollHandle::new(),
            drag_position: None,
        }
    }

    fn generate(&mut self) {
        self.quotes = (0..TOTAL_ITEMS).map(|_| Rc::new(Quote::random())).collect();
    }

    fn table_bounds(&self) -> Bounds<Pixels> {
        self.scroll_handle.0.borrow().base_handle.bounds()
    }

    fn scroll_top(&self) -> Pixels {
        self.scroll_handle.0.borrow().base_handle.offset().y
    }

    fn scroll_height(&self) -> Pixels {
        self.scroll_handle
            .0
            .borrow()
            .last_item_size
            .unwrap_or_default()
            .contents
            .height
    }

    fn render_scrollbar(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scroll_height = self.scroll_height();
        let table_bounds = self.table_bounds();
        let table_height = table_bounds.size.height;
        if table_height == px(0.) {
            return div().id("scrollbar");
        }

        let percentage = -self.scroll_top() / scroll_height;
        let offset_top = (table_height * percentage).clamp(
            px(4.),
            (table_height - SCROLLBAR_THUMB_HEIGHT - px(4.)).max(px(4.)),
        );
        let entity = cx.entity();
        let scroll_handle = self.scroll_handle.0.borrow().base_handle.clone();

        div()
            .id("scrollbar")
            .absolute()
            .top(offset_top)
            .right_1()
            .h(SCROLLBAR_THUMB_HEIGHT)
            .w(SCROLLBAR_THUMB_WIDTH)
            .bg(rgb(0xC0C0C0))
            .hover(|this| this.bg(rgb(0xA0A0A0)))
            .rounded_lg()
            .child(
                canvas(
                    |_, _, _| (),
                    move |thumb_bounds, _, window, _| {
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |ev: &MouseDownEvent, _, _, cx| {
                                if !thumb_bounds.contains(&ev.position) {
                                    return;
                                }

                                entity.update(cx, |this, _| {
                                    this.drag_position = Some(
                                        ev.position - thumb_bounds.origin - table_bounds.origin,
                                    );
                                })
                            }
                        });
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |_: &MouseUpEvent, _, _, cx| {
                                entity.update(cx, |this, _| {
                                    this.drag_position = None;
                                })
                            }
                        });

                        window.on_mouse_event(move |ev: &MouseMoveEvent, _, _, cx| {
                            if !ev.dragging() {
                                return;
                            }

                            let Some(drag_pos) = entity.read(cx).drag_position else {
                                return;
                            };

                            let inside_offset = drag_pos.y;
                            let percentage = ((ev.position.y - table_bounds.origin.y
                                + inside_offset)
                                / (table_bounds.size.height))
                                .clamp(0., 1.);

                            let offset_y = ((scroll_height - table_bounds.size.height)
                                * percentage)
                                .clamp(px(0.), scroll_height - SCROLLBAR_THUMB_HEIGHT);
                            scroll_handle.set_offset(point(px(0.), -offset_y));
                            cx.notify(entity.entity_id());
                        })
                    },
                )
                .size_full(),
            )
    }
}

impl Render for DataTable {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::white())
            .text_sm()
            .size_full()
            .p_4()
            .gap_2()
            .flex()
            .flex_col()
            .child(format!(
                "Total {} items, visible range: {:?}",
                self.quotes.len(),
                self.visible_range
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .border_1()
                    .border_color(rgb(0xE0E0E0))
                    .rounded_sm()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .overflow_hidden()
                            .border_b_1()
                            .border_color(rgb(0xE0E0E0))
                            .text_color(rgb(0x555555))
                            .bg(rgb(0xF0F0F0))
                            .py_1()
                            .px_2()
                            .text_xs()
                            .children(FIELDS.map(|(key, width)| {
                                div()
                                    .whitespace_nowrap()
                                    .flex_shrink_0()
                                    .truncate()
                                    .px_1()
                                    .w(px(width))
                                    .child(key.replace("_", " ").to_uppercase())
                            })),
                    )
                    .child(
                        div()
                            .relative()
                            .size_full()
                            .child(
                                uniform_list(
                                    "items",
                                    self.quotes.len(),
                                    cx.processor(move |this, range: Range<usize>, _, _| {
                                        this.visible_range = range.clone();
                                        let mut items = Vec::with_capacity(range.end - range.start);
                                        for i in range {
                                            if let Some(quote) = this.quotes.get(i) {
                                                items.push(TableRow::new(i, quote.clone()));
                                            }
                                        }
                                        items
                                    }),
                                )
                                .size_full()
                                .track_scroll(self.scroll_handle.clone()),
                            )
                            .child(self.render_scrollbar(window, cx)),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1280.0), px(1000.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| {
                    let mut table = DataTable::new();
                    table.generate();
                    table
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
