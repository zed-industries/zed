use super::{Edge, Node, SubgraphHeader};

#[derive(Debug, Clone)]
pub(crate) struct StyleStmt {
    pub target: String,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ClassDefStmt {
    pub ids: Vec<String>,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ClassAssignStmt {
    pub targets: Vec<String>,
    pub class_name: String,
}

#[derive(Debug, Clone)]
pub(crate) enum ClickAction {
    Callback,
    Link {
        href: String,
        target: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct ClickStmt {
    pub ids: Vec<String>,
    pub tooltip: Option<String>,
    pub action: ClickAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LinkStylePos {
    Default,
    Index(usize),
}

#[derive(Debug, Clone)]
pub(crate) struct LinkStyleStmt {
    pub positions: Vec<LinkStylePos>,
    pub interpolate: Option<String>,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct FlowchartAst {
    pub keyword: String,
    pub direction: Option<String>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) struct SubgraphBlock {
    pub header: SubgraphHeader,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Chain { nodes: Vec<Node>, edges: Vec<Edge> },
    Node(Box<Node>),
    Subgraph(SubgraphBlock),
    Direction(String),
    Style(StyleStmt),
    ClassDef(ClassDefStmt),
    ClassAssign(ClassAssignStmt),
    Click(ClickStmt),
    LinkStyle(LinkStyleStmt),
    ShapeData { target: String, yaml: String },
}
