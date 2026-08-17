use enumflags2::{bitflags, BitFlag, BitFlags, FromBitsError};
use serde::{
	de::{self, Deserializer, Visitor},
	ser::{SerializeSeq, Serializer},
	Deserialize, Serialize,
};
use std::{convert::Infallible, fmt, str::FromStr};
use zvariant::{Signature, Type};

/// Used by various interfaces indicating every possible state
/// of an accessibility object.
#[bitflags]
#[non_exhaustive]
#[repr(u64)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "kebab-case")]
pub enum State {
	/// Indicates an invalid state - probably an error condition.
	#[default]
	Invalid,
	/// Indicates a window is currently the active window, or
	/// an object is the active subelement within a container or table.
	///
	/// `Active` should not be used for objects which have
	/// [`State::Focusable`] or [`State::Selectable`]: Those objects should use
	/// [`State::Focused`] and [`State::Selected`] respectively.
	///
	/// `Active` is a means to indicate that an object which is not
	/// focusable and not selectable is the currently-active item within its
	/// parent container.
	Active,
	/// Indicates that the object is armed.
	Armed,
	/// Indicates the current object is busy, i.e. onscreen
	/// representation is in the process of changing, or       the object is
	/// temporarily unavailable for interaction due to activity already in progress.
	Busy,
	/// Indicates this object is currently checked.
	Checked,
	/// Indicates this object is collapsed.
	Collapsed,
	/// Indicates that this object no longer has a valid
	/// backing widget        (for instance, if its peer object has been destroyed).
	Defunct,
	/// Indicates the user can change the contents of this object.
	Editable,
	/// Indicates that this object is enabled, i.e. that it
	/// currently reflects some application state. Objects that are "greyed out"
	/// may lack this state, and may lack the [`State::Sensitive`] if direct
	/// user interaction cannot cause them to acquire `Enabled`.
	///
	/// See [`State::Sensitive`].
	Enabled,
	/// Indicates this object allows progressive
	/// disclosure of its children.
	Expandable,
	/// Indicates this object is expanded.
	Expanded,
	/// Indicates this object can accept keyboard focus,
	/// which means all events resulting from typing on the keyboard will
	/// normally be passed to it when it has focus.
	Focusable,
	/// Indicates this object currently has the keyboard focus.
	Focused,
	/// Indicates that the object has an associated tooltip.
	HasTooltip,
	/// Indicates the orientation of this object is horizontal.
	Horizontal,
	/// Indicates this object is minimized and is
	/// represented only by an icon.
	Iconified,
	/// Indicates something must be done with this object
	/// before the user can interact with an object in a different window.
	Modal,
	/// Indicates this (text) object can contain multiple
	/// lines of text.
	MultiLine,
	/// Indicates this object allows more than one of
	/// its children to be selected at the same time, or in the case of text
	/// objects, that the object supports non-contiguous text selections.
	Multiselectable,
	/// Indicates this object paints every pixel within its
	/// rectangular region. It also indicates an alpha value of unity, if it
	/// supports alpha blending.
	Opaque,
	/// Indicates this object is currently pressed.
	Pressed,
	/// Indicates the size of this object's size is not fixed.
	Resizable,
	/// Indicates this object is the child of an object
	/// that allows its children to be selected and that this child is one of
	/// those children       that can be selected.
	Selectable,
	/// Indicates this object is the child of an object that
	/// allows its children to be selected and that this child is one of those
	/// children that has been selected.
	Selected,
	/// Indicates this object is sensitive, e.g. to user
	/// interaction. `Sensitive` usually accompanies.
	/// [`State::Enabled`] for user-actionable controls, but may be found in the
	/// absence of [`State::Enabled`] if the current visible state of the control
	/// is "disconnected" from the application state.  In such cases, direct user
	/// interaction can often result in the object gaining `Sensitive`,
	/// for instance if a user makes an explicit selection using an object whose
	/// current state is ambiguous or undefined.
	///
	/// See [`State::Enabled`], [`State::Indeterminate`].
	Sensitive,
	/// Indicates this object, the object's parent, the
	/// object's parent's parent, and so on, are all 'shown' to the end-user,
	/// i.e. subject to "exposure" if blocking or obscuring objects do not
	/// interpose between this object and the top of the window stack.
	Showing,
	/// Indicates this (text) object can contain only a
	/// single line of text.
	SingleLine,
	/// Indicates that the information returned for this object
	/// may no longer be synchronized with the application state.  This can occur
	/// if the object has [`State::Transient`], and can also occur towards the
	/// end of the object peer's lifecycle.
	Stale,
	/// Indicates this object is transient.
	Transient,
	/// Indicates the orientation of this object is vertical;
	/// for example this state may appear on such objects as scrollbars, text
	/// objects (with vertical text flow), separators, etc.
	Vertical,
	/// Indicates this object is visible, e.g. has been
	/// explicitly marked for exposure to the user. `Visible` is no
	/// guarantee that the object is actually unobscured on the screen, only that
	/// it is 'potentially' visible, barring obstruction, being scrolled or clipped
	/// out of the field of view, or having an ancestor container that has not yet
	/// made visible. A widget is potentially onscreen if it has both
	/// `Visible` and [`State::Showing`]. The absence of
	/// `Visible` and [`State::Showing`] is
	/// semantically equivalent to saying that an object is 'hidden'.
	Visible,
	/// Indicates that "active-descendant-changed"
	/// event is sent when children become 'active' (i.e. are selected or
	/// navigated to onscreen).  Used to prevent need to enumerate all children
	/// in very large containers, like tables. The presence of
	/// `ManagesDescendants` is an indication to the client that the
	/// children should not, and need not, be enumerated by the client.
	/// Objects implementing this state are expected to provide relevant state
	/// notifications to listening clients, for instance notifications of
	/// visibility changes and activation of their contained child objects, without
	/// the client having previously requested references to those children.
	ManagesDescendants,
	/// Indicates that a check box or other boolean
	/// indicator is in a state other than checked or not checked.
	///
	/// This usually means that the boolean value reflected or controlled by the
	/// object does not apply consistently to the entire current context.
	/// For example, a checkbox for the "Bold" attribute of text may have
	/// `Indeterminate` if the currently selected text contains a mixture
	/// of weight attributes. In many cases interacting with a
	/// `Indeterminate` object will cause the context's corresponding
	/// boolean attribute to be homogenized, whereupon the object will lose
	/// `Indeterminate` and a corresponding state-changed event will be
	/// fired.
	Indeterminate,
	/// Indicates that user interaction with this object is
	/// 'required' from the user, for instance before completing the
	/// processing of a form.
	Required,
	/// Indicates that an object's onscreen content
	/// is truncated, e.g. a text value in a spreadsheet cell.
	Truncated,
	/// Indicates this object's visual representation is
	/// dynamic, not static. This state may be applied to an object during an
	/// animated 'effect' and be removed from the object once its visual
	/// representation becomes static. Some applications, notably content viewers,
	/// may not be able to detect all kinds of animated content.  Therefore the
	/// absence of this state should not be taken as
	/// definitive evidence that the object's visual representation is
	/// static; this state is advisory.
	Animated,
	/// This object has indicated an error condition
	/// due to failure of input validation.  For instance, a form control may
	/// acquire this state in response to invalid or malformed user input.
	InvalidEntry,
	/// This state indicates that the object
	/// in question implements some form of typeahead or
	/// pre-selection behavior whereby entering the first character of one or more
	/// sub-elements causes those elements to scroll into view or become
	/// selected. Subsequent character input may narrow the selection further as
	/// long as one or more sub-elements match the string. This state is normally
	/// only useful and encountered on objects that implement [`crate::interface::Interface::Selection`].
	/// In some cases the typeahead behavior may result in full or partial
	/// completion of the data in the input field, in which case
	/// these input events may trigger text-changed events from the source.
	SupportsAutocompletion,
	/// Indicates that the object in
	/// question supports text selection. It should only be exposed on objects
	/// which implement the [`crate::interface::Interface::Text`] interface, in order to distinguish this state
	/// from [`State::Selectable`], which infers that the object in question is a
	/// selectable child of an object which implements [`crate::interface::Interface::Selection`]. While
	/// similar, text selection and subelement selection are distinct operations.
	SelectableText,
	/// Indicates that the object in question is
	/// the 'default' interaction object in a dialog, i.e. the one that gets
	/// activated if the user presses "Enter" when the dialog is initially
	/// posted.
	IsDefault,
	/// Indicates that the object (typically a
	/// hyperlink) has already been activated or invoked, with the result that
	/// some backing data has been downloaded or rendered.
	Visited,
	/// Indicates this object has the potential to
	/// be checked, such as a checkbox or toggle-able table cell.
	Checkable,
	/// Indicates that the object has a popup
	/// context menu or sub-level menu which may or may not be
	/// showing. This means that activation renders conditional content.
	/// Note that ordinary tooltips are not considered popups in this
	/// context.
	HasPopup,
	/// Indicates that an object which is [`State::Enabled`] and
	/// [`State::Sensitive`] has a value which can be read, but not modified, by the
	/// user.
	ReadOnly,
}

