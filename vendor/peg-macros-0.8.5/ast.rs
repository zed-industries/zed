use proc_macro2::{Group, Ident, Literal, Span, TokenStream};

#[derive(Debug)]
pub struct Grammar {
    pub doc: Option<TokenStream>,
    pub visibility: Option<TokenStream>,
    pub name: Ident,
    pub lifetime_params: Option<Vec<TokenStream>>,
    pub args: Vec<(Ident, TokenStream)>,
    pub items: Vec<Item>,
    pub input_type: TokenStream,
}

impl Grammar {
    pub fn iter_rules(&self) -> impl Iterator<Item = &Rule> {
        self.items.iter().filter_map(|item| match item {
            Item::Rule(r) => Some(r),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub enum Item {
    Use(TokenStream),
    Rule(Rule),
}

#[derive(Debug)]
pub enum Cache {
    Simple,
    Recursive
}

#[derive(Debug)]
pub struct Rule {
    pub span: Span,
    pub name: Ident,
    pub ty_params: Option<Vec<TokenStream>>,
    pub params: Vec<RuleParam>,
    pub expr: SpannedExpr,
    pub ret_type: Option<TokenStream>,
    pub where_clause: Option<TokenStream>,
    pub doc: Option<TokenStream>,
    pub visibility: Option<TokenStream>,
    pub cache: Option<Cache>,
    pub no_eof: bool,
}

#[derive(Debug)]
pub struct RuleParam {
    pub name: Ident,
    pub ty: RuleParamTy,
}

#[derive(Debug)]
pub enum RuleParamTy {
    Rust(TokenStream),
    Rule(TokenStream),
}

#[derive(Debug, Clone)]
pub struct TaggedExpr {
    pub name: Option<Ident>,
    pub expr: SpannedExpr,
}
#[derive(Debug, Clone)]
pub struct SpannedExpr {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    LiteralExpr(Literal),
    PatternExpr(Group),
    RuleExpr(Ident, Option<TokenStream>, Vec<RuleArg>),
    MethodExpr(Ident, TokenStream),
    CustomExpr(Group),
    ChoiceExpr(Vec<SpannedExpr>),
    OptionalExpr(Box<SpannedExpr>),
    Repeat { inner: Box<SpannedExpr>, bound: BoundedRepeat, sep: Option<Box<SpannedExpr>> },
    PosAssertExpr(Box<SpannedExpr>),
    NegAssertExpr(Box<SpannedExpr>),
    ActionExpr(Vec<TaggedExpr>, Option<Group>),
    MatchStrExpr(Box<SpannedExpr>),
    PositionExpr,
    QuietExpr(Box<SpannedExpr>),
    FailExpr(Group),
    PrecedenceExpr {
        levels: Vec<PrecedenceLevel>,
    },
    MarkerExpr(bool),
}

impl Expr {
    pub fn at(self, sp: Span) -> SpannedExpr {
        SpannedExpr { expr: self, span:sp }
    }
}

#[derive(Debug, Clone)]
pub enum RuleArg {
    Rust(TokenStream),
    Peg(SpannedExpr),
}

#[derive(Debug, Clone)]
pub struct PrecedenceLevel {
    pub operators: Vec<PrecedenceOperator>,
}

#[derive(Debug, Clone)]
pub struct PrecedenceOperator {
    pub span: Span,
    pub elements: Vec<TaggedExpr>,
    pub action: Group,
}

#[derive(Debug, Clone)]
pub enum BoundedRepeat {
    None,
    Plus,
    Exact(TokenStream),
    Both(Option<TokenStream>, Option<TokenStream>),
}

impl BoundedRepeat {
    pub fn has_lower_bound(&self) -> bool {
        match self {
            BoundedRepeat::None | BoundedRepeat::Both(None, _) => false,
            BoundedRepeat::Plus | BoundedRepeat::Exact(_) | BoundedRepeat::Both(Some(_), _) => true
        }
    }

    pub fn has_upper_bound(&self) -> bool {
        match self {
            BoundedRepeat::None |  BoundedRepeat::Plus | BoundedRepeat::Both(_, None) => false,
            BoundedRepeat::Exact(_) | BoundedRepeat::Both(_, Some(_)) => true
        }
    }
}
