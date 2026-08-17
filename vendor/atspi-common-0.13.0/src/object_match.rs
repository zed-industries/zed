use std::{borrow::Borrow, collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Serialize};
use zvariant::{Signature, Type};

use crate::{Interface, InterfaceSet, Role, State, StateSet};

/// Defines how an object-tree is to be traversed.
/// Used in `CollectionProxy`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
pub enum TreeTraversalType {
	/// When traversing the tree, restrict to children of the current object.
	RestrictChildren,

	/// When traversing the tree, restrict to siblings of the current object.
	RestrictSibling,

	/// Traverse the tree in order of appearance.
	#[default]
	Inorder,
}

/// Definition of match rules for accessible objects.
/// Rule(s) against which we can match an  object or a collection thereof.
///
/// # Examples
/// ```rust
/// # use zbus::MatchRule;
/// let builder = MatchRule::builder();
/// ```
///
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObjectMatchRule {
	pub states: StateSet,
	pub states_mt: MatchType,
	pub attr: HashMap<String, String>,
	pub attr_mt: MatchType,
	pub roles: Vec<Role>,
	pub roles_mt: MatchType,
	pub ifaces: InterfaceSet,
	pub ifaces_mt: MatchType,
	pub invert: bool,
	// Private phantom, gets compiled away.
	// Here to ensure the builder is the only route to obtain a `MatchRule`
	#[serde(skip)]
	_marker: std::marker::PhantomData<()>,
}

/// !!! WARNING !!! :
///
/// State and Role are defined as u32 in Accessible.xml but as i32 in Collection.xml
///
/// The signature on `StateSet` is defined inconsistently in the XMLs:
///
/// - `Accessible.xml`: `GetState type="au"`
/// - `Collection.xml`: `GetMatches` argument `name="rule" type="(aiia{ss}iaiiasib)"`
///
/// The latter starts with ai, which is a signature for an array of signed ints i32.
///
/// <https://gitlab.gnome.org/federico/at-spi2-core/-/commit/4885efedeef71e0df8210622771a0b1bb10e194d>
impl Type for ObjectMatchRule {
	const SIGNATURE: &'static Signature = &Signature::static_structure(&[
		<Vec<i32>>::SIGNATURE,
		&Signature::I32,
		<HashMap<&str, &str>>::SIGNATURE,
		&Signature::I32,
		<Vec<i32>>::SIGNATURE,
		&Signature::I32,
		<Vec<&str>>::SIGNATURE,
		&Signature::I32,
		&Signature::Bool,
	]);
}

impl ObjectMatchRule {
	/// Create a new `MatchRuleBuilder`
	#[must_use]
	pub fn builder() -> ObjectMatchRuleBuilder {
		ObjectMatchRuleBuilder::default()
	}
}

/// The 'builder' type for `MatchRule`.
/// Use its methods to set match criteria.
#[derive(Debug, Clone, Default)]
pub struct ObjectMatchRuleBuilder {
	states: StateSet,
	states_mt: MatchType,
	attr: HashMap<String, String>,
	attr_mt: MatchType,
	roles: Vec<Role>,
	roles_mt: MatchType,
	ifaces: InterfaceSet,
	ifaces_mt: MatchType,
	invert: bool,
}

impl ObjectMatchRuleBuilder {
	/// Insert a `StateSet` to the builder
	#[must_use]
	pub fn states<I>(mut self, states: I, mt: MatchType) -> Self
	where
		I: IntoIterator,
		I::Item: Borrow<State>,
	{
		self.states = states.into_iter().map(|state| *state.borrow()).collect();
		self.states_mt = mt;
		self
	}

	/// Insert a map of attributes
	#[must_use]
	pub fn attributes(mut self, attributes: HashMap<String, String>, mt: MatchType) -> Self {
		self.attr = attributes;
		self.attr_mt = mt;
		self
	}

	/// Insert a slice of `Role`s
	#[must_use]
	pub fn roles(mut self, roles: &[Role], mt: MatchType) -> Self {
		self.roles = roles.into();
		self.roles_mt = mt;
		self
	}

	/// Insert an `InterfaceSet` from a collection of `Interface`s
	#[must_use]
	pub fn interfaces<I>(mut self, interfaces: I, mt: MatchType) -> Self
	where
		I: IntoIterator,
		I::Item: Borrow<Interface>,
	{
		self.ifaces = interfaces.into_iter().map(|iface| *iface.borrow()).collect();
		self.ifaces_mt = mt;
		self
	}

	/// Sets the inversion of the `MatchRule`, defaults to `false`, no inversion.
	#[must_use]
	pub fn invert(mut self, invert: bool) -> Self {
		self.invert = invert;
		self
	}

