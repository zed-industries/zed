use gpui::{Action, actions};
use gpui_macros::register_action;
use schemars::JsonSchema;
use serde_derive::Deserialize;

#[test]
fn test_action_macros() {
    actions!(
        test_only,
        [
            SomeAction,
            /// Documented action
            SomeActionWithDocs,
        ]
    );

    #[derive(PartialEq, Clone, Deserialize, JsonSchema, Action)]
    #[action(namespace = test_only)]
    struct AnotherSomeAction;

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

        fn name(&self) -> &'static str {
            unimplemented!()
        }

        fn name_for_type() -> &'static str
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