impl State {
	// "Matches on enums with undeclared discriminants (such that the compiler can choose sequential values)
	// are optimized down to a jump table"
	// https://users.rust-lang.org/t/match-statement-efficiency/4488/2

	/// Returns the state as a static string.
	#[must_use]
	pub fn to_static_str(&self) -> &'static str {
		match self {
			State::Invalid => "invalid",
			State::Active => "active",
			State::Armed => "armed",
			State::Busy => "busy",
			State::Checked => "checked",
			State::Collapsed => "collapsed",
			State::Defunct => "defunct",
			State::Editable => "editable",
			State::Enabled => "enabled",
			State::Expandable => "expandable",
			State::Expanded => "expanded",
			State::Focusable => "focusable",
			State::Focused => "focused",
			State::HasTooltip => "has-tooltip",
			State::Horizontal => "horizontal",
			State::Iconified => "iconified",
			State::Modal => "modal",
			State::MultiLine => "multi-line",
			State::Multiselectable => "multiselectable",
			State::Opaque => "opaque",
			State::Pressed => "pressed",
			State::Resizable => "resizable",
			State::Selectable => "selectable",
			State::Selected => "selected",
			State::Sensitive => "sensitive",
			State::Showing => "showing",
			State::SingleLine => "single-line",
			State::Stale => "stale",
			State::Transient => "transient",
			State::Vertical => "vertical",
			State::Visible => "visible",
			State::ManagesDescendants => "manages-descendants",
			State::Indeterminate => "indeterminate",
			State::Required => "required",
			State::Truncated => "truncated",
			State::Animated => "animated",
			State::InvalidEntry => "invalid-entry",
			State::SupportsAutocompletion => "supports-autocompletion",
			State::SelectableText => "selectable-text",
			State::IsDefault => "is-default",
			State::Visited => "visited",
			State::Checkable => "checkable",
			State::HasPopup => "has-popup",
			State::ReadOnly => "read-only",
		}
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(self.to_static_str())
	}
}

