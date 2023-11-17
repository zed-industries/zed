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
/// action for each listed action name.
/// ```rust
/// actions!(MoveUp, MoveDown, MoveLeft, MoveRight, Newline);
/// ```
/// More complex data types can also be actions. If you annotate your type with the action derive macro
/// it will be implemented and registered automatically.
/// ```
/// #[derive(Clone, PartialEq, serde_derive::Deserialize, Action)]
/// pub struct SelectNext {
///     pub replace_newest: bool,
/// }
///
/// If you want to control the behavior of the action trait manually, you can use the lower-level `#[register_action]`
/// macro, which only generates the code needed to register your action before `main`.
///
/// ```
/// #[gpui::register_action]
/// #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::fmt::Debug)]
/// pub struct Paste {
///     pub content: SharedString,
/// }
///
/// impl gpui::Action for Paste {
///      ///...
/// }
/// ```
pub trait Action: 'static {
    fn boxed_clone(&self) -> Box<dyn Action>;
    fn as_any(&self) -> &dyn Any;
    fn partial_eq(&self, action: &dyn Action) -> bool;
    fn name(&self) -> &str;

    fn debug_name() -> &'static str
    where
        Self: Sized;
    fn build(value: serde_json::Value) -> Result<Box<dyn Action>>
    where
        Self: Sized;
}

impl std::fmt::Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn Action")
            .field("type_name", &self.name())
            .finish()
    }
}

impl dyn Action {
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
/// But it's existence is an implementation detail and should not be used directly.
#[doc(hidden)]
#[linkme::distributed_slice]
pub static __GPUI_ACTIONS: [MacroActionBuilder];

impl ActionRegistry {
    /// Load all registered actions into the registry.
    pub(crate) fn load_actions(&mut self) {
        for builder in __GPUI_ACTIONS {
            let action = builder();
            let name: SharedString = qualify_action(action.name).into();
            self.builders_by_name.insert(name.clone(), action.build);
            self.names_by_type_id.insert(action.type_id, name.clone());
            self.all_names.push(name);
        }
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
/// To use more complex data types as actions, annotate your type with the #[action] macro.
#[macro_export]
macro_rules! actions {
    () => {};

    ( $name:ident ) => {
        #[derive(::std::cmp::PartialEq, ::std::clone::Clone, ::std::default::Default, gpui::serde_derive::Deserialize, gpui::Action)]
        pub struct $name;
    };

    ( $name:ident, $($rest:tt)* ) => {
        actions!($name);
        actions!($($rest)*);
    };
}

/// This used by our macros to pre-process the action name deterministically
#[doc(hidden)]
pub fn qualify_action(action_name: &'static str) -> String {
    let mut separator_matches = action_name.rmatch_indices("::");
    separator_matches.next().unwrap();
    let name_start_ix = separator_matches.next().map_or(0, |(ix, _)| ix + 2);
    // todo!() remove the 2 replacement when migration is done
    action_name[name_start_ix..]
        .replace("2::", "::")
        .to_string()
}

mod no_action {
    use crate as gpui;

    actions!(NoAction);
}
