use std::any::TypeId;

use collections::HashSet;
use gpui::{Action, AppContext, Global};

#[derive(Default)]
pub struct CommandPaletteFilter {
    pub hidden_namespaces: HashSet<&'static str>,
    pub hidden_action_types: HashSet<TypeId>,
}

impl Global for CommandPaletteFilter {}

pub struct CommandPaletteInterceptor(
    pub Box<dyn Fn(&str, &AppContext) -> Option<CommandInterceptResult>>,
);

impl Global for CommandPaletteInterceptor {}

pub struct CommandInterceptResult {
    pub action: Box<dyn Action>,
    pub string: String,
    pub positions: Vec<usize>,
}
