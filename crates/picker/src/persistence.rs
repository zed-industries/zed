use ui::Window;

use crate::shape;

/// Describes a resized picker for persisting it's size
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PickerConfig {
    width: f32,        // relative fraction of viewport
    height: f32,       // relative fraction of viewport
    preview_size: f32, // relative fraction of viewport
}

impl PickerConfig {
    // TODO!(yara) make this infaillible and a from impl
    fn from(shape: shape::Centered, window: &Window) -> Self {
        PickerConfig {
            width: shape.width.as_viewport_fraction(window).raw(),
            height: shape.height.as_viewport_fraction(window).raw(),
            preview_size: shape.preview_size.raw(),
        }
    }
}

/// A string uniquely identifying a picker + preview layout
pub struct PickerKey(String);

pub fn update_picker_db(picker: PickerKey, config: PickerConfig, cx: gpui::App) {
    cx.spawn(async move |_| write_picker_config(picker, config).await)
        .detach_and_log_err();
}

async fn write_picker_config(
    kvp: &KeyValueStore,
    picker: PickerKey,
    config: PickerConfig,
) -> anyhow::Result<()> {
    let json_str = serde_json(config);
    kvp.write_kvp(key.0, json_str)?;
    Ok(())
}
