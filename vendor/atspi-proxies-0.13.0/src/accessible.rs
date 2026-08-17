//! # `AccessibleProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Accessible`
//! interface.
//!
//! Accessible is the interface which is implemented by all accessible objects.
//!

use crate::common::{InterfaceSet, ObjectRef, RelationType, Role, StateSet};
use crate::AtspiError;
use atspi_common::object_ref::ObjectRefOwned;
use zbus::names::BusName;

/// # `AccessibleProxy`
///
/// A handle for a remote object implementing the `org.a11y.atspi.Accessible`
/// interface.
///
/// Accessible is the interface which is implemented by all accessible objects.
///
#[zbus::proxy(
	interface = "org.a11y.atspi.Accessible",
	default_path = "/org/a11y/atspi/accessible/root",
	assume_defaults = true
)]
pub trait Accessible {
	/// Returns an [`ObjectRef`] which refers to the `Application` object of the application.
	/// This object will have [`Application`] interface implemented.
	///
	/// The application object is the root of the accessibility hierarchy for the application.
	/// It is the only object in the hierarchy that does not have a parent.
	///
	/// ## Notes
	/// The application object is the only object in the accessibility hierarchy that is
	/// guaranteed to be persistent for the lifetime of the application.
	/// All other objects in the accessibility hierarchy may be created and destroyed dynamically.
	///
	/// [`ObjectRef`]: [`crate::common::events::ObjectRef`]
	/// [`Application`]: [`crate::application::ApplicationProxy`]
	fn get_application(&self) -> zbus::Result<ObjectRefOwned>;

	/// Gets a list of name/value pairs of attributes or annotations for this object.
	///
	/// ## Disambiguation
	/// For	typographic, textual, or textually-semantic attributes,
	/// see [`TextProxy`]'s [`get_attributes`] method instead.
	///
	/// [`TextProxy`]: [`crate::text::TextProxy`]
	/// [`get_attributes`]: [`crate::text::TextProxy#method.get_attributes`]
	fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

	/// Retrieve child by index (starting from 0),
	///
	/// Queries the N-th accessible child of `self`. It is expected that this
	/// will correspond to the order that the [`get_children`] method would return.
	///
	/// ## Notes
	/// Implementations vary in their behavior when the index is out of range.
	/// GTK4 returns an error, while atk-adaptor (e.g. Gtk3) returns the
	/// null object path "/org/a11y/atspi/null".
	///
	/// Documentation advises implementors to return a `DBus` Error when the index is
	/// out of range, to "keep the type system gods happy".
	///
	/// [`get_children`]: #method.get_children
	fn get_child_at_index(&self, index: i32) -> zbus::Result<ObjectRefOwned>;

	/// Retrieves a list of the object's accessible children.
	///
	/// Each array element is an [`Accessible`] representing the accessible child object.
	///
	/// ## Registry
	///
	/// On the [`Accessible`] interface of `org.a11y.atspi.Registry`, the registry daemon, this method retrieves a list
	/// of all accessible applications' root objects on the bus.
	///
	/// [`Accessible`]: [`crate::accessible::AccessibleProxy`]
	fn get_children(&self) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// This object resides in its parent's list of children.
	/// This returns its position in this list of children, starting from 0.
	///
	/// The function returns -1 if the object does not have a parent or
	/// if an exception occurs.
	fn get_index_in_parent(&self) -> zbus::Result<i32>;

	/// Returns an [`InterfaceSet`] accessible interface names supported by the `self` object.
	/// [`InterfaceSet`]: [`crate::common::InterfaceSet`]
	fn get_interfaces(&self) -> zbus::Result<InterfaceSet>;

	/// Gets a `String` corresponding to the name of the role played by an object,
	/// translated to the current locale.
	///
	/// ## Notes
	///
	/// This method will return useful values for roles that fall outside the
	/// enumeration used in the [`get_role`] method.
	///
	/// For applications, implementing this method is optional, and it may be removed
	/// in a future version of the API.
	///
	/// For example, [`libatspi`] will only call it in the event of an unknown role.
	///
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/main/atspi>
	/// [`get_role`]: #method.get_role
	fn get_localized_role_name(&self) -> zbus::Result<String>;

