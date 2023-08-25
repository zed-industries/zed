use crate::{
    color::Color,
    font_cache::FamilyId,
    json::{json, ToJson},
    text_layout::RunStyle,
    FontCache,
};
use anyhow::{anyhow, Result};
pub use font_kit::{
    metrics::Metrics,
    properties::{Properties, Stretch, Style, Weight},
};
use ordered_float::OrderedFloat;
use refineable::Refineable;
use schemars::JsonSchema;
use serde::{de, Deserialize, Serialize};
use serde_json::Value;
use std::{cell::RefCell, sync::Arc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, JsonSchema)]
pub struct FontId(pub usize);

pub type GlyphId = u32;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Features {
    pub calt: Option<bool>,
    pub case: Option<bool>,
    pub cpsp: Option<bool>,
    pub frac: Option<bool>,
    pub liga: Option<bool>,
    pub onum: Option<bool>,
    pub ordn: Option<bool>,
    pub pnum: Option<bool>,
    pub ss01: Option<bool>,
    pub ss02: Option<bool>,
    pub ss03: Option<bool>,
    pub ss04: Option<bool>,
    pub ss05: Option<bool>,
    pub ss06: Option<bool>,
    pub ss07: Option<bool>,
    pub ss08: Option<bool>,
    pub ss09: Option<bool>,
    pub ss10: Option<bool>,
    pub ss11: Option<bool>,
    pub ss12: Option<bool>,
    pub ss13: Option<bool>,
    pub ss14: Option<bool>,
    pub ss15: Option<bool>,
    pub ss16: Option<bool>,
    pub ss17: Option<bool>,
    pub ss18: Option<bool>,
    pub ss19: Option<bool>,
    pub ss20: Option<bool>,
    pub subs: Option<bool>,
    pub sups: Option<bool>,
    pub swsh: Option<bool>,
    pub titl: Option<bool>,
    pub tnum: Option<bool>,
    pub zero: Option<bool>,
}

#[derive(Clone, Debug, JsonSchema, Refineable)]
pub struct TextStyle {
    pub color: Color,
    pub font_family_name: Arc<str>,
    pub font_family_id: FamilyId,
    pub font_id: FontId,
    pub font_size: f32,
    #[schemars(with = "PropertiesDef")]
    pub font_properties: Properties,
    pub underline: Underline,
    pub soft_wrap: bool,
}

impl TextStyle {
    pub fn for_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn refine(self, refinement: TextStyleRefinement) -> TextStyle {
        TextStyle {
            color: refinement.color.unwrap_or(self.color),
            font_family_name: refinement
                .font_family_name
                .unwrap_or_else(|| self.font_family_name.clone()),
            font_family_id: refinement.font_family_id.unwrap_or(self.font_family_id),
            font_id: refinement.font_id.unwrap_or(self.font_id),
            font_size: refinement.font_size.unwrap_or(self.font_size),
            font_properties: refinement.font_properties.unwrap_or(self.font_properties),
            underline: refinement.underline.unwrap_or(self.underline),
            soft_wrap: refinement.soft_wrap.unwrap_or(self.soft_wrap),
        }
    }
}

#[derive(JsonSchema)]
#[serde(remote = "Properties")]
pub struct PropertiesDef {
    /// The font style, as defined in CSS.
    pub style: StyleDef,
    /// The font weight, as defined in CSS.
    pub weight: f32,
    /// The font stretchiness, as defined in CSS.
    pub stretch: f32,
}

#[derive(JsonSchema)]
#[schemars(remote = "Style")]
pub enum StyleDef {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct HighlightStyle {
    pub color: Option<Color>,
    #[schemars(with = "Option::<f32>")]
    pub weight: Option<Weight>,
    pub italic: Option<bool>,
    pub underline: Option<Underline>,
    pub fade_out: Option<f32>,
}

impl Eq for HighlightStyle {}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, JsonSchema)]
pub struct Underline {
    pub color: Option<Color>,
    #[schemars(with = "f32")]
    pub thickness: OrderedFloat<f32>,
    pub squiggly: bool,
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
    #[serde(default)]
    features: Features,
    weight: Option<WeightJson>,
    size: f32,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    underline: UnderlineStyleJson,
}

#[derive(Deserialize)]
struct HighlightStyleJson {
    color: Option<Color>,
    weight: Option<WeightJson>,
    italic: Option<bool>,
    underline: Option<UnderlineStyleJson>,
    fade_out: Option<f32>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UnderlineStyleJson {
    Underlined(bool),
    UnderlinedWithProperties {
        #[serde(default)]
        color: Option<Color>,
        #[serde(default)]
        thickness: Option<f32>,
        #[serde(default)]
        squiggly: bool,
    },
}

impl TextStyle {
    pub fn new(
        font_family_name: impl Into<Arc<str>>,
        font_size: f32,
        font_properties: Properties,
        font_features: Features,
        underline: Underline,
        color: Color,
        font_cache: &FontCache,
    ) -> Result<Self> {
        let font_family_name = font_family_name.into();
        let font_family_id = font_cache.load_family(&[&font_family_name], &font_features)?;
        let font_id = font_cache.select_font(font_family_id, &font_properties)?;
        Ok(Self {
            color,
            font_family_name,
            font_family_id,
            font_id,
            font_size,
            font_properties,
            underline,
            soft_wrap: false,
        })
    }

