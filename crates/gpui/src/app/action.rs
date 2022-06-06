use std::any::{Any, TypeId};

pub trait Action: 'static {
    fn id(&self) -> TypeId;
    fn name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn boxed_clone(&self) -> Box<dyn Action>;
    fn eq(&self, other: &dyn Action) -> bool;

    fn qualified_name() -> &'static str
    where
        Self: Sized;
    fn from_json_str(json: &str) -> anyhow::Result<Box<dyn Action>>
    where
        Self: Sized;
}

/// Define a set of unit struct types that all implement the `Action` trait.
///
/// The first argument is a namespace that will be associated with each of
/// the given action types, to ensure that they have globally unique
/// qualified names for use in keymap files.
#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            #[derive(Clone, Debug, Default, PartialEq, Eq)]
            pub struct $name;
            $crate::__impl_action! {
                $namespace,
                $name,
                fn from_json_str(_: &str) -> $crate::anyhow::Result<Box<dyn $crate::Action>> {
                    Ok(Box::new(Self))
                }
            }
        )*
    };
}

/// Implement the `Action` trait for a set of existing types.
///
/// The first argument is a namespace that will be associated with each of
/// the given action types, to ensure that they have globally unique
/// qualified names for use in keymap files.
#[macro_export]
macro_rules! impl_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            $crate::__impl_action! {
                $namespace,
                $name,
                fn from_json_str(json: &str) -> $crate::anyhow::Result<Box<dyn $crate::Action>> {
                    Ok(Box::new($crate::serde_json::from_str::<Self>(json)?))
                }
            }
        )*
    };
}

/// Implement the `Action` trait for a set of existing types that are
/// not intended to be constructed via a keymap file, but only dispatched
/// internally.
#[macro_export]
macro_rules! impl_internal_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            $crate::__impl_action! {
                $namespace,
                $name,
                fn from_json_str(_: &str) -> $crate::anyhow::Result<Box<dyn $crate::Action>> {
                    Err($crate::anyhow::anyhow!("internal action"))
                }
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_action {
    ($namespace:path, $name:ident, $from_json_fn:item) => {
        impl $crate::action::Action for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn qualified_name() -> &'static str {
                concat!(
                    stringify!($namespace),
                    "::",
                    stringify!($name),
                )
            }

            fn id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$name>()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn boxed_clone(&self) -> Box<dyn $crate::Action> {
                Box::new(self.clone())
            }

            fn eq(&self, other: &dyn $crate::Action) -> bool {
                if let Some(other) = other.as_any().downcast_ref::<Self>() {
                    self == other
                } else {
                    false
                }
            }

            $from_json_fn
        }
    };
}
