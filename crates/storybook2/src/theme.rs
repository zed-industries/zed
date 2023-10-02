use gpui3::{Element, Hsla, Layout, LayoutId, Result, StackContext, ViewContext, WindowContext};
use serde::{de::Visitor, Deserialize, Deserializer};
use std::{collections::HashMap, fmt};

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

pub fn themed<E, F>(theme: Theme, cx: &mut ViewContext<E::State>, build_child: F) -> Themed<E>
where
    E: Element,
    F: FnOnce(&mut ViewContext<E::State>) -> E,
{
    let child = cx.with_state(theme.clone(), |cx| build_child(cx));
    Themed { theme, child }
}

pub struct Themed<E> {
    pub(crate) theme: Theme,
    pub(crate) child: E,
}

impl<E: Element> Element for Themed<E> {
    type State = E::State;
    type FrameState = E::FrameState;

    fn layout(
        &mut self,
        state: &mut E::State,
        cx: &mut ViewContext<E::State>,
    ) -> anyhow::Result<(LayoutId, Self::FrameState)>
    where
        Self: Sized,
    {
        cx.with_state(self.theme.clone(), |cx| self.child.layout(state, cx))
    }

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()>
    where
        Self: Sized,
    {
        cx.with_state(self.theme.clone(), |cx| {
            self.child.paint(layout, state, frame_state, cx)
        })
    }
}

// fn preferred_theme<V: 'static>(cx: &AppContext) -> Theme {
//     settings::get::<ThemeSettings>(cx)
//         .theme
//         .deserialized_base_theme
//         .lock()
//         .get_or_insert_with(|| {
//             let theme: Theme =
//                 serde_json::from_value(settings::get::<ThemeSettings>(cx).theme.base_theme.clone())
//                     .unwrap();
//             Box::new(theme)
//         })
//         .downcast_ref::<Theme>()
//         .unwrap()
//         .clone()
// }

pub fn theme<'a>(cx: &'a WindowContext) -> &'a Theme {
    cx.state()
}
