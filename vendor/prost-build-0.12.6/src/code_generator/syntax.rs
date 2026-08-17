#[derive(PartialEq)]
pub(super) enum Syntax {
    Proto2,
    Proto3,
}
impl From<Option<&str>> for Syntax {
    fn from(optional_str: Option<&str>) -> Self {
        match optional_str {
            None | Some("proto2") => Syntax::Proto2,
            Some("proto3") => Syntax::Proto3,
            Some(s) => panic!("unknown syntax: {}", s),
        }
    }
}
