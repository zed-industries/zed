use crate::json::{json, ToJson};
pub use font_kit::{
    metrics::Metrics,
    properties::{Properties, Stretch, Style, Weight},
};
use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(pub usize);

pub type GlyphId = u32;

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
struct PropertiesJson {
    weight: Option<WeightJson>,
    #[serde(default)]
    italic: bool,
}

impl Into<Properties> for PropertiesJson {
    fn into(self) -> Properties {
        let mut result = Properties::new();
        result.weight = match self.weight.unwrap_or(WeightJson::normal) {
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
        if self.italic {
            result.style = Style::Italic;
        }
        result
    }
}

pub fn deserialize_option_font_properties<'de, D>(
    deserializer: D,
) -> Result<Option<Properties>, D::Error>
where
    D: Deserializer<'de>,
{
    let json: Option<PropertiesJson> = Deserialize::deserialize(deserializer)?;
    Ok(json.map(Into::into))
}

pub fn deserialize_font_properties<'de, D>(deserializer: D) -> Result<Properties, D::Error>
where
    D: Deserializer<'de>,
{
    let json: PropertiesJson = Deserialize::deserialize(deserializer)?;
    Ok(json.into())
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
