use serde::{Deserialize, Serialize};
use zvariant::Type;

/// Describes a relationship between one object and another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, Hash)]
pub enum RelationType {
	/// Not a meaningful relationship; clients should not normally encounter this value.
	Null = 0,

	/// Object is a label for one or more other objects.
	LabelFor,

	/// Object is labelled by one or more other objects.
	LabelledBy,

	/// Object is an interactive object which modifies the state,
	/// onscreen location, or other attributes of one or more target objects.
	ControllerFor,

	/// Object state, position, etc. is modified/controlled by user interaction
	/// with one or more other objects.
	/// For instance a viewport or scroll pane may be `ControlledBy` scrollbars.
	ControlledBy,

	/// Object has a grouping relationship (e.g. 'same group as') to one or more other objects.
	MemberOf,

	/// Object is a tooltip associated with another object.
	TooltipFor,

	/// Object is a child of the target.
	NodeChildOf,

	/// Object is a parent of the target.
	NodeParentOf,

	/// Used to indicate that a relationship exists, but its type is not
	/// specified in the enumeration.
	Extended,

	/// Object renders content which flows logically to another object.
	/// For instance, text in a paragraph may flow to another object
	/// which is not the 'next sibling' in the accessibility hierarchy.
	FlowsTo,

	/// Reciprocal of `FlowsTo`.
	FlowsFrom,

	/// Object is visually and semantically considered a subwindow of another object,
	/// even though it is not the object's child.
	/// Useful when dealing with embedded applications and other cases where the
	/// widget hierarchy does not map cleanly to the onscreen presentation.
	SubwindowOf,

	/// Similar to `SubwindowOf`, but specifically used for cross-process embedding.
	Embeds,

	/// Reciprocal of `Embeds`. Used to denote content rendered by embedded renderers
	/// that live in a separate process space from the embedding context.
	EmbeddedBy,

	///Denotes that the object is a transient window or frame associated with another
	/// onscreen object. Similar to `TooltipFor`, but more general.
	/// Useful for windows which are technically toplevels but which, for one or more reasons,
	/// do not explicitly cause their associated window to lose 'window focus'.
	/// Creation of an [`crate::Role::Window`] object with the `PopupFor` relation usually requires
	/// some presentation action on the part of assistive technology clients,
	/// even though the previous toplevel [`crate::Role::Frame`] object may still be the active window.
	PopupFor,

	/// This is the reciprocal relation to `PopupFor`.
	ParentWindowOf,

	/// Reciprocal of `DescribedBy`. Indicates that this object provides descriptive information
	/// about the target object(s). See also `DetailsFor` and `ErrorFor`.
	DescriptionFor,

	/// Reciprocal of `DescriptionFor`.
	/// Indicates that one or more target objects provide descriptive information
	/// about this object. This relation type is most appropriate for information
	/// that is not essential as its presentation may be user-configurable and/or
	/// limited to an on-demand mechanism such as an assistive technology command.
	/// For brief, essential information such as can be found in a widget's on-screen
	/// label, use `LabelledBy`. For an on-screen error message, use `ErrorMessage`.
	/// For lengthy extended descriptive information contained in an on-screen object,
	/// consider using `Details` as assistive technologies may provide a means
	/// for the user to navigate to objects containing detailed descriptions so that
	/// their content can be more closely reviewed.
	DescribedBy,

	/// Reciprocal of `DetailsFor`.
	/// Indicates that this object has a detailed or extended description,
	/// the contents of which can be found in the target object(s).
	/// This relation type is most appropriate for information that is sufficiently lengthy
	/// as to make navigation to the container of that information desirable.
	/// For less verbose information suitable for announcement only, see `DescribedBy`.
	/// If the detailed information describes an error condition, `ErrorFor` should be used instead.
	/// Included in upstream [AT-SPI2-CORE](https://gitlab.gnome.org/GNOME/at-spi2-core) since 2.26.
	Details,

	/// Reciprocal of `Details`.
	/// Indicates that this object provides a detailed or extended description about the target
	/// object(s). See also `DescriptionFor` and `ErrorFor`.
	/// Included in upstream [AT-SPI2-CORE](https://gitlab.gnome.org/GNOME/at-spi2-core) since 2.26.
	DetailsFor,

	/// Reciprocal of `ErrorFor`.
	/// Indicates that this object has one or more errors, the nature of which is
	/// described in the contents of the target object(s). Objects that have this
	/// relation type should also contain [`crate::state::State::InvalidEntry`] when their
	/// `GetState` method is called.
	///
	/// Included in upstream [AT-SPI2-CORE](https://gitlab.gnome.org/GNOME/at-spi2-core) since 2.26.
	ErrorMessage,

	///  Reciprocal of `ErrorMessage`.
	/// Indicates that this object contains an error message describing an invalid condition
	/// in the target object(s).
	/// Included in upstream [AT-SPI2-CORE](https://gitlab.gnome.org/GNOME/at-spi2-core) since 2.26.
	ErrorFor,
}
