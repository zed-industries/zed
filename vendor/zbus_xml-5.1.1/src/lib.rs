#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/z-galaxy/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

mod error;
pub use error::{Error, Result};

use quick_xml::{de::Deserializer, se::to_writer};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufReader, Read, Write},
    ops::Deref,
};

use zbus_names::{InterfaceName, MemberName, PropertyName};

/// Annotations are generic key/value pairs of metadata.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Annotation {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
}

impl Annotation {
    /// Return the annotation name/key.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the annotation value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// A direction of an argument
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArgDirection {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

/// An argument
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Arg {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@type")]
    ty: Signature,
    #[serde(rename = "@direction")]
    direction: Option<ArgDirection>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Arg {
    /// Return the argument name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the argument type.
    pub fn ty(&self) -> &Signature {
        &self.ty
    }

    /// Return the argument direction, if any.
    pub fn direction(&self) -> Option<ArgDirection> {
        self.direction
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A method
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Method<'a> {
    #[serde(rename = "@name", borrow)]
    name: MemberName<'a>,
    #[serde(rename = "arg", default)]
    args: Vec<Arg>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Method<'_> {
    /// Return the method name.
    pub fn name(&self) -> MemberName<'_> {
        self.name.as_ref()
    }

    /// Return the method arguments.
    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    /// Return the method annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// A signal
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Signal<'a> {
    #[serde(rename = "@name", borrow)]
    name: MemberName<'a>,

    #[serde(rename = "arg", default)]
    args: Vec<Arg>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Signal<'_> {
    /// Return the signal name.
    pub fn name(&self) -> MemberName<'_> {
        self.name.as_ref()
    }

    /// Return the signal arguments.
    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    /// Return the signal annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// The possible property access types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyAccess {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "readwrite")]
    ReadWrite,
}

impl PropertyAccess {
    pub fn read(&self) -> bool {
        matches!(self, PropertyAccess::Read | PropertyAccess::ReadWrite)
    }

    pub fn write(&self) -> bool {
        matches!(self, PropertyAccess::Write | PropertyAccess::ReadWrite)
    }
}

/// A property
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Property<'a> {
    #[serde(rename = "@name", borrow)]
    name: PropertyName<'a>,

    #[serde(rename = "@type")]
    ty: Signature,
    #[serde(rename = "@access")]
    access: PropertyAccess,

    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Property<'_> {
    /// Returns the property name.
    pub fn name(&self) -> PropertyName<'_> {
        self.name.as_ref()
    }

    /// Returns the property type.
    pub fn ty(&self) -> &Signature {
        &self.ty
    }

    /// Returns the property access flags (should be "read", "write" or "readwrite").
    pub fn access(&self) -> PropertyAccess {
        self.access
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An interface
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Interface<'a> {
    #[serde(rename = "@name", borrow)]
    name: InterfaceName<'a>,

    #[serde(rename = "method", default)]
    methods: Vec<Method<'a>>,
    #[serde(rename = "property", default)]
    properties: Vec<Property<'a>>,
    #[serde(rename = "signal", default)]
    signals: Vec<Signal<'a>>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl<'a> Interface<'a> {
    /// Returns the interface name.
    pub fn name(&self) -> InterfaceName<'_> {
        self.name.as_ref()
    }

    /// Returns the interface methods.
    pub fn methods(&self) -> &[Method<'a>] {
        &self.methods
    }

    /// Returns the interface signals.
    pub fn signals(&self) -> &[Signal<'a>] {
        &self.signals
    }

    /// Returns the interface properties.
    pub fn properties(&self) -> &[Property<'_>] {
        &self.properties
    }

    /// Return the associated annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }
}

/// An introspection tree node (typically the root of the XML document).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Node<'a> {
    #[serde(rename = "@name")]
    name: Option<String>,

    #[serde(rename = "interface", default, borrow)]
    interfaces: Vec<Interface<'a>>,
    #[serde(rename = "node", default, borrow)]
    nodes: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    /// Parse the introspection XML document from reader.
    pub fn from_reader<R: Read>(reader: R) -> Result<Node<'a>> {
        let mut deserializer = Deserializer::from_reader(BufReader::new(reader));
        deserializer.event_buffer_size(Some(4096_usize.try_into().unwrap()));
        Ok(Node::deserialize(&mut deserializer)?)
    }

    /// Write the XML document to writer.
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<()> {
        // Need this wrapper until this is resolved: https://github.com/tafia/quick-xml/issues/499
        struct Writer<T>(T);

        impl<T> std::fmt::Write for Writer<T>
        where
            T: Write,
        {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
            }
        }

        to_writer(Writer(writer), &self)?;

        Ok(())
    }

    /// Returns the node name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the children nodes.
    pub fn nodes(&self) -> &[Node<'a>] {
        &self.nodes
    }

    /// Returns the interfaces on this node.
    pub fn interfaces(&self) -> &[Interface<'a>] {
        &self.interfaces
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = Error;

    /// Parse the introspection XML document from `s`.
    fn try_from(s: &'a str) -> Result<Node<'a>> {
        let mut deserializer = Deserializer::from_str(s);
        deserializer.event_buffer_size(Some(4096_usize.try_into().unwrap()));
        Ok(Node::deserialize(&mut deserializer)?)
    }
}

/// A thin wrapper around `zvariant::parsed::Signature`.
///
/// This is to allow `Signature` to be deserialized from an owned string, which is what quick-xml2
/// deserializer does.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Signature(zvariant::Signature);

impl Signature {
    /// Return the inner `zvariant::Signature`.
    pub fn inner(&self) -> &zvariant::Signature {
        &self.0
    }

    /// Convert this `Signature` into the inner `zvariant::parsed::Signature`.
    pub fn into_inner(self) -> zvariant::Signature {
        self.0
    }
}

impl<'de> serde::de::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| {
            zvariant::Signature::try_from(s.as_bytes())
                .map_err(serde::de::Error::custom)
                .map(Signature)
        })
    }
}

impl Deref for Signature {
    type Target = zvariant::Signature;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl PartialEq<str> for Signature {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