	/// Returns a set of relationships between the this `self` object and others.
	///
	/// This vector of tuples contains a [`RelationType`] and a vector of [`Accessible`]'s to which that
	/// relationship applies.
	/// These relationships allow for better identification of how objects are associated with one another.
	///
	/// For example, the relationship [`RelationType::LabelledBy`] can be used to identify labeling information
	/// that should accompany the accessible [`name`] property when presenting an object's content or identity
	/// to the end user.
	///
	/// Similarly, [`RelationType::ControllerFor`] can be used to specify the context in which a valuator is useful
	/// and/or the other UI components that are directly affected by user interactions with the valuator.
	/// Common examples include the association of scrollbars with the viewport or panel that they control.
	///
	/// [`RelationType`]: [`crate::common::RelationType`]
	/// [`RelationType::LabelledBy`]: [`crate::common::RelationType::LabelledBy`]
	/// [`RelationType::ControllerFor`]: [`crate::common::RelationType::ControllerFor`]
	/// [`name`]: #method.name
	/// [`Accessible`]: [`crate::common::events::Accessible`]
	fn get_relation_set(&self) -> zbus::Result<Vec<(RelationType, Vec<ObjectRefOwned>)>>;

	/// Gets the [`Role`] that the current accessible object represents.
	///
	/// Roles make it possible for various UI toolkits to expose their controls to
	/// assistive technologies (ATs) with a standard interface, regardless of toolkit.
	///
	/// For example, a widget that acts like a conventional push button
	/// (appears unpressed; presses	when acted upon; invokes a certain action
	/// when pressed) can expose an	[`Role::Button`] role.
	///
	/// [`Role::Button`]: [`crate::common::Role::Button`]
	/// [`Role`]: [`crate::common::Role`]
	fn get_role(&self) -> zbus::Result<Role>;

	/// Gets a `String` corresponding to the name of the role played by an object,
	/// translated to the current locale.
	///
	/// ## Notes
	///
	/// This method will return useful values for roles that fall outside the
	/// enumeration used in the `get_role` method.
	///
	/// For applications, implementing this method is optional, and it may be removed
	/// in a future version of the API.
	///
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/main/atspi>
	/// [`libatspi`]: <https://gitlab.gnome.org/GNOME/at-spi2-core/>
	fn get_role_name(&self) -> zbus::Result<String>;

	/// Method to retrieve the [`StateSet`] of states currently held by `self`.
	/// [`StateSet`]: [`crate::common::StateSet`]
	fn get_state(&self) -> zbus::Result<StateSet>;

	/// Application-specific identifier for the current object.
	///
	/// A special id given to an object.
	/// Accessible application developers can use this to give a special id to an object
	/// to use in tests, for example, "`my_widget`".
	///
	/// Note that there is no way to directly find an object by its id;
	/// a test program may have to recursively get the children to find a specific id.
	/// This is because accessible objects can be created dynamically, and they do not always
	/// correspond to a static view of an application's data.
	#[zbus(property)]
	fn accessible_id(&self) -> zbus::Result<String>;

	/// Number of accessible children for the current object.
	#[zbus(property)]
	fn child_count(&self) -> zbus::Result<i32>;

	/// Human-readable, localized description of `self` in more detail.
	///
	/// This is a longer description than the [`Name`][name] property.
	///
	/// For example, a button might have a name of "OK", but a description of "OK button".
	///
	/// While the Name property is meant to be a short string that screen readers say
	/// during normal navigation, the Description property is for when the user asks for
	/// more detail.
	///
	/// [name]: #method.name
	#[zbus(property)]
	fn description(&self) -> zbus::Result<String>;

	/// Unix locale for the current object.
	///
	/// This is a string in the form of "`language_territory.codeset`".
	/// For example, "en_US.UTF-8" or "de_DE.UTF-8".
	///
	/// For an application, this may be the locale for the language that the application
	/// shows in its user interface.
	///
	/// For a document being shown in an application, or a paragraph within a document,
	/// the locale may refer to that object exclusively. For example:
	/// an application may be showing itself in English ("en"), but it may be used to
	/// display a document in Spanish ("es").
	/// In the latter case, a screen reader will want to know that it should switch to
	/// Spanish while reading the document.
	#[zbus(property)]
	fn locale(&self) -> zbus::Result<String>;

