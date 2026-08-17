#[derive(Debug, Clone)]
pub(crate) struct Note {
    pub position: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ClickStmt {
    pub id: String,
    pub url: String,
    pub tooltip: String,
}

#[derive(Debug, Clone)]
pub(crate) struct StateStmt {
    pub id: String,
    pub ty: String,
    pub description: Option<String>,
    pub descriptions: Vec<String>,
    pub doc: Option<Vec<Stmt>>,
    pub note: Option<Note>,
    pub classes: Vec<String>,
    pub styles: Vec<String>,
    pub text_styles: Vec<String>,
    pub start: Option<bool>,
}

impl StateStmt {
    pub(crate) fn new(id: String) -> Self {
        Self {
            id,
            ty: "default".to_string(),
            description: None,
            descriptions: Vec::new(),
            doc: None,
            note: None,
            classes: Vec::new(),
            styles: Vec::new(),
            text_styles: Vec::new(),
            start: None,
        }
    }

    pub(crate) fn new_typed(id: String, ty: &str) -> Self {
        Self {
            ty: ty.to_string(),
            ..Self::new(id)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RelationStmt {
    pub state1: StateStmt,
    pub state2: StateStmt,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Noop,
    State(StateStmt),
    Relation(Box<RelationStmt>),
    ClassDef { id: String, classes: String },
    ApplyClass { ids: String, class_name: String },
    Style { ids: String, styles: String },
    Direction(String),
    AccTitle(String),
    AccDescr(String),
    Click(ClickStmt),
}
