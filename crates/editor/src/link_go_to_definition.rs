use std::{
    ops::Range,
    time::{Duration, Instant},
};

use gpui::{
    actions,
    elements::{Flex, MouseEventHandler, Padding, Text},
    impl_internal_actions,
    platform::CursorStyle,
    Axis, Element, ElementBox, ModelHandle, MutableAppContext, RenderContext, Task, ViewContext,
};
use language::Bias;
use project::{HoverBlock, Project};
use util::TryFutureExt;

use crate::{
    display_map::ToDisplayPoint, Anchor, AnchorRangeExt, DisplayPoint, Editor, EditorSnapshot,
    EditorStyle,
};

#[derive(Clone, PartialEq)]
pub struct FetchDefinition {
    pub point: Option<DisplayPoint>,
}

#[derive(Clone, PartialEq)]
pub struct GoToFetchedDefinition {
    pub point: Option<DisplayPoint>,
}

impl_internal_actions!(edtior, [FetchDefinition, GoToFetchedDefinition]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(fetch_definition);
    cx.add_action(go_to_fetched_definition);
}

pub fn fetch_definition(
    editor: &mut Editor,
    FetchDefinition { point }: &FetchDefinition,
    cx: &mut ViewContext<Editor>,
) {
}

pub fn go_to_fetched_definition(
    editor: &mut Editor,
    GoToFetchedDefinition { point }: &GoToFetchedDefinition,
    cx: &mut ViewContext<Editor>,
) {
}

#[derive(Default)]
pub struct LinkGoToDefinitionState {
    pub triggered_from
    pub symbol_range: Option<Range<Anchor>>,
    pub task: Option<Task<Option<()>>>,
}