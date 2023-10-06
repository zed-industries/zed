use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use gpui2::color::Hsla;
use gpui2::element::Element;
use gpui2::{serde_json, AppContext, IntoElement, Vector2F, ViewContext, WindowContext};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use theme::ThemeSettings;

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Theme {
    pub name: String,
    pub is_light: bool,
    pub lowest: Layer,
    pub middle: Layer,
    pub highest: Layer,
    pub popover_shadow: Shadow,
    pub modal_shadow: Shadow,
    #[serde(deserialize_with = "deserialize_player_colors")]
    pub players: Vec<PlayerColors>,
    #[serde(deserialize_with = "deserialize_syntax_colors")]
    pub syntax: HashMap<String, Hsla>,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Layer {
    pub base: StyleSet,
    pub variant: StyleSet,
    pub on: StyleSet,
    pub accent: StyleSet,
    pub positive: StyleSet,
    pub warning: StyleSet,
    pub negative: StyleSet,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct StyleSet {
    #[serde(rename = "default")]
    pub default: ContainerColors,
    pub hovered: ContainerColors,
    pub pressed: ContainerColors,
    pub active: ContainerColors,
    pub disabled: ContainerColors,
    pub inverted: ContainerColors,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct ContainerColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct PlayerColors {
    pub selection: Hsla,
    pub cursor: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Shadow {
    pub blur: u8,
    pub color: Hsla,
    pub offset: Vec<u8>,
}

fn deserialize_player_colors<'de, D>(deserializer: D) -> Result<Vec<PlayerColors>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PlayerArrayVisitor;

    impl<'de> Visitor<'de> for PlayerArrayVisitor {
        type Value = Vec<PlayerColors>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object with integer keys")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(
            self,
            mut map: A,
        ) -> Result<Self::Value, A::Error> {
            let mut players = Vec::with_capacity(8);
            while let Some((key, value)) = map.next_entry::<usize, PlayerColors>()? {
                if key < 8 {
                    players.push(value);
                } else {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(key as u64),
                        &"a key in range 0..7",
                    ));
                }
            }
            Ok(players)
        }
    }

    deserializer.deserialize_map(PlayerArrayVisitor)
}

fn deserialize_syntax_colors<'de, D>(deserializer: D) -> Result<HashMap<String, Hsla>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ColorWrapper {
        color: Hsla,
    }

    struct SyntaxVisitor;

    impl<'de> Visitor<'de> for SyntaxVisitor {
        type Value = HashMap<String, Hsla>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with keys and objects with a single color field as values")
        }

        fn visit_map<M>(self, mut map: M) -> Result<HashMap<String, Hsla>, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut result = HashMap::new();
            while let Some(key) = map.next_key()? {
                let wrapper: ColorWrapper = map.next_value()?; // Deserialize values as Hsla
                result.insert(key, wrapper.color);
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(SyntaxVisitor)
}

#[derive(IntoElement)]
pub struct Themed<V: 'static, E: Element<V>> {
    pub(crate) theme: Theme,
    pub(crate) child: E,
    pub(crate) view_type: PhantomData<V>,
}

impl<V: 'static, E: Element<V>> Element<V> for Themed<V, E> {
    type PaintState = E::PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> anyhow::Result<(gpui2::LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        cx.push_theme(self.theme.clone());
        let result = self.child.layout(view, cx);
        cx.pop_theme();
        result
    }

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &gpui2::Layout,
        state: &mut Self::PaintState,
        cx: &mut ViewContext<V>,
    ) where
        Self: Sized,
    {
        cx.push_theme(self.theme.clone());
        self.child.paint(view, parent_origin, layout, state, cx);
        cx.pop_theme();
    }
}

fn preferred_theme<V: 'static>(cx: &AppContext) -> Theme {
    settings::get::<ThemeSettings>(cx)
        .theme
        .deserialized_base_theme
        .lock()
        .get_or_insert_with(|| {
            let theme: Theme =
                serde_json::from_value(settings::get::<ThemeSettings>(cx).theme.base_theme.clone())
                    .unwrap();
            Box::new(theme)
        })
        .downcast_ref::<Theme>()
        .unwrap()
        .clone()
}

pub fn theme(cx: &WindowContext) -> Arc<Theme> {
    cx.theme::<Theme>()
}
