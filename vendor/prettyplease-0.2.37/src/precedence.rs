use syn::{
    AttrStyle, Attribute, BinOp, Expr, ExprArray, ExprAsync, ExprAwait, ExprBlock, ExprBreak,
    ExprCall, ExprConst, ExprContinue, ExprField, ExprForLoop, ExprIf, ExprIndex, ExprInfer,
    ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRepeat,
    ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprUnsafe, ExprWhile, ExprYield,
    ReturnType,
};

// Reference: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    // return, break, closures
    Jump,
    // = += -= *= /= %= &= |= ^= <<= >>=
    Assign,
    // .. ..=
    Range,
    // ||
    Or,
    // &&
    And,
    // let
    Let,
    // == != < > <= >=
    Compare,
    // |
    BitOr,
    // ^
    BitXor,
    // &
    BitAnd,
    // << >>
    Shift,
    // + -
    Sum,
    // * / %
    Product,
    // as
    Cast,
    // unary - * ! & &mut
    Prefix,
    // paths, loops, function calls, array indexing, field expressions, method calls
    Unambiguous,
}

impl Precedence {
    pub(crate) const MIN: Self = Precedence::Jump;

    pub(crate) fn of_binop(op: &BinOp) -> Self {
        match op {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            BinOp::Add(_) | BinOp::Sub(_) => Precedence::Sum,
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => Precedence::Product,
            BinOp::And(_) => Precedence::And,
            BinOp::Or(_) => Precedence::Or,
            BinOp::BitXor(_) => Precedence::BitXor,
            BinOp::BitAnd(_) => Precedence::BitAnd,
            BinOp::BitOr(_) => Precedence::BitOr,
            BinOp::Shl(_) | BinOp::Shr(_) => Precedence::Shift,

            BinOp::Eq(_)
            | BinOp::Lt(_)
            | BinOp::Le(_)
            | BinOp::Ne(_)
            | BinOp::Ge(_)
            | BinOp::Gt(_) => Precedence::Compare,

            BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_) => Precedence::Assign,

            _ => Precedence::MIN,
        }
    }

    pub(crate) fn of(e: &Expr) -> Self {
        fn prefix_attrs(attrs: &[Attribute]) -> Precedence {
            for attr in attrs {
                if let AttrStyle::Outer = attr.style {
                    return Precedence::Prefix;
                }
            }
            Precedence::Unambiguous
        }

        match e {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Expr::Closure(e) => match e.output {
                ReturnType::Default => Precedence::Jump,
                ReturnType::Type(..) => prefix_attrs(&e.attrs),
            },

            Expr::Break(ExprBreak { expr, .. })
            | Expr::Return(ExprReturn { expr, .. })
            | Expr::Yield(ExprYield { expr, .. }) => match expr {
                Some(_) => Precedence::Jump,
                None => Precedence::Unambiguous,
            },

            Expr::Assign(_) => Precedence::Assign,
            Expr::Range(_) => Precedence::Range,
            Expr::Binary(e) => Precedence::of_binop(&e.op),
            Expr::Let(_) => Precedence::Let,
            Expr::Cast(_) => Precedence::Cast,
            Expr::RawAddr(_) | Expr::Reference(_) | Expr::Unary(_) => Precedence::Prefix,

            Expr::Array(ExprArray { attrs, .. })
            | Expr::Async(ExprAsync { attrs, .. })
            | Expr::Await(ExprAwait { attrs, .. })
            | Expr::Block(ExprBlock { attrs, .. })
            | Expr::Call(ExprCall { attrs, .. })
            | Expr::Const(ExprConst { attrs, .. })
            | Expr::Continue(ExprContinue { attrs, .. })
            | Expr::Field(ExprField { attrs, .. })
            | Expr::ForLoop(ExprForLoop { attrs, .. })
            | Expr::If(ExprIf { attrs, .. })
            | Expr::Index(ExprIndex { attrs, .. })
            | Expr::Infer(ExprInfer { attrs, .. })
            | Expr::Lit(ExprLit { attrs, .. })
            | Expr::Loop(ExprLoop { attrs, .. })
            | Expr::Macro(ExprMacro { attrs, .. })
            | Expr::Match(ExprMatch { attrs, .. })
            | Expr::MethodCall(ExprMethodCall { attrs, .. })
            | Expr::Paren(ExprParen { attrs, .. })
            | Expr::Path(ExprPath { attrs, .. })
            | Expr::Repeat(ExprRepeat { attrs, .. })
            | Expr::Struct(ExprStruct { attrs, .. })
            | Expr::Try(ExprTry { attrs, .. })
            | Expr::TryBlock(ExprTryBlock { attrs, .. })
            | Expr::Tuple(ExprTuple { attrs, .. })
            | Expr::Unsafe(ExprUnsafe { attrs, .. })
            | Expr::While(ExprWhile { attrs, .. }) => prefix_attrs(attrs),

            Expr::Group(e) => Precedence::of(&e.expr),

            Expr::Verbatim(_) => Precedence::Unambiguous,

            _ => Precedence::Unambiguous,
        }
    }
}
