use language_model::RequestUsage;
use ui::{component_prelude::*, prelude::*};
use zed_llm_client::Plan;

#[derive(RegisterComponent)]
pub struct UsageCallout {
    plan: Plan,
    usage: RequestUsage,
}

impl Component for UsageCallout {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AgentUsageCallout"
    }

    fn preview(_window: &mut ui::Window, _cx: &mut ui::App) -> Option<ui::AnyElement> {
        Some(div().into_any_element()) // preview here
    }
}
