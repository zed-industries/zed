use ui::{Component, ComponentScope, RegisterComponent};

#[derive(IntoElement, RegisterComponent)]
pub struct BreakpointIndicator {
    line: usize,
    active: bool,
    color: Hsla,
}

impl Component for BreakpointIndicator {
    fn scope() -> ComponentScope {
        ComponentScope::Debugger
    }

    fn preview() {
        // Implement preview logic here
    }
}
