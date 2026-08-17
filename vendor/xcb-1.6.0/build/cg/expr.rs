use super::r#enum::{map_enum_item_name, map_mask_item_name};
use super::TypeInfo;
use crate::cg::{self, CodeGen, Field, QualifiedRsTyp};
use crate::ir;

// use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum Expr {
    FieldRef(String),
    ParamRef(String),
    EnumRef {
        module: Option<String>,
        rs_typ: String,
        item: String,
    },
    MaskRef {
        module: Option<String>,
        rs_typ: String,
        item: String,
    },
    Value(usize),
    Op(String, Box<Expr>, Box<Expr>),
    Unop(String, Box<Expr>),
    Popcount(Box<Expr>),
    SumOf(String, Option<Box<Expr>>),
    ListElementRef,
    AlignPad(usize, Box<Expr>),
    UntilEnd,
    Unknown(String),
}

impl CodeGen {
    pub(super) fn resolve_expr(&self, expr: &ir::Expr) -> Expr {
        match expr {
            ir::Expr::Value(size) => Expr::Value(*size),
            ir::Expr::FieldRef(name) => Expr::FieldRef(cg::rust_field_name(name)),
            ir::Expr::ParamRef(name) => Expr::ParamRef(cg::rust_field_name(name)),
            ir::Expr::EnumRef { name, item } => {
                let typinfo = self.find_typinfo(None, name);
                match typinfo {
                    TypeInfo::Enum { module, rs_typ, .. } => Expr::EnumRef {
                        module: module.clone(),
                        rs_typ: rs_typ.clone(),
                        item: map_enum_item_name(item),
                    },
                    TypeInfo::Mask { module, rs_typ, .. } => Expr::MaskRef {
                        module: module.clone(),
                        rs_typ: rs_typ.clone(),
                        item: map_mask_item_name(item),
                    },
                    _ => unreachable!(),
                }
            }
            ir::Expr::SumOf(name, expr) => Expr::SumOf(
                cg::rust_field_name(name),
                expr.as_ref().map(|e| Box::new(self.resolve_expr(e))),
            ),
            ir::Expr::Popcount(expr) => Expr::Popcount(Box::new(self.resolve_expr(expr))),
            ir::Expr::Op(op, lhs, rhs) => Expr::Op(
                op.clone(),
                Box::new(self.resolve_expr(lhs)),
                Box::new(self.resolve_expr(rhs)),
            ),
            ir::Expr::Unop(op, rhs) if op == "~" => {
                Expr::Unop("!".to_string(), Box::new(self.resolve_expr(rhs)))
            }
            ir::Expr::Unop(op, rhs) => Expr::Unop(op.clone(), Box::new(self.resolve_expr(rhs))),
            ir::Expr::ListElementRef => Expr::ListElementRef,
        }
    }

    /// Build a Rust expression. The type of the expr is always usize.
    pub(super) fn build_rs_expr(
        &self,
        expr: &Expr,
        acc_pref: &str,
        acc_post: &str,
        fields: &[Field],
    ) -> String {
        match expr {
            Expr::Value(val) => format!("{}usize", val),
            Expr::FieldRef(name) => {
                let is_mask = fields
                    .iter()
                    .find_map(|f| match f {
                        Field::Field {
                            name: n, is_mask, ..
                        } if n == name => Some(*is_mask),
                        _ => None,
                    })
                    .unwrap_or(false);
                format!(
                    "({}{}{}{} as usize)",
                    acc_pref,
                    name,
                    acc_post,
                    if is_mask { ".bits()" } else { "" }
                )
            }
            Expr::ParamRef(name) => name.clone(),
            Expr::Op(op, lhs, rhs) => {
                format!(
                    "({} {} {})",
                    self.build_rs_expr(lhs, acc_pref, acc_post, fields),
                    op,
                    self.build_rs_expr(rhs, acc_pref, acc_post, fields),
                )
            }
            Expr::Unop(op, rhs) => {
                format!(
                    "({}{})",
                    op,
                    self.build_rs_expr(rhs, acc_pref, acc_post, fields)
                )
            }
            Expr::Popcount(expr) => {
                format!(
                    "({}.count_ones() as usize)",
                    self.build_rs_expr(expr, acc_pref, acc_post, fields)
                )
            }
            Expr::EnumRef {
                module,
                rs_typ,
                item,
            } => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                format!("({}::{} as usize)", q_rs_typ, item)
            }
            Expr::MaskRef {
                module,
                rs_typ,
                item,
            } => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                format!("({}::{}.bits() as usize)", q_rs_typ, item)
            }
            Expr::AlignPad(sz, expr) => {
                format!(
                    "base::align_pad({}, {})",
                    self.build_rs_expr(expr, acc_pref, acc_post, fields),
                    sz
                )
            }
            Expr::SumOf(name, None) => {
                format!(
                    "({}{}{}.iter().sum::<u8>() as usize)",
                    acc_pref, name, acc_post
                )
            }
            Expr::SumOf(name, Some(fr)) => match &**fr {
                Expr::FieldRef(fname) => {
                    format!(
                        "({}{}{}.iter().map(|el| el.{}()).sum::<u8>() as usize)",
                        acc_pref, name, acc_post, fname
                    )
                }
                Expr::Popcount(expr) => {
                    if matches!(&**expr, Expr::ListElementRef) {
                        // count_ones returns u32 for all implementations
                        format!(
                            "({}{}{}.iter().map(|el| el.count_ones()).sum::<u32>() as usize)",
                            acc_pref, name, acc_post
                        )
                    } else {
                        unreachable!("{:#?}", expr);
                    }
                }
                _ => unreachable!("{:#?}", fr),
            },
            Expr::UntilEnd => unreachable!("UntilEnd must be handled up-front"),
            Expr::Unknown(tag) => {
                format!("(unimplemented!(\"{} expressions\") as usize)", tag)
            }
            ex => {
                format!("(unimplemented!(\"{:?} expressions\") as usize)", ex)
            }
        }
    }
}

