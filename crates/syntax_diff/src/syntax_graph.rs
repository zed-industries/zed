use crate::SyntaxId;

pub struct SyntaxVertex {
    lhs: Option<SyntaxId>,
    rhs: Option<SyntaxId>,
}

pub enum SyntaxEdge {}
