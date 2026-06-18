use anyhow::{Context, anyhow};
use db::kvp::KeyValueStore;
use gpui::App;
use ui::Window;

use crate::preview;
use crate::shape::{self, Centered, RelativeHeight, RelativeWidth, Shape, ViewportFraction};

const PICKERS_NAMESPACE: &str = "pickers";

pub(crate) fn store_shape_for_this_layout(
    picker_delegate: &'static str,
    preview_layout: Option<preview::Layout>,
    shape: shape::Centered,
    window: &ui::Window,
    cx: &App,
) {
    let shape = PickerConfig::from_centered(shape, window);

    let kvp = KeyValueStore::global(cx);
    db::write_and_log(cx, async move || {
        kvp.scoped(PICKERS_NAMESPACE)
            .write(
                shape_key(picker_delegate, preview_layout),
                serde_json::to_string(&shape).context("Could not serialize size of picker")?,
            )
            .await?;
        // Must be written after the shape or loading can break
        // if this future is ever cancelled.
        kvp.scoped(PICKERS_NAMESPACE)
            .write(
                last_layout_key(picker_delegate),
                layout_as_str(preview_layout).to_string(),
            )
            .await
    });
}

pub(crate) fn try_load_shape(
    picker_delegate: &'static str,
    preview_layout: impl Into<Option<preview::Layout>>,
    cx: &App,
) -> anyhow::Result<Option<Shape>> {
    let Some(shape) = KeyValueStore::global(cx)
        .scoped(PICKERS_NAMESPACE)
        .read(&shape_key(picker_delegate, preview_layout.into()))
        .context("Could not read picker shape from KeyValueStore")?
    else {
        return Ok(None);
    };

    let shape = serde_json::from_str::<PickerConfig>(&shape)
        .context("Could not deserialize loaded picker shape from persistence")?
        .into_centered();
    Ok(Some(Shape::HorizontallyCentered(shape)))
}

pub(crate) fn load_last_preview_layout(
    picker_delegate: &'static str,
    cx: &App,
) -> anyhow::Result<Option<preview::Layout>> {
    let Some(last_layout) = KeyValueStore::global(cx)
        .scoped(PICKERS_NAMESPACE)
        .read(&last_layout_key(picker_delegate))
        .context("Could not read last picker layout from KeyValueStore")?
    else {
        return Ok(None);
    };

    parse_layout(&last_layout)
}

fn shape_key(picker_delegate: &'static str, preview_layout: Option<preview::Layout>) -> String {
    format!("{picker_delegate}/{}", layout_as_str(preview_layout))
}

fn last_layout_key(picker_delegate: &'static str) -> String {
    format!("{picker_delegate}/LAST_PREVIEW_LAYOUT")
}

fn layout_as_str(layout: Option<preview::Layout>) -> &'static str {
    match layout {
        Some(preview::Layout::Hidden) => "hidden",
        Some(preview::Layout::Below) => "below",
        Some(preview::Layout::Right) => "right",
        None => "none",
    }
}

fn parse_layout(s: &str) -> anyhow::Result<Option<preview::Layout>> {
    Ok(Some(match s {
        "hidden" => preview::Layout::Hidden,
        "below" => preview::Layout::Below,
        "right" => preview::Layout::Right,
        "none" => return Ok(None),
        _ => return Err(anyhow!("Unknown layout: `{}`", s)),
    }))
}

/// A resized picker for persisting its size. All values are stored as fractions
/// of the viewport so they remain meaningful across window sizes.
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) struct PickerConfig {
    width: f32,        // relative fraction of viewport
    height: f32,       // relative fraction of viewport
    preview_size: f32, // relative fraction of viewport
}

impl PickerConfig {
    pub(crate) fn from_centered(shape: Centered, window: &Window) -> Self {
        PickerConfig {
            width: shape.width.as_viewport_fraction(window).raw(),
            height: shape.height.as_viewport_fraction(window).raw(),
            preview_size: shape.preview_size.raw(),
        }
    }

    pub(crate) fn into_centered(self) -> Centered {
        Centered {
            width: RelativeWidth::viewport(self.width.clamp(0.0, 1.0)),
            height: RelativeHeight::viewport(self.height.clamp(0.0, 1.0)),
            preview_size: ViewportFraction::fraction(self.preview_size.clamp(0.0, 1.0)),
        }
    }
}
