use zbus_names::{BusName, InterfaceName};
use zvariant::ObjectPath;

/// Trait for the default associated values of a proxy.
///
/// The trait is automatically implemented by the [`macro@crate::proxy`] macro on your behalf, and
/// may be later used to retrieve the associated constants.
pub trait Defaults {
    const INTERFACE: &'static Option<InterfaceName<'static>>;
    const DESTINATION: &'static Option<BusName<'static>>;
    const PATH: &'static Option<ObjectPath<'static>>;
}

impl Defaults for super::Proxy<'_> {
    const INTERFACE: &'static Option<InterfaceName<'static>> = &None;
    const DESTINATION: &'static Option<BusName<'static>> = &None;
    const PATH: &'static Option<ObjectPath<'static>> = &None;
}
