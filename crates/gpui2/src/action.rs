use crate::SharedString;
use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use lazy_static::lazy_static;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use serde::Deserialize;
use std::any::{type_name, Any, TypeId};

/// Actions are used to implement keyboard-driven UI.
/// When you declare an action, you can bind keys to the action in the keymap and
/// listeners for that action in the element tree.
///
/// To declare a list of simple actions, you can use the actions! macro, which defines a simple unit struct
/// action for each listed action name.
/// ```rust
/// actions!(MoveUp, MoveDown, MoveLeft, MoveRight, Newline);
/// ```
/// More complex data types can also be actions. If you annotate your type with the `#[action]` proc macro,
/// it will automatically
/// ```
/// #[action]
/// pub struct SelectNext {
///     pub replace_newest: bool,
/// }
///
/// Any type A that satisfies the following bounds is automatically an action:
///
/// ```
/// A: for<'a> Deserialize<'a> + PartialEq + Clone + Default + std::fmt::Debug + 'static,
/// ```
///
/// The `#[action]` annotation will derive these implementations for your struct automatically. If you
/// want to control them manually, you can use the lower-level `#[register_action]` macro, which only
/// generates the code needed to register your action before `main`. Then you'll need to implement all
/// the traits manually.
///
/// ```
/// #[gpui::register_action]
/// #[derive(gpui::serde::Deserialize, std::cmp::PartialEq, std::clone::Clone, std::fmt::Debug)]
/// pub struct Paste {
///     pub content: SharedString,
/// }
///
/// impl std::default::Default for Paste {
///     fn default() -> Self {
///         Self {
///             content: SharedString::from("🍝"),
///         }
///     }
/// }
/// ```
pub trait Action: std::fmt::Debug + 'static {
    fn qualified_name() -> SharedString
    where
        Self: Sized;
    fn build(value: Option<serde_json::Value>) -> Result<Box<dyn Action>>
    where
        Self: Sized;

    fn partial_eq(&self, action: &dyn Action) -> bool;
    fn boxed_clone(&self) -> Box<dyn Action>;
    fn as_any(&self) -> &dyn Any;
}

// Types become actions by satisfying a list of trait bounds.
impl<A> Action for A
where
    A: for<'a> Deserialize<'a> + PartialEq + Clone + Default + std::fmt::Debug + 'static,
{
    fn qualified_name() -> SharedString {
        // todo!() remove the 2 replacement when migration is done
        type_name::<A>().replace("2::", "::").into()
    }

    fn build(params: Option<serde_json::Value>) -> Result<Box<dyn Action>>
    where
        Self: Sized,
    {
        let action = if let Some(params) = params {
            serde_json::from_value(params).context("failed to deserialize action")?
        } else {
            Self::default()
        };
        Ok(Box::new(action))
    }

    fn partial_eq(&self, action: &dyn Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn boxed_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn Action {
    pub fn type_id(&self) -> TypeId {
        self.as_any().type_id()
    }

    pub fn name(&self) -> SharedString {
        ACTION_REGISTRY
            .read()
            .names_by_type_id
            .get(&self.type_id())
            .expect("type is not a registered action")
            .clone()
    }
}

type ActionBuilder = fn(json: Option<serde_json::Value>) -> anyhow::Result<Box<dyn Action>>;

lazy_static! {
    static ref ACTION_REGISTRY: RwLock<ActionRegistry> = RwLock::default();
}

#[derive(Default)]
struct ActionRegistry {
    builders_by_name: HashMap<SharedString, ActionBuilder>,
    names_by_type_id: HashMap<TypeId, SharedString>,
    all_names: Vec<SharedString>, // So we can return a static slice.
}

/// Register an action type to allow it to be referenced in keymaps.
pub fn register_action<A: Action>() {
    let name = A::qualified_name();
    let mut lock = ACTION_REGISTRY.write();
    lock.builders_by_name.insert(name.clone(), A::build);
    lock.names_by_type_id
        .insert(TypeId::of::<A>(), name.clone());
    lock.all_names.push(name);
}

/// Construct an action based on its name and optional JSON parameters sourced from the keymap.
pub fn build_action_from_type(type_id: &TypeId) -> Result<Box<dyn Action>> {
    let lock = ACTION_REGISTRY.read();
    let name = lock
        .names_by_type_id
        .get(type_id)
        .ok_or_else(|| anyhow!("no action type registered for {:?}", type_id))?
        .clone();
    drop(lock);

    build_action(&name, None)
}

/// Construct an action based on its name and optional JSON parameters sourced from the keymap.
pub fn build_action(name: &str, params: Option<serde_json::Value>) -> Result<Box<dyn Action>> {
    let lock = ACTION_REGISTRY.read();

    let build_action = lock
        .builders_by_name
        .get(name)
        .ok_or_else(|| anyhow!("no action type registered for {}", name))?;
    (build_action)(params)
}

pub fn all_action_names() -> MappedRwLockReadGuard<'static, [SharedString]> {
    let lock = ACTION_REGISTRY.read();
    RwLockReadGuard::map(lock, |registry: &ActionRegistry| {
        registry.all_names.as_slice()
    })
}

/// Defines unit structs that can be used as actions.
/// To use more complex data types as actions, annotate your type with the #[action] macro.
#[macro_export]
macro_rules! actions {
    () => {};

    ( $name:ident ) => {
        #[gpui::action]
        pub struct $name;
    };

    ( $name:ident, $($rest:tt)* ) => {
        actions!($name);
        actions!($($rest)*);
    };
}
