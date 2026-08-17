use super::{
    md::{MdFunc, MdHeading, MdNamedType, MdNodeRef, MdSection, ToMarkdown},
    Documentation,
};
use crate::{
    ast::*,
    layout::Layout,
    polyfill::{FuncPolyfill, ModulePolyfill, ParamPolyfill, Polyfill, TypePolyfill},
    RepEquality,
};
use std::collections::HashMap;

fn heading_from_node(node: &MdNodeRef, levels_down: usize) -> MdHeading {
    MdHeading::new_header(node.borrow().ancestors().len() + levels_down)
}

impl ToMarkdown for Document {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        let types = node.new_child(MdSection::new(heading, "Types"));

        let mut constants_by_name = HashMap::new();
        for c in self.constants() {
            constants_by_name.entry(&c.ty).or_insert(Vec::new()).push(c);
        }

        for d in self.typenames() {
            let name = d.name.as_str();
            let child = types.new_child(MdNamedType::new(
                heading.new_level_down(),
                name,
                name,
                format!(
                    "{}\nSize: {}\n\nAlignment: {}\n",
                    &d.docs,
                    &d.mem_size(),
                    &d.mem_align()
                )
                .as_str(),
            ));
            if let Some(constants) = constants_by_name.remove(&d.name) {
                let heading = heading_from_node(&child, 1);
                child.new_child(MdSection::new(heading, "Constants"));
                for constant in constants {
                    child.new_child(MdNamedType::new(
                        MdHeading::new_bullet(),
                        format!("{}.{}", name, constant.name.as_str()).as_str(),
                        constant.name.as_str(),
                        &constant.docs,
                    ));
                }
            }
            d.generate(child.clone());
        }

        let modules = node.new_child(MdSection::new(heading, "Modules"));
        for d in self.modules() {
            let mut content = MdSection::new(heading.new_level_down(), d.name.as_str());
            content.id = Some(d.name.as_str().to_owned());
            let child = modules.new_child(content);
            d.generate(child.clone());
        }

        assert!(constants_by_name.is_empty());
    }
}

impl ToMarkdown for TypeRef {
    fn generate(&self, node: MdNodeRef) {
        match self {
            TypeRef::Value(v) => {
                v.generate(node.clone());
                node.content_ref_mut::<MdNamedType>().ty = Some(format!("`{}`", self.type_name()));
            }
            TypeRef::Name(n) => {
                node.content_ref_mut::<MdNamedType>().ty =
                    Some(format!("[`{0}`](#{0})", n.name.as_str().to_owned()));
            }
        }
    }
}

impl ToMarkdown for NamedType {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
    }
}

impl ToMarkdown for Type {
    fn generate(&self, node: MdNodeRef) {
        match self {
            Self::Record(a) => a.generate(node.clone()),
            Self::Variant(a) => a.generate(node.clone()),
            Self::Handle(a) => a.generate(node.clone()),
            Self::List(_) => {}
            Self::Pointer(_) => {}
            Self::ConstPointer(_) => {}
            Self::Builtin(_) => {}
        }
    }
}

impl ToMarkdown for RecordDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Record members"));

        for member_layout in &self.member_layout() {
            let member = member_layout.member;
            let offset = member_layout.offset;
            let name = member.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let (div, offset_desc) = if self.bitflags_repr().is_some() {
                (4, "Bit")
            } else {
                (1, "Offset")
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                format!("{}\n{}: {}\n", &member.docs, offset_desc, offset / div).as_str(),
            ));
            member.tref.generate(n.clone());
        }
    }
}

impl ToMarkdown for Variant {
    fn generate(&self, node: MdNodeRef) {
        if self.is_bool() {
            return;
        }
        if self.cases.iter().any(|c| c.tref.is_some()) {
            let heading = heading_from_node(&node, 1);
            node.new_child(MdSection::new(heading, "Variant Layout"));

            let whole = self.mem_size_align();
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("size: {}", whole.size),
            ));
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("align: {}", whole.align),
            ));

            let tag = self.tag_repr.mem_size_align();
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("tag_size: {}", tag.size),
            ));
        }

        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Variant cases"));

        for case in self.cases.iter() {
            let name = case.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                &case.docs,
            ));
            if let Some(ty) = &case.tref {
                ty.generate(n.clone());
            }
        }
    }
}

