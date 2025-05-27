use gpui::{actions, impl_actions};
use gpui_macros::register_action;
use schemars::JsonSchema;
use serde_derive::Deserialize;

#[test]
fn test_action_macros() {
    actions!(test, [TestAction]);

    #[derive(PartialEq, Clone, Deserialize, JsonSchema)]
    struct AnotherTestAction;

    impl_actions!(test, [AnotherTestAction]);

    #[derive(PartialEq, Clone, gpui::private::serde_derive::Deserialize)]
    struct RegisterableAction {}

    register_action!(RegisterableAction);

    impl gpui::Action for RegisterableAction {
        fn boxed_clone(&self) -> Box<dyn gpui::Action> {
            unimplemented!()
        }

        fn partial_eq(&self, _action: &dyn gpui::Action) -> bool {
            unimplemented!()
        }

        fn name(&self) -> &str {
            unimplemented!()
        }

        fn debug_name() -> &'static str
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn build(_value: serde_json::Value) -> anyhow::Result<Box<dyn gpui::Action>>
        where
            Self: Sized,
        {
            unimplemented!()
        }
    }
}
