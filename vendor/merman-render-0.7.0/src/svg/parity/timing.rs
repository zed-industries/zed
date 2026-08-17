use web_time::{Duration, Instant};

fn parse_env_bool(v: &str) -> bool {
    !matches!(
        v.trim().to_ascii_lowercase().as_str(),
        "" | "0" | "false" | "no" | "off"
    )
}

pub(crate) fn render_timing_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("MERMAN_RENDER_TIMING")
            .ok()
            .is_some_and(|v| parse_env_bool(&v))
    })
}

#[derive(Debug, Default, Clone)]
pub(crate) struct RenderTimings {
    pub total: Duration,
    pub deserialize_model: Duration,
    pub build_ctx: Duration,
    pub viewbox: Duration,
    pub render_svg: Duration,
    pub finalize_svg: Duration,
}

#[derive(Debug)]
pub(crate) struct TimingGuard<'a> {
    dst: &'a mut Duration,
    start: Instant,
}

impl<'a> TimingGuard<'a> {
    pub(crate) fn new(dst: &'a mut Duration) -> Self {
        Self {
            dst,
            start: Instant::now(),
        }
    }
}

impl Drop for TimingGuard<'_> {
    fn drop(&mut self) {
        *self.dst += self.start.elapsed();
    }
}
