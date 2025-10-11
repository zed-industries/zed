#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorKind {
    Modify,
    Insert,
    Delete,
}

#[derive(Debug, Clone)]
pub struct ConnectorCurve {
    pub focus_line: usize,
    pub left_start: usize,
    pub left_end: usize,
    pub right_start: usize,
    pub right_end: usize,
    pub kind: ConnectorKind,
    pub left_crushed: bool,
    pub right_crushed: bool,
}

impl ConnectorCurve {
    pub fn new(
        focus_line: usize,
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
        kind: ConnectorKind,
        left_crushed: bool,
        right_crushed: bool,
    ) -> Self {
        Self {
            focus_line,
            left_start: left_start.min(left_end),
            left_end: left_end.max(left_start),
            right_start: right_start.min(right_end),
            right_end: right_end.max(right_start),
            kind,
            left_crushed,
            right_crushed,
        }
    }
}
