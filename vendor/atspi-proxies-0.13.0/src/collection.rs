//! # [`CollectionProxy`]
//!
//! A handle to a remote object implementing the `org.a11y.atspi.Collection`
//! interface.
//!
//! `Collection` is the interface which is implemented by objects that contain
//! other objects, such as a window or a table.
//!
//! See the documentation on the individual methods for details:
//!
//! * [`get_matches`](struct.CollectionProxy.html#method.get_matches)
//! * [`get_matches_from`](struct.CollectionProxy.html#method.get_matches_from)
//! * [`get_matches_to`](struct.CollectionProxy.html#method.get_matches_to)
//!
//! [`CollectionProxy`]: crate::collection::CollectionProxy

use atspi_common::object_ref::ObjectRefOwned;

use crate::common::{ObjectMatchRule, SortOrder, TreeTraversalType};

#[zbus::proxy(interface = "org.a11y.atspi.Collection", assume_defaults = true)]
pub trait Collection {
	/// The active descendant of the given object.
	///
	/// May not be implemented by any known toolkit or private implementation.
	///
	/// See [atspi/collection.c](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/main/atspi/atspi-collection.c?ref_type=heads#L272)
	///
	fn get_active_descendant(&self) -> zbus::Result<ObjectRefOwned>;

	/// Retrieves a list of objects that match the specified `ObjectMatchRule`, ordered according to `SortOrder` and limited by the count parameter.
	///
	/// # Arguments
	///
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported.
	///
	/// [`ObjectMatchRule`]: [`atspi_common::object_match::ObjectMatchRule`]
	/// [`SortOrder`]: [`atspi_common::SortOrder`]
	fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// Retrieves objects from the collection, after `current_object`, matching a given `rule`.
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] specifying restrictions on the objects to be traversed.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported by the known implementation (atk-collection).
	///
	/// [`ObjectMatchRule`]: atspi_common::object_match::ObjectMatchRule
	/// [`SortOrder`]: atspi_common::SortOrder
	/// [`TreeTraversalType`]: atspi_common::TreeTraversalType
	fn get_matches_from(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// Retrieves objects from the collection, before `current_object`, matching a given `rule`.
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] specifying restrictions on the objects to be traversed.
	/// * `limit_scope` - If `true`, only descendants of `current_object`'s parent will be returned.
	///    Otherwise (if `false`), any accessible may be returned if it would preceed `current_object` in a flattened hierarchy.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported by the known implementation (atk-collection).
	///
	/// [`ObjectMatchRule`]: atspi_common::object_match::ObjectMatchRule
	/// [`SortOrder`]: atspi_common::SortOrder
	/// [`TreeTraversalType`]: atspi_common::TreeTraversalType
	#[allow(clippy::too_many_arguments)]
	fn get_matches_to(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;
}
