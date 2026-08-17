//! # zbus-lockstep
//!
//! Is a collection of helpers for retrieving `DBus` type signatures from XML descriptions.
//! Useful for comparing these with your types' signatures to ensure that they are compatible.
//!
//! It offers functions that retrieve the signature of a method's argument type, of a method's
//! return type, pf a signal's body type or of a property's type from `DBus` XML.
//!
//! These functions require that you provide the file path to the XML file, the interface name,
//! and the interface member wherein the signature resides.
//!
//! Corresponding to each of these functions, macros are provided which do not
//! require you to exactly point out where the signature is found. These will just search
//! by interface member name.
//!
//! The macros assume that the file path to the XML files is either:
//!
//! - `xml` or `XML`, the default path for `DBus` XML files - or is set by the
//! - `LOCKSTEP_XML_PATH`, the env variable that overrides the default.
#![doc(html_root_url = "https://docs.rs/zbus-lockstep/0.5.2")]
#![allow(clippy::missing_errors_doc)]

mod error;
mod macros;

use std::{io::Read, str::FromStr};

pub use error::LockstepError;
pub use macros::resolve_xml_path;
pub use zbus_xml::{
    self,
    ArgDirection::{In, Out},
    Node,
};
use zvariant::Signature;
use LockstepError::{ArgumentNotFound, InterfaceNotFound, MemberNotFound, PropertyNotFound};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MsgType {
    Method,
    Signal,
    Property,
}

/// Retrieve a signal's body type signature from `DBus` XML.
///
/// If you provide an argument name, then the signature of that argument is returned.
/// If you do not provide an argument name, then the signature of all arguments is returned.
///
/// # Examples
///
/// ```rust
/// # use std::fs::File;
/// # use std::io::{Seek, SeekFrom, Write};
/// # use tempfile::tempfile;
/// use zvariant::{Signature, Type, OwnedObjectPath};
/// use zbus_lockstep::get_signal_body_type;
///
/// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
/// <node xmlns:doc="http://www.freedesktop.org/dbus/1.0/doc.dtd">
/// <interface name="org.freedesktop.bolt1.Manager">
///   <signal name="DeviceAdded">
///    <arg name="device" type="o"/>
///  </signal>
/// </interface>
/// </node>
/// "#;
///
/// let mut xml_file: File = tempfile().unwrap();
/// xml_file.write_all(xml.as_bytes()).unwrap();
/// xml_file.seek(SeekFrom::Start(0)).unwrap();
///
/// #[derive(Debug, PartialEq, Type)]
/// #[zvariant(signature = "o")]
/// struct DeviceEvent {
///    device: OwnedObjectPath,
/// }
///
/// let interface_name = "org.freedesktop.bolt1.Manager";
/// let member_name = "DeviceAdded";
///
/// let signature = get_signal_body_type(xml_file, interface_name, member_name, None).unwrap();
///
/// assert_eq!(&signature, DeviceEvent::SIGNATURE);
/// ```
pub fn get_signal_body_type(
    mut xml: impl Read,
    interface_name: &str,
    member_name: &str,
    arg: Option<&str>,
) -> Result<Signature> {
    let node = Node::from_reader(&mut xml)?;

    let interfaces = node.interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.name() == interface_name)
        .ok_or(InterfaceNotFound(interface_name.to_owned()))?;

    let signals = interface.signals();
    let signal = signals
        .iter()
        .find(|signal| signal.name() == member_name)
        .ok_or(MemberNotFound(member_name.to_owned()))?;

    let signature = {
        if let Some(arg_name) = arg {
            let args = signal.args();
            let arg = args
                .iter()
                .find(|arg| arg.name() == Some(arg_name))
                .ok_or(ArgumentNotFound(arg_name.to_owned()))?;
            arg.ty().to_string()
        } else {
            signal
                .args()
                .iter()
                .map(|arg| arg.ty().to_string())
                .collect::<String>()
        }
    };
    Ok(Signature::from_str(&signature).map_err(|_| "Invalid signature")?)
}