impl Expr {
    pub(super) fn fixed_length(&self) -> Option<usize> {
        match self {
            Expr::EnumRef { .. } => None, // FIXME: get the value of the enum item
            Expr::Value(val) => Some(*val),
            Expr::Popcount(ex) => ex.fixed_length().map(|sz| sz.count_ones() as _),
            Expr::Op(op, lhs, rhs) => match (lhs.fixed_length(), rhs.fixed_length()) {
                (Some(lhs), Some(rhs)) => match op.as_str() {
                    "+" => Some(lhs + rhs),
                    "-" => Some(lhs - rhs),
                    "*" => Some(lhs * rhs),
                    "/" => Some(lhs / rhs),
                    _ => panic!("Unexpected binary operator in Expr: {}", op),
                },
                _ => None,
            },
            Expr::Unop(op, val) => val.fixed_length().map(|val| match op.as_str() {
                "!" => !val,
                _ => panic!("Unexpected unary operator in Expr: {}", op),
            }),
            Expr::AlignPad(align, expr) => expr.fixed_length().map(|val| align_pad(val, *align)),
            _ => None,
        }
    }

    // pub(super) fn len_field(&self) -> Option<&str> {
    //     match self {
    //         Expr::FieldRef(name) => Some(name),
    //         Expr::Op(_, lhs, rhs) => lhs.len_field().or_else(|| rhs.len_field()),
    //         Expr::Unop(_, rhs) => rhs.len_field(),
    //         Expr::Popcount(e) => e.len_field(),
    //         _ => None,
    //     }
    // }

    // pub(super) fn depth(&self) -> u32 {
    //     match self {
    //         Expr::Value(_) => 0,
    //         Expr::FieldRef(_) => 1,
    //         Expr::ParamRef(_) => 1,
    //         Expr::Op(_, lhs, rhs) =>
    //             lhs.depth().max(rhs.depth()) + 1,

    //         Expr::Unop(_, rhs) =>
    //             rhs.depth() + 1,

    //         Expr::Popcount(expr) => expr.depth() + 1,
    //         Expr::SumOf(_, expr) => expr.as_ref().map(|e| e.depth()).unwrap_or(0) + 1,
    //         Expr::EnumRef{..} => 1,
    //         Expr::ListElementRef => 1,

    //         Expr::AlignUp(_, expr) => expr.depth() + 1,
    //         Expr::Unknown(_) => 1,
    //     }
    // }

    pub(super) fn reduce(self) -> Expr {
        match self {
            Expr::Value(..) => self,
            expr => {
                if let Some(len) = expr.fixed_length() {
                    Expr::Value(len)
                } else if let Expr::Op(op, lhs, rhs) = expr {
                    match (op.as_str(), lhs.fixed_length(), rhs.fixed_length()) {
                        ("*", Some(lhs), _) if lhs == 1 => *rhs,
                        ("*", _, Some(rhs)) if rhs == 1 => *lhs,
                        ("+", Some(lhs), _) if lhs == 0 => *rhs,
                        ("+", _, Some(rhs)) if rhs == 0 => *lhs,
                        ("/", _, Some(rhs)) if rhs == 1 => *lhs,
                        ("-", _, Some(rhs)) if rhs == 0 => *lhs,
                        _ => Expr::Op(op, lhs, rhs),
                    }
                } else {
                    expr
                }
            }
        }
    }

