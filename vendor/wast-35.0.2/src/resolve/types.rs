use crate::ast::*;
use crate::resolve::gensym;
use std::collections::HashMap;

pub fn expand<'a>(fields: &mut Vec<ModuleField<'a>>) {
    let mut expander = Expander::default();
    expander.process(fields);
}

#[derive(Default)]
struct Expander<'a> {
    // See the comment in `process` for why this exists.
    process_imports_early: bool,

    // Maps used to "intern" types. These maps are populated as type annotations
    // are seen and inline type annotations use previously defined ones if
    // there's a match.
    func_type_to_idx: HashMap<FuncKey<'a>, Index<'a>>,
    instance_type_to_idx: HashMap<InstanceKey<'a>, Index<'a>>,
    module_type_to_idx: HashMap<ModuleKey<'a>, Index<'a>>,

    /// Fields, during processing, which should be prepended to the
    /// currently-being-processed field. This should always be empty after
    /// processing is complete.
    to_prepend: Vec<ModuleField<'a>>,
}

impl<'a> Expander<'a> {
    fn process(&mut self, fields: &mut Vec<ModuleField<'a>>) {
        // For the given list of fields this determines whether imports are
        // processed as part of `expand_header` or as part of `expand`. The
        // reason for this distinction is that pre-module-linking types were
        // always sorted to the front of the module so new types were always
        // appended to the end. After module-linking, however, types are
        // interspersed with imports and order matters. This means that imports
        // can't use intern'd types which appear later.
        //
        // This is a bit of a hack and ideally something that needs to be
        // addressed in the upstream spec. WebAssembly/module-linking#25
        // represents this issue.
        self.process_imports_early = fields.iter().any(|f| match f {
            ModuleField::Alias(_) | ModuleField::NestedModule(_) | ModuleField::Instance(_) => true,
            _ => false,
        });

        // Next we expand "header" fields which are those like types and
        // imports. In this context "header" is defined by the previous
        // `process_imports_early` annotation.
        let mut cur = 0;
        while cur < fields.len() {
            self.expand_header(&mut fields[cur]);
            for item in self.to_prepend.drain(..) {
                fields.insert(cur, item);
                cur += 1;
            }
            cur += 1;
        }

        // Next after we've done that we expand remaining fields. Note that
        // after this we actually append instead of prepend. This is because
        // injected types are intended to come at the end of the type section
        // and types will be sorted before all other items processed here in the
        // final module anyway.
        for field in fields.iter_mut() {
            self.expand(field);
        }
        fields.extend(self.to_prepend.drain(..));
    }

    fn expand_header(&mut self, item: &mut ModuleField<'a>) {
        match item {
            ModuleField::Type(ty) => {
                let id = gensym::fill(ty.span, &mut ty.id);
                match &mut ty.def {
                    TypeDef::Func(f) => {
                        f.key().insert(self, Index::Id(id));
                    }
                    TypeDef::Instance(i) => {
                        i.expand(self);
                        i.key().insert(self, Index::Id(id));
                    }
                    TypeDef::Module(m) => {
                        m.expand(self);
                        m.key().insert(self, Index::Id(id));
                    }
                    TypeDef::Array(_) | TypeDef::Struct(_) => {}
                }
            }
            ModuleField::Import(i) if self.process_imports_early => {
                self.expand_item_sig(&mut i.item);
            }
            _ => {}
        }
    }

    fn expand(&mut self, item: &mut ModuleField<'a>) {
        match item {
            // This is pre-expanded above
            ModuleField::Type(_) => {}

            ModuleField::Import(i) => {
                // Only expand here if not expanded above
                if !self.process_imports_early {
                    self.expand_item_sig(&mut i.item);
                }
            }
            ModuleField::Func(f) => {
                self.expand_type_use(&mut f.ty);
                if let FuncKind::Inline { expression, .. } = &mut f.kind {
                    self.expand_expression(expression);
                }
            }
            ModuleField::Global(g) => {
                if let GlobalKind::Inline(expr) = &mut g.kind {
                    self.expand_expression(expr);
                }
            }
            ModuleField::Data(d) => {
                if let DataKind::Active { offset, .. } = &mut d.kind {
                    self.expand_expression(offset);
                }
            }
            ModuleField::Elem(e) => {
                if let ElemKind::Active { offset, .. } = &mut e.kind {
                    self.expand_expression(offset);
                }
            }
            ModuleField::Event(e) => match &mut e.ty {
                EventType::Exception(ty) => {
                    self.expand_type_use(ty);
                }
            },
            ModuleField::NestedModule(m) => {
                if let NestedModuleKind::Inline { fields } = &mut m.kind {
                    Expander::default().process(fields);
                }
            }

            ModuleField::Alias(_)
            | ModuleField::Instance(_)
            | ModuleField::Table(_)
            | ModuleField::Memory(_)
            | ModuleField::Start(_)
            | ModuleField::Export(_)
            | ModuleField::Custom(_) => {}
        }
    }

