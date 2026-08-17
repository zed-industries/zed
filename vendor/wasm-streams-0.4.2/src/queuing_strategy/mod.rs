pub mod sys;

#[derive(Debug)]
pub(crate) struct QueuingStrategy {
    raw: sys::QueuingStrategy,
}

impl QueuingStrategy {
    pub fn new(high_water_mark: f64) -> Self {
        let raw = sys::QueuingStrategy::new();
        raw.set_high_water_mark(high_water_mark);
        Self { raw }
    }

    #[inline]
    pub fn into_raw(self) -> web_sys::QueuingStrategy {
        self.raw
    }
}
