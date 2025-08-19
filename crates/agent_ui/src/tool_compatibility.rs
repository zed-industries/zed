use agent::{Thread, ThreadEvent};
use assistant_tool::{Tool, ToolSource};
use collections::HashMap;
use gpui::{App, Context, Entity, IntoElement, Render, Subscription, Window};
use language_model::{LanguageModel, LanguageModelToolSchemaFormat};
use std::sync::Arc;
use ui::prelude::*;

pub struct IncompatibleToolsState {
    cache: HashMap<LanguageModelToolSchemaFormat, Vec<Arc<dyn Tool>>>,
    thread: Entity<Thread>,
    _thread_subscription: Subscription,
}

impl IncompatibleToolsState {
    pub fn new(thread: Entity<Thread>, cx: &mut Context<Self>) -> Self {
        let _tool_working_set_subscription = cx.subscribe(&thread, |this, _, event, _| {
            if let ThreadEvent::ProfileChanged = event {
                this.cache.clear();
            }
        });

        Self {
            cache: HashMap::default(),
            thread,
            _thread_subscription: _tool_working_set_subscription,
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
                self.thread
                    .read(cx)
                    .profile()
                    .enabled_tools(cx)
                    .iter()
                    .filter(|(_, tool)| tool.input_schema(model.tool_input_format()).is_err())
                    .map(|(_, tool)| tool.clone())
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
