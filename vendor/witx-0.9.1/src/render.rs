use crate::ast::*;
use std::fmt;

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.typenames() {
            write!(f, "{}\n", d.to_sexpr())?;
        }
        for m in self.modules() {
            write!(f, "{}\n", m.to_sexpr())?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpr {
    Vec(Vec<SExpr>),
    Word(String),
    Ident(String),
    Quote(String),
    /// Short for Annotation
    Annot(String),
    /// Doc comment
    Docs(String, Box<SExpr>),
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExpr::Vec(vs) => {
                write!(f, "(")?;
                let mut vss = Vec::new();
                for v in vs {
                    vss.push(format!("{}", v));
                }
                f.write_str(&vss.join(" "))?;
                write!(f, ")")
            }
            SExpr::Word(w) => write!(f, "{}", w),
            SExpr::Ident(i) => write!(f, "${}", i),
            SExpr::Quote(q) => write!(f, "\"{}\"", q),
            SExpr::Annot(a) => write!(f, "@{}", a),
            SExpr::Docs(d, s) => write!(f, "(;; {} ;) {}", d, s),
        }
    }
}

impl SExpr {
    pub fn word(s: &str) -> SExpr {
        SExpr::Word(s.to_string())
    }
    pub fn ident(s: &str) -> SExpr {
        SExpr::Ident(s.to_string())
    }
    pub fn quote(s: &str) -> SExpr {
        SExpr::Quote(s.to_string())
    }
    pub fn annot(s: &str) -> SExpr {
        SExpr::Annot(s.to_string())
    }
    pub fn docs(d: &str, s: SExpr) -> SExpr {
        if d.is_empty() {
            s
        } else {
            SExpr::Docs(d.to_string(), Box::new(s))
        }
    }
}

impl Id {
    pub fn to_sexpr(&self) -> SExpr {
        SExpr::ident(self.as_str())
    }
}

impl BuiltinType {
    pub fn to_sexpr(&self) -> SExpr {
        match self {
            BuiltinType::Char => SExpr::word("char"),
            BuiltinType::U8 { lang_c_char: true } => {
                SExpr::Vec(vec![SExpr::annot("witx"), SExpr::word("char8")])
            }
            BuiltinType::U8 { lang_c_char: false } => SExpr::word("u8"),
            BuiltinType::U16 => SExpr::word("u16"),
            BuiltinType::U32 {
                lang_ptr_size: false,
            } => SExpr::word("u32"),
            BuiltinType::U32 {
                lang_ptr_size: true,
            } => SExpr::Vec(vec![SExpr::annot("witx"), SExpr::word("usize")]),
            BuiltinType::U64 => SExpr::word("u64"),
            BuiltinType::S8 => SExpr::word("s8"),
            BuiltinType::S16 => SExpr::word("s16"),
            BuiltinType::S32 => SExpr::word("s32"),
            BuiltinType::S64 => SExpr::word("s64"),
            BuiltinType::F32 => SExpr::word("f32"),
            BuiltinType::F64 => SExpr::word("f64"),
        }
    }
}

impl NamedType {
    pub fn to_sexpr(&self) -> SExpr {
        let body = self.tref.to_sexpr();
        SExpr::docs(
            &self.docs,
            SExpr::Vec(vec![SExpr::word("typename"), self.name.to_sexpr(), body]),
        )
    }
}

impl TypeRef {
    pub fn to_sexpr(&self) -> SExpr {
        match self {
            TypeRef::Name(n) => n.name.to_sexpr(),
            TypeRef::Value(v) => v.to_sexpr(),
        }
    }
}

impl Type {
    pub fn to_sexpr(&self) -> SExpr {
        match self {
            Type::Record(a) => a.to_sexpr(),
            Type::Variant(a) => a.to_sexpr(),
            Type::Handle(a) => a.to_sexpr(),
            Type::List(a) => SExpr::Vec(vec![SExpr::word("list"), a.to_sexpr()]),
            Type::Pointer(p) => SExpr::Vec(vec![
                SExpr::annot("witx"),
                SExpr::word("pointer"),
                p.to_sexpr(),
            ]),
            Type::ConstPointer(p) => SExpr::Vec(vec![
                SExpr::annot("witx"),
                SExpr::word("const_pointer"),
                p.to_sexpr(),
            ]),
            Type::Builtin(b) => b.to_sexpr(),
        }
    }
}

