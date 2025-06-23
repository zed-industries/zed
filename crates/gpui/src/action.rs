use crate::SharedString;
use anyhow::{Context as _, Result};
use collections::HashMap;
pub use no_action::{NoAction, is_no_action};
use serde_json::json;
use std::{
    any::{Any, TypeId},
    fmt::Display,
};

/// Actions are used to implement keyboard-driven UI.
/// When you declare an action, you can bind keys to the action in the keymap and
/// listeners for that action in the element tree.
///
/// To declare a list of simple actions, you can use the actions! macro, which defines a simple unit struct
/// action for each listed action name in the given namespace.
/// ```rust
/// actions!(editor, [MoveUp, MoveDown, MoveLeft, MoveRight, Newline]);
/// ```
/// More complex data types can also be actions, providing they implement Clone, PartialEq,
/// and serde_derive::Deserialize.
/// Use `impl_actions!` to automatically implement the action in the given namespace.
/// ```
/// #[derive(Clone, PartialEq, serde_derive::Deserialize)]
/// pub struct SelectNext {
///     pub replace_newest: bool,
/// }
/// impl_actions!(editor, [SelectNext]);
/// ```
///
/// If you want to control the behavior of the action trait manually, you can use the lower-level `#[register_action]`
/// macro, which only generates the code needed to register your action before `main`.
///
/// ```
/// #[derive(gpui::private::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone)]
/// pub struct Paste {
///     pub content: SharedString,
/// }
///
/// ```ignore
/// impl gpui::Action for Paste {
///      ///...
/// }
/// register_action!(Paste);
/// ```
///
/// # Derive Macro
///
/// The `Action` trait can be automatically implemented using the derive macro:
///
/// ```ignore
/// #[derive(Clone, Default, PartialEq, Action)]
/// struct Cut;
/// ```
///
/// The derive macro can be configured using the `#[action(...)]` attribute:
///
/// - `name = "namespace::ActionName"` - Override the action's display name
/// - `namespace = identifier` - Set just the namespace (name will be struct name)
/// - `internal` - Mark the action as internal-only and so does not support json deserialization
/// - `deprecated_aliases = ["alias1", "namespace::alias2"]` - Specify deprecated aliases
///
/// ## Examples
///
/// ```ignore
/// // Simple action
/// #[derive(Clone, Default, PartialEq, Action)]
/// struct Cut;
///
/// // Action with custom name
/// #[derive(Clone, Default, PartialEq, Action)]
/// #[action(name = "editor::SaveFile")]
/// struct Save;
///
/// // Action with fields that can be deserialized
/// #[derive(Clone, Default, PartialEq, Deserialize, JsonSchema, Action)]
/// struct Find {
///     query: String,
/// }
///
/// // Internal action that can't be deserialized
/// #[derive(Clone, Default, PartialEq, Action)]
/// #[action(internal)]
/// struct InternalAction {
///     state: u32,
/// }
///
/// // Action with deprecated aliases
/// #[derive(Clone, Default, PartialEq, Action)]
/// #[action(deprecated_aliases = ["editor::RevertFile", "RevertBuffer"])]
/// struct RestoreFile;
/// ```
pub trait Action: Any + Send {
    /// Clone the action into a new box
    fn boxed_clone(&self) -> Box<dyn Action>;

    /// Do a partial equality check on this action and the other
    fn partial_eq(&self, action: &dyn Action) -> bool;

    /// Get the name of this action, for displaying in UI
    fn name(&self) -> &str;

    /// Get the name of this action type (static)
    fn name_for_type() -> &'static str
    where
        Self: Sized;

    /// Build this action from a JSON value. This is used to construct actions from the keymap.
    /// A value of `{}` will be passed for actions that don't have any parameters.
    fn build(value: serde_json::Value) -> Result<Box<dyn Action>>
    where
        Self: Sized;

    /// Optional JSON schema for the action's input data.
    fn action_json_schema(
        _: &mut schemars::r#gen::SchemaGenerator,
    ) -> Option<schemars::schema::Schema>
    where
        Self: Sized,
    {
        None
    }

    /// A list of alternate, deprecated names for this action.
    fn deprecated_aliases() -> &'static [&'static str]
    where
        Self: Sized,
    {
        &[]
    }
}

impl std::fmt::Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn Action")
            .field("name", &self.name())
            .finish()
    }
}

impl dyn Action {
    /// Type-erase Action type.
    pub fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

/// Error type for `Keystroke::parse`. This is used instead of `anyhow::Error` so that Zed can use
/// markdown to display it.
#[derive(Debug)]
pub enum ActionBuildError {
    /// Indicates that an action with this name has not been registered.
    NotFound {
        /// Name of the action that was not found.
        name: String,
    },
    /// Indicates that an error occurred while building the action, typically a JSON deserialization
    /// error.
    BuildError {
        /// Name of the action that was attempting to be built.
        name: String,
        /// Error that occurred while building the action.
        error: anyhow::Error,
    },
}

impl std::error::Error for ActionBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ActionBuildError::NotFound { .. } => None,
            ActionBuildError::BuildError { error, .. } => error.source(),
        }
    }
}

impl Display for ActionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionBuildError::NotFound { name } => {
                write!(f, "Didn't find an action named \"{name}\"")
            }
            ActionBuildError::BuildError { name, error } => {
                write!(f, "Error while building action \"{name}\": {error}")
            }
        }
    }
}

type ActionBuilder = fn(json: serde_json::Value) -> anyhow::Result<Box<dyn Action>>;