    // pub(super) fn has_fieldref(&self, fieldref: &str) -> bool {
    //     match self {
    //         Expr::FieldRef(name) => name == fieldref,
    //         Expr::Op(_, lhs, rhs) => lhs.has_fieldref(fieldref) || rhs.has_fieldref(fieldref),
    //         Expr::Unop(_, rhs) => rhs.has_fieldref(fieldref),
    //         Expr::SumOf(name, _) => name == fieldref,
    //         Expr::Popcount(expr) => expr.has_fieldref(fieldref),
    //         Expr::AlignUp(_, expr) => expr.has_fieldref(fieldref),
    //         _ => false,
    //     }
    // }

    pub(super) fn fetch_fieldrefs<'a>(&'a self, fieldrefs: &mut Vec<&'a str>) {
        match self {
            Expr::FieldRef(name) => {
                fieldrefs.push(name.as_str());
            }
            Expr::Op(_, lhs, rhs) => {
                lhs.fetch_fieldrefs(fieldrefs);
                rhs.fetch_fieldrefs(fieldrefs);
            }
            Expr::Unop(_, rhs) => {
                rhs.fetch_fieldrefs(fieldrefs);
            }
            Expr::SumOf(name, _) => {
                fieldrefs.push(name.as_str());
            }
            Expr::Popcount(expr) => {
                expr.fetch_fieldrefs(fieldrefs);
            }
            Expr::AlignPad(_, expr) => {
                expr.fetch_fieldrefs(fieldrefs);
            }
            _ => {}
        }
    }

    pub(super) fn fetch_fieldrefs_owned(&self, fieldrefs: &mut Vec<String>) {
        match self {
            Expr::FieldRef(name) => {
                fieldrefs.push(name.clone());
            }
            Expr::Op(_, lhs, rhs) => {
                lhs.fetch_fieldrefs_owned(fieldrefs);
                rhs.fetch_fieldrefs_owned(fieldrefs);
            }
            Expr::Unop(_, rhs) => {
                rhs.fetch_fieldrefs_owned(fieldrefs);
            }
            Expr::SumOf(name, _) => {
                fieldrefs.push(name.clone());
            }
            Expr::Popcount(expr) => {
                expr.fetch_fieldrefs_owned(fieldrefs);
            }
            Expr::AlignPad(_, expr) => {
                expr.fetch_fieldrefs_owned(fieldrefs);
            }
            _ => {}
        }
    }

    pub(crate) fn fieldrefs(&self) -> Vec<&str> {
        let mut fieldrefs = Vec::new();
        self.fetch_fieldrefs(&mut fieldrefs);
        fieldrefs
    }

    // pub(crate) fn fieldrefs_str(&self) -> String {
    //     let fields = self.fieldrefs();
    //     let mut field_str = String::new();
    //     for f in fields {
    //         field_str.push_str(", ");
    //         field_str.push_str(f);
    //         field_str.push_str(": usize");
    //     }
    //     field_str
    // }

    fn fetch_params<'a>(&'a self, params: &mut Vec<&'a str>) {
        match self {
            Expr::ParamRef(name) => {
                params.push(name.as_str());
            }
            Expr::Op(_, lhs, rhs) => {
                lhs.fetch_params(params);
                rhs.fetch_params(params);
            }
            Expr::Unop(_, rhs) => {
                rhs.fetch_params(params);
            }
            Expr::SumOf(_, Some(expr)) => {
                expr.fetch_params(params);
            }
            Expr::Popcount(expr) => {
                expr.fetch_params(params);
            }
            Expr::AlignPad(_, expr) => {
                expr.fetch_params(params);
            }
            _ => {}
        }
    }

    pub(super) fn fetch_paramrefs_owned(&self, params: &mut Vec<String>) {
        match self {
            Expr::ParamRef(name) => {
                params.push(name.clone());
            }
            Expr::Op(_, lhs, rhs) => {
                lhs.fetch_paramrefs_owned(params);
                rhs.fetch_paramrefs_owned(params);
            }
            Expr::Unop(_, rhs) => {
                rhs.fetch_paramrefs_owned(params);
            }
            Expr::SumOf(_, Some(expr)) => {
                expr.fetch_paramrefs_owned(params);
            }
            Expr::Popcount(expr) => {
                expr.fetch_paramrefs_owned(params);
            }
            Expr::AlignPad(_, expr) => {
                expr.fetch_paramrefs_owned(params);
            }
            _ => {}
        }
    }

    pub(crate) fn params(&self) -> Vec<&str> {
        let mut params = Vec::new();
        self.fetch_params(&mut params);
        params
    }

    pub(crate) fn params_str(&self) -> String {
        let params = self.params();
        let mut param_str = String::new();
        for p in params {
            param_str.push_str(", ");
            param_str.push_str(p);
            param_str.push_str(": usize");
        }
        param_str
    }
}

fn align_pad(base: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two(), "`align` must be a power of two");

    let base = base as isize;
    let align = align as isize;
    (-base & (align - 1)) as usize
}
