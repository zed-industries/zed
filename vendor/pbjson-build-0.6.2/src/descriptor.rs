//! This module contains code to parse and extract the protobuf descriptor
//! format for use by the rest of the codebase

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind, Result};

use itertools::{EitherOrBoth, Itertools};
use prost_types::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, FileDescriptorSet, MessageOptions, OneofDescriptorProto,
};

use crate::escape::{escape_ident, escape_type};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Package {
    path: Vec<TypeName>,
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path[0].to_snake_case_ident())?;
        for element in &self.path[1..self.path.len()] {
            write!(f, ".{}", element.to_snake_case_ident())?;
        }
        Ok(())
    }
}

impl Package {
    pub fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        assert!(
            !s.starts_with('.'),
            "package cannot start with \'.\', got \"{}\"",
            s
        );

        Self {
            path: s.split('.').map(TypeName::new).collect(),
        }
    }

    pub fn path(&self) -> &[TypeName] {
        self.path.as_slice()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TypeName(String);

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TypeName {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        assert!(
            !s.contains('.'),
            "type name cannot contain \'.\', got \"{}\"",
            s
        );
        Self(s)
    }

    pub fn to_snake_case_ident(&self) -> String {
        use heck::ToSnakeCase;
        escape_ident(self.0.to_snake_case())
    }

    pub fn to_upper_camel_case(&self) -> String {
        use heck::ToUpperCamelCase;
        escape_type(self.0.to_upper_camel_case())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TypePath {
    package: Package,
    path: Vec<TypeName>,
}

impl Display for TypePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.package.fmt(f)?;
        for element in &self.path {
            write!(f, ".{}", element)?;
        }
        Ok(())
    }
}

impl TypePath {
    pub fn new(package: Package) -> Self {
        Self {
            package,
            path: Default::default(),
        }
    }

    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Returns the length of the path
    pub fn len(&self) -> usize {
        self.path.len() + self.package.path.len()
    }

    /// Returns the full path
    pub fn path(&self) -> impl Iterator<Item = &'_ TypeName> + '_ {
        self.package.path().iter().chain(self.path.iter())
    }

    pub fn child(&self, name: TypeName) -> Self {
        let path = self
            .path
            .iter()
            .cloned()
            .chain(std::iter::once(name))
            .collect();

        Self {
            package: self.package.clone(),
            path,
        }
    }

    /// Performs a prefix match, returning the length of the match in path segments if any
    pub fn prefix_match(&self, prefix: &str) -> Option<usize> {
        let prefix = match prefix.strip_prefix('.') {
            Some(prefix) => prefix,
            None => panic!("prefix must start with a '.'"),
        };

        if prefix.is_empty() {
            return Some(0);
        }

        let mut match_len = 0_usize;
        let split = prefix.split('.');
        for zipped in self.path().zip_longest(split) {
            match zipped {
                EitherOrBoth::Both(a, b) if a.0.as_str() == b => match_len += 1,
                EitherOrBoth::Left(_) => return Some(match_len),
                _ => return None,
            }
        }
        Some(match_len)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DescriptorSet {
    descriptors: BTreeMap<TypePath, Descriptor>,
}

impl DescriptorSet {
    pub fn register_encoded(&mut self, encoded: &[u8]) -> Result<()> {
        let descriptors: FileDescriptorSet =
            prost::Message::decode(encoded).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        for file in descriptors.file {
            self.register_file_descriptor(file);
        }

        Ok(())
    }

    pub fn register_file_descriptor(&mut self, file: FileDescriptorProto) {
        let syntax = match file.syntax.as_deref() {
            None | Some("proto2") => Syntax::Proto2,
            Some("proto3") => Syntax::Proto3,
            Some(s) => panic!("unknown syntax: {}", s),
        };

        let package = Package::new(file.package.expect("expected package"));
        let path = TypePath::new(package);

        for descriptor in file.message_type {
            self.register_message(&path, descriptor, syntax)
        }

        for descriptor in file.enum_type {
            self.register_enum(&path, descriptor)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypePath, &Descriptor)> {
        self.descriptors.iter()
    }

    fn register_message(&mut self, path: &TypePath, descriptor: DescriptorProto, syntax: Syntax) {
        let name = TypeName::new(descriptor.name.expect("expected name"));
        let child_path = path.child(name);

        for child_descriptor in descriptor.enum_type {
            self.register_enum(&child_path, child_descriptor)
        }

        for child_descriptor in descriptor.nested_type {
            self.register_message(&child_path, child_descriptor, syntax)
        }

        self.register_descriptor(
            child_path.clone(),
            Descriptor::Message(MessageDescriptor {
                path: child_path,
                options: descriptor.options,
                one_of: descriptor.oneof_decl,
                fields: descriptor.field,
                syntax,
            }),
        );
    }

    fn register_enum(&mut self, path: &TypePath, descriptor: EnumDescriptorProto) {
        let name = TypeName::new(descriptor.name.expect("expected name"));
        self.register_descriptor(
            path.child(name),
            Descriptor::Enum(EnumDescriptor {
                values: descriptor.value,
            }),
        );
    }

    fn register_descriptor(&mut self, path: TypePath, descriptor: Descriptor) {
        match self.descriptors.entry(path) {
            Entry::Occupied(o) => panic!("descriptor already registered for {}", o.key()),
            Entry::Vacant(v) => v.insert(descriptor),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Proto2,
    Proto3,
}

#[derive(Debug, Clone)]
pub enum Descriptor {
    Enum(EnumDescriptor),
    Message(MessageDescriptor),
}

#[derive(Debug, Clone)]
pub struct EnumDescriptor {
    pub values: Vec<EnumValueDescriptorProto>,
}

#[derive(Debug, Clone)]
pub struct MessageDescriptor {
    pub path: TypePath,
    pub options: Option<MessageOptions>,
    pub one_of: Vec<OneofDescriptorProto>,
    pub fields: Vec<FieldDescriptorProto>,
    pub syntax: Syntax,
}

impl MessageDescriptor {
    /// Whether this is an auto-generated type for the map field
    pub fn is_map(&self) -> bool {
        self.options
            .as_ref()
            .and_then(|options| options.map_entry)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prefix_match() {
        let t = TypePath::new(Package::new("foo.bar"))
            .child(TypeName::new("Baz"))
            .child(TypeName::new("Bar"));

        assert_eq!(t.prefix_match("."), Some(0));
        assert_eq!(t.prefix_match(".."), None);
        assert_eq!(t.prefix_match(".foo"), Some(1));
        assert_eq!(t.prefix_match(".foo."), None);
        assert_eq!(t.prefix_match(".foo.bar"), Some(2));
        assert_eq!(t.prefix_match(".foo.bar.Bar"), None);
        assert_eq!(t.prefix_match(".foo.bar.Baz"), Some(3));
        assert_eq!(t.prefix_match(".foo.bar.Baza"), None);
        assert_eq!(t.prefix_match(".foo.bar.Baz.Bar"), Some(4));
        assert_eq!(t.prefix_match(".foo.bar.Baz.Bar.Boo"), None);
    }

    #[test]
    fn test_handle_camel_case_in_package() {
        assert_eq!(
            Package::new("fooBar.baz.boo").to_string(),
            String::from("foo_bar.baz.boo")
        )
    }

    #[test]
    fn escape_keywords() {
        assert_eq!(
            Package::new("type.abstract").to_string(),
            "r#type.r#abstract"
        );
    }
}