/// Retrieve the signature of a property's type from XML.
///
/// # Examples
///
/// ```rust
/// use std::fs::File;
/// use std::io::{Seek, SeekFrom, Write};
/// use tempfile::tempfile;
/// use zvariant::Type;
/// use zbus_lockstep::get_property_type;
///
/// #[derive(Debug, PartialEq, Type)]
/// struct InUse(bool);
///
/// let xml = String::from(r#"
/// <node>
/// <interface name="org.freedesktop.GeoClue2.Manager">
///   <property type="b" name="InUse" access="read"/>
/// </interface>
/// </node>
/// "#);
///
/// let mut xml_file: File = tempfile().unwrap();
/// xml_file.write_all(xml.as_bytes()).unwrap();
/// xml_file.seek(SeekFrom::Start(0)).unwrap();
///
/// let interface_name = "org.freedesktop.GeoClue2.Manager";
/// let property_name = "InUse";
///
/// let signature = get_property_type(xml_file, interface_name, property_name).unwrap();
/// assert_eq!(signature, *InUse::SIGNATURE);
/// ```
pub fn get_property_type(
    mut xml: impl Read,
    interface_name: &str,
    property_name: &str,
) -> Result<Signature> {
    let node = Node::from_reader(&mut xml)?;

    let interfaces = node.interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.name() == interface_name)
        .ok_or(InterfaceNotFound(interface_name.to_string()))?;

    let properties = interface.properties();
    let property = properties
        .iter()
        .find(|property| property.name() == property_name)
        .ok_or(PropertyNotFound(property_name.to_owned()))?;

    let signature = property.ty().to_string();
    Ok(Signature::from_str(&signature).map_err(|_| "Invalid signature")?)
}

/// Retrieve the signature of a method's return type from XML.
///
/// If you provide an argument name, then the signature of that argument is returned.
/// If you do not provide an argument name, then the signature of all arguments is returned.
///
///
/// # Examples
///
/// ```rust
/// use std::fs::File;
/// use std::io::{Seek, SeekFrom, Write};
/// use tempfile::tempfile;
/// use zvariant::Type;
/// use zbus_lockstep::get_method_return_type;
///
/// #[derive(Debug, PartialEq, Type)]
/// #[repr(u32)]
/// enum Role {
///     Invalid,
///     TitleBar,
///     MenuBar,
///     ScrollBar,
/// }
///
/// let xml = String::from(r#"
/// <node>
/// <interface name="org.a11y.atspi.Accessible">
///    <method name="GetRole">
///       <arg name="role" type="u" direction="out"/>
///   </method>
/// </interface>
/// </node>
/// "#);
///
/// let mut xml_file: File = tempfile().unwrap();
/// xml_file.write_all(xml.as_bytes()).unwrap();
/// xml_file.seek(SeekFrom::Start(0)).unwrap();
///
/// let interface_name = "org.a11y.atspi.Accessible";
/// let member_name = "GetRole";
///
/// let signature = get_method_return_type(xml_file, interface_name, member_name, None).unwrap();
/// assert_eq!(signature, *Role::SIGNATURE);
/// ```
pub fn get_method_return_type(
    mut xml: impl Read,
    interface_name: &str,
    member_name: &str,
    arg_name: Option<&str>,
) -> Result<Signature> {
    let node = Node::from_reader(&mut xml)?;

    let interfaces = node.interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.name() == interface_name)
        .ok_or(InterfaceNotFound(interface_name.to_string()))?;

    let methods = interface.methods();
    let method = methods
        .iter()
        .find(|method| method.name() == member_name)
        .ok_or(MemberNotFound(member_name.to_string()))?;

    let args = method.args();

    let signature = {
        if let Some(known_arg_name) = arg_name {
            args.iter()
                .find(|arg| arg.name() == Some(known_arg_name))
                .ok_or(ArgumentNotFound(known_arg_name.to_string()))?
                .ty()
                .to_string()
        } else {
            args.iter()
                .filter(|arg| arg.direction() == Some(Out))
                .map(|arg| arg.ty().to_string())
                .collect::<String>()
        }
    };

    Ok(Signature::from_str(&signature).map_err(|_| "Invalid signature")?)
}