impl ToMarkdown for HandleDatatype {
    fn generate(&self, node: MdNodeRef) {
        // TODO this needs more work
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Supertypes"));
    }
}

impl ToMarkdown for Module {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        let imports = node.new_child(MdSection::new(heading, "Imports"));
        for import in self.imports() {
            let child = imports.new_child(MdSection::new(heading.new_level_down(), ""));
            import.generate(child.clone());
        }

        let funcs = node.new_child(MdSection::new(heading, "Functions"));
        for func in self.funcs() {
            let name = func.name.as_str();
            let child = funcs.new_child(MdFunc::new(
                heading.new_level_down(),
                name,
                name,
                &func.docs,
            ));
            func.generate(child.clone());
        }
    }
}

impl ToMarkdown for ModuleImport {
    fn generate(&self, node: MdNodeRef) {
        match self.variant {
            ModuleImportVariant::Memory => {
                node.content_ref_mut::<MdSection>().title = "Memory".to_owned();
            }
        }
    }
}

impl ToMarkdown for InterfaceFunc {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Params"));
        for param in &self.params {
            let name = param.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let child = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                param.name.as_str(),
            ));
            param.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_ref_mut::<MdFunc>()
                .inputs
                .push((param.name.as_str().to_owned(), param.tref.type_name()));
        }

        node.new_child(MdSection::new(heading, "Results"));
        for result in &self.results {
            let name = result.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let child = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                result.name.as_str(),
            ));
            result.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_ref_mut::<MdFunc>()
                .outputs
                .push(result.tref.type_name());
        }
    }
}

impl ToMarkdown for InterfaceFuncParam {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
        node.content_ref_mut::<MdNamedType>().docs = self.docs.clone();
    }
}

impl BuiltinType {
    pub fn type_name(&self) -> &'static str {
        match self {
            BuiltinType::Char => "char",
            BuiltinType::U8 { .. } => "u8",
            BuiltinType::U16 => "u16",
            BuiltinType::U32 {
                lang_ptr_size: false,
            } => "u32",
            BuiltinType::U32 {
                lang_ptr_size: true,
            } => "usize",
            BuiltinType::U64 => "u64",
            BuiltinType::S8 => "s8",
            BuiltinType::S16 => "s16",
            BuiltinType::S32 => "s32",
            BuiltinType::S64 => "s64",
            BuiltinType::F32 => "f32",
            BuiltinType::F64 => "f64",
        }
    }
}

impl TypeRef {
    pub fn type_name(&self) -> String {
        match self {
            TypeRef::Name(n) => n.name.as_str().to_string(),
            TypeRef::Value(v) => match &**v {
                Type::List(a) => match &**a.type_() {
                    Type::Builtin(BuiltinType::Char) => "string".to_string(),
                    _ => format!("List<{}>", a.type_name()),
                },
                Type::Pointer(p) => format!("Pointer<{}>", p.type_name()),
                Type::ConstPointer(p) => format!("ConstPointer<{}>", p.type_name()),
                Type::Builtin(b) => b.type_name().to_string(),
                Type::Record(RecordDatatype {
                    kind: RecordKind::Tuple,
                    members,
                }) => {
                    let mut ret = "(".to_string();
                    for (i, member) in members.iter().enumerate() {
                        if i > 0 {
                            ret.push_str(", ");
                        }
                        ret.push_str(&member.tref.type_name());
                    }
                    ret.push_str(")");
                    ret
                }
                Type::Record { .. } => {
                    format!("Record")
                }
                Type::Handle { .. } => {
                    format!("Handle")
                }
                Type::Variant(v) => {
                    if let Some((ok, err)) = v.as_expected() {
                        let ok = match ok {
                            Some(ty) => ty.type_name(),
                            None => "()".to_string(),
                        };
                        let err = match err {
                            Some(ty) => ty.type_name(),
                            None => "()".to_string(),
                        };
                        format!("Result<{}, {}>", ok, err)
                    } else if v.is_bool() {
                        format!("bool")
                    } else {
                        format!("Variant")
                    }
                }
            },
        }
    }
}

