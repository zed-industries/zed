use std::any::{Any, TypeId};

pub trait Action: 'static {
    fn id(&self) -> TypeId;
    fn namespace(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn boxed_clone(&self) -> Box<dyn Action>;
    fn boxed_clone_as_any(&self) -> Box<dyn Any>;
}

#[macro_export]
macro_rules! impl_actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            impl $crate::action::Action for $name {
                fn id(&self) -> std::any::TypeId {
                    std::any::TypeId::of::<$name>()
                }

                fn namespace(&self) -> &'static str {
                    stringify!($namespace)
                }

                fn name(&self) -> &'static str {
                    stringify!($name)
                }

                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }

                fn boxed_clone(&self) -> Box<dyn $crate::action::Action> {
                    Box::new(self.clone())
                }

                fn boxed_clone_as_any(&self) -> Box<dyn std::any::Any> {
                    Box::new(self.clone())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {

        $(
            #[derive(Clone, Debug, Default, PartialEq, Eq)]
            pub struct $name;
        )*

        $crate::impl_actions!($namespace, [ $($name),* ]);
    };
}
