use crate::SharedString;
use anyhow::{anyhow, Context, Result};
use collections::HashMap;
pub use no_action::NoAction;
use serde_json::json;
use std::any::{Any, TypeId};

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
pub trait Action: 'static + Send {
    /// Clone the action into a new box
    fn boxed_clone(&self) -> Box<dyn Action>;

    /// Cast the action to the any type
    fn as_any(&self) -> &dyn Any;

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
}

impl std::fmt::Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn Action")
            .field("name", &self.name())
            .finish()
    }
}

impl dyn Action {
    /// Get the type id of this action
    pub fn type_id(&self) -> TypeId {
        self.as_any().type_id()
    }
}

type ActionBuilder = fn(json: serde_json::Value) -> anyhow::Result<Box<dyn Action>>;

pub(crate) struct ActionRegistry {
    builders_by_name: HashMap<SharedString, ActionBuilder>,
    names_by_type_id: HashMap<TypeId, SharedString>,
    all_names: Vec<SharedString>, // So we can return a static slice.
}

impl Default for ActionRegistry {
    fn default() -> Self {
        let mut this = ActionRegistry {
            builders_by_name: Default::default(),
            names_by_type_id: Default::default(),
            all_names: Default::default(),
        };

        this.load_actions();

        this
    }
}

/// This type must be public so that our macros can build it in other crates.
/// But this is an implementation detail and should not be used directly.
#[doc(hidden)]
pub type MacroActionBuilder = fn() -> ActionData;

/// This type must be public so that our macros can build it in other crates.
/// But this is an implementation detail and should not be used directly.
#[doc(hidden)]
pub struct ActionData {
    pub name: &'static str,
    pub type_id: TypeId,
    pub build: ActionBuilder,
}

/// This constant must be public to be accessible from other crates.
/// But its existence is an implementation detail and should not be used directly.
#[doc(hidden)]
#[linkme::distributed_slice]
pub static __GPUI_ACTIONS: [MacroActionBuilder];

impl ActionRegistry {
    /// Load all registered actions into the registry.
    pub(crate) fn load_actions(&mut self) {
        for builder in __GPUI_ACTIONS {
            let action = builder();
            self.insert_action(action);
        }
    }

    #[cfg(test)]
    pub(crate) fn load_action<A: Action>(&mut self) {
        self.insert_action(ActionData {
            name: A::debug_name(),
            type_id: TypeId::of::<A>(),
            build: A::build,
        });
    }

    fn insert_action(&mut self, action: ActionData) {
        let name: SharedString = action.name.into();
        self.builders_by_name.insert(name.clone(), action.build);
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

        self.build_action(&name, None)
    }

    /// Construct an action based on its name and optional JSON parameters sourced from the keymap.
    pub fn build_action(
        &self,
        name: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Box<dyn Action>> {
        let build_action = self
            .builders_by_name
            .get(name)
            .ok_or_else(|| anyhow!("no action type registered for {}", name))?;
        (build_action)(params.unwrap_or_else(|| json!({})))
            .with_context(|| format!("Attempting to build action {}", name))
    }

    pub fn all_action_names(&self) -> &[SharedString] {
        self.all_names.as_slice()
    }
}

/// Defines unit structs that can be used as actions.
/// To use more complex data types as actions, use `impl_actions!`
#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            #[doc = "The `"]
            #[doc = stringify!($name)]
            #[doc = "` action, see [`gpui::actions!`]"]
            #[derive(::std::cmp::PartialEq, ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, gpui::private::serde_derive::Deserialize)]
            #[serde(crate = "gpui::private::serde")]
            pub struct $name;

            gpui::__impl_action!($namespace, $name, $name,
                fn build(_: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    Ok(Box::new(Self))
                }
            );

            gpui::register_action!($name);
        )*
    };
}

/// Defines a unit struct that can be used as an actions, with a name
/// that differs from it's type name.
///
/// To use more complex data types as actions, and rename them use
/// `impl_action_as!`
#[macro_export]
macro_rules! action_as {
    ($namespace:path, $name:ident as $visual_name:tt) => {
        #[doc = "The `"]
        #[doc = stringify!($name)]
        #[doc = "` action, see [`gpui::actions!`]"]
        #[derive(
            ::std::cmp::PartialEq,
            ::std::clone::Clone,
            ::std::default::Default,
            ::std::fmt::Debug,
            gpui::private::serde_derive::Deserialize,
        )]
        #[serde(crate = "gpui::private::serde")]
        pub struct $name;

        gpui::__impl_action!(
            $namespace,
            $name,
            $visual_name,
            fn build(
                _: gpui::private::serde_json::Value,
            ) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                Ok(Box::new(Self))
            }
        );

        gpui::register_action!($name);
    };
}

/// Implements the Action trait for any struct that implements Clone, Default, PartialEq, and serde_deserialize::Deserialize
#[macro_export]
macro_rules! impl_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            gpui::__impl_action!($namespace, $name, $name,
                fn build(value: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    Ok(std::boxed::Box::new(gpui::private::serde_json::from_value::<Self>(value)?))
                }
            );

            gpui::register_action!($name);
        )*
    };
}

/// Implements the Action trait for a struct that implements Clone, Default, PartialEq, and serde_deserialize::Deserialize
/// Allows you to rename the action visually, without changing the struct's name
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
            }
        );

        gpui::register_action!($name);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_action {
    ($namespace:path, $name:ident, $visual_name:tt, $build:item) => {
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

            $build

            fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
                action
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn boxed_clone(&self) ->  std::boxed::Box<dyn gpui::Action> {
                ::std::boxed::Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
        }
    };
}

mod no_action {
    use crate as gpui;

    actions!(zed, [NoAction]);
}
