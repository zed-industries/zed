pub(crate) enum Action {
    SetDirection(String),
    SetAccTitle(String),
    SetAccDescr(String),

    AddNamespace {
        id: String,
        label: Option<String>,
    },
    AddClassesToNamespace {
        namespace: String,
        class_ids: Vec<String>,
    },
    PopNamespace,

    AddClass {
        id: String,
    },
    SetClassLabel {
        id: String,
        label: String,
    },
    SetCssClass {
        ids: String,
        css_class: String,
    },
    SetCssStyle {
        id: String,
        raw: String,
    },
    DefineClass {
        id: String,
        raw: String,
    },
    SetLink {
        id: String,
        url: String,
        target: Option<String>,
    },
    SetTooltip {
        id: String,
        tooltip: String,
    },
    SetClickEvent {
        id: String,
        function: String,
        args: Option<String>,
    },
    AddMembers {
        id: String,
        members: Vec<String>,
    },
    AddMember {
        id: String,
        member: String,
    },
    AddAnnotation {
        id: String,
        annotation: String,
    },
    AddRelation {
        data: RelationData,
    },
    AddNote {
        class_id: Option<String>,
        text: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Relation {
    pub type1: i32,
    pub type2: i32,
    pub line_type: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct RelationData {
    pub id1: String,
    pub id2: String,
    pub relation: Relation,
    pub relation_title1: Option<String>,
    pub relation_title2: Option<String>,
    pub title: Option<String>,
}
