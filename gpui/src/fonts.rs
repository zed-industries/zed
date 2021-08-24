use crate::{
    color::Color,
    json::{json, ToJson},
};
pub use font_kit::{
    metrics::Metrics,
    properties::{Properties, Stretch, Style, Weight},
};
use serde::{de, Deserialize};
use serde_json::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(pub usize);

pub type GlyphId = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextStyle {
    pub color: Color,
    pub font_properties: Properties,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Color::from_u32(0xff0000ff),
            font_properties: Default::default(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum WeightJson {
    thin,
    extra_light,
    light,
    normal,
    medium,
    semibold,
    bold,
    extra_bold,
    black,
}

#[derive(Deserialize)]
struct TextStyleJson {
    color: Color,
    weight: Option<WeightJson>,
    #[serde(default)]
    italic: bool,
}

impl<'de> Deserialize<'de> for TextStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;
        if json.is_object() {
            let style_json: TextStyleJson =
                serde_json::from_value(json).map_err(de::Error::custom)?;
            Ok(style_json.into())
        } else {
            Ok(Self {
                color: serde_json::from_value(json).map_err(de::Error::custom)?,
                font_properties: Properties::new(),
            })
        }
    }
}

impl From<Color> for TextStyle {
    fn from(color: Color) -> Self {
        Self {
            color,
            font_properties: Default::default(),
        }
    }
}

impl ToJson for TextStyle {
    fn to_json(&self) -> Value {
        json!({
            "color": self.color.to_json(),
            "font_properties": self.font_properties.to_json(),
        })
    }
}

impl Into<TextStyle> for TextStyleJson {
    fn into(self) -> TextStyle {
        let weight = match self.weight.unwrap_or(WeightJson::normal) {
            WeightJson::thin => Weight::THIN,
            WeightJson::extra_light => Weight::EXTRA_LIGHT,
            WeightJson::light => Weight::LIGHT,
            WeightJson::normal => Weight::NORMAL,
            WeightJson::medium => Weight::MEDIUM,
            WeightJson::semibold => Weight::SEMIBOLD,
            WeightJson::bold => Weight::BOLD,
            WeightJson::extra_bold => Weight::EXTRA_BOLD,
            WeightJson::black => Weight::BLACK,
        };
        let style = if self.italic {
            Style::Italic
        } else {
            Style::Normal
        };
        TextStyle {
            color: self.color,
            font_properties: *Properties::new().weight(weight).style(style),
        }
    }
}

impl ToJson for Properties {
    fn to_json(&self) -> crate::json::Value {
        json!({
            "style": self.style.to_json(),
            "weight": self.weight.to_json(),
            "stretch": self.stretch.to_json(),
        })
    }
}

impl ToJson for Style {
    fn to_json(&self) -> crate::json::Value {
        match self {
            Style::Normal => json!("normal"),
            Style::Italic => json!("italic"),
            Style::Oblique => json!("oblique"),
        }
    }
}

impl ToJson for Weight {
    fn to_json(&self) -> crate::json::Value {
        if self.0 == Weight::THIN.0 {
            json!("thin")
        } else if self.0 == Weight::EXTRA_LIGHT.0 {
            json!("extra light")
        } else if self.0 == Weight::LIGHT.0 {
            json!("light")
        } else if self.0 == Weight::NORMAL.0 {
            json!("normal")
        } else if self.0 == Weight::MEDIUM.0 {
            json!("medium")
        } else if self.0 == Weight::SEMIBOLD.0 {
            json!("semibold")
        } else if self.0 == Weight::BOLD.0 {
            json!("bold")
        } else if self.0 == Weight::EXTRA_BOLD.0 {
            json!("extra bold")
        } else if self.0 == Weight::BLACK.0 {
            json!("black")
        } else {
            json!(self.0)
        }
    }
}

impl ToJson for Stretch {
    fn to_json(&self) -> serde_json::Value {
        json!(self.0)
    }
}