impl From<String> for State {
	fn from(string: String) -> State {
		(&*string).into()
	}
}

impl FromStr for State {
	type Err = Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(s.into())
	}
}

impl From<&str> for State {
	fn from(string: &str) -> State {
		match string {
			"active" => State::Active,
			"armed" => State::Armed,
			"busy" => State::Busy,
			"checked" => State::Checked,
			"collapsed" => State::Collapsed,
			"defunct" => State::Defunct,
			"editable" => State::Editable,
			"enabled" => State::Enabled,
			"expandable" => State::Expandable,
			"expanded" => State::Expanded,
			"focusable" => State::Focusable,
			"focused" => State::Focused,
			"has-tooltip" => State::HasTooltip,
			"horizontal" => State::Horizontal,
			"iconified" => State::Iconified,
			"modal" => State::Modal,
			"multi-line" => State::MultiLine,
			"multiselectable" => State::Multiselectable,
			"opaque" => State::Opaque,
			"pressed" => State::Pressed,
			"resizable" => State::Resizable,
			"selectable" => State::Selectable,
			"selected" => State::Selected,
			"sensitive" => State::Sensitive,
			"showing" => State::Showing,
			"single-line" => State::SingleLine,
			"stale" => State::Stale,
			"transient" => State::Transient,
			"vertical" => State::Vertical,
			"visible" => State::Visible,
			"manages-descendants" => State::ManagesDescendants,
			"indeterminate" => State::Indeterminate,
			"required" => State::Required,
			"truncated" => State::Truncated,
			"animated" => State::Animated,
			"invalid-entry" => State::InvalidEntry,
			"supports-autocompletion" => State::SupportsAutocompletion,
			"selectable-text" => State::SelectableText,
			"is-default" => State::IsDefault,
			"visited" => State::Visited,
			"checkable" => State::Checkable,
			"has-popup" => State::HasPopup,
			"read-only" => State::ReadOnly,
			_ => State::Invalid,
		}
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
/// The bitflag representation of all states an object may have.
pub struct StateSet(BitFlags<State>);

impl StateSet {
	/// Create a new `StateSet`.
	///
	/// ## Example
	/// ```rust
	/// # use atspi_common::{State, StateSet};
	/// let states = State::Focusable | State::Sensitive | State::Active;
	/// let set = StateSet::new(states);
	///
	/// assert!(set.contains(State::Active));
	/// assert!(!set.contains(State::Busy));
	/// ```
	pub fn new<B: Into<BitFlags<State>>>(value: B) -> Self {
		Self(value.into())
	}

	/// Returns the `StateSet` that corresponds to the provided `u64`s bit pattern.
	/// # Errors
	/// When the argument encodes an undefined [`State`].
	pub fn from_bits(bits: u64) -> Result<StateSet, FromBitsError<State>> {
		Ok(StateSet(BitFlags::from_bits(bits)?))
	}

	#[must_use]
	/// Create an empty `StateSet`
	pub fn empty() -> StateSet {
		StateSet(State::empty())
	}

	#[must_use]
	/// Returns the state as represented by a u64.
	pub fn bits(&self) -> u64 {
		self.0.bits()
	}

	/// Whether the `StateSet` contains a [`State`].
	pub fn contains<B: Into<BitFlags<State>>>(self, other: B) -> bool {
		self.0.contains(other)
	}

	/// Removes a [`State`] (optionally) previously contained in the `StateSet`.
	pub fn remove<B: Into<BitFlags<State>>>(&mut self, other: B) {
		self.0.remove(other);
	}

	///  Inserts a [`State`] in the `StateSet`.
	pub fn insert<B: Into<BitFlags<State>>>(&mut self, other: B) {
		self.0.insert(other);
	}

	/// Returns an iterator that yields each set [`State`].
	#[must_use]
	pub fn iter(self) -> enumflags2::Iter<State> {
		self.0.iter()
	}

	#[must_use]
	/// Checks if all states are unset.
	pub fn is_empty(self) -> bool {
		self.0.is_empty()
	}

	/// Returns true if at least one flag is shared.
	pub fn intersects<B: Into<BitFlags<State>>>(self, other: B) -> bool {
		self.0.intersects(other)
	}

	/// Toggles the matching bits.
	pub fn toggle<B: Into<BitFlags<State>>>(&mut self, other: B) {
		self.0.toggle(other);
	}
}

impl IntoIterator for StateSet {
	type IntoIter = enumflags2::Iter<State>;
	type Item = State;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl IntoIterator for &StateSet {
	type IntoIter = enumflags2::Iter<State>;
	type Item = State;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl FromIterator<State> for StateSet {
	fn from_iter<I: IntoIterator<Item = State>>(iter: I) -> Self {
		StateSet(iter.into_iter().collect())
	}
}

impl<'a> FromIterator<&'a State> for StateSet {
	fn from_iter<I: IntoIterator<Item = &'a State>>(iter: I) -> Self {
		StateSet(iter.into_iter().copied().collect())
	}
}

impl<'de> Deserialize<'de> for StateSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct StateSetVisitor;

		impl<'de> Visitor<'de> for StateSetVisitor {
			type Value = StateSet;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter
					.write_str("a sequence comprised of two u32 that represents a valid StateSet")
			}

			fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: Deserializer<'de>,
			{
				match <Vec<u32> as Deserialize>::deserialize(deserializer) {
					Ok(states) if states.len() == 2 => {
						let mut bits = u64::from(states[0]);
						bits |= (u64::from(states[1])) << 32;
						StateSet::from_bits(bits).map_err(|_| de::Error::custom("invalid state"))
					}
					Ok(states) => Err(de::Error::invalid_length(states.len(), &"array of size 2")),
					Err(e) => Err(e),
				}
			}
		}

		deserializer.deserialize_newtype_struct("StateSet", StateSetVisitor)
	}
}

