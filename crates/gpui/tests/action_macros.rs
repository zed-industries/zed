use gpui::{actions, impl_actions};
use gpui_macros::register_action;
use schemars::JsonSchema;
use serde_derive::Deserialize;

#[test]
fn test_action_macros() {
    actions!(
        test,
        [
            TestAction,
            /// Documented action
            TestActionWithDocs,
        ]
    );

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

#[test]
fn test_multiple_action_attributes() {
    use gpui::Action;

    // Test action with multiple attributes
    #[derive(Clone, Default, PartialEq, gpui::Action)]
    #[action(namespace = test)]
    #[action(deprecated_aliases = ["OldTestAction", "LegacyTestAction"])]
    struct MultiAttributeAction;

    // Verify the action name is correctly set
    assert_eq!(
        MultiAttributeAction::debug_name(),
        "test::MultiAttributeAction"
    );

    // Verify deprecated aliases are correctly set
    assert_eq!(
        MultiAttributeAction::deprecated_aliases(),
        &["OldTestAction", "LegacyTestAction"]
    );

    // Test with custom name and deprecated aliases
    #[derive(Clone, Default, PartialEq, gpui::Action)]
    #[action(name = "custom::CustomName")]
    #[action(deprecated_aliases = ["OldName"])]
    struct CustomNameAction;

    assert_eq!(CustomNameAction::debug_name(), "custom::CustomName");
    assert_eq!(CustomNameAction::deprecated_aliases(), &["OldName"]);
}
