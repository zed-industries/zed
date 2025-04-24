use crate::SharedString;
use anyhow::{Result, anyhow};
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
    fn name(&self) -> &str;

    /// Get the name of this action for debugging
    fn debug_name() -> &'static str
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
            name: A::debug_name(),
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
            .ok_or_else(|| anyhow!("no action type registered for {:?}", type_id))?
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

/// Defines and registers unit structs that can be used as actions.
///
/// To use more complex data types as actions, use `impl_actions!`
#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            // Unfortunately rust-analyzer doesn't display the name due to
            // https://github.com/rust-lang/rust-analyzer/issues/8092
            #[doc = stringify!($name)]
            #[doc = "action generated by `gpui::actions!`"]
            #[derive(::std::clone::Clone,::std::cmp::PartialEq, ::std::default::Default)]
            pub struct $name;

            gpui::__impl_action!($namespace, $name, $name,
                fn build(_: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    Ok(Box::new(Self))
                },
                fn action_json_schema(
                    _: &mut gpui::private::schemars::r#gen::SchemaGenerator,
                ) -> Option<gpui::private::schemars::schema::Schema> {
                    None
                }
            );

            gpui::register_action!($name);
        )*
    };
}

/// Defines and registers a unit struct that can be used as an actions, with a name that differs
/// from it's type name.
///
/// To use more complex data types as actions, and rename them use `impl_action_as!`
#[macro_export]
macro_rules! action_as {
    ($namespace:path, $name:ident as $visual_name:ident) => {
        // Unfortunately rust-analyzer doesn't display the name due to
        // https://github.com/rust-lang/rust-analyzer/issues/8092
        #[doc = stringify!($name)]
        #[doc = "action generated by `gpui::action_as!`"]
        #[derive(
            ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq,
        )]
        pub struct $name;

        gpui::__impl_action!(
            $namespace,
            $name,
            $visual_name,
            fn build(
                _: gpui::private::serde_json::Value,
            ) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                Ok(Box::new(Self))
            },
            fn action_json_schema(
                generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                None
            }
        );

        gpui::register_action!($name);
    };
}

/// Defines and registers a unit struct that can be used as an action, with some deprecated aliases.
#[macro_export]
macro_rules! action_with_deprecated_aliases {
    ($namespace:path, $name:ident, [$($alias:literal),* $(,)?]) => {
        // Unfortunately rust-analyzer doesn't display the name due to
        // https://github.com/rust-lang/rust-analyzer/issues/8092
        #[doc = stringify!($name)]
        #[doc = "action, generated by `gpui::action_with_deprecated_aliases!`"]
        #[derive(
            ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq,
        )]
        pub struct $name;

        gpui::__impl_action!(
            $namespace,
            $name,
            $name,
            fn build(
                value: gpui::private::serde_json::Value,
            ) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                Ok(Box::new(Self))
            },

            fn action_json_schema(
                generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                None
            },

            fn deprecated_aliases() -> &'static [&'static str] {
                &[
                    $($alias),*
                ]
            }
        );

        gpui::register_action!($name);
    };
}

/// Registers the action and implements the Action trait for any struct that implements Clone,
/// Default, PartialEq, serde_deserialize::Deserialize, and schemars::JsonSchema.
///
/// Similar to `impl_actions!`, but only handles one struct, and registers some deprecated aliases.
#[macro_export]
macro_rules! impl_action_with_deprecated_aliases {
    ($namespace:path, $name:ident, [$($alias:literal),* $(,)?]) => {
        gpui::__impl_action!(
            $namespace,
            $name,
            $name,
            fn build(
                value: gpui::private::serde_json::Value,
            ) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                Ok(std::boxed::Box::new(gpui::private::serde_json::from_value::<Self>(value)?))
            },

            fn action_json_schema(
                generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                Some(<Self as gpui::private::schemars::JsonSchema>::json_schema(
                    generator,
                ))
            },

            fn deprecated_aliases() -> &'static [&'static str] {
                &[
                    $($alias),*
                ]
            }
        );

        gpui::register_action!($name);
    };
}

/// Registers the action and implements the Action trait for any struct that implements Clone,
/// Default, PartialEq, serde_deserialize::Deserialize, and schemars::JsonSchema.
///
/// Similar to `actions!`, but accepts structs with fields.
///
/// Fields and variants that don't make sense for user configuration should be annotated with
/// #[serde(skip)].
#[macro_export]
macro_rules! impl_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            gpui::__impl_action!($namespace, $name, $name,
                fn build(value: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    Ok(std::boxed::Box::new(gpui::private::serde_json::from_value::<Self>(value)?))
                },
                fn action_json_schema(
                    generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
                ) -> Option<gpui::private::schemars::schema::Schema> {
                    Some(<Self as gpui::private::schemars::JsonSchema>::json_schema(
                        generator,
                    ))
                }
            );

            gpui::register_action!($name);
        )*
    };
}

/// Implements the Action trait for internal action structs that implement Clone, Default,
/// PartialEq. The purpose of this is to conveniently define values that can be passed in `dyn
/// Action`.
///
/// These actions are internal and so are not registered and do not support deserialization.
#[macro_export]
macro_rules! impl_internal_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            gpui::__impl_action!($namespace, $name, $name,
                fn build(value: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    gpui::Result::Err(gpui::private::anyhow::anyhow!(
                        concat!(
                            stringify!($namespace),
                            "::",
                            stringify!($visual_name),
                            " is an internal action, so cannot be built from JSON."
                        )))
                },
                fn action_json_schema(
                    generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
                ) -> Option<gpui::private::schemars::schema::Schema> {
                    None
                }
            );
        )*
    };
}

/// Implements the Action trait for a struct that implements Clone, Default, PartialEq, and
/// serde_deserialize::Deserialize. Allows you to rename the action visually, without changing the
/// struct's name.
///
/// Fields and variants that don't make sense for user configuration should be annotated with
/// #[serde(skip)].
#[macro_export]
macro_rules! impl_action_as {
    ($namespace:path, $name:ident as $visual_name:tt ) => {
        gpui::__impl_action!(
            $namespace,
            $name,
            $visual_name,
            fn build(
                value: gpui::private::serde_json::Value,
            ) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                Ok(std::boxed::Box::new(
                    gpui::private::serde_json::from_value::<Self>(value)?,
                ))
            },
            fn action_json_schema(
                generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                Some(<Self as gpui::private::schemars::JsonSchema>::json_schema(
                    generator,
                ))
            }
        );

        gpui::register_action!($name);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_action {
    ($namespace:path, $name:ident, $visual_name:tt, $($items:item),*) => {
        impl gpui::Action for $name {
            fn name(&self) -> &'static str
            {
                concat!(
                    stringify!($namespace),
                    "::",
                    stringify!($visual_name),
                )
            }

            fn debug_name() -> &'static str
            where
                Self: ::std::marker::Sized
            {
                concat!(
                    stringify!($namespace),
                    "::",
                    stringify!($visual_name),
                )
            }

            fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
                action
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn boxed_clone(&self) ->  std::boxed::Box<dyn gpui::Action> {
                ::std::boxed::Box::new(self.clone())
            }


            $($items)*
        }
    };
}

mod no_action {
    use crate as gpui;
    use std::any::Any as _;

    actions!(zed, [NoAction]);

    /// Returns whether or not this action represents a removed key binding.
    pub fn is_no_action(action: &dyn gpui::Action) -> bool {
        action.as_any().type_id() == (NoAction {}).type_id()
    }
}