impl Serialize for StateSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut seq = serializer.serialize_seq(Some(2))?;
		let bits = self.bits();

		// This cast is safe and truncation is intentional.
		//The shift is sound provided that `State` is `#[repr(u64)]`
		#[allow(clippy::cast_possible_truncation)]
		seq.serialize_element(&(bits as u32))?;
		seq.serialize_element(&((bits >> 32) as u32))?;
		seq.end()
	}
}

impl Type for StateSet {
	const SIGNATURE: &'static Signature = <Vec<u32> as Type>::SIGNATURE;
}

impl From<State> for StateSet {
	fn from(value: State) -> Self {
		Self(value.into())
	}
}

impl std::ops::BitXor for StateSet {
	type Output = StateSet;

	fn bitxor(self, other: Self) -> Self::Output {
		StateSet(self.0 ^ other.0)
	}
}
impl std::ops::BitXorAssign for StateSet {
	fn bitxor_assign(&mut self, other: Self) {
		self.0 = self.0 ^ other.0;
	}
}
impl std::ops::BitOr for StateSet {
	type Output = StateSet;

	fn bitor(self, other: Self) -> Self::Output {
		StateSet(self.0 | other.0)
	}
}
impl std::ops::BitOrAssign for StateSet {
	fn bitor_assign(&mut self, other: Self) {
		self.0 = self.0 | other.0;
	}
}
impl std::ops::BitAnd for StateSet {
	type Output = StateSet;