	/// Builds the [`ObjectMatchRule`]
	///
	/// [`ObjectMatchRule``]: crate::object_match::ObjectMatchRule
	#[must_use]
	pub fn build(self) -> ObjectMatchRule {
		ObjectMatchRule {
			states: self.states,
			states_mt: self.states_mt,
			attr: self.attr,
			attr_mt: self.attr_mt,
			roles: self.roles,
			roles_mt: self.roles_mt,
			ifaces: self.ifaces,
			ifaces_mt: self.ifaces_mt,
			invert: self.invert,
			_marker: PhantomData,
		}
	}
}

/// Enumeration used by [`ObjectMatchRule`] to specify how to interpret [`ObjectRef`] objects.
///
/// [`ObjectRef`]: crate::object_ref::ObjectRef
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[repr(i32)]
pub enum MatchType {
	#[default]
	/// Invalidates match criterion. Meanting: the search of this property will not be performed.
	Invalid,

	/// All of the criteria must be met.
	All,

	/// Any of the criteria must criteria must be met.
	Any,

	/// None of the criteria must be met.
	NA,

	/// Same as [`Self::All`] if the criterion item is non-empty - All of the criteria must be met.
	/// For empty criteria this rule requires the returned value to also have empty set.
	Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[repr(u32)]
/// Enumeration used by interface `CollectionProxy` to specify the way [`ObjectRef`]
/// objects should be sorted.
///
/// # Note
///
/// Only `Canonical` and `ReverseCanonical` are currently implemented in the known implementation of `Collection`.
///
/// see: [issue 140. "Are all the AtspiCollectionSortOrder types really a thing?"](https://gitlab.gnome.org/GNOME/at-spi2-core/-/issues/140)
///
/// [`ObjectRef`]: crate::object_ref::ObjectRef
pub enum SortOrder {
	/// Invalid sort order
	///
	/// Unimplemented
	Invalid,

	/// Canonical sort order
	Canonical,

	/// Flow sort order
	///
	/// Unimplemented
	Flow,

	/// Tab sort order
	///
	/// Unimplemented
	Tab,

	/// Reverse canonical sort order
	ReverseCanonical,

	/// Reverse flow sort order
	///
	/// Unimplemented
	ReverseFlow,

	/// Reverse tab sort order
	///
	/// Unimplemented
	ReverseTab,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{SortOrder, State};
	use std::str::FromStr;
	use zbus_lockstep::method_args_signature;

	#[test]
	fn validate_match_rule_signature() {
		let signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "rule");
		assert_eq!(*<ObjectMatchRule as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_match_type_signature() {
		let rule_signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "rule");
		// The match type signature is the fourth element in the signature
		let match_type_signature_str = rule_signature.to_string();
		let match_type_signature = Signature::from_str(&match_type_signature_str.as_str()[3..4])
			.expect("Valid signature pattern");
		assert_eq!(*<MatchType as Type>::SIGNATURE, match_type_signature);
	}

	#[test]
	fn validate_tree_traversal_type_signature() {
		let signature = method_args_signature!(member: "GetMatchesTo", interface: "org.a11y.atspi.Collection", argument: "tree");
		assert_eq!(*<TreeTraversalType as Type>::SIGNATURE, signature);
	}

	#[test]
	fn validate_sort_order_signature() {
		let signature = method_args_signature!(member: "GetMatches", interface: "org.a11y.atspi.Collection", argument: "sortby");
		assert_eq!(*<SortOrder as Type>::SIGNATURE, signature);
	}

	#[test]
	fn create_empty_object_match_rule() {
		let rule = ObjectMatchRule::builder().build();

		assert_eq!(rule.states, StateSet::default());
		assert_eq!(rule.attr, HashMap::new());
		assert_eq!(rule.roles, Vec::new());
		assert_eq!(rule.ifaces, InterfaceSet::default());
		assert!(!rule.invert);
	}

	#[test]
	fn create_object_match_rule() {
		let rule = ObjectMatchRule::builder()
			.states(StateSet::new(State::Active), MatchType::All)
			.attributes(
				[("name".to_string(), "value".to_string())].iter().cloned().collect(),
				MatchType::Any,
			)
			.roles(&[Role::Alert], MatchType::All)
			.interfaces([Interface::Action], MatchType::Any)
			.invert(true)
			.build();

		assert_eq!(rule.states, StateSet::new(State::Active));
		assert_eq!(
			rule.attr,
			[("name".to_string(), "value".to_string())].iter().cloned().collect()
		);
		assert_eq!(rule.roles, vec![Role::Alert]);
		assert_eq!(rule.ifaces, InterfaceSet::new(Interface::Action));
		assert!(rule.invert);
	}
}
