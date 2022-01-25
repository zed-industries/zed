use crate::{
    color::Color,
    font_cache::FamilyId,
    json::{json, ToJson},
    text_layout::RunStyle,
    FontCache,
};
use anyhow::anyhow;
pub use font_kit::{
    metrics::Metrics,
    properties::{Properties, Stretch, Style, Weight},
};
use serde::{de, Deserialize};
use serde_json::Value;
use std::{cell::RefCell, sync::Arc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(pub usize);

pub type GlyphId = u32;

#[derive(Clone, Debug)]
pub struct TextStyle {
    pub color: Color,
    pub font_family_name: Arc<str>,
    pub font_family_id: FamilyId,
    pub font_id: FontId,
    pub font_size: f32,
    pub font_properties: Properties,
    pub underline: Option<Color>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct HighlightStyle {
    pub color: Color,
    pub font_properties: Properties,
    pub underline: Option<Color>,
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

thread_local! {
    static FONT_CACHE: RefCell<Option<Arc<FontCache>>> = Default::default();
}

#[derive(Deserialize)]
struct TextStyleJson {
    color: Color,
    family: String,
    weight: Option<WeightJson>,
    size: f32,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    underline: UnderlineStyleJson,
}

#[derive(Deserialize)]
struct HighlightStyleJson {
    color: Color,
    weight: Option<WeightJson>,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    underline: UnderlineStyleJson,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UnderlineStyleJson {
    Underlined(bool),
    UnderlinedWithColor(Color),
}

impl TextStyle {
    pub fn new(
        font_family_name: impl Into<Arc<str>>,
        font_size: f32,
        font_properties: Properties,
        underline: Option<Color>,
        color: Color,
        font_cache: &FontCache,
    ) -> anyhow::Result<Self> {
        let font_family_name = font_family_name.into();
        let font_family_id = font_cache.load_family(&[&font_family_name])?;
        let font_id = font_cache.select_font(font_family_id, &font_properties)?;
        Ok(Self {
            color,
            font_family_name,
            font_family_id,
            font_id,
            font_size,
            font_properties,
            underline,
        })
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn to_run(&self) -> RunStyle {
        RunStyle {
            font_id: self.font_id,
            color: self.color,
            underline: self.underline,
        }
    }

    fn from_json(json: TextStyleJson) -> anyhow::Result<Self> {
        FONT_CACHE.with(|font_cache| {
            if let Some(font_cache) = font_cache.borrow().as_ref() {
                let font_properties = properties_from_json(json.weight, json.italic);
                Self::new(
                    json.family,
                    json.size,
                    font_properties,
                    underline_from_json(json.underline, json.color),
                    json.color,
                    font_cache,
                )
            } else {
                Err(anyhow!(
                    "TextStyle can only be deserialized within a call to with_font_cache"
                ))
            }
        })
    }

    pub fn line_height(&self, font_cache: &FontCache) -> f32 {
        font_cache.line_height(self.font_id, self.font_size)
    }

    pub fn cap_height(&self, font_cache: &FontCache) -> f32 {
        font_cache.cap_height(self.font_id, self.font_size)
    }

    pub fn x_height(&self, font_cache: &FontCache) -> f32 {
        font_cache.x_height(self.font_id, self.font_size)
    }

    pub fn em_width(&self, font_cache: &FontCache) -> f32 {
        font_cache.em_width(self.font_id, self.font_size)
    }

    pub fn em_advance(&self, font_cache: &FontCache) -> f32 {
        font_cache.em_advance(self.font_id, self.font_size)
    }

    pub fn descent(&self, font_cache: &FontCache) -> f32 {
        font_cache.metric(self.font_id, |m| m.descent) * self.em_scale(font_cache)
    }

    pub fn baseline_offset(&self, font_cache: &FontCache) -> f32 {
        font_cache.baseline_offset(self.font_id, self.font_size)
    }

    fn em_scale(&self, font_cache: &FontCache) -> f32 {
        font_cache.em_scale(self.font_id, self.font_size)
    }
}

impl From<TextStyle> for HighlightStyle {
    fn from(other: TextStyle) -> Self {
        Self {
            color: other.color,
            font_properties: other.font_properties,
            underline: other.underline,
        }
    }
}

impl Default for UnderlineStyleJson {
    fn default() -> Self {
        Self::Underlined(false)
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        FONT_CACHE.with(|font_cache| {
            let font_cache = font_cache.borrow();
            let font_cache = font_cache
                .as_ref()
                .expect("TextStyle::default can only be called within a call to with_font_cache");

            let font_family_name = Arc::from("Courier");
            let font_family_id = font_cache.load_family(&[&font_family_name]).unwrap();
            let font_id = font_cache
                .select_font(font_family_id, &Default::default())
                .unwrap();
            Self {
                color: Default::default(),
                font_family_name,
                font_family_id,
                font_id,
                font_size: 14.,
                font_properties: Default::default(),
                underline: Default::default(),
            }
        })
    }
}

impl HighlightStyle {
    fn from_json(json: HighlightStyleJson) -> Self {
        let font_properties = properties_from_json(json.weight, json.italic);
        Self {
            color: json.color,
            font_properties,
            underline: underline_from_json(json.underline, json.color),
        }
    }
}

impl From<Color> for HighlightStyle {
    fn from(color: Color) -> Self {
        Self {
            color,
            font_properties: Default::default(),
            underline: None,
        }
    }
}

impl<'de> Deserialize<'de> for TextStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_json(TextStyleJson::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(e))?)
    }
}

impl ToJson for TextStyle {
    fn to_json(&self) -> Value {
        json!({
            "color": self.color.to_json(),
            "font_family": self.font_family_name.as_ref(),
            "font_properties": self.font_properties.to_json(),
        })
    }
}

impl<'de> Deserialize<'de> for HighlightStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = serde_json::Value::deserialize(deserializer)?;
        if json.is_object() {
            Ok(Self::from_json(
                serde_json::from_value(json).map_err(de::Error::custom)?,
            ))
        } else {
            Ok(Self {
                color: serde_json::from_value(json).map_err(de::Error::custom)?,
                font_properties: Properties::new(),
                underline: None,
            })
        }
    }
}

fn underline_from_json(json: UnderlineStyleJson, text_color: Color) -> Option<Color> {
    match json {
        UnderlineStyleJson::Underlined(false) => None,
        UnderlineStyleJson::Underlined(true) => Some(text_color),
        UnderlineStyleJson::UnderlinedWithColor(color) => Some(color),
    }
}

fn properties_from_json(weight: Option<WeightJson>, italic: bool) -> Properties {
    let weight = match weight.unwrap_or(WeightJson::normal) {
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
    let style = if italic { Style::Italic } else { Style::Normal };
    *Properties::new().weight(weight).style(style)
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

pub fn with_font_cache<F, T>(font_cache: Arc<FontCache>, callback: F) -> T
where
    F: FnOnce() -> T,
{
    FONT_CACHE.with(|cache| {
        *cache.borrow_mut() = Some(font_cache);
        let result = callback();
        cache.borrow_mut().take();
        result
    })
}