    fn expand_item_sig(&mut self, item: &mut ItemSig<'a>) {
        match &mut item.kind {
            ItemKind::Func(t) | ItemKind::Event(EventType::Exception(t)) => {
                self.expand_type_use(t);
            }
            ItemKind::Instance(t) => {
                self.expand_type_use(t);
                t.inline.take();
            }
            ItemKind::Module(m) => {
                self.expand_type_use(m);
                m.inline.take();
            }
            ItemKind::Global(_) | ItemKind::Table(_) | ItemKind::Memory(_) => {}
        }
    }

    fn expand_expression(&mut self, expr: &mut Expression<'a>) {
        for instr in expr.instrs.iter_mut() {
            self.expand_instr(instr);
        }
    }

    fn expand_instr(&mut self, instr: &mut Instruction<'a>) {
        match instr {
            Instruction::Block(bt)
            | Instruction::If(bt)
            | Instruction::Loop(bt)
            | Instruction::Let(LetType { block: bt, .. })
            | Instruction::Try(bt) => {
                // No expansion necessary, a type reference is already here.
                // We'll verify that it's the same as the inline type, if any,
                // later.
                if bt.ty.index.is_some() {
                    return;
                }

                match &bt.ty.inline {
                    // Only actually expand `TypeUse` with an index which appends a
                    // type if it looks like we need one. This way if the
                    // multi-value proposal isn't enabled and/or used we won't
                    // encode it.
                    Some(inline) => {
                        if inline.params.len() == 0 && inline.results.len() <= 1 {
                            return;
                        }
                    }

                    // If we didn't have either an index or an inline type
                    // listed then assume our block has no inputs/outputs, so
                    // fill in the inline type here.
                    //
                    // Do not fall through to expanding the `TypeUse` because
                    // this doesn't force an empty function type to go into the
                    // type section.
                    None => {
                        bt.ty.inline = Some(FunctionType::default());
                        return;
                    }
                }
                self.expand_type_use(&mut bt.ty);
            }
            Instruction::FuncBind(b) => {
                self.expand_type_use(&mut b.ty);
            }
            Instruction::CallIndirect(c) | Instruction::ReturnCallIndirect(c) => {
                self.expand_type_use(&mut c.ty);
            }
            _ => {}
        }
    }

    fn expand_type_use<T>(&mut self, item: &mut TypeUse<'a, T>) -> Index<'a>
    where
        T: TypeReference<'a>,
    {
        if let Some(idx) = &item.index {
            match idx {
                ItemRef::Item { idx, exports, .. } => {
                    debug_assert!(exports.len() == 0);
                    return idx.clone();
                }
                ItemRef::Outer { .. } => unreachable!(),
            }
        }
        let key = match item.inline.as_mut() {
            Some(ty) => {
                ty.expand(self);
                ty.key()
            }
            None => T::default().key(),
        };
        let span = Span::from_offset(0); // FIXME: don't manufacture
        let idx = self.key_to_idx(span, key);
        item.index = Some(ItemRef::Item {
            idx,
            kind: kw::r#type(span),
            exports: Vec::new(),
        });
        return idx;
    }

    fn key_to_idx(&mut self, span: Span, key: impl TypeKey<'a>) -> Index<'a> {
        // First see if this `key` already exists in the type definitions we've
        // seen so far...
        if let Some(idx) = key.lookup(self) {
            return idx;
        }

        // ... and failing that we insert a new type definition.
        let id = gensym::gen(span);
        self.to_prepend.push(ModuleField::Type(Type {
            span,
            id: Some(id),
            def: key.to_def(span),
        }));
        let idx = Index::Id(id);
        key.insert(self, idx);

        return idx;
    }
}

trait TypeReference<'a>: Default {
    type Key: TypeKey<'a>;
    fn key(&self) -> Self::Key;
    fn expand(&mut self, cx: &mut Expander<'a>);
}

trait TypeKey<'a> {
    fn lookup(&self, cx: &Expander<'a>) -> Option<Index<'a>>;
    fn to_def(&self, span: Span) -> TypeDef<'a>;
    fn insert(&self, cx: &mut Expander<'a>, id: Index<'a>);
}

type FuncKey<'a> = (Box<[ValType<'a>]>, Box<[ValType<'a>]>);

impl<'a> TypeReference<'a> for FunctionType<'a> {
    type Key = FuncKey<'a>;

    fn key(&self) -> Self::Key {
        let params = self.params.iter().map(|p| p.2).collect();
        let results = self.results.clone();
        (params, results)
    }

    fn expand(&mut self, _cx: &mut Expander<'a>) {}
}

impl<'a> TypeKey<'a> for FuncKey<'a> {
    fn lookup(&self, cx: &Expander<'a>) -> Option<Index<'a>> {
        cx.func_type_to_idx.get(self).cloned()
    }

    fn to_def(&self, _span: Span) -> TypeDef<'a> {
        TypeDef::Func(FunctionType {
            params: self.0.iter().map(|t| (None, None, *t)).collect(),
            results: self.1.clone(),
        })
    }

    fn insert(&self, cx: &mut Expander<'a>, idx: Index<'a>) {
        cx.func_type_to_idx.entry(self.clone()).or_insert(idx);
    }
}

// A list of the exports of a module as well as the signature they export.
type InstanceKey<'a> = Vec<(&'a str, Item<'a>)>;

impl<'a> TypeReference<'a> for InstanceType<'a> {
    type Key = InstanceKey<'a>;