impl RecordDatatype {
    pub fn to_sexpr(&self) -> SExpr {
        match self.kind {
            RecordKind::Tuple => {
                let mut tuple = vec![SExpr::word("tuple")];
                for m in self.members.iter() {
                    tuple.push(SExpr::docs(&m.docs, m.tref.to_sexpr()));
                }
                SExpr::Vec(tuple)
            }
            RecordKind::Bitflags(repr) => {
                let mut flags = vec![SExpr::word("flags")];
                flags.push(SExpr::Vec(vec![
                    SExpr::word("@witx"),
                    SExpr::word("repr"),
                    repr.to_sexpr(),
                ]));
                flags.extend(
                    self.members
                        .iter()
                        .map(|m| SExpr::docs(&m.docs, m.name.to_sexpr())),
                );
                SExpr::Vec(flags)
            }
            RecordKind::Other => {
                let header = vec![SExpr::word("record")];
                let members = self
                    .members
                    .iter()
                    .map(|m| {
                        SExpr::docs(
                            &m.docs,
                            SExpr::Vec(vec![
                                SExpr::word("field"),
                                m.name.to_sexpr(),
                                m.tref.to_sexpr(),
                            ]),
                        )
                    })
                    .collect::<Vec<SExpr>>();
                SExpr::Vec([header, members].concat())
            }
        }
    }
}

impl Variant {
    pub fn to_sexpr(&self) -> SExpr {
        let mut list = Vec::new();
        if self.is_bool() {
            return SExpr::word("bool");
        } else if self.is_enum() {
            list.push(SExpr::word("enum"));
            list.push(SExpr::Vec(vec![
                SExpr::word("@witx"),
                SExpr::word("tag"),
                self.tag_repr.to_sexpr(),
            ]));
            for case in self.cases.iter() {
                list.push(SExpr::docs(&case.docs, case.name.to_sexpr()));
            }
        } else {
            list.push(SExpr::word("variant"));
            list.push(SExpr::Vec(vec![
                SExpr::word("@witx"),
                SExpr::word("tag"),
                self.tag_repr.to_sexpr(),
            ]));
            for case in self.cases.iter() {
                let mut case_expr = vec![SExpr::word("case"), case.name.to_sexpr()];
                if let Some(ty) = &case.tref {
                    case_expr.push(ty.to_sexpr());
                }
                list.push(SExpr::docs(&case.docs, SExpr::Vec(case_expr)));
            }
        }
        SExpr::Vec(list)
    }
}

impl HandleDatatype {
    pub fn to_sexpr(&self) -> SExpr {
        SExpr::Vec(vec![SExpr::word("handle")])
    }
}

impl IntRepr {
    pub fn to_sexpr(&self) -> SExpr {
        match self {
            IntRepr::U8 => SExpr::word("u8"),
            IntRepr::U16 => SExpr::word("u16"),
            IntRepr::U32 => SExpr::word("u32"),
            IntRepr::U64 => SExpr::word("u64"),
        }
    }
}

impl Module {
    pub fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("module"), self.name.to_sexpr()];
        let definitions = self
            .imports()
            .map(|i| i.to_sexpr())
            .chain(self.funcs().map(|f| f.to_sexpr()))
            .collect::<Vec<SExpr>>();
        SExpr::docs(&self.docs, SExpr::Vec([header, definitions].concat()))
    }
}

impl ModuleImport {
    pub fn to_sexpr(&self) -> SExpr {
        let variant = match self.variant {
            ModuleImportVariant::Memory => SExpr::Vec(vec![SExpr::word("memory")]),
        };
        SExpr::docs(
            &self.docs,
            SExpr::Vec(vec![
                SExpr::word("import"),
                SExpr::quote(self.name.as_str()),
                variant,
            ]),
        )
    }
}

impl InterfaceFunc {
    pub fn to_sexpr(&self) -> SExpr {
        let header = vec![
            SExpr::annot("interface"),
            SExpr::word("func"),
            SExpr::Vec(vec![
                SExpr::word("export"),
                SExpr::quote(self.name.as_str()),
            ]),
        ];
        let params = self
            .params
            .iter()
            .map(|f| {
                SExpr::docs(
                    &f.docs,
                    SExpr::Vec(vec![
                        SExpr::word("param"),
                        f.name.to_sexpr(),
                        f.tref.to_sexpr(),
                    ]),
                )
            })
            .collect();
        let results = self
            .results
            .iter()
            .map(|f| {
                SExpr::docs(
                    &f.docs,
                    SExpr::Vec(vec![
                        SExpr::word("result"),
                        f.name.to_sexpr(),
                        f.tref.to_sexpr(),
                    ]),
                )
            })
            .collect();
        let attrs = if self.noreturn {
            vec![SExpr::Vec(vec![
                SExpr::annot("witx"),
                SExpr::word("noreturn"),
            ])]
        } else {
            vec![]
        };
        SExpr::docs(
            &self.docs,
            SExpr::Vec([header, params, results, attrs].concat()),
        )
    }
}