	fn bitand(self, other: Self) -> Self::Output {
		StateSet(self.0 & other.0)
	}
}
impl std::ops::BitAndAssign for StateSet {
	fn bitand_assign(&mut self, other: Self) {
		self.0 = self.0 & other.0;
	}
}

#[cfg(test)]
mod tests {
	use super::{State, StateSet};
	use zvariant::serialized::{Context, Data};
	use zvariant::{to_bytes, LE};

	#[test]
	fn serialize_empty_state_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &StateSet::empty()).unwrap();
		assert_eq!(encoded.bytes(), &[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_empty_state_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ctxt);
		let (decoded, _) = data.deserialize::<StateSet>().unwrap();
		assert_eq!(decoded, StateSet::empty());
	}

	#[test]
	fn serialize_state_set_invalid() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &StateSet::new(State::Invalid)).unwrap();
		assert_eq!(encoded.bytes(), &[8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_state_set_invalid() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], ctxt);
		let (decoded, _) = data.deserialize::<StateSet>().unwrap();
		assert_eq!(decoded, StateSet::new(State::Invalid));
	}

	#[test]
	fn serialize_state_set_manages_descendants() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &StateSet::new(State::ManagesDescendants)).unwrap();
		assert_eq!(encoded.bytes(), &[8, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_state_set_manages_descendants() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0], ctxt);
		let (decoded, _) = data.deserialize::<StateSet>().unwrap();
		assert_eq!(decoded, StateSet::new(State::ManagesDescendants));
	}

	#[test]
	fn serialize_state_set_indeterminate() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &StateSet::new(State::Indeterminate)).unwrap();
		assert_eq!(encoded.bytes(), &[8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]);
	}

	#[test]
	fn deserialize_state_set_indeterminate() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], ctxt);
		let (decoded, _) = data.deserialize::<StateSet>().unwrap();
		assert_eq!(decoded, StateSet::new(State::Indeterminate));
	}

	#[test]
	fn serialize_state_set_focusable_focused() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &StateSet::new(State::Focusable | State::Focused)).unwrap();
		assert_eq!(encoded.bytes(), &[8, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_state_set_focusable_focused() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0], ctxt);
		let (decoded, _) = data.deserialize::<StateSet>().unwrap();
		assert_eq!(decoded, StateSet::new(State::Focusable | State::Focused));
	}

	#[test]
	fn cannot_deserialize_state_set_invalid_length() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ctxt);
		let decode_result = data.deserialize::<StateSet>();
		assert!(decode_result.is_err());
	}

	#[test]
	fn cannot_deserialize_state_set_invalid_flag() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(&[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32], ctxt);
		let decode_result = data.deserialize::<StateSet>();
		assert!(decode_result.is_err());
	}

	#[test]
	fn convert_state_direct_string() {
		for state in
			StateSet::from_bits(0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111).unwrap()
		{
			let state_str: String = state.to_string();
			let state_two: State = state_str.clone().into();
			assert_eq!(
				state, state_two,
				"The {state:?} was serialized as {state_str}, which deserializes to {state_two:?}"
			);
		}
	}
	#[test]
	fn convert_state_direct_string_is_equal_to_serde_output() {
		for state in
			StateSet::from_bits(0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111).unwrap()
		{
			let serde_state_str: String = serde_plain::to_string(&state).unwrap();
			let state_str: String = state.to_string();
			assert_eq!(serde_state_str, state_str);
			let state_two: State = serde_plain::from_str(&state_str).unwrap();
			assert_eq!(state, state_two, "The {state:?} was serialized as {state_str}, which deserializes to {state_two:?} (serde)");
		}
	}

	#[test]
	fn collect_stateset_from_owned_states() {
		let states = vec![State::Active, State::Focused, State::Focusable];
		let set = StateSet::from_iter(states);
		assert!(set.contains(State::Active));
		assert!(set.contains(State::Focused));
		assert!(set.contains(State::Focusable));
	}

	#[test]
	fn collect_stateset_from_borrowed_states() {
		// &[T].iter() yields &T
		let states = &[State::Active, State::Focused, State::Focusable];
		let set = states.iter().collect::<StateSet>();
		assert!(set.contains(State::Active));
		assert!(set.contains(State::Focused));
		assert!(set.contains(State::Focusable));
	}

	#[test]
	fn into_iterator_owned_stateset() {
		let set = StateSet::new(State::Active | State::Focused | State::Focusable);
		let states: Vec<State> = set.into_iter().collect();
		assert_eq!(states.len(), 3);
		assert!(states.contains(&State::Active));
		assert!(states.contains(&State::Focused));
		assert!(states.contains(&State::Focusable));
	}

	#[test]
	fn into_iterator_borrowed_stateset() {
		let set = StateSet::new(State::Active | State::Focused | State::Focusable);
		let states: Vec<State> = (&set).into_iter().collect();
		assert_eq!(states.len(), 3);
		assert!(states.contains(&State::Active));
		assert!(states.contains(&State::Focused));
		assert!(states.contains(&State::Focusable));
	}
}
