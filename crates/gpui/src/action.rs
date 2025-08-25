use anyhow::{Context as _, Result};
use collections::HashMap;
pub use gpui_macros::Action;
pub use no_action::{NoAction, is_no_action};
use serde_json::json;
use std::{
    any::{Any, TypeId},
    fmt::Display,
};

/// Defines and registers unit structs that can be used as actions. For more complex data types, derive `Action`.
///
/// For example:
///
/// ```
/// actions!(editor, [MoveUp, MoveDown, MoveLeft, MoveRight, Newline]);
/// ```
///
/// This will create actions with names like `editor::MoveUp`, `editor::MoveDown`, etc.
///
/// The namespace argument `editor` can also be omitted, though it is required for Zed actions.
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
    ([ $( $(#[$attr:meta])* $name:ident),* $(,)? ]) => {
        $(
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug, gpui::Action)]
            $(#[$attr])*
            pub struct $name;
        )*
    };
}

/// Actions are used to implement keyboard-driven UI. When you declare an action, you can bind keys
/// to the action in the keymap and listeners for that action in the element tree.
///
/// To declare a list of simple actions, you can use the actions! macro, which defines a simple unit
/// struct action for each listed action name in the given namespace.
///
/// ```
/// actions!(editor, [MoveUp, MoveDown, MoveLeft, MoveRight, Newline]);
/// ```
///
/// Registering the actions with the same name will result in a panic during  `App` creation.
///
/// # Derive Macro
///
/// More complex data types can also be actions, by using the derive macro for `Action`:
///
/// ```
/// #[derive(Clone, PartialEq, serde::Deserialize, schemars::JsonSchema, Action)]
/// #[action(namespace = editor)]
/// pub struct SelectNext {
///     pub replace_newest: bool,
/// }
/// ```
///
/// The derive macro for `Action` requires that the type implement `Clone` and `PartialEq`. It also
/// requires `serde::Deserialize` and `schemars::JsonSchema` unless `#[action(no_json)]` is
/// specified. In Zed these trait impls are used to load keymaps from JSON.
///
/// Multiple arguments separated by commas may be specified in `#[action(...)]`:
///
/// - `namespace = some_namespace` sets the namespace. In Zed this is required.
///
/// - `name = "ActionName"` overrides the action's name. This must not contain `::`.
///
/// - `no_json` causes the `build` method to always error and `action_json_schema` to return `None`,
///   and allows actions not implement `serde::Serialize` and `schemars::JsonSchema`.
///
/// - `no_register` skips registering the action. This is useful for implementing the `Action` trait
///   while not supporting invocation by name or JSON deserialization.
///
/// - `deprecated_aliases = ["editor::SomeAction"]` specifies deprecated old names for the action.
///   These action names should *not* correspond to any actions that are registered. These old names
///   can then still be used to refer to invoke this action. In Zed, the keymap JSON schema will
///   accept these old names and provide warnings.
///
/// - `deprecated = "Message about why this action is deprecation"` specifies a deprecation message.
///   In Zed, the keymap JSON schema will cause this to be displayed as a warning.
///
/// # Manual Implementation
///
/// If you want to control the behavior of the action trait manually, you can use the lower-level
/// `#[register_action]` macro, which only generates the code needed to register your action before
/// `main`.
///
/// ```
/// #[derive(gpui::private::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone)]
/// pub struct Paste {
///     pub content: SharedString,
/// }
///
/// impl gpui::Action for Paste {
///      ///...
/// }
/// register_action!(Paste);
/// ```
pub trait Action: Any + Send {
    /// Clone the action into a new box
    fn boxed_clone(&self) -> Box<dyn Action>;

    /// Do a partial equality check on this action and the other
    fn partial_eq(&self, action: &dyn Action) -> bool;

    /// Get the name of this action, for displaying in UI
    fn name(&self) -> &'static str;

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
    fn action_json_schema(_: &mut schemars::SchemaGenerator) -> Option<schemars::Schema>
    where
        Self: Sized,
    {
        None
    }

    /// A list of alternate, deprecated names for this action. These names can still be used to
    /// invoke the action. In Zed, the keymap JSON schema will accept these old names and provide
    /// warnings.
    fn deprecated_aliases() -> &'static [&'static str]
    where
        Self: Sized,
    {
        &[]
    }

    /// Returns the deprecation message for this action, if any. In Zed, the keymap JSON schema will
    /// cause this to be displayed as a warning.
    fn deprecation_message() -> Option<&'static str>
    where
        Self: Sized,
    {
        None
    }

    /// The documentation for this action, if any. When using the derive macro for actions
    /// this will be automatically generated from the doc comments on the action struct.
    fn documentation() -> Option<&'static str>
    where
        Self: Sized,
    {
        None
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
    by_name: HashMap<&'static str, ActionData>,
    names_by_type_id: HashMap<TypeId, &'static str>,
    all_names: Vec<&'static str>, // So we can return a static slice.
    deprecated_aliases: HashMap<&'static str, &'static str>, // deprecated name -> preferred name
    deprecation_messages: HashMap<&'static str, &'static str>, // action name -> deprecation message
    documentation: HashMap<&'static str, &'static str>, // action name -> documentation
}

impl Default for ActionRegistry {
    fn default() -> Self {
        let mut this = ActionRegistry {
            by_name: Default::default(),
            names_by_type_id: Default::default(),
            documentation: Default::default(),
            all_names: Default::default(),
            deprecated_aliases: Default::default(),
            deprecation_messages: Default::default(),
        };

        this.load_actions();

        this
    }
}

struct ActionData {
    pub build: ActionBuilder,
    pub json_schema: fn(&mut schemars::SchemaGenerator) -> Option<schemars::Schema>,
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
    pub type_id: TypeId,
    pub build: ActionBuilder,
    pub json_schema: fn(&mut schemars::SchemaGenerator) -> Option<schemars::Schema>,
    pub deprecated_aliases: &'static [&'static str],
    pub deprecation_message: Option<&'static str>,
    pub documentation: Option<&'static str>,
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
            type_id: TypeId::of::<A>(),
            build: A::build,
            json_schema: A::action_json_schema,
            deprecated_aliases: A::deprecated_aliases(),
            deprecation_message: A::deprecation_message(),
            documentation: A::documentation(),
        });
    }

    fn insert_action(&mut self, action: MacroActionData) {
        let name = action.name;
        if self.by_name.contains_key(name) {
            panic!(
                "Action with name `{name}` already registered \
                (might be registered in `#[action(deprecated_aliases = [...])]`."
            );
        }
        self.by_name.insert(
            name,
            ActionData {
                build: action.build,
                json_schema: action.json_schema,
            },
        );
        for &alias in action.deprecated_aliases {
            if self.by_name.contains_key(alias) {
                panic!(
                    "Action with name `{alias}` already registered. \
                    `{alias}` is specified in `#[action(deprecated_aliases = [...])]` for action `{name}`."
                );
            }
            self.by_name.insert(
                alias,
                ActionData {
                    build: action.build,
                    json_schema: action.json_schema,
                },
            );
            self.deprecated_aliases.insert(alias, name);
            self.all_names.push(alias);
        }
        self.names_by_type_id.insert(action.type_id, name);
        self.all_names.push(name);
        if let Some(deprecation_msg) = action.deprecation_message {
            self.deprecation_messages.insert(name, deprecation_msg);
        }
        if let Some(documentation) = action.documentation {
            self.documentation.insert(name, documentation);
        }
    }

    /// Construct an action based on its name and optional JSON parameters sourced from the keymap.
    pub fn build_action_type(&self, type_id: &TypeId) -> Result<Box<dyn Action>> {
        let name = self
            .names_by_type_id
            .get(type_id)
            .with_context(|| format!("no action type registered for {type_id:?}"))?;

        Ok(self.build_action(name, None)?)
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

    pub fn all_action_names(&self) -> &[&'static str] {
        self.all_names.as_slice()
    }

    pub fn action_schemas(
        &self,
        generator: &mut schemars::SchemaGenerator,
    ) -> Vec<(&'static str, Option<schemars::Schema>)> {
        // Use the order from all_names so that the resulting schema has sensible order.
        self.all_names
            .iter()
            .map(|name| {
                let action_data = self
                    .by_name
                    .get(name)
                    .expect("All actions in all_names should be registered");
                (*name, (action_data.json_schema)(generator))
            })
            .collect::<Vec<_>>()
    }

    pub fn deprecated_aliases(&self) -> &HashMap<&'static str, &'static str> {
        &self.deprecated_aliases
    }

    pub fn deprecation_messages(&self) -> &HashMap<&'static str, &'static str> {
        &self.deprecation_messages
    }

    pub fn documentation(&self) -> &HashMap<&'static str, &'static str> {
        &self.documentation
    }
}

/// Generate a list of all the registered actions.
/// Useful for transforming the list of available actions into a
/// format suited for static analysis such as in validating keymaps, or
/// generating documentation.
pub fn generate_list_of_all_registered_actions() -> impl Iterator<Item = MacroActionData> {
    inventory::iter::<MacroActionBuilder>
        .into_iter()
        .map(|builder| builder.0())
}

mod no_action {
    use crate as gpui;
    use std::any::Any as _;

    actions!(
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
