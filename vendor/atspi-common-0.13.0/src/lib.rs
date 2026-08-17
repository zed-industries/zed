#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code, rustdoc::all)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]

//! # atspi-common
//!
//! Defines all common types, events, and data structures for `atspi-proxies` and `atspi-connection`.
//! Since `atspi-proxies` and `atspi-connection` are downstream crates, the documentation can not link to it directly.
//! Any type ending in `*Proxy` is in `atspi-proxies`.
//!

#[macro_use]
extern crate static_assertions;
#[macro_use]
pub(crate) mod macros;

pub mod action;
#[cfg(feature = "wrappers")]
pub use crate::events::event_wrappers::{
	CacheEvents, DocumentEvents, Event, EventListenerEvents, FocusEvents, KeyboardEvents,
	MouseEvents, ObjectEvents, TerminalEvents, WindowEvents,
};
pub use action::Action;
pub mod object_match;
pub use object_match::{MatchType, ObjectMatchRule, SortOrder, TreeTraversalType};
pub mod object_ref;
pub use object_ref::{ObjectRef, ObjectRefOwned};
pub mod operation;
pub use operation::Operation;
pub mod interface;
pub use interface::{Interface, InterfaceSet};
pub mod state;
pub use state::{State, StateSet};
pub mod cache;
pub use cache::{CacheItem, LegacyCacheItem};
pub mod error;
pub use error::AtspiError;
pub mod events;
pub use events::{EventProperties, EventTypeProperties};
mod role;
pub use role::Role;
mod relation_type;
pub use relation_type::RelationType;

use serde::{Deserialize, Serialize};
use zvariant::Type;

pub type Result<T> = std::result::Result<T, AtspiError>;

/// Describes a selection of text, including selections across object boundaries.
///
/// For example, selecting from the beginning of a paragraph to half way through a link would cause
/// the start and end object references to be different.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct TextSelection {
	/// starting object reference
	start_obj: ObjectRefOwned,
	/// text offset within `start_obj`
	start_idx: i32,
	/// ending object reference
	end_obj: ObjectRefOwned,
	/// text offset within `end_obj`
	end_idx: i32,
	/// is the `start_obj` active;
	///
	/// This is the same as querying for the [`StateSet`], then checking if [`State::Active`] is contained.
	/// See `atspi_proxies::accessible::AccessibleProxy` for more information on checking state.
	start_is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// The coordinate type encodes the frame of reference.
pub enum CoordType {
	/// In relation to the entire screen.
	Screen,
	/// In relation to only the window.
	Window,
	/// In relation to the parent of the element being checked.
	Parent,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// Enumeration used by `TextProxy` to indicate how to treat characters intersecting bounding boxes.
pub enum ClipType {
	/// No characters/glyphs are omitted.
	Neither,
	/// Characters/glyphs clipped by the minimum coordinate are omitted.
	Min,
	/// Characters/glyphs which intersect the maximum coordinate are omitted.
	Max,
	/// Only glyphs falling entirely within the region bounded by min and max are retained.
	Both,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// Level of granularity to get text of, in relation to a cursor position.
pub enum Granularity {
	/// Gives the character at the index of the cursor. With a line-style cursor (which is standard) this will get the character that appears after the cursor.
	Char,
	/// Gives the entire word in front of, or which contains, the cursor. TODO: confirm that it always chooses the word in front of the cursor.
	Word,
	/// Gives entire sentence in front of, or which contains, the cursor. TODO: confirm that it always chooses the sentence after the cursor.
	Sentence,
	/// Gives the line, as seen visually of which the cursor is situated within.
	Line,
	/// Gives the entire block of text, regardless of where the cursor lies within it.
	Paragraph,
}

/// Indicates relative stacking order of a `atspi_proxies::component::ComponentProxy` with respect to the
/// onscreen visual representation of the UI.
///
/// The layer index, in combination with the component's extents,
/// can be used to compute the visibility of all or part of a component.
/// This is important in programmatic determination of region-of-interest for magnification,
/// and in flat screen review models of the screen, as well as for other uses.
/// Objects residing in two of the `Layer` categories support further z-ordering information,
/// with respect to their peers in the same layer:
/// namely, [`Layer::Window`] and [`Layer::Mdi`].
/// Relative stacking order for other objects within the same layer is not available;
/// the recommended heuristic is first child paints first. In other words,
/// assume that the first siblings in the child list are subject to being
/// overpainted by later siblings if their bounds intersect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum Layer {
	/// Indicates an error condition or uninitialized value.
	Invalid,
	/// Reserved for the desktop background; this is the bottom-most layer,
	/// over which everything else is painted.
	Background,
	/// The 'background' layer for most content renderers and
	/// UI `atspi_proxies::component::ComponentProxy` containers.
	Canvas,
	/// The layer in which the majority of ordinary 'foreground' widgets reside.
	Widget,
	/// A special layer between [`Layer::Canvas`] and [`Layer::Widget`], in which the
	/// 'pseudo windows' (e.g. the Multiple-Document Interface frames) reside.
	///
	/// See `atspi_proxies::component::ComponentProxy::get_mdizorder`.
	Mdi,
	/// A layer for popup window content, above [`Layer::Widget`].
	Popup,
	/// The topmost layer.
	Overlay,
	/// The layer in which a toplevel window background usually resides.
	Window,
}

/// Enumeration used by interface the [`crate::interface::Interface::Accessible`] to specify where an object should be placed on the screen when using `scroll_to`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum ScrollType {
	/// Scroll the object to the top left corner of the window.
	TopLeft,
	/// Scroll the object to the bottom right corner of the window.
	BottomRight,
	/// Scroll the object to the top edge of the window.
	TopEdge,
	/// Scroll the object to the bottom edge of the window.
	BottomEdge,
	/// Scroll the object to the left edge of the window.
	LeftEdge,
	/// Scroll the object to the right edge of the window.
	RightEdge,
	/// Scroll the object to application-dependent position on the window.
	Anywhere,
}

