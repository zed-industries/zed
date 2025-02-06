use gpui::{
    div, point, prelude::*, px, rgb, size, uniform_list, App, Application, Bounds, Context, Pixels,
    Render, SharedString, Window, WindowBounds, WindowOptions,
};
use std::rc::Rc;

pub struct Quote {
    name: SharedString,
    symbol: SharedString,
    last_done: f64,
    prev_close: f64,
    open: f64,
    high: f64,
    low: f64,
    timestamp: i64,
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
        let mut rng = rand::thread_rng();
        // simulate a base price in a realistic range
        let prev_close = rng.gen_range(100.0..200.0);
        let change = rng.gen_range(-5.0..5.0);
        let last_done = prev_close + change;
        let open = prev_close + rng.gen_range(-3.0..3.0);
        let high = (prev_close + rng.gen_range::<f64, _>(0.0..10.0)).max(open);
        let low = (prev_close - rng.gen_range::<f64, _>(0.0..10.0)).min(open);
        let timestamp = 1651115955 + rng.gen_range(-1000..1000);
        let volume = rng.gen_range(1_000_000..100_000_000);
        let turnover = last_done * volume as f64;
        let symbol = {
            let mut ticker = String::new();
            if rng.gen_bool(0.5) {
                ticker.push_str(&format!(
                    "{:03}.{}",
                    rng.gen_range(100..1000),
                    rng.gen_range(0..10)
                ));
            } else {
                ticker.push_str(&format!(
                    "{}{}",
                    rng.gen_range('A'..='Z'),
                    rng.gen_range('A'..='Z')
                ));
            }
            ticker.push_str(&format!(".{}", rng.gen_range('A'..='Z')));
            ticker
        };
        let name = format!(
            "{} {} - #{}",
            symbol,
            rng.gen_range(1..100),
            rng.gen_range(10000..100000)
        );
        let ttm = rng.gen_range(0.0..10.0);
        let market_cap = rng.gen_range(100.0..1000.0);
        let float_cap = rng.gen_range(100.0..1000.0);
        let shares = rng.gen_range(100.0..1000.0);
        let pb = market_cap / shares;
        let pe = market_cap / shares;
        let eps = market_cap / shares;
        let dividend = rng.gen_range(0.0..10.0);
        let dividend_yield = rng.gen_range(0.0..10.0);
        let dividend_per_share = rng.gen_range(0.0..10.0);
        let dividend_date = SharedString::new(format!(
            "{}-{}-{}",
            rng.gen_range(2000..2023),
            rng.gen_range(1..12),
            rng.gen_range(1..28)
        ));
        let dividend_payment = rng.gen_range(0.0..10.0);

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
        self.turnover / self.volume as f64 * 100.0
    }
}

#[derive(IntoElement)]
struct Item {
    quote: Rc<Quote>,
}
impl Item {
    fn new(quote: Rc<Quote>) -> Self {
        Item { quote }
    }

    fn render_cell(&self, key: &str, width: Pixels, color: gpui::Hsla) -> impl IntoElement {
        div()
            .whitespace_nowrap()
            .truncate()
            .w(width)
            .child(match key {
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
                    .child(format!("{}", self.quote.timestamp)),
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
                "market_cap" => div().child(format!("{:.2}", self.quote.market_cap)),
                "float_cap" => div().child(format!("{:.2}", self.quote.float_cap)),
                "turnover" => div().child(format!("{:.2}", self.quote.turnover)),
                "volume" => div().child(format!("{:.2}", self.quote.volume)),
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

const FIELDS: [(&str, f32); 23] = [
    ("symbol", 64.),
    ("name", 220.),
    ("last_done", 80.),
    ("prev_close", 80.),
    ("open", 80.),
    ("low", 80.),
    ("high", 80.),
    ("ttm", 50.),
    ("market_cap", 96.),
    ("float_cap", 96.),
    ("turnover", 96.),
    ("volume", 96.),
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
impl RenderOnce for Item {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.quote.change_color();
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .py_0p5()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width), color)))
    }
}

struct DataTable {
    quotes: Vec<Rc<Quote>>,
}

impl DataTable {
    fn new() -> Self {
        Self { quotes: Vec::new() }
    }

    fn generate(&mut self) {
        self.quotes = (0..10000).map(|_| Rc::new(Quote::random())).collect();
    }
}

impl Render for DataTable {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .text_sm()
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(format!("Total: {} items", self.quotes.len()))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .py_0p5()
                    .border_b_1()
                    .border_color(rgb(0xE0E0E0))
                    .children(FIELDS.map(|(key, width)| {
                        div()
                            .whitespace_nowrap()
                            .flex_shrink_0()
                            .truncate()
                            .w(px(width))
                            .child(key)
                    })),
            )
            .child(
                uniform_list(entity, "items", self.quotes.len(), {
                    move |this, range, _, _| {
                        let mut items = Vec::with_capacity(range.end - range.start);
                        for i in range {
                            if let Some(quote) = this.quotes.get(i) {
                                items.push(Item::new(quote.clone()));
                            }
                        }
                        items
                    }
                })
                .size_full(),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.0)),
                    size: size(px(1200.0), px(1000.0)),
                })),
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
