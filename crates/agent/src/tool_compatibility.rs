use std::sync::Arc;

use assistant_tool::{Tool, ToolSource, ToolWorkingSet, ToolWorkingSetEvent};
use collections::HashMap;
use gpui::{App, Context, Entity, IntoElement, Render, Subscription, Window};
use language_model::{LanguageModel, LanguageModelToolSchemaFormat};
use ui::prelude::*;

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

pub struct IncompatibleToolsTooltip {
    pub incompatible_tools: Vec<Arc<dyn Tool>>,
}

impl Render for IncompatibleToolsTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::tooltip_container(window, cx, |container, _, cx| {
            container
                .w_72()
                .child(Label::new("Incompatible Tools").size(LabelSize::Small))
                .child(
                    Label::new(
                        "This model is incompatible with the following tools from your MCPs:",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .child(
                    v_flex()
                        .my_1p5()
                        .py_0p5()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                        .children(
                            self.incompatible_tools
                                .iter()
                                .map(|tool| h_flex().gap_4().child(Label::new(tool.name()).size(LabelSize::Small)).map(|parent|
                                    match tool.source() {
                                        ToolSource::Native => parent,
                                        ToolSource::ContextServer { id } => parent.child(Label::new(id).size(LabelSize::Small).color(Color::Muted)),
                                    }
                                )),
                        ),
                )
                .child(Label::new("What To Do Instead").size(LabelSize::Small))
                .child(
                    Label::new(
                        "Every other tool continues to work with this model, but to specifically use those, switch to another model.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
        })
    }
}