pub(crate) struct ActionRegistry {
    by_name: HashMap<SharedString, ActionData>,
    names_by_type_id: HashMap<TypeId, SharedString>,
    all_names: Vec<SharedString>, // So we can return a static slice.
    deprecations: HashMap<SharedString, SharedString>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        let mut this = ActionRegistry {
            by_name: Default::default(),
            names_by_type_id: Default::default(),
            all_names: Default::default(),
            deprecations: Default::default(),
        };

        this.load_actions();

        this
    }
}

struct ActionData {
    pub build: ActionBuilder,
    pub json_schema: fn(&mut schemars::r#gen::SchemaGenerator) -> Option<schemars::schema::Schema>,
}

/// This type must be public so that our macros can build it in other crates.
/// But this is an implementation detail and should not be used directly.
#[doc(hidden)]
pub struct MacroActionBuilder(pub fn() -> MacroActionData);

/// This type must be public so that our macros can build it in other crates.
/// But this is an implementation detail and should not be used directly.
#[doc(hidden)]
pub struct MacroActionData {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub type_id: TypeId,
    pub build: ActionBuilder,
    pub json_schema: fn(&mut schemars::r#gen::SchemaGenerator) -> Option<schemars::schema::Schema>,
}

inventory::collect!(MacroActionBuilder);

impl ActionRegistry {
    /// Load all registered actions into the registry.
    pub(crate) fn load_actions(&mut self) {
        for builder in inventory::iter::<MacroActionBuilder> {
            let action = builder.0();
            self.insert_action(action);
        }
    }

    #[cfg(test)]
    pub(crate) fn load_action<A: Action>(&mut self) {
        self.insert_action(MacroActionData {
            name: A::name_for_type(),
            aliases: A::deprecated_aliases(),
            type_id: TypeId::of::<A>(),
            build: A::build,
            json_schema: A::action_json_schema,
        });
    }

    fn insert_action(&mut self, action: MacroActionData) {
        let name: SharedString = action.name.into();
        self.by_name.insert(
            name.clone(),
            ActionData {
                build: action.build,
                json_schema: action.json_schema,
            },
        );
        for &alias in action.aliases {
            let alias: SharedString = alias.into();
            self.by_name.insert(
                alias.clone(),
                ActionData {
                    build: action.build,
                    json_schema: action.json_schema,
                },
            );
            self.deprecations.insert(alias.clone(), name.clone());
            self.all_names.push(alias);
        }
        self.names_by_type_id.insert(action.type_id, name.clone());
        self.all_names.push(name);
    }

    /// Construct an action based on its name and optional JSON parameters sourced from the keymap.
    pub fn build_action_type(&self, type_id: &TypeId) -> Result<Box<dyn Action>> {
        let name = self
            .names_by_type_id
            .get(type_id)
            .with_context(|| format!("no action type registered for {type_id:?}"))?
            .clone();

        Ok(self.build_action(&name, None)?)
    }

    /// Construct an action based on its name and optional JSON parameters sourced from the keymap.
    pub fn build_action(
        &self,
        name: &str,
        params: Option<serde_json::Value>,
    ) -> std::result::Result<Box<dyn Action>, ActionBuildError> {
        let build_action = self
            .by_name
            .get(name)
            .ok_or_else(|| ActionBuildError::NotFound {
                name: name.to_owned(),
            })?
            .build;
        (build_action)(params.unwrap_or_else(|| json!({}))).map_err(|e| {
            ActionBuildError::BuildError {
                name: name.to_owned(),
                error: e,
            }
        })
    }

    pub fn all_action_names(&self) -> &[SharedString] {
        self.all_names.as_slice()
    }

    pub fn action_schemas(
        &self,
        generator: &mut schemars::r#gen::SchemaGenerator,
    ) -> Vec<(SharedString, Option<schemars::schema::Schema>)> {
        // Use the order from all_names so that the resulting schema has sensible order.
        self.all_names
            .iter()
            .map(|name| {
                let action_data = self
                    .by_name
                    .get(name)
                    .expect("All actions in all_names should be registered");
                (name.clone(), (action_data.json_schema)(generator))
            })
            .collect::<Vec<_>>()
    }

    pub fn action_deprecations(&self) -> &HashMap<SharedString, SharedString> {
        &self.deprecations
    }
}

/// Generate a list of all the registered actions.
/// Useful for transforming the list of available actions into a
/// format suited for static analysis such as in validating keymaps, or
/// generating documentation.
pub fn generate_list_of_all_registered_actions() -> Vec<MacroActionData> {
    let mut actions = Vec::new();
    for builder in inventory::iter::<MacroActionBuilder> {
        actions.push(builder.0());
    }
    actions
}

pub use gpui_macros::Action;

/// Defines and registers unit structs that can be used as actions. For more complex data types, derive `Action`.
#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $( $(#[$attr:meta])* $name:ident),* $(,)? ]) => {
        $(
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug, gpui::Action)]
            #[action(namespace = $namespace)]
            $(#[$attr])*
            pub struct $name;
        )*
    };
}

mod no_action {
    use crate as gpui;
    use std::any::Any as _;

    gpui::actions!(
        zed,
        [
            /// Action with special handling which unbinds the keybinding this is associated with,
            /// if it is the highest precedence match.
            NoAction
        ]
    );

    /// Returns whether or not this action represents a removed key binding.
    pub fn is_no_action(action: &dyn gpui::Action) -> bool {
        action.as_any().type_id() == (NoAction {}).type_id()
    }
}
