use crate::descriptor::FileDescriptorProto;

/// `.proto` file syntax.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Syntax {
    /// `syntax = "proto2"`.
    Proto2,
    /// `syntax = "proto3"`.
    Proto3,
}

impl Syntax {
    pub(crate) fn parse(syntax: &str) -> Option<Syntax> {
        match syntax {
            "" | "proto2" => Some(Syntax::Proto2),
            "proto3" => Some(Syntax::Proto3),
            _ => None,
        }
    }

    pub(crate) fn of_file(file: &FileDescriptorProto) -> Syntax {
        Syntax::parse(file.syntax()).unwrap_or(Syntax::Proto2)
    }
}
