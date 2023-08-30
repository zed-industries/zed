use crate::{
    color::Hsla,
    element::{Element, PaintContext},
    layout_context::LayoutContext,
};
use gpui::WindowContext;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::{collections::HashMap, fmt, marker::PhantomData};

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Theme {
    name: String,
    is_light: bool,
    lowest: Layer,
    middle: Layer,
    highest: Layer,
    popover_shadow: Shadow,
    modal_shadow: Shadow,
    #[serde(deserialize_with = "deserialize_player_colors")]
    players: Vec<PlayerColors>,
    #[serde(deserialize_with = "deserialize_syntax_colors")]
    syntax: HashMap<String, Hsla>,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Layer {
    base: StyleSet,
    variant: StyleSet,
    on: StyleSet,
    accent: StyleSet,
    positive: StyleSet,
    warning: StyleSet,
    negative: StyleSet,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct StyleSet {
    #[serde(rename = "default")]
    default: ContainerColors,
    hovered: ContainerColors,
    pressed: ContainerColors,
    active: ContainerColors,
    disabled: ContainerColors,
    inverted: ContainerColors,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct ContainerColors {
    background: Hsla,
    foreground: Hsla,
    border: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct PlayerColors {
    selection: Hsla,
    cursor: Hsla,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Shadow {
    blur: u8,
    color: Hsla,
    offset: Vec<u8>,
}

pub fn theme<'a>(cx: &'a WindowContext) -> &'a Theme {
    cx.theme::<Theme>()
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
                let hsla: Hsla = map.next_value()?; // Deserialize values as Hsla
                result.insert(key, hsla);
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(SyntaxVisitor)
}

pub struct Themed<V: 'static, E> {
    pub(crate) theme: Theme,
    pub(crate) child: E,
    pub(crate) view_type: PhantomData<V>,
}

impl<V: 'static, E: Element<V>> Element<V> for Themed<V, E> {
    type PaintState = E::PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> anyhow::Result<(gpui::LayoutId, Self::PaintState)>
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
        layout: &gpui::Layout,
        state: &mut Self::PaintState,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        cx.push_theme(self.theme.clone());
        self.child.paint(view, layout, state, cx);
        cx.pop_theme();
    }
}
