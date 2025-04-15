use std::sync::Arc;

use assistant_tool::{Tool, ToolWorkingSet, ToolWorkingSetEvent};
use collections::HashMap;
use gpui::{App, Context, Entity, Subscription};
use language_model::{LanguageModel, LanguageModelToolSchemaFormat};

pub struct IncompatibleToolsState {
    cache: HashMap<LanguageModelToolSchemaFormat, Vec<Arc<dyn Tool>>>,
    tool_working_set: Entity<ToolWorkingSet>,
    _tool_working_set_subscription: Subscription,
}

impl IncompatibleToolsState {
    pub fn new(tool_working_set: Entity<ToolWorkingSet>, cx: &mut Context<Self>) -> Self {
        let _tool_working_set_subscription =
            cx.subscribe(&tool_working_set, |this, _, event, _| match event {
                ToolWorkingSetEvent::EnabledToolsChanged => {
                    this.cache.clear();
                }
            });

        Self {
            cache: HashMap::default(),
            tool_working_set,
            _tool_working_set_subscription,
        }
    }

    pub fn has_incompatible_tools(&mut self, model: &Arc<dyn LanguageModel>, cx: &App) -> bool {
        self.incompatible_tools(model, cx).len() > 0
    }

    pub fn incompatible_tools(
        &mut self,
        model: &Arc<dyn LanguageModel>,
        cx: &App,
    ) -> &[Arc<dyn Tool>] {
        self.cache
            .entry(model.tool_input_format())
            .or_insert_with(|| {
                self.tool_working_set
                    .read(cx)
                    .enabled_tools(cx)
                    .iter()
                    .filter(|tool| tool.input_schema(model.tool_input_format()).is_err())
                    .cloned()
                    .collect()
            })
    }
}
