use serde_derive::Deserialize;

#[test]
fn test_derive() {
    use gpui2 as gpui;

    #[derive(PartialEq, Clone, Deserialize, gpui2_macros::Action)]
    struct AnotherTestAction;

    #[gpui2_macros::register_action]
    #[derive(PartialEq, Clone, gpui::serde_derive::Deserialize)]
    struct RegisterableAction {}

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
