use gpui2::{
    AnyElement, AppContext, Bounds, Component, Element, Hsla, LayoutId, Pixels, Result,
    ViewContext, WindowContext,
};
use serde::{de::Visitor, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

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

pub fn themed<V, E, F>(theme: Theme, cx: &mut ViewContext<V>, build_child: F) -> Themed<E>
where
    V: 'static,
    E: Element<V>,
    F: FnOnce(&mut ViewContext<V>) -> E,
{
    cx.default_global::<ThemeStack>().0.push(theme.clone());
    let child = build_child(cx);
    cx.default_global::<ThemeStack>().0.pop();
    Themed { theme, child }
}

pub struct Themed<E> {
    pub(crate) theme: Theme,
    pub(crate) child: E,
}

impl<V, E> Component<V> for Themed<E>
where
    V: 'static,
    E: 'static + Element<V> + Send,
    E::ElementState: Send,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

#[derive(Default)]
struct ThemeStack(Vec<Theme>);

impl<V, E: 'static + Element<V> + Send> Element<V> for Themed<E>
where
    V: 'static,
    E::ElementState: Send,
{
    type ElementState = E::ElementState;

    fn id(&self) -> Option<gpui2::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        cx.default_global::<ThemeStack>().0.push(self.theme.clone());
        let element_state = self.child.initialize(view_state, element_state, cx);
        cx.default_global::<ThemeStack>().0.pop();
        element_state
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId
    where
        Self: Sized,
    {
        cx.default_global::<ThemeStack>().0.push(self.theme.clone());
        let layout_id = self.child.layout(view_state, element_state, cx);
        cx.default_global::<ThemeStack>().0.pop();
        layout_id
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        frame_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) where
        Self: Sized,
    {
        cx.default_global::<ThemeStack>().0.push(self.theme.clone());
        self.child.paint(bounds, view_state, frame_state, cx);
        cx.default_global::<ThemeStack>().0.pop();
    }
}

pub fn old_theme(cx: &WindowContext) -> Arc<Theme> {
    Arc::new(cx.global::<Theme>().clone())
}

pub fn theme(cx: &AppContext) -> Arc<theme2::Theme> {
    theme2::active_theme(cx).clone()
}