// TODO
// Generate Markdown tree for the polyfill
impl Documentation for Polyfill {
    fn to_md(&self) -> String {
        let module_docs = self
            .modules
            .iter()
            .map(|m| m.to_md())
            .collect::<Vec<String>>()
            .join("\n");
        let type_docs = self
            .type_polyfills()
            .iter()
            .filter_map(|t| {
                if t.repeq() == RepEquality::Eq {
                    None
                } else {
                    Some(t.to_md())
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "# Modules\n{}\n# Type Conversions\n{}\n",
            module_docs, type_docs
        )
    }
}

impl Documentation for ModulePolyfill {
    fn to_md(&self) -> String {
        format!(
            "## `{}` in terms of `{}`\n{}",
            self.new.name.as_str(),
            self.old.name.as_str(),
            self.funcs
                .iter()
                .map(|f| f.to_md())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Documentation for FuncPolyfill {
    fn to_md(&self) -> String {
        if self.full_compat() {
            format!("* `{}`: full compatibility", self.new.name.as_str())
        } else {
            let name = if self.new.name != self.old.name {
                format!(
                    "* `{}` => `{}`",
                    self.old.name.as_str(),
                    self.new.name.as_str()
                )
            } else {
                format!("* `{}`", self.new.name.as_str())
            };
            let mut contents = Vec::new();
            for p in self.mapped_params.iter() {
                contents.push(if !p.full_compat() {
                    format!("param {}", p.to_md())
                } else {
                    format!("param `{}`: compatible", p.new.name.as_str())
                })
            }
            for u in self.unknown_params.iter() {
                contents.push(format!(
                    "{} param `{}`: no corresponding result!",
                    u.which(),
                    u.param().name.as_str()
                ))
            }
            for r in self.mapped_results.iter() {
                contents.push(if !r.full_compat() {
                    format!("result {}", r.to_md())
                } else {
                    format!("result `{}`: compatible", r.new.name.as_str())
                })
            }
            for u in self.unknown_results.iter() {
                contents.push(format!(
                    "{} result `{}`: no corresponding result!",
                    u.which(),
                    u.param().name.as_str()
                ))
            }
            let contents = if contents.is_empty() {
                String::new()
            } else {
                format!(":\n    - {}", contents.join("\n    - "))
            };
            format!("{}{}", name, contents)
        }
    }
}

impl Documentation for ParamPolyfill {
    fn to_md(&self) -> String {
        let name = if self.new.name != self.old.name {
            format!(
                "`{}` => `{}`",
                self.old.name.as_str(),
                self.new.name.as_str()
            )
        } else {
            format!("`{}`", self.new.name.as_str())
        };
        let repr = match self.repeq() {
            RepEquality::Eq => "compatible types".to_string(),
            RepEquality::Superset => format!(
                "`{}` is superset-compatible with `{}`",
                self.old.tref.type_name(),
                self.new.tref.type_name()
            ),
            RepEquality::NotEq => format!(
                "`{}` is incompatible with new `{}`",
                self.old.tref.type_name(),
                self.new.tref.type_name()
            ),
        };
        format!("{}: {}", name, repr)
    }
}

impl Documentation for TypePolyfill {
    fn to_md(&self) -> String {
        fn repeq_name(r: RepEquality) -> &'static str {
            match r {
                RepEquality::Eq => ": compatible",
                RepEquality::Superset => ": superset",
                RepEquality::NotEq => "",
            }
        }
        match self {
            TypePolyfill::OldToNew(o, n) => format!(
                "* old `{}` => new `{}`{}",
                o.type_name(),
                n.type_name(),
                repeq_name(self.repeq())
            ),
            TypePolyfill::NewToOld(n, o) => format!(
                "* new `{}` => old `{}`{}",
                n.type_name(),
                o.type_name(),
                repeq_name(self.repeq())
            ),
        }
    }
}
