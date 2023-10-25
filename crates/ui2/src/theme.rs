use gpui2::{
    AnyElement, Bounds, Element, Hsla, IntoAnyElement, LayoutId, Pixels, Result, ViewContext,
    WindowContext,
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

pub fn themed<E, F>(theme: Theme, cx: &mut ViewContext<E::ViewState>, build_child: F) -> Themed<E>
where
    E: Element,
    F: FnOnce(&mut ViewContext<E::ViewState>) -> E,
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

impl<E> IntoAnyElement<E::ViewState> for Themed<E>
where
    E: 'static + Element + Send + Sync,
    E::ElementState: Send + Sync,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

#[derive(Default)]
struct ThemeStack(Vec<Theme>);

impl<E: 'static + Element + Send + Sync> Element for Themed<E>
where
    E::ElementState: Send + Sync,
{
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn id(&self) -> Option<gpui2::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        cx.default_global::<ThemeStack>().0.push(self.theme.clone());
        let element_state = self.child.initialize(view_state, element_state, cx);
        cx.default_global::<ThemeStack>().0.pop();
        element_state
    }

    fn layout(
        &mut self,
        view_state: &mut E::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<E::ViewState>,
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
        view_state: &mut Self::ViewState,
        frame_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
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

pub fn theme(cx: &WindowContext) -> Arc<theme2::Theme> {
    cx.global::<Arc<theme2::Theme>>().clone()
}
