//! Intermediate representation module

#[derive(Debug, Clone)]
pub struct ExtInfo {
    pub name: String,
    pub xname: String,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct DocError {
    pub typ: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct DocSee {
    pub typ: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
    pub fields: Vec<DocField>,
    pub errors: Vec<DocError>,
    pub sees: Vec<DocSee>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    FieldRef(String),
    ParamRef(String),
    EnumRef { name: String, item: String },
    Value(usize),
    Op(String, Box<Expr>, Box<Expr>),
    Unop(String, Box<Expr>),
    Popcount(Box<Expr>),
    SumOf(String, Option<Box<Expr>>),
    ListElementRef,
}

#[derive(Debug, Clone)]
pub enum EnumItem {
    Bit(String, u32),
    Value(String, u32),
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub bit: bool,
    pub name: Option<String>,
    pub exprs: Vec<Expr>,
    pub fields: Vec<Field>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum Field {
    Pad(usize),
    AlignPad(usize),
    Field {
        name: String,
        typ: String,
        r#enum: Option<String>,
        mask: Option<String>,
        altenum: Option<String>,
        altmask: Option<String>,
    },
    List {
        name: String,
        typ: String,
        len_expr: Expr,
        r#enum: Option<String>,
        mask: Option<String>,
    },
    ListNoLen {
        name: String,
        typ: String,
        r#enum: Option<String>,
        mask: Option<String>,
    },
    Expr {
        name: String,
        typ: String,
        expr: Expr,
    },
    ValueParam {
        mask_typ: String,
        mask_name: String,
        list_name: String,
    },
    Fd(String),
    Switch(String, Expr, Vec<SwitchCase>),
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub fields: Vec<Field>,
    pub doc: Option<Doc>,
}

#[derive(Debug, Clone)]
pub struct EventSelector {
    pub extension: String,
    pub xge: bool,
    pub opcode_range: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum Item {
    Error {
        name: String,
        number: i32,
        fields: Vec<Field>,
        doc: Option<Doc>,
    },
    ErrorCopy {
        name: String,
        number: i32,
        r#ref: String,
    },
    Typedef {
        old_typ: String,
        new_typ: String,
    },
    XidType {
        typ: String,
    },
    XidUnion {
        typ: String,
        xidtypes: Vec<String>,
    },
    Enum {
        typ: String,
        items: Vec<EnumItem>,
        doc: Option<Doc>,
    },
    Struct {
        typ: String,
        fields: Vec<Field>,
        doc: Option<Doc>,
    },
    Union {
        typ: String,
        fields: Vec<Field>,
        doc: Option<Doc>,
    },
    Event {
        number: i32,
        name: String,
        fields: Vec<Field>,
        no_seq_number: bool,
        xge: bool,
        doc: Option<Doc>,
    },
    EventCopy {
        name: String,
        number: i32,
        r#ref: String,
    },
    EventStruct {
        typ: String,
        selectors: Vec<EventSelector>,
    },
    Request {
        name: String,
        opcode: u32,
        params: Vec<Field>,
        reply: Option<Reply>,
        doc: Option<Doc>,
    },
}
