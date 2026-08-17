use serde::{Deserialize, Serialize};
use zvariant::Type;

/// An action which may be triggered through the accessibility API.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct Action {
	/// The name of the action
	pub name: String,
	/// Description of the action
	pub description: String,
	// TODO: should be an enum/stricter type; this is why it's in its own file.
	/// The keybinding(s) used to trigger it (without the AT).
	pub keybinding: String,
}

#[cfg(test)]
mod test {
	use super::Action;
	use zbus_lockstep::method_return_signature;
	use zvariant::Type;
	#[test]
	fn validate_action_signature() {
		// signature is of type `a(sss)`, where `(sss)` is the type we're validating.
		let action_signature =
			method_return_signature!(member: "GetActions", interface: "org.a11y.atspi.Action");
		assert_eq!(Vec::<Action>::SIGNATURE, &action_signature);
	}
}