/// Enumeration used to indicate a type of live region and how assertive it
/// should be in terms of speaking notifications.
///
/// Currently, this is only used
/// for `Announcement` events, but it may be used for additional purposes
/// in the future.
/// The argument in the `Announcement` event is named `politeness`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum Politeness {
	/// No live region.
	#[default]
	None = 0,
	/// This live region should be considered polite.
	Polite = 1,
	/// This live region should be considered assertive.
	Assertive = 2,
}

impl TryFrom<i32> for Politeness {
	type Error = AtspiError;

	fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
		match value {
			0 => Ok(Politeness::None),
			1 => Ok(Politeness::Polite),
			2 => Ok(Politeness::Assertive),
			_ => Err(AtspiError::Conversion("Unknown Politeness variant")),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;
	use zbus_lockstep::{
		method_args_signature, method_return_signature, signal_body_type_signature,
	};
	use zvariant::Signature;

	#[test]
	fn convert_i32_to_live() {
		assert_eq!(Politeness::None, Politeness::try_from(0).unwrap());
		assert_eq!(Politeness::Polite, Politeness::try_from(1).unwrap());
		assert_eq!(Politeness::Assertive, Politeness::try_from(2).unwrap());
		assert!(Politeness::try_from(3).is_err());
		assert!(Politeness::try_from(-1).is_err());
	}

	#[test]
	fn validate_live_signature() {
		let signature = signal_body_type_signature!("Announcement");
		let politeness_signature_str = &signature.to_string_no_parens();
		let politeness_signature = Signature::from_str(&politeness_signature_str.as_str()[1..2])
			.expect("Valid signature pattern");
		assert_eq!(*<Politeness as Type>::SIGNATURE, politeness_signature);
	}

	#[test]
	fn validate_scroll_type_signature() {
		let signature = method_args_signature!(member: "ScrollTo", interface: "org.a11y.atspi.Component", argument: "type");
		assert_eq!(*<ScrollType as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_layer_signature() {
		let signature = method_return_signature!("GetLayer");
		assert_eq!(*<Layer as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_granularity_signature() {
		let signature = method_args_signature!(member: "GetStringAtOffset", interface: "org.a11y.atspi.Text", argument: "granularity");
		assert_eq!(*<Granularity as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_clip_type_signature() {
		let signature = method_args_signature!(member: "GetTextAtOffset", interface: "org.a11y.atspi.Text", argument: "type");
		assert_eq!(*<ClipType as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_coord_type_signature() {
		let signature = method_args_signature!(member: "GetImagePosition", interface: "org.a11y.atspi.Image", argument: "coordType");
		assert_eq!(*<CoordType as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_match_type_signature() {
		let rule_signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "rule");
		let match_type_signature_str = rule_signature.to_string();
		let match_type_signature = Signature::from_str(&match_type_signature_str.as_str()[3..4])
			.expect("Valid signature pattern");
		assert_eq!(*<MatchType as Type>::SIGNATURE, match_type_signature);
	}

	#[test]
	fn validate_text_selection_signature() {
		let selection_signature = method_args_signature!(member: "GetTextSelections", interface: "org.a11y.atspi.Document", argument: "selections");
		let selection_signature_str = selection_signature.to_string();
		let selection_signature = Signature::from_str(&selection_signature_str.as_str()[1..])
			.expect("Valid signature pattern");
		// this signature is written: `a(...)`, where `(...)` is the signature we want to compare against
		assert_eq!(*<TextSelection as Type>::SIGNATURE, selection_signature);
	}
}
