use gpui::{
    Action, action_as, action_with_deprecated_aliases, actions, impl_action_as,
    impl_action_with_deprecated_aliases, impl_actions, impl_internal_actions,
};
use serde::{Deserialize, Serialize};

// Test basic actions! macro
actions!(test, [SimpleAction, AnotherSimpleAction]);

// Test actions! with doc comments
actions!(
    test,
    [
        /// This action has custom documentation.
        DocumentedAction,
        /// Another documented action.
        AnotherDocumentedAction,
    ]
);

// Test action_as! macro
action_as!(test, InternalName as VisibleName);
action_as!(
    /// This action is renamed for clarity
    test, DocumentedInternalName as DocumentedVisibleName
);

// Test action_with_deprecated_aliases! macro
action_with_deprecated_aliases!(test, ModernAction, [OldAction, LegacyAction]);
action_with_deprecated_aliases!(
    /// This replaces old versions
    test, DocumentedModernAction, [OldV1, OldV2]
);

// Test impl_actions! macro with complex types
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
struct ComplexAction {
    value: String,
    count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
struct AnotherComplexAction {
    enabled: bool,
}

impl_actions!(test, [ComplexAction, AnotherComplexAction]);

// Test impl_action_as! macro
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
struct InternalComplexAction {
    data: String,
}

impl_action_as!(test, InternalComplexAction as VisibleComplexAction);

// Test impl_internal_actions! macro
#[derive(Clone, Debug, Default, PartialEq)]
struct InternalOnlyAction {
    internal_state: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct AnotherInternalAction;

impl_internal_actions!(test, [InternalOnlyAction, AnotherInternalAction]);

// Test impl_action_with_deprecated_aliases! macro
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
struct ComplexActionWithAliases {
    setting: String,
}

impl_action_with_deprecated_aliases!(
    test,
    ComplexActionWithAliases,
    [OldComplexAction, LegacyComplexAction]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_actions() {
        // Test creation and cloning
        let action = SimpleAction;
        assert_eq!(action.name(), "test::SimpleAction");
        assert_eq!(SimpleAction::debug_name(), "test::SimpleAction");

        let cloned = action.boxed_clone();
        assert!(action.partial_eq(cloned.as_ref()));
    }

    #[test]
    fn test_documented_actions() {
        let action = DocumentedAction;
        assert_eq!(action.name(), "test::DocumentedAction");
    }

    #[test]
    fn test_action_as() {
        let action = InternalName;
        assert_eq!(action.name(), "test::VisibleName");
        assert_eq!(InternalName::debug_name(), "test::VisibleName");
    }

    #[test]
    fn test_deprecated_aliases() {
        let action = ModernAction;
        assert_eq!(action.name(), "test::ModernAction");
        assert_eq!(
            ModernAction::deprecated_aliases(),
            &["test::OldAction", "test::LegacyAction"]
        );
    }

    #[test]
    fn test_complex_actions() {
        let action = ComplexAction {
            value: "test".to_string(),
            count: 42,
        };
        assert_eq!(action.name(), "test::ComplexAction");
        assert_eq!(ComplexAction::debug_name(), "test::ComplexAction");

        // Test cloning
        let cloned = action.boxed_clone();
        assert!(action.partial_eq(cloned.as_ref()));
    }

    #[test]
    fn test_complex_action_as() {
        let action = InternalComplexAction {
            data: "test".to_string(),
        };
        assert_eq!(action.name(), "test::VisibleComplexAction");
    }

    #[test]
    fn test_internal_actions() {
        let action = InternalOnlyAction {
            internal_state: 123,
        };
        assert_eq!(action.name(), "test::InternalOnlyAction");

        // Verify it cannot be built from JSON
        let result = InternalOnlyAction::build(serde_json::json!({}));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("internal action"));
    }

    #[test]
    fn test_complex_action_with_aliases() {
        let action = ComplexActionWithAliases {
            setting: "test".to_string(),
        };
        assert_eq!(action.name(), "test::ComplexActionWithAliases");
        assert_eq!(
            ComplexActionWithAliases::deprecated_aliases(),
            &["test::OldComplexAction", "test::LegacyComplexAction"]
        );
    }

    #[test]
    fn test_action_serialization() {
        // Test that complex actions can be serialized/deserialized
        let action = ComplexAction {
            value: "hello".to_string(),
            count: 10,
        };

        let json = serde_json::to_value(&action).unwrap();
        let built = ComplexAction::build(json).unwrap();
        let built_action = built.as_any().downcast_ref::<ComplexAction>().unwrap();

        assert_eq!(built_action.value, "hello");
        assert_eq!(built_action.count, 10);
    }

    #[test]
    fn test_json_schema_generation() {
        // Test that actions with JsonSchema generate schemas
        let mut generator = schemars::r#gen::SchemaGenerator::default();

        // Simple actions don't have schemas
        assert!(SimpleAction::action_json_schema(&mut generator).is_none());

        // Complex actions have schemas
        assert!(ComplexAction::action_json_schema(&mut generator).is_some());
        assert!(ComplexActionWithAliases::action_json_schema(&mut generator).is_some());

        // Internal actions don't have schemas
        assert!(InternalOnlyAction::action_json_schema(&mut generator).is_none());
    }
}
