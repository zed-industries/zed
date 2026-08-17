//! # [`ActionProxy`]
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Action`
//! interface.
//!
//! The `Action` interface allows exploring and invoking the actions of a
//! user-actionable UI component.
//!
//! For example, a button may expose a "click" action - a popup menu may
//! expose an "open" action.
//!
//! Components which are not "passive" providers of UI information should
//! implement this interface, unless there is a more specialized interface for
//! interaction like [`org.a11y.atspi.Text`][TextProxy] or [`org.a11y.atspi.Value`][ValueProxy].
//!  
//! [ActionProxy]: crate::action::ActionProxy
//! [TextProxy]: crate::text::TextProxy
//! [ValueProxy]: crate::value::ValueProxy

use atspi_common::Action;

/// A handle for a remote object implementing the `org.a11y.atspi.Action`
/// interface.
///
/// The `Action` interface allows exploring and invoking the actions of a
/// user-actionable UI component.
///
/// For example, a button may expose a "click" action - a popup menu may
/// expose an "open" action.
///
/// Components which are not "passive" providers of UI information should
/// implement this interface, unless there is a more specialized interface for
/// interaction like [`org.a11y.atspi.Text`][TextProxy] or [`org.a11y.atspi.Value`][ValueProxy].
///  
/// [TextProxy]: crate::text::TextProxy
/// [ValueProxy]: crate::value::ValueProxy
#[zbus::proxy(interface = "org.a11y.atspi.Action", assume_defaults = true)]
pub trait Action {
	/// Performs the specified action on the object.
	///
	/// Returns: Ok(true) on success, Ok(false) otherwise.
	///
	/// # Arguments
	///
	/// * `index` - The index of the action to perform.
	fn do_action(&self, index: i32) -> zbus::Result<bool>;

	/// Returns an array of localized name, localized
	/// description, keybinding for the actions that an object
	/// supports.
	///
	/// See [`get_key_binding`] method for a description of that
	/// field's syntax.
	///
	/// This is equivalent to using the methods [`get_localized_name`],
	/// [`get_description`] and	[`get_key_binding`] for each action,
	/// but with a single call and thus less `DBus` traffic.
	///
	///	By convention, if there is more than one action available,
	/// the first one is considered the "default" action of the object.
	///
	/// [`get_key_binding`]: ActionProxy#method.get_key_binding
	/// [`get_localized_name`]: ActionProxy#method.get_localized_name
	/// [`get_description`]: ActionProxy#method.get_description
	fn get_actions(&self) -> zbus::Result<Vec<Action>>;

	/// Returns the localized description for the action at the specified
	/// index, starting at zero.
	///   
	/// For	example, a screen reader will read out this description when
	/// the user asks for extra detail on an action.
	/// For example, "Clicks the button" for the "click" action of a button.
	fn get_description(&self, index: i32) -> zbus::Result<String>;

	/// Returns the keybinding for the action, specified by a
	/// zero-based index.
	///
	/// Gets the keybinding which can be used to invoke this action,
	/// if one exists.
	///
	/// The string returned should contain localized, human-readable,
	/// key sequences as they would appear when displayed on screen.
	/// It must be in the format "mnemonic;sequence;shortcut".
	///
	/// - The mnemonic key activates the object if it is presently
	/// enabled on screen.
	/// This typically corresponds to the underlined letter within
	/// the widget. Example: "n" in a traditional "Ṉew..." menu
	/// item or the "a" in "Apply" for a button.
	///
	/// - The sequence is the full list of keys which invoke the action
	/// even if the relevant element is not currently shown on screen.
	/// For instance, for a menu item the sequence is the keybindings
	/// used to open the parent menus before invoking.
	///
	/// The sequence string is colon-delimited. Example: "Alt+F:N" in a
	/// traditional "Ṉew..." menu item.
	///
	/// - The shortcut, if it exists, will invoke the same action without
	/// showing the component or its enclosing menus or dialogs.
	/// Example: "Ctrl+N" in a traditional "Ṉew..." menu item.
	/// The shortcut string is colon-delimited. Example: "Ctrl+N" in a
	/// traditional "Ṉew..." menu item.
	///
	/// Example: For a traditional "Ṉew..." menu item, the expected return
	/// value would be: "N;Alt+F:N;Ctrl+N" for the English locale and
	/// "N;Alt+D:N;Strg+N" for the German locale.
	/// If, hypothetically, this menu item lacked a mnemonic, it would be
	/// represented by ";;Ctrl+N" and ";;Strg+N" respectively.
	///
	/// If there is no key binding for this action, "" is returned.
	fn get_key_binding(&self, index: i32) -> zbus::Result<String>;

	/// Returns a short, localized name for the action at the specified by a
	/// zero-based index.
	///
	/// This is	what screen readers will read out during normal navigation.
	/// For example, "Click" for a button.
	fn get_localized_name(&self, index: i32) -> zbus::Result<String>;

	/// Returns a machine-readable name for the action at the specified,
	/// zero-based index.
	fn get_name(&self, index: i32) -> zbus::Result<String>;

	/// Returns the number of available actions.
	///
	/// By convention, if there is more than one action available,
	/// the first one is considered the "default" action of the object.
	#[zbus(property)]
	fn nactions(&self) -> zbus::Result<i32>;
}
