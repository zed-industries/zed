mod tool_preview {
    use ui::{AnyElement, App, Window, component_prelude::*};

    #[derive(RegisterComponent)]
    pub struct Thinking {}

    impl Component for Thinking {
        fn scope() -> ComponentScope {
            ComponentScope::Agent
        }

        fn description() -> Option<&'static str> {
            None
        }

        fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
            None
        }
    }
}
