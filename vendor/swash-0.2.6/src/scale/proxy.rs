use super::{
    super::{metrics::MetricsProxy, strike::BitmapStrikesProxy, FontRef},
    color::ColorProxy,
};

#[derive(Copy, Clone)]
pub struct ScalerProxy {
    pub metrics: MetricsProxy,
    pub color: ColorProxy,
    pub bitmaps: BitmapStrikesProxy,
    pub coord_count: u16,
}

impl ScalerProxy {
    pub fn from_font(font: &FontRef) -> Self {
        Self {
            metrics: MetricsProxy::from_font(font),
            color: ColorProxy::from_font(font),
            bitmaps: BitmapStrikesProxy::from_font(font),
            coord_count: font.variations().len() as u16,
        }
    }
}
