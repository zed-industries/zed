use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonNumberLit(pub String);

impl fmt::Display for JsonNumberLit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
