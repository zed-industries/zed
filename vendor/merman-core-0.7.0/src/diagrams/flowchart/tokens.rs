use super::{
    ClassAssignStmt, ClassDefStmt, ClickStmt, LabeledText, LinkStyleStmt, LinkToken, StyleStmt,
    SubgraphHeader,
};

#[derive(Debug, Clone)]
pub(crate) enum Tok {
    KwGraph,
    KwFlowchart,
    KwFlowchartElk,
    KwSubgraph,
    KwEnd,

    Sep,
    Amp,
    StyleSep,
    NodeLabel(NodeLabelToken),

    Direction(String),
    DirectionStmt(String),
    Id(String),
    Arrow(LinkToken),
    EdgeLabel(LabeledText),
    SubgraphHeader(SubgraphHeader),

    StyleStmt(StyleStmt),
    ClassDefStmt(ClassDefStmt),
    ClassAssignStmt(ClassAssignStmt),
    ClickStmt(ClickStmt),
    LinkStyleStmt(LinkStyleStmt),

    EdgeId(String),
    ShapeData(String),
}

#[derive(Debug, Clone)]
pub(crate) struct NodeLabelToken {
    pub shape: String,
    pub text: LabeledText,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub(crate) struct LexError {
    pub message: String,
}