    fn key(&self) -> Self::Key {
        self.exports
            .iter()
            .map(|export| (export.name, Item::new(&export.item)))
            .collect()
    }

    fn expand(&mut self, cx: &mut Expander<'a>) {
        for export in self.exports.iter_mut() {
            cx.expand_item_sig(&mut export.item);
        }
    }
}

impl<'a> TypeKey<'a> for InstanceKey<'a> {
    fn lookup(&self, cx: &Expander<'a>) -> Option<Index<'a>> {
        cx.instance_type_to_idx.get(self).cloned()
    }

    fn to_def(&self, span: Span) -> TypeDef<'a> {
        let exports = self
            .iter()
            .map(|(name, item)| ExportType {
                span,
                name,
                item: item.to_sig(span),
            })
            .collect();
        TypeDef::Instance(InstanceType { exports })
    }

    fn insert(&self, cx: &mut Expander<'a>, idx: Index<'a>) {
        cx.instance_type_to_idx.entry(self.clone()).or_insert(idx);
    }
}

// The first element of this pair is the list of imports in the module, and the
// second element is the list of exports.
type ModuleKey<'a> = (
    Vec<(&'a str, Option<&'a str>, Item<'a>)>,
    Vec<(&'a str, Item<'a>)>,
);

impl<'a> TypeReference<'a> for ModuleType<'a> {
    type Key = ModuleKey<'a>;

    fn key(&self) -> Self::Key {
        let imports = self
            .imports
            .iter()
            .map(|import| (import.module, import.field, Item::new(&import.item)))
            .collect();
        let exports = self
            .exports
            .iter()
            .map(|export| (export.name, Item::new(&export.item)))
            .collect();
        (imports, exports)
    }

    fn expand(&mut self, cx: &mut Expander<'a>) {
        for export in self.exports.iter_mut() {
            cx.expand_item_sig(&mut export.item);
        }
        for import in self.imports.iter_mut() {
            cx.expand_item_sig(&mut import.item);
        }
    }
}

impl<'a> TypeKey<'a> for ModuleKey<'a> {
    fn lookup(&self, cx: &Expander<'a>) -> Option<Index<'a>> {
        cx.module_type_to_idx.get(self).cloned()
    }

    fn to_def(&self, span: Span) -> TypeDef<'a> {
        let imports = self
            .0
            .iter()
            .map(|(module, field, item)| Import {
                span,
                module,
                field: *field,
                item: item.to_sig(span),
            })
            .collect();
        let exports = self
            .1
            .iter()
            .map(|(name, item)| ExportType {
                span,
                name,
                item: item.to_sig(span),
            })
            .collect();
        TypeDef::Module(ModuleType { imports, exports })
    }

    fn insert(&self, cx: &mut Expander<'a>, idx: Index<'a>) {
        cx.module_type_to_idx.entry(self.clone()).or_insert(idx);
    }
}

// A lookalike to `ItemKind` except without all non-relevant information for
// hashing. This is used as a hash key for instance/module type lookup.
#[derive(Clone, PartialEq, Eq, Hash)]
enum Item<'a> {
    Func(Index<'a>),
    Table(TableType<'a>),
    Memory(MemoryType),
    Global(GlobalType<'a>),
    Event(Index<'a>),
    Module(Index<'a>),
    Instance(Index<'a>),
}

impl<'a> Item<'a> {
    fn new(item: &ItemSig<'a>) -> Item<'a> {
        match &item.kind {
            ItemKind::Func(f) => Item::Func(*f.index.as_ref().unwrap().unwrap_index()),
            ItemKind::Instance(f) => Item::Instance(*f.index.as_ref().unwrap().unwrap_index()),
            ItemKind::Module(f) => Item::Module(*f.index.as_ref().unwrap().unwrap_index()),
            ItemKind::Event(EventType::Exception(f)) => {
                Item::Event(*f.index.as_ref().unwrap().unwrap_index())
            }
            ItemKind::Table(t) => Item::Table(t.clone()),
            ItemKind::Memory(t) => Item::Memory(t.clone()),
            ItemKind::Global(t) => Item::Global(t.clone()),
        }
    }

    fn to_sig(&self, span: Span) -> ItemSig<'a> {
        let kind = match self {
            Item::Func(index) => ItemKind::Func(TypeUse::new_with_index(*index)),
            Item::Event(index) => {
                ItemKind::Event(EventType::Exception(TypeUse::new_with_index(*index)))
            }
            Item::Instance(index) => ItemKind::Instance(TypeUse::new_with_index(*index)),
            Item::Module(index) => ItemKind::Module(TypeUse::new_with_index(*index)),
            Item::Table(t) => ItemKind::Table(t.clone()),
            Item::Memory(t) => ItemKind::Memory(t.clone()),
            Item::Global(t) => ItemKind::Global(t.clone()),
        };
        ItemSig {
            span,
            id: None,
            name: None,
            kind,
        }
    }
}