	/// Human-readable, localized, short name for the object.
	///
	/// Applications should have this set for objects which do not
	/// have a [`RelationType::LabelledBy`] relation.
	///
	/// Consider a widget to select RGB colors by setting three sliders.
	/// The	names for the sliders would be "Red", "Green", "Blue", respectively, or
	/// their translations to application's locale.  The names would be unnecessary if each
	/// slider had a `LabeledBy` relation to corresponding labels visible in the user
	/// interface.
	///
	/// [`RelationType::LabelledBy`]: [`crate::common::RelationType::LabelledBy`]
	#[zbus(property)]
	fn name(&self) -> zbus::Result<String>;

	/// `ObjectRef` parent object of the current object.
	///
	/// Null parent:
	/// If the object has no parent (e.g. the application's root object is being queried),
	/// The application should return "" for the application name name and "/org/a11y/atspi/null"
	/// for the object path.
	///
	/// Root object:
	/// An application must have a single root object, called "/org/a11y/atspi/accessible/root".
	/// All other objects should have that one as their highest-level ancestor.
	#[zbus(property)]
	fn parent(&self) -> zbus::Result<ObjectRefOwned>;

	/// Help text for the current object.
	#[zbus(property)]
	fn help_text(&self) -> zbus::Result<String>;
}

impl TryFrom<AccessibleProxy<'_>> for ObjectRefOwned {
	type Error = AtspiError;
	fn try_from(proxy: AccessibleProxy<'_>) -> Result<ObjectRefOwned, Self::Error> {
		let sender = proxy.inner().destination();
		let path = proxy.inner().path();
		let object_ref = ObjectRef::try_from_bus_name_and_path(sender.into(), path.into())?;
		Ok(ObjectRefOwned::from(object_ref))
	}
}

impl TryFrom<&AccessibleProxy<'_>> for ObjectRefOwned {
	type Error = AtspiError;
	fn try_from(proxy: &AccessibleProxy<'_>) -> Result<ObjectRefOwned, Self::Error> {
		let sender = proxy.inner().destination().clone();
		let path = proxy.inner().path().clone();
		let object_ref = ObjectRef::try_from_bus_name_and_path(sender, path)?;
		Ok(ObjectRefOwned::from(object_ref))
	}
}

pub trait ObjectRefExt {
	/// Returns an [`AccessibleProxy`], the handle to the object's  `Accessible` interface.
	///
	/// # Errors  
	/// If the `ObjectRef` is null, this method returns an error.
	/// Users are advised to check if the `ObjectRef` is null before calling this method.
	fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;

	/// Returns an [`AccessibleProxy`], the handle to the object's  `Accessible` interface.
	///
	/// # Errors  
	/// If the `ObjectRef` is null, this method returns an error.
	/// Users are advised to check if the `ObjectRef` is null before calling this method.
	fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;
}

impl ObjectRefExt for ObjectRefOwned {
	async fn as_accessible_proxy(
		&self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		if self.is_null() {
			return Err(AtspiError::NullRef(
				"`as_accessible_proxy` called on null-reference ObjectRef",
			));
		}

		let name: BusName = self.name().ok_or(AtspiError::MissingName)?.clone().into();
		let path = self.path();

		AccessibleProxy::builder(conn)
			.destination(name)?
			.path(path)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}

	async fn into_accessible_proxy(
		self,
		conn: &zbus::Connection,
	) -> Result<AccessibleProxy<'_>, AtspiError> {
		if self.is_null() {
			return Err(AtspiError::NullRef(
				"`into_accessible_proxy` called on null-reference ObjectRef",
			));
		}

		let name: BusName = self.name().ok_or(AtspiError::MissingName)?.clone().into();
		let path = self.path();

		AccessibleProxy::builder(conn)
			.destination(name)?
			.path(path)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}
}

#[cfg(test)]
mod tests {
	use crate::accessible::Role;

	#[test]
	fn test_output_of_role_name() {
		assert_eq!(Role::Invalid.name(), "invalid");
		assert_eq!(Role::PushButtonMenu.name(), "push button menu");
	}
}
