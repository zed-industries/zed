#[derive(Clone, Debug)]
pub struct GraphLine {
    pub from_lane: usize,
    pub to_lane: usize,
    pub line_type: LineType,
    pub color_idx: usize,
    pub continues_from_above: bool,
    pub ends_at_commit: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Straight,
    MergeDown,
    BranchOut,
}
