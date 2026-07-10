use std::rc::Rc;

use crate::{
    Action, AsKeystroke, DummyKeyboardMapper, InvalidKeystrokeError, KeyBindingContextPredicate,
    KeybindingKeystroke, Keystroke, PlatformKeyboardMapper, SharedString,
};
use smallvec::SmallVec;

/// A keybinding and its associated metadata, from the keymap.
pub struct KeyBinding {
    pub(crate) action: Box<dyn Action>,
    pub(crate) keystrokes: SmallVec<[KeybindingKeystroke; 2]>,
    pub(crate) context_predicate: Option<Rc<KeyBindingContextPredicate>>,
    pub(crate) meta: Option<KeyBindingMetaIndex>,
    /// The json input string used when building the keybinding, if any
    pub(crate) action_input: Option<SharedString>,
    /// Whether this is a library default registered via [`crate::keybinding!`].
    pub(crate) default: bool,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
            meta: self.meta,
            action_input: self.action_input.clone(),
            default: self.default,
        }
    }
}

impl KeyBinding {
    /// Construct a new keybinding from the given data. Panics on parse error.
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate =
            context.map(|context| KeyBindingContextPredicate::parse(context).unwrap().into());
        Self::load(
            keystrokes,
            Box::new(action),
            context_predicate,
            false,
            None,
            &DummyKeyboardMapper,
        )
        .unwrap()
    }

    /// Load a keybinding from the given raw data.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context_predicate: Option<Rc<KeyBindingContextPredicate>>,
        use_key_equivalents: bool,
        action_input: Option<SharedString>,
        keyboard_mapper: &dyn PlatformKeyboardMapper,
    ) -> std::result::Result<Self, InvalidKeystrokeError> {
        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| {
                let keystroke = Keystroke::parse(source)?;
                Ok(KeybindingKeystroke::new_with_mapper(
                    keystroke,
                    use_key_equivalents,
                    keyboard_mapper,
                ))
            })
            .collect::<std::result::Result<_, _>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate,
            meta: None,
            action_input,
            default: false,
        })
    }

    /// Whether this is a library default registered via [`crate::keybinding!`].
    pub fn is_default(&self) -> bool {
        self.default
    }

    /// Set the metadata for this binding.
    pub fn with_meta(mut self, meta: KeyBindingMetaIndex) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Set the metadata for this binding.
    pub fn set_meta(&mut self, meta: KeyBindingMetaIndex) {
        self.meta = Some(meta);
    }

    /// Check if the given keystrokes match this binding.
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        if self.keystrokes.len() < typed.len() {
            return None;
        }

        for (target, typed) in self.keystrokes.iter().zip(typed.iter()) {
            if !typed.as_keystroke().should_match(target) {
                return None;
            }
        }

        Some(self.keystrokes.len() > typed.len())
    }

    /// Get the keystrokes associated with this binding
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
        self.keystrokes.as_slice()
    }

    /// Get the action associated with this binding
    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }

    /// Get the predicate used to match this binding
    pub fn predicate(&self) -> Option<Rc<KeyBindingContextPredicate>> {
        self.context_predicate.as_ref().map(|rc| rc.clone())
    }

    /// Get the metadata for this binding
    pub fn meta(&self) -> Option<KeyBindingMetaIndex> {
        self.meta
    }

    /// Get the action input associated with the action for this binding
    pub fn action_input(&self) -> Option<SharedString> {
        self.action_input.clone()
    }
}

impl std::fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyBinding")
            .field("keystrokes", &self.keystrokes)
            .field("context_predicate", &self.context_predicate)
            .field("action", &self.action.name())
            .finish()
    }
}

/// A unique identifier for retrieval of metadata associated with a key binding.
/// Intended to be used as an index or key into a user-defined store of metadata
/// associated with the binding, such as the source of the binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBindingMetaIndex(pub u32);

/// A default key binding registered at link time via [`crate::keybinding!`].
/// Collected into the keymap when an [`crate::App`] is created, before any
/// user bindings, so user bindings take precedence via declaration order.
///
/// This type is public so the macro can construct it in other crates; it is
/// not intended to be used directly.
#[doc(hidden)]
pub struct DefaultKeyBinding {
    /// The keystrokes to bind, in the same format as [`KeyBinding::new`].
    pub keystrokes: &'static str,
    /// An optional key context predicate to scope the binding.
    pub context: Option<&'static str>,
    /// Builds the action this binding dispatches.
    pub build_action: fn() -> Box<dyn Action>,
}

inventory::collect!(DefaultKeyBinding);

impl DefaultKeyBinding {
    /// Construct a registration record. Used by [`crate::keybinding!`].
    #[doc(hidden)]
    pub const fn new(
        keystrokes: &'static str,
        context: Option<&'static str>,
        build_action: fn() -> Box<dyn Action>,
    ) -> Self {
        DefaultKeyBinding {
            keystrokes,
            context,
            build_action,
        }
    }

    /// Load all registered default key bindings, marking them as defaults.
    /// Invalid registrations are programmer errors in the registering crate:
    /// they panic in debug builds and are logged and skipped in release.
    pub(crate) fn load_all() -> Vec<KeyBinding> {
        let mut bindings = Vec::new();
        for registration in inventory::iter::<DefaultKeyBinding> {
            let action = (registration.build_action)();
            let context_predicate = match registration.context {
                Some(context) => match KeyBindingContextPredicate::parse(context) {
                    Ok(predicate) => Some(Rc::new(predicate)),
                    Err(error) => {
                        gpui_util::debug_panic!(
                            "invalid context {:?} in default key binding for {}: {}",
                            context,
                            action.name(),
                            error
                        );
                        continue;
                    }
                },
                None => None,
            };
            match KeyBinding::load(
                registration.keystrokes,
                action,
                context_predicate,
                false,
                None,
                &DummyKeyboardMapper,
            ) {
                Ok(mut binding) => {
                    binding.default = true;
                    bindings.push(binding);
                }
                Err(error) => {
                    gpui_util::debug_panic!(
                        "invalid keystrokes {:?} in default key binding: {}",
                        registration.keystrokes,
                        error
                    );
                }
            }
        }
        bindings
    }
}

/// Declares an action and registers a default key binding for it, in one step.
///
/// The action is registered under the crate's namespace (like `actions!` with
/// the crate name as the namespace), and the binding is added to every
/// [`crate::App`]'s keymap at creation time — before any user bindings, so
/// user keymaps shadow it via declaration-order precedence. Applications can
/// opt out of all library defaults with
/// [`crate::Application::without_default_key_bindings`], or mask individual
/// ones by binding [`crate::NoAction`] over them.
///
/// ```ignore
/// keybinding!("enter", Confirm);
/// keybinding!("escape", Cancel, "TextInput");  // scoped to a key context
/// ```
#[macro_export]
macro_rules! keybinding {
    ($keystrokes:literal, $name:ident) => {
        gpui::keybinding!(@impl $keystrokes, $name, ::std::option::Option::None);
    };
    ($keystrokes:literal, $name:ident, $context:literal) => {
        gpui::keybinding!(@impl $keystrokes, $name, ::std::option::Option::Some($context));
    };
    (@impl $keystrokes:literal, $name:ident, $context:expr) => {
        #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug, gpui::Action)]
        #[action(namespace = crate)]
        pub struct $name;

        const _: () = {
            fn build_action() -> ::std::boxed::Box<dyn gpui::Action> {
                ::std::boxed::Box::new($name)
            }
            gpui::private::inventory::submit! {
                gpui::DefaultKeyBinding::new($keystrokes, $context, build_action)
            }
        };
    };
}