    pub fn default(font_cache: &FontCache) -> Self {
        let font_family_id = font_cache.known_existing_family();
        let font_id = font_cache
            .select_font(font_family_id, &Default::default())
            .expect("did not have any font in system-provided family");
        let font_family_name = font_cache
            .family_name(font_family_id)
            .expect("we loaded this family from the font cache, so this should work");

        Self {
            color: Color::default(),
            font_family_name,
            font_family_id,
            font_id,
            font_size: 14.,
            font_properties: Default::default(),
            underline: Default::default(),
            soft_wrap: true,
        }
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn highlight(mut self, style: HighlightStyle, font_cache: &FontCache) -> Result<Self> {
        let mut font_properties = self.font_properties;
        if let Some(weight) = style.weight {
            font_properties.weight(weight);
        }
        if let Some(italic) = style.italic {
            if italic {
                font_properties.style(Style::Italic);
            } else {
                font_properties.style(Style::Normal);
            }
        }

        if self.font_properties != font_properties {
            self.font_id = font_cache.select_font(self.font_family_id, &font_properties)?;
        }
        if let Some(color) = style.color {
            self.color = Color::blend(color, self.color);
        }
        if let Some(factor) = style.fade_out {
            self.color.fade_out(factor);
        }
        if let Some(underline) = style.underline {
            self.underline = underline;
        }

        Ok(self)
    }

    pub fn to_run(&self) -> RunStyle {
        RunStyle {
            font_id: self.font_id,
            color: self.color,
            underline: self.underline,
        }
    }

    fn from_json(json: TextStyleJson) -> Result<Self> {
        FONT_CACHE.with(|font_cache| {
            if let Some(font_cache) = font_cache.borrow().as_ref() {
                let font_properties = properties_from_json(json.weight, json.italic);
                Self::new(
                    json.family,
                    json.size,
                    font_properties,
                    json.features,
                    underline_from_json(json.underline),
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
        font_cache.line_height(self.font_size)
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
        Self::from(&other)
    }
}

impl From<&TextStyle> for HighlightStyle {
    fn from(other: &TextStyle) -> Self {
        Self {
            color: Some(other.color),
            weight: Some(other.font_properties.weight),
            italic: Some(other.font_properties.style == Style::Italic),
            underline: Some(other.underline),
            fade_out: None,
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
            Self::default(font_cache)
        })
    }
}

impl HighlightStyle {
    fn from_json(json: HighlightStyleJson) -> Self {
        Self {
            color: json.color,
            weight: json.weight.map(weight_from_json),
            italic: json.italic,
            underline: json.underline.map(underline_from_json),
            fade_out: json.fade_out,
        }
    }

    pub fn highlight(&mut self, other: HighlightStyle) {
        match (self.color, other.color) {
            (Some(self_color), Some(other_color)) => {
                self.color = Some(Color::blend(other_color, self_color));
            }
            (None, Some(other_color)) => {
                self.color = Some(other_color);
            }
            _ => {}
        }

        if other.weight.is_some() {
            self.weight = other.weight;
        }

        if other.italic.is_some() {
            self.italic = other.italic;
        }

        if other.underline.is_some() {
            self.underline = other.underline;
        }

        match (other.fade_out, self.fade_out) {
            (Some(source_fade), None) => self.fade_out = Some(source_fade),
            (Some(source_fade), Some(dest_fade)) => {
                let source_alpha = 1. - source_fade;
                let dest_alpha = 1. - dest_fade;
                let blended_alpha = source_alpha + (dest_alpha * source_fade);
                let blended_fade = 1. - blended_alpha;
                self.fade_out = Some(blended_fade);
            }
            _ => {}
        }
    }
}

impl From<Color> for HighlightStyle {
    fn from(color: Color) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}

impl<'de> Deserialize<'de> for TextStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_json(TextStyleJson::deserialize(deserializer)?).map_err(de::Error::custom)
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
                ..Default::default()
            })
        }
    }
}

fn underline_from_json(json: UnderlineStyleJson) -> Underline {
    match json {
        UnderlineStyleJson::Underlined(false) => Underline::default(),
        UnderlineStyleJson::Underlined(true) => Underline {
            color: None,
            thickness: 1.0.into(),
            squiggly: false,
        },
        UnderlineStyleJson::UnderlinedWithProperties {
            color,
            thickness,
            squiggly,
        } => Underline {
            color,
            thickness: thickness.unwrap_or(1.).into(),
            squiggly,
        },
    }
}

fn properties_from_json(weight: Option<WeightJson>, italic: bool) -> Properties {
    let weight = weight.map(weight_from_json).unwrap_or_default();
    let style = if italic { Style::Italic } else { Style::Normal };
    *Properties::new().weight(weight).style(style)
}

fn weight_from_json(weight: WeightJson) -> Weight {
    match weight {
        WeightJson::thin => Weight::THIN,
        WeightJson::extra_light => Weight::EXTRA_LIGHT,
        WeightJson::light => Weight::LIGHT,
        WeightJson::normal => Weight::NORMAL,
        WeightJson::medium => Weight::MEDIUM,
        WeightJson::semibold => Weight::SEMIBOLD,
        WeightJson::bold => Weight::BOLD,
        WeightJson::extra_bold => Weight::EXTRA_BOLD,
        WeightJson::black => Weight::BLACK,
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
