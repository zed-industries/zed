use collections::HashMap;
use gpui::{AnyElement, WindowContext};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// A function that returns a preview of a component.
pub type ComponentPreviewFn = fn(&WindowContext) -> AnyElement;

static COMPONENTS: Lazy<Mutex<HashMap<&'static str, Vec<(&'static str, ComponentPreviewFn)>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

/// Registers a component in the component registry.
pub fn register_component(scope: &'static str, name: &'static str, preview: ComponentPreviewFn) {
    let mut components = COMPONENTS.lock().unwrap();
    components
        .entry(scope)
        .or_insert_with(Vec::new)
        .push((name, preview));
}

/// Initializes all components that have been registered
/// in the UI component registry.
pub fn init_component_registry() {
    for register in __COMPONENT_REGISTRATIONS {
        register();
    }
}

/// Returns a map of all registered components and their previews.
pub fn get_all_component_previews() -> HashMap<&'static str, Vec<(&'static str, ComponentPreviewFn)>>
{
    COMPONENTS.lock().unwrap().clone()
}

#[doc(hidden)]
#[linkme::distributed_slice]
pub static __COMPONENT_REGISTRATIONS: [fn()];

/// Defines components that should be registered in the component registry.
///
/// This allows components to be previewed, and eventually tracked for documentation
/// purposes and to help the systems team to understand component usage across the codebase.
#[macro_export]
macro_rules! register_components {
    ($scope:ident, [ $($component:ty),+ $(,)? ]) => {
        const _: () = {
            #[linkme::distributed_slice($crate::component_registry::__COMPONENT_REGISTRATIONS)]
            fn register() {
                $(
                    $crate::component_registry::register_component(
                        stringify!($scope),
                        stringify!($component),
                        |cx: &$crate::WindowContext| <$component>::render_preview(cx),
                    );
                )+
            }
        };
    };
}
