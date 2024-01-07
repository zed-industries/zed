use gpui::{actions, impl_actions};
use gpui_macros::register_action;
use serde_derive::Deserialize;

#[test]
fn test_action_macros() {
    actions!(test, [TestAction]);

    #[derive(PartialEq, Clone, Deserialize)]
    struct AnotherTestAction;

    impl_actions!(test, [AnotherTestAction]);

    #[derive(PartialEq, Clone, gpui::private::serde_derive::Deserialize)]
    struct RegisterableAction {}

    register_action!(RegisterableAction);

    impl gpui::Action for RegisterableAction {
        fn boxed_clone(&self) -> Box<dyn gpui::Action> {
            todo!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            todo!()
        }

        fn partial_eq(&self, _action: &dyn gpui::Action) -> bool {
            todo!()
        }

        fn name(&self) -> &str {
            todo!()
        }

        fn debug_name() -> &'static str
        where
            Self: Sized,
        {
            todo!()
        }

        fn build(_value: serde_json::Value) -> anyhow::Result<Box<dyn gpui::Action>>
        where
            Self: Sized,
        {
            todo!()
        }
    }
}
