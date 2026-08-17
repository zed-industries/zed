use std::fmt::{self, Display};

/// Path to the current value in the input, like `dependencies.serde.typo1`.
#[derive(Copy, Clone)]
pub enum Path<'a> {
    Root,
    Seq { parent: &'a Path<'a>, index: usize },
    Map { parent: &'a Path<'a>, key: &'a str },
    Alias { parent: &'a Path<'a> },
    Unknown { parent: &'a Path<'a> },
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        struct Parent<'a>(&'a Path<'a>);

        impl<'a> Display for Parent<'a> {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                match self.0 {
                    Path::Root => Ok(()),
                    path => write!(formatter, "{}.", path),
                }
            }
        }

        match self {
            Path::Root => formatter.write_str("."),
            Path::Seq { parent, index } => write!(formatter, "{}[{}]", parent, index),
            Path::Map { parent, key } => write!(formatter, "{}{}", Parent(parent), key),
            Path::Alias { parent } => write!(formatter, "{}", parent),
            Path::Unknown { parent } => write!(formatter, "{}?", Parent(parent)),
        }
    }
}