/// Retrieve the signature of a method's argument type from XML.
///
/// Useful when one or more arguments, used to call a method, outline a useful type.
///
/// If you provide an argument name, then the signature of that argument is returned.
/// If you do not provide an argument name, then the signature of all arguments to the call is
/// returned.
///
/// # Examples
///
/// ```rust
/// use std::fs::File;
/// use std::collections::HashMap;
/// use std::io::{Seek, SeekFrom, Write};
/// use tempfile::tempfile;
/// use zvariant::{Type, Value};
/// use zbus_lockstep::get_method_args_type;
///
/// let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
/// <node xmlns:doc="http://www.freedesktop.org/dbus/1.0/doc.dtd">
///  <interface name="org.freedesktop.Notifications">
///    <method name="Notify">
///      <arg type="s" name="app_name" direction="in"/>
///      <arg type="u" name="replaces_id" direction="in"/>
///      <arg type="s" name="app_icon" direction="in"/>
///      <arg type="s" name="summary" direction="in"/>
///      <arg type="s" name="body" direction="in"/>
///      <arg type="as" name="actions" direction="in"/>
///      <arg type="a{sv}" name="hints" direction="in"/>
///      <arg type="i" name="expire_timeout" direction="in"/>
///      <arg type="u" name="id" direction="out"/>
///    </method>
///  </interface>
/// </node>
/// "#;
///
/// #[derive(Debug, PartialEq, Type)]
/// struct Notification<'a> {
///    app_name: String,
///    replaces_id: u32,
///    app_icon: String,
///    summary: String,
///    body: String,
///    actions: Vec<String>,
///    hints: HashMap<String, Value<'a>>,
///    expire_timeout: i32,
/// }
///
/// let mut xml_file = tempfile().unwrap();
/// xml_file.write_all(xml.as_bytes()).unwrap();
/// xml_file.seek(SeekFrom::Start(0)).unwrap();
///
/// let interface_name = "org.freedesktop.Notifications";
/// let member_name = "Notify";
///
/// let signature = get_method_args_type(xml_file, interface_name, member_name, None).unwrap();
/// assert_eq!(&signature, Notification::SIGNATURE);
/// ```
pub fn get_method_args_type(
    mut xml: impl Read,
    interface_name: &str,
    member_name: &str,
    arg_name: Option<&str>,
) -> Result<Signature> {
    let node = Node::from_reader(&mut xml)?;

    let interfaces = node.interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.name() == interface_name)
        .ok_or(InterfaceNotFound(interface_name.to_owned()))?;

    let methods = interface.methods();
    let method = methods
        .iter()
        .find(|method| method.name() == member_name)
        .ok_or(member_name.to_owned())?;

    let args = method.args();

    let signature = if let Some(known_arg_name) = arg_name {
        args.iter()
            .find(|arg| arg.name() == Some(known_arg_name))
            .ok_or(ArgumentNotFound(known_arg_name.to_string()))?
            .ty()
            .to_string()
    } else {
        args.iter()
            .filter(|arg| arg.direction() == Some(In))
            .map(|arg| arg.ty().to_string())
            .collect::<String>()
    };

    Ok(Signature::from_str(&signature).map_err(|_| "Invalid signature")?)
}

#[cfg(test)]
mod test {
    use std::io::{Seek, SeekFrom, Write};

    use tempfile::tempfile;
    use zvariant::{OwnedObjectPath, Type};

    use crate::get_signal_body_type;

    #[test]
    fn test_get_signature_of_cache_add_accessible() {
        #[derive(Debug, PartialEq, Type)]
        struct Accessible {
            name: String,
            path: OwnedObjectPath,
        }

        #[derive(Debug, PartialEq, Type)]
        struct CacheItem {
            obj: Accessible,
            application: Accessible,
            parent: Accessible,
            index_in_parent: i32,
            child_count: i32,
            interfaces: Vec<String>,
            name: String,
            role: u32,
            description: String,
            state_set: Vec<u32>,
        }

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <node xmlns:doc="http://www.freedesktop.org/dbus/1.0/doc.dtd">
                <interface name="org.a11y.atspi.Cache">
                    <signal name="AddAccessible">
                        <arg name="nodeAdded" type="((so)(so)(so)iiassusau)"/>
                        <annotation name="org.qtproject.QtDBus.QtTypeName.In0" value="QSpiAccessibleCacheItem"/>
                    </signal>
                </interface>
            </node>
        "#;

        let mut xml_file = tempfile().unwrap();
        xml_file.write_all(xml.as_bytes()).unwrap();
        xml_file.seek(SeekFrom::Start(0)).unwrap();

        let interface_name = "org.a11y.atspi.Cache";
        let member_name = "AddAccessible";

        let signature = get_signal_body_type(xml_file, interface_name, member_name, None).unwrap();
        assert_eq!(signature, *CacheItem::SIGNATURE);
    }
}
