use std::collections::{hash_map, HashMap};

use itertools::Itertools;

use crate::ident::{to_snake, to_upper_camel};

fn validate_proto_path(path: &str) -> Result<(), String> {
    if path.chars().next().map(|c| c != '.').unwrap_or(true) {
        return Err(format!(
            "Protobuf paths must be fully qualified (begin with a leading '.'): {}",
            path
        ));
    }
    if path.split('.').skip(1).any(str::is_empty) {
        return Err(format!("invalid fully-qualified Protobuf path: {}", path));
    }
    Ok(())
}

#[derive(Debug)]
pub struct ExternPaths {
    extern_paths: HashMap<String, String>,
}

impl ExternPaths {
    pub fn new(paths: &[(String, String)], prost_types: bool) -> Result<ExternPaths, String> {
        let mut extern_paths = ExternPaths {
            extern_paths: HashMap::new(),
        };

        for (proto_path, rust_path) in paths {
            extern_paths.insert(proto_path.clone(), rust_path.clone())?;
        }

        if prost_types {
            extern_paths.insert(".google.protobuf".to_string(), "::prost_types".to_string())?;
            extern_paths.insert(".google.protobuf.BoolValue".to_string(), "bool".to_string())?;
            extern_paths.insert(
                ".google.protobuf.BytesValue".to_string(),
                "::prost::alloc::vec::Vec<u8>".to_string(),
            )?;
            extern_paths.insert(
                ".google.protobuf.DoubleValue".to_string(),
                "f64".to_string(),
            )?;
            extern_paths.insert(".google.protobuf.Empty".to_string(), "()".to_string())?;
            extern_paths.insert(".google.protobuf.FloatValue".to_string(), "f32".to_string())?;
            extern_paths.insert(".google.protobuf.Int32Value".to_string(), "i32".to_string())?;
            extern_paths.insert(".google.protobuf.Int64Value".to_string(), "i64".to_string())?;
            extern_paths.insert(
                ".google.protobuf.StringValue".to_string(),
                "::prost::alloc::string::String".to_string(),
            )?;
            extern_paths.insert(
                ".google.protobuf.UInt32Value".to_string(),
                "u32".to_string(),
            )?;
            extern_paths.insert(
                ".google.protobuf.UInt64Value".to_string(),
                "u64".to_string(),
            )?;
        }

        Ok(extern_paths)
    }

    fn insert(&mut self, proto_path: String, rust_path: String) -> Result<(), String> {
        validate_proto_path(&proto_path)?;
        match self.extern_paths.entry(proto_path) {
            hash_map::Entry::Occupied(occupied) => {
                return Err(format!(
                    "duplicate extern Protobuf path: {}",
                    occupied.key()
                ));
            }
            hash_map::Entry::Vacant(vacant) => vacant.insert(rust_path),
        };
        Ok(())
    }

    pub fn resolve_ident(&self, pb_ident: &str) -> Option<String> {
        // protoc should always give fully qualified identifiers.
        assert_eq!(".", &pb_ident[..1]);

        if let Some(rust_path) = self.extern_paths.get(pb_ident) {
            return Some(rust_path.clone());
        }

        // TODO(danburkert): there must be a more efficient way to do this, maybe a trie?
        for (idx, _) in pb_ident.rmatch_indices('.') {
            if let Some(rust_path) = self.extern_paths.get(&pb_ident[..idx]) {
                let mut segments = pb_ident[idx + 1..].split('.');
                let ident_type = segments.next_back().map(|segment| to_upper_camel(&segment));

                return Some(
                    rust_path
                        .split("::")
                        .chain(segments)
                        .enumerate()
                        .map(|(idx, segment)| {
                            if idx == 0 && segment == "crate" {
                                // If the first segment of the path is 'crate', then do not escape
                                // it into a raw identifier, since it's being used as the keyword.
                                segment.to_owned()
                            } else {
                                to_snake(&segment)
                            }
                        })
                        .chain(ident_type.into_iter())
                        .join("::"),
                );
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extern_paths() {
        let paths = ExternPaths::new(
            &[
                (".foo".to_string(), "::foo1".to_string()),
                (".foo.bar".to_string(), "::foo2".to_string()),
                (".foo.baz".to_string(), "::foo3".to_string()),
                (".foo.Fuzz".to_string(), "::foo4::Fuzz".to_string()),
                (".a.b.c.d.e.f".to_string(), "::abc::def".to_string()),
            ],
            false,
        )
        .unwrap();

        let case = |proto_ident: &str, resolved_ident: &str| {
            assert_eq!(paths.resolve_ident(proto_ident).unwrap(), resolved_ident);
        };

        case(".foo", "::foo1");
        case(".foo.Foo", "::foo1::Foo");
        case(".foo.bar", "::foo2");
        case(".foo.Bas", "::foo1::Bas");

        case(".foo.bar.Bar", "::foo2::Bar");
        case(".foo.Fuzz.Bar", "::foo4::fuzz::Bar");

        case(".a.b.c.d.e.f", "::abc::def");
        case(".a.b.c.d.e.f.g.FooBar.Baz", "::abc::def::g::foo_bar::Baz");

        assert!(paths.resolve_ident(".a").is_none());
        assert!(paths.resolve_ident(".a.b").is_none());
        assert!(paths.resolve_ident(".a.c").is_none());
    }

    #[test]
    fn test_well_known_types() {
        let paths = ExternPaths::new(&[], true).unwrap();

        let case = |proto_ident: &str, resolved_ident: &str| {
            assert_eq!(paths.resolve_ident(proto_ident).unwrap(), resolved_ident);
        };

        case(".google.protobuf.Value", "::prost_types::Value");
        case(".google.protobuf.Duration", "::prost_types::Duration");
        case(".google.protobuf.Empty", "()");
    }
}
