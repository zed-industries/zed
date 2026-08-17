use super::{ParamList, WorldOrInterface};
use crate::ast::toposort::toposort;
use crate::*;
use anyhow::bail;
use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Default)]
pub struct Resolver<'a> {
    /// Current package name learned through the ASTs pushed onto this resolver.
    package_name: Option<(PackageName, Span)>,

    /// Package docs.
    package_docs: Docs,

    /// All non-`package` WIT decls are going to be resolved together.
    decl_lists: Vec<ast::DeclList<'a>>,

    // Arenas that get plumbed to the final `UnresolvedPackage`
    types: Arena<TypeDef>,
    interfaces: Arena<Interface>,
    worlds: Arena<World>,

    // Interning structure for types which-need-not-be-named such as
    // `list<string>` and such.
    anon_types: HashMap<Key, TypeId>,

    /// The index within `self.ast_items` that lookups should go through. This
    /// is updated as the ASTs are walked.
    cur_ast_index: usize,

    /// A map per `ast::DeclList` which keeps track of the file's top level
    /// names in scope. This maps each name onto either a world or an interface,
    /// handling things like `use` at the top level.
    ast_items: Vec<IndexMap<&'a str, AstItem>>,

    /// A map for the entire package being created of all names defined within,
    /// along with the ID they're mapping to.
    package_items: IndexMap<&'a str, AstItem>,

    /// A per-interface map of name to item-in-the-interface. This is the same
    /// length as `self.types` and is pushed to whenever `self.types` is pushed
    /// to.
    interface_types: Vec<IndexMap<&'a str, (TypeOrItem, Span)>>,

    /// Metadata about foreign dependencies which are not defined in this
    /// package. This map is keyed by the name of the package being imported
    /// from. The next level of key is the name of the interface being imported
    /// from, and the final value is a tuple containing the assigned ID of the
    /// dependency, and a Vector of the Stability attributes associated with each
    /// of its imports.
    foreign_deps: IndexMap<PackageName, IndexMap<&'a str, (AstItem, Vec<Stability>)>>,

    /// All interfaces that are present within `self.foreign_deps`.
    foreign_interfaces: HashSet<InterfaceId>,

    foreign_worlds: HashSet<WorldId>,

    /// The current type lookup scope which will eventually make its way into
    /// `self.interface_types`.
    type_lookup: IndexMap<&'a str, (TypeOrItem, Span)>,

    /// An assigned span for where all types inserted into `self.types` as
    /// imported from foreign interfaces. These types all show up first in the
    /// `self.types` arena and this span is used to generate an error message
    /// pointing to it if the item isn't actually defined.
    unknown_type_spans: Vec<Span>,

    /// Spans for each world in `self.worlds`
    world_spans: Vec<WorldSpan>,

    /// Spans for each type in `self.types`
    type_spans: Vec<Span>,

    /// The span of each interface's definition which is used for error
    /// reporting during the final `Resolve` phase.
    interface_spans: Vec<InterfaceSpan>,

    /// Spans per entry in `self.foreign_deps` for where the dependency was
    /// introduced to print an error message if necessary.
    foreign_dep_spans: Vec<Span>,

    /// A list of `TypeDefKind::Unknown` types which are required to be
    /// resources when this package is resolved against its dependencies.
    required_resource_types: Vec<(TypeId, Span)>,
}

#[derive(PartialEq, Eq, Hash)]
enum Key {
    Variant(Vec<(String, Option<Type>)>),
    BorrowHandle(TypeId),
    Record(Vec<(String, Type)>),
    Flags(Vec<String>),
    Tuple(Vec<Type>),
    Enum(Vec<String>),
    List(Type),
    Map(Type, Type),
    FixedSizeList(Type, u32),
    Option(Type),
    Result(Option<Type>, Option<Type>),
    Future(Option<Type>),
    Stream(Option<Type>),
}

enum TypeItem<'a, 'b> {
    Use(&'b ast::Use<'a>),
    Def(&'b ast::TypeDef<'a>),
}

enum TypeOrItem {
    Type(TypeId),
    Item(&'static str),
}

impl<'a> Resolver<'a> {
    pub(super) fn push(&mut self, file: ast::PackageFile<'a>) -> Result<()> {
        // As each WIT file is pushed into this resolver keep track of the
        // current package name assigned. Only one file needs to mention it, but
        // if multiple mention it then they must all match.
        if let Some(cur) = &file.package_id {
            let cur_name = cur.package_name();
            if let Some((prev, _)) = &self.package_name {
                if cur_name != *prev {
                    bail!(Error::new(
                        cur.span,
                        format!(
                            "package identifier `{cur_name}` does not match \
                             previous package name of `{prev}`"
                        ),
                    ))
                }
            }
            self.package_name = Some((cur_name, cur.span));

            // At most one 'package' item can have doc comments.
            let docs = self.docs(&cur.docs);
            if docs.contents.is_some() {
                if self.package_docs.contents.is_some() {
                    bail!(Error::new(
                        cur.docs.span,
                        "found doc comments on multiple 'package' items"
                    ))
                }
                self.package_docs = docs;
            }
        }

        // Ensure that there are no nested packages in `file`. Note that for
        // top level files nested packages are handled separately in `ast.rs`
        // with their own resolver.
        for item in file.decl_list.items.iter() {
            let span = match item {
                ast::AstItem::Package(pkg) => pkg.package_id.as_ref().unwrap().span,
                _ => continue,
            };
            bail!(Error::new(
                span,
                "nested packages must be placed at the top-level"
            ))
        }

        self.decl_lists.push(file.decl_list);
        Ok(())
    }

    pub(crate) fn resolve(&mut self) -> Result<UnresolvedPackage> {
        // At least one of the WIT files must have a `package` annotation.
        let (name, package_name_span) = match &self.package_name {
            Some(name) => name.clone(),
            None => {
                bail!("no `package` header was found in any WIT file for this package")
            }
        };

        // First populate information about foreign dependencies and the general
        // structure of the package. This should resolve the "base" of many
        // `use` statements and additionally generate a topological ordering of
        // all interfaces in the package to visit.
        let decl_lists = mem::take(&mut self.decl_lists);
        self.populate_foreign_deps(&decl_lists);
        let (iface_order, world_order) = self.populate_ast_items(&decl_lists)?;
        self.populate_foreign_types(&decl_lists)?;

        // Use the topological ordering of all interfaces to resolve all
        // interfaces in-order. Note that a reverse-mapping from ID to AST is
        // generated here to assist with this.
        let mut iface_id_to_ast = IndexMap::new();
        let mut world_id_to_ast = IndexMap::new();
        for (i, decl_list) in decl_lists.iter().enumerate() {
            for item in decl_list.items.iter() {
                match item {
                    ast::AstItem::Interface(iface) => {
                        let id = match self.ast_items[i][iface.name.name] {
                            AstItem::Interface(id) => id,
                            AstItem::World(_) => unreachable!(),
                        };
                        iface_id_to_ast.insert(id, (iface, i));
                    }
                    ast::AstItem::World(world) => {
                        let id = match self.ast_items[i][world.name.name] {
                            AstItem::World(id) => id,
                            AstItem::Interface(_) => unreachable!(),
                        };
                        world_id_to_ast.insert(id, (world, i));
                    }
                    ast::AstItem::Use(_) => {}
                    ast::AstItem::Package(_) => unreachable!(),
                }
            }
        }

        for id in iface_order {
            let (interface, i) = &iface_id_to_ast[&id];
            self.cur_ast_index = *i;
            self.resolve_interface(id, &interface.items, &interface.docs, &interface.attributes)?;
        }

        for id in world_order {
            let (world, i) = &world_id_to_ast[&id];
            self.cur_ast_index = *i;
            self.resolve_world(id, world)?;
        }

        self.decl_lists = decl_lists;
        Ok(UnresolvedPackage {
            package_name_span,
            name,
            docs: mem::take(&mut self.package_docs),
            worlds: mem::take(&mut self.worlds),
            types: mem::take(&mut self.types),
            interfaces: mem::take(&mut self.interfaces),
            foreign_deps: self
                .foreign_deps
                .iter()
                .map(|(name, deps)| {
                    (
                        name.clone(),
                        deps.iter()
                            .map(|(name, (id, stabilities))| {
                                (name.to_string(), (*id, stabilities.clone()))
                            })
                            .collect(),
                    )
                })
                .collect(),
            unknown_type_spans: mem::take(&mut self.unknown_type_spans),
            interface_spans: mem::take(&mut self.interface_spans),
            world_spans: mem::take(&mut self.world_spans),
            type_spans: mem::take(&mut self.type_spans),
            foreign_dep_spans: mem::take(&mut self.foreign_dep_spans),
            required_resource_types: mem::take(&mut self.required_resource_types),
        })
    }

    /// Registers all foreign dependencies made within the ASTs provided.
    ///
    /// This will populate the `self.foreign_{deps,interfaces,worlds}` maps with all
    /// `UsePath::Package` entries.
    fn populate_foreign_deps(&mut self, decl_lists: &[ast::DeclList<'a>]) {
        let mut foreign_deps = mem::take(&mut self.foreign_deps);
        let mut foreign_interfaces = mem::take(&mut self.foreign_interfaces);
        let mut foreign_worlds = mem::take(&mut self.foreign_worlds);
        for decl_list in decl_lists {
            decl_list
                .for_each_path(&mut |_, attrs, path, _names, world_or_iface| {
                    let (id, name) = match path {
                        ast::UsePath::Package { id, name } => (id, name),
                        _ => return Ok(()),
                    };

                    let stability = self.stability(attrs)?;

                    let deps = foreign_deps.entry(id.package_name()).or_insert_with(|| {
                        self.foreign_dep_spans.push(id.span);
                        IndexMap::new()
                    });
                    let (id, stabilities) = deps.entry(name.name).or_insert_with(|| {
                        let id = match world_or_iface {
                            WorldOrInterface::World => {
                                log::trace!(
                                    "creating a world for foreign dep: {}/{}",
                                    id.package_name(),
                                    name.name
                                );
                                AstItem::World(self.alloc_world(name.span))
                            }
                            WorldOrInterface::Interface | WorldOrInterface::Unknown => {
                                // Currently top-level `use` always assumes an interface, so the
                                // `Unknown` case is the same as `Interface`.
                                log::trace!(
                                    "creating an interface for foreign dep: {}/{}",
                                    id.package_name(),
                                    name.name
                                );
                                AstItem::Interface(self.alloc_interface(name.span))
                            }
                        };
                        (id, Vec::new())
                    });

                    stabilities.push(stability);

                    let _ = match *id {
                        AstItem::Interface(id) => foreign_interfaces.insert(id),
                        AstItem::World(id) => foreign_worlds.insert(id),
                    };

                    Ok(())
                })
                .unwrap();
        }
        self.foreign_deps = foreign_deps;
        self.foreign_interfaces = foreign_interfaces;
        self.foreign_worlds = foreign_worlds;
    }

    fn alloc_interface(&mut self, span: Span) -> InterfaceId {
        self.interface_types.push(IndexMap::new());
        self.interface_spans.push(InterfaceSpan {
            span,
            funcs: Vec::new(),
        });
        self.interfaces.alloc(Interface {
            name: None,
            types: IndexMap::new(),
            docs: Docs::default(),
            stability: Default::default(),
            functions: IndexMap::new(),
            package: None,
        })
    }

    fn alloc_world(&mut self, span: Span) -> WorldId {
        self.world_spans.push(WorldSpan {
            span,
            imports: Vec::new(),
            exports: Vec::new(),
            includes: Vec::new(),
        });
        self.worlds.alloc(World {
            name: String::new(),
            docs: Docs::default(),
            exports: IndexMap::new(),
            imports: IndexMap::new(),
            package: None,
            includes: Default::default(),
            include_names: Default::default(),
            stability: Default::default(),
        })
    }

    /// This method will create a `World` and an `Interface` for all items
    /// present in the specified set of ASTs. Additionally maps for each AST are
    /// generated for resolving use-paths later on.
    fn populate_ast_items(
        &mut self,
        decl_lists: &[ast::DeclList<'a>],
    ) -> Result<(Vec<InterfaceId>, Vec<WorldId>)> {
        let mut package_items = IndexMap::new();

        // Validate that all worlds and interfaces have unique names within this
        // package across all ASTs which make up the package.
        let mut names = HashMap::new();
        let mut decl_list_namespaces = Vec::new();
        let mut order = IndexMap::new();
        for decl_list in decl_lists {
            let mut decl_list_ns = IndexMap::new();
            for item in decl_list.items.iter() {
                match item {
                    ast::AstItem::Interface(i) => {
                        if package_items.insert(i.name.name, i.name.span).is_some() {
                            bail!(Error::new(
                                i.name.span,
                                format!("duplicate item named `{}`", i.name.name),
                            ))
                        }
                        let prev = decl_list_ns.insert(i.name.name, ());
                        assert!(prev.is_none());
                        let prev = order.insert(i.name.name, Vec::new());
                        assert!(prev.is_none());
                        let prev = names.insert(i.name.name, item);
                        assert!(prev.is_none());
                    }
                    ast::AstItem::World(w) => {
                        if package_items.insert(w.name.name, w.name.span).is_some() {
                            bail!(Error::new(
                                w.name.span,
                                format!("duplicate item named `{}`", w.name.name),
                            ))
                        }
                        let prev = decl_list_ns.insert(w.name.name, ());
                        assert!(prev.is_none());
                        let prev = order.insert(w.name.name, Vec::new());
                        assert!(prev.is_none());
                        let prev = names.insert(w.name.name, item);
                        assert!(prev.is_none());
                    }
                    // These are processed down below.
                    ast::AstItem::Use(_) => {}

                    ast::AstItem::Package(_) => unreachable!(),
                }
            }
            decl_list_namespaces.push(decl_list_ns);
        }

        // Next record dependencies between interfaces as induced via `use`
        // paths. This step is used to perform a topological sort of all
        // interfaces to ensure there are no cycles and to generate an ordering
        // which we can resolve in.
        enum ItemSource<'a> {
            Foreign,
            Local(ast::Id<'a>),
        }

        for decl_list in decl_lists {
            // Record, in the context of this file, what all names are defined
            // at the top level and whether they point to other items in this
            // package or foreign items. Foreign deps are ignored for
            // topological ordering.
            let mut decl_list_ns = IndexMap::new();
            for item in decl_list.items.iter() {
                let (name, src) = match item {
                    ast::AstItem::Use(u) => {
                        let name = u.as_.as_ref().unwrap_or(u.item.name());
                        let src = match &u.item {
                            ast::UsePath::Id(id) => ItemSource::Local(id.clone()),
                            ast::UsePath::Package { .. } => ItemSource::Foreign,
                        };
                        (name, src)
                    }
                    ast::AstItem::Interface(i) => (&i.name, ItemSource::Local(i.name.clone())),
                    ast::AstItem::World(w) => (&w.name, ItemSource::Local(w.name.clone())),
                    ast::AstItem::Package(_) => unreachable!(),
                };
                if decl_list_ns.insert(name.name, (name.span, src)).is_some() {
                    bail!(Error::new(
                        name.span,
                        format!("duplicate name `{}` in this file", name.name),
                    ));
                }
            }

            // With this file's namespace information look at all `use` paths
            // and record dependencies between interfaces.
            decl_list.for_each_path(&mut |iface, _attrs, path, _names, _| {
                // If this import isn't contained within an interface then it's
                // in a world and it doesn't need to participate in our
                // topo-sort.
                let iface = match iface {
                    Some(name) => name,
                    None => return Ok(()),
                };
                let used_name = match path {
                    ast::UsePath::Id(id) => id,
                    ast::UsePath::Package { .. } => return Ok(()),
                };
                match decl_list_ns.get(used_name.name) {
                    Some((_, ItemSource::Foreign)) => return Ok(()),
                    Some((_, ItemSource::Local(id))) => {
                        order[iface.name].push(id.clone());
                    }
                    None => match package_items.get(used_name.name) {
                        Some(_) => {
                            order[iface.name].push(used_name.clone());
                        }
                        None => {
                            bail!(Error::new(
                                used_name.span,
                                format!(
                                    "interface or world `{name}` not found in package",
                                    name = used_name.name
                                ),
                            ))
                        }
                    },
                }
                Ok(())
            })?;
        }

        let order = toposort("interface or world", &order)?;
        log::debug!("toposort for interfaces and worlds in order: {order:?}");

        // Allocate interfaces in-order now that the ordering is defined. This
        // is then used to build up internal maps for each AST which are stored
        // in `self.ast_items`.
        let mut ids = IndexMap::new();
        let mut iface_id_order = Vec::new();
        let mut world_id_order = Vec::new();
        for name in order {
            match names.get(name).unwrap() {
                ast::AstItem::Interface(_) => {
                    let id = self.alloc_interface(package_items[name]);
                    self.interfaces[id].name = Some(name.to_string());
                    let prev = ids.insert(name, AstItem::Interface(id));
                    assert!(prev.is_none());
                    iface_id_order.push(id);
                }
                ast::AstItem::World(_) => {
                    let id = self.alloc_world(package_items[name]);
                    self.worlds[id].name = name.to_string();
                    let prev = ids.insert(name, AstItem::World(id));
                    assert!(prev.is_none());
                    world_id_order.push(id);
                }
                ast::AstItem::Use(_) | ast::AstItem::Package(_) => unreachable!(),
            };
        }
        for decl_list in decl_lists {
            let mut items = IndexMap::new();
            for item in decl_list.items.iter() {
                let (name, ast_item) = match item {
                    ast::AstItem::Use(u) => {
                        if !u.attributes.is_empty() {
                            bail!(Error::new(
                                u.span,
                                format!("attributes not allowed on top-level use"),
                            ))
                        }
                        let name = u.as_.as_ref().unwrap_or(u.item.name());
                        let item = match &u.item {
                            ast::UsePath::Id(name) => *ids.get(name.name).ok_or_else(|| {
                                Error::new(
                                    name.span,
                                    format!(
                                        "interface or world `{name}` does not exist",
                                        name = name.name
                                    ),
                                )
                            })?,
                            ast::UsePath::Package { id, name } => {
                                self.foreign_deps[&id.package_name()][name.name].0
                            }
                        };
                        (name.name, item)
                    }
                    ast::AstItem::Interface(i) => {
                        let iface_item = ids[i.name.name];
                        assert!(matches!(iface_item, AstItem::Interface(_)));
                        (i.name.name, iface_item)
                    }
                    ast::AstItem::World(w) => {
                        let world_item = ids[w.name.name];
                        assert!(matches!(world_item, AstItem::World(_)));
                        (w.name.name, world_item)
                    }
                    ast::AstItem::Package(_) => unreachable!(),
                };
                let prev = items.insert(name, ast_item);
                assert!(prev.is_none());

                // Items defined via `use` don't go into the package namespace,
                // only the file namespace.
                if !matches!(item, ast::AstItem::Use(_)) {
                    let prev = self.package_items.insert(name, ast_item);
                    assert!(prev.is_none());
                }
            }
            self.ast_items.push(items);
        }
        Ok((iface_id_order, world_id_order))
    }

    /// Generate a `Type::Unknown` entry for all types imported from foreign
    /// packages.
    ///
    /// This is done after all interfaces are generated so `self.resolve_path`
    /// can be used to determine if what's being imported from is a foreign
    /// interface or not.
    fn populate_foreign_types(&mut self, decl_lists: &[ast::DeclList<'a>]) -> Result<()> {
        for (i, decl_list) in decl_lists.iter().enumerate() {
            self.cur_ast_index = i;
            decl_list.for_each_path(&mut |_, attrs, path, names, _| {
                let names = match names {
                    Some(names) => names,
                    None => return Ok(()),
                };
                let stability = self.stability(attrs)?;
                let (item, name, span) = self.resolve_ast_item_path(path)?;
                let iface = self.extract_iface_from_item(&item, &name, span)?;
                if !self.foreign_interfaces.contains(&iface) {
                    return Ok(());
                }

                let lookup = &mut self.interface_types[iface.index()];
                for name in names {
                    // If this name has already been defined then use that prior
                    // definition, otherwise create a new type with an unknown
                    // representation and insert it into the various maps.
                    if lookup.contains_key(name.name.name) {
                        continue;
                    }
                    let id = self.types.alloc(TypeDef {
                        docs: Docs::default(),
                        stability: stability.clone(),
                        kind: TypeDefKind::Unknown,
                        name: Some(name.name.name.to_string()),
                        owner: TypeOwner::Interface(iface),
                    });
                    self.unknown_type_spans.push(name.name.span);
                    self.type_spans.push(name.name.span);
                    lookup.insert(name.name.name, (TypeOrItem::Type(id), name.name.span));
                    self.interfaces[iface]
                        .types
                        .insert(name.name.name.to_string(), id);
                }

                Ok(())
            })?;
        }
        Ok(())
    }

    fn resolve_world(&mut self, world_id: WorldId, world: &ast::World<'a>) -> Result<WorldId> {
        let docs = self.docs(&world.docs);
        self.worlds[world_id].docs = docs;
        let stability = self.stability(&world.attributes)?;
        self.worlds[world_id].stability = stability;

        self.resolve_types(
            TypeOwner::World(world_id),
            world.items.iter().filter_map(|i| match i {
                ast::WorldItem::Use(u) => Some(TypeItem::Use(u)),
                ast::WorldItem::Type(t) => Some(TypeItem::Def(t)),
                ast::WorldItem::Import(_) | ast::WorldItem::Export(_) => None,
                // should be handled in `wit-parser::resolve`
                ast::WorldItem::Include(_) => None,
            }),
        )?;

        // resolve include items
        let items = world.items.iter().filter_map(|i| match i {
            ast::WorldItem::Include(i) => Some(i),
            _ => None,
        });
        for include in items {
            self.resolve_include(world_id, include)?;
        }

        let mut export_spans = Vec::new();
        let mut import_spans = Vec::new();
        for (name, (item, span)) in self.type_lookup.iter() {
            match *item {
                TypeOrItem::Type(id) => {
                    let prev = self.worlds[world_id]
                        .imports
                        .insert(WorldKey::Name(name.to_string()), WorldItem::Type(id));
                    if prev.is_some() {
                        bail!(Error::new(
                            *span,
                            format!("import `{name}` conflicts with prior import of same name"),
                        ))
                    }
                    import_spans.push(*span);
                }
                TypeOrItem::Item(_) => unreachable!(),
            }
        }

        let mut imported_interfaces = HashSet::new();
        let mut exported_interfaces = HashSet::new();
        for item in world.items.iter() {
            let (docs, attrs, kind, desc, spans, interfaces) = match item {
                ast::WorldItem::Import(import) => (
                    &import.docs,
                    &import.attributes,
                    &import.kind,
                    "import",
                    &mut import_spans,
                    &mut imported_interfaces,
                ),
                ast::WorldItem::Export(export) => (
                    &export.docs,
                    &export.attributes,
                    &export.kind,
                    "export",
                    &mut export_spans,
                    &mut exported_interfaces,
                ),

                ast::WorldItem::Type(ast::TypeDef {
                    name,
                    ty: ast::Type::Resource(r),
                    ..
                }) => {
                    for func in r.funcs.iter() {
                        import_spans.push(func.named_func().name.span);
                        let func = self.resolve_resource_func(func, name)?;
                        let prev = self.worlds[world_id]
                            .imports
                            .insert(WorldKey::Name(func.name.clone()), WorldItem::Function(func));
                        // Resource names themselves are unique, and methods are
                        // uniquely named, so this should be possible to assert
                        // at this point and never trip.
                        assert!(prev.is_none());
                    }
                    continue;
                }

                // handled in `resolve_types`
                ast::WorldItem::Use(_) | ast::WorldItem::Type(_) | ast::WorldItem::Include(_) => {
                    continue;
                }
            };

            let world_item = self.resolve_world_item(docs, attrs, kind)?;
            let key = match kind {
                // Interfaces are always named exactly as they are in the WIT.
                ast::ExternKind::Interface(name, _) => WorldKey::Name(name.name.to_string()),

                // Functions, however, might get mangled (e.g. with async)
                // meaning that the item's name comes from the function, not
                // from the in-WIT name.
                ast::ExternKind::Func(..) => {
                    let func = match &world_item {
                        WorldItem::Function(f) => f,
                        _ => unreachable!(),
                    };
                    WorldKey::Name(func.name.clone())
                }

                ast::ExternKind::Path(path) => {
                    let (item, name, span) = self.resolve_ast_item_path(path)?;
                    let id = self.extract_iface_from_item(&item, &name, span)?;
                    WorldKey::Interface(id)
                }
            };
            if let WorldItem::Interface { id, .. } = world_item {
                if !interfaces.insert(id) {
                    bail!(Error::new(
                        kind.span(),
                        format!("interface cannot be {desc}ed more than once"),
                    ))
                }
            }
            let dst = if desc == "import" {
                &mut self.worlds[world_id].imports
            } else {
                &mut self.worlds[world_id].exports
            };
            let prev = dst.insert(key.clone(), world_item);
            if let Some(prev) = prev {
                let prev = match prev {
                    WorldItem::Interface { .. } => "interface",
                    WorldItem::Function(..) => "func",
                    WorldItem::Type(..) => "type",
                };
                let name = match key {
                    WorldKey::Name(name) => name,
                    WorldKey::Interface(..) => unreachable!(),
                };
                bail!(Error::new(
                    kind.span(),
                    format!("{desc} `{name}` conflicts with prior {prev} of same name",),
                ))
            }
            spans.push(kind.span());
        }
        self.world_spans[world_id.index()].imports = import_spans;
        self.world_spans[world_id.index()].exports = export_spans;
        self.type_lookup.clear();

        Ok(world_id)
    }

    fn resolve_world_item(
        &mut self,
        docs: &ast::Docs<'a>,
        attrs: &[ast::Attribute<'a>],
        kind: &ast::ExternKind<'a>,
    ) -> Result<WorldItem> {
        match kind {
            ast::ExternKind::Interface(name, items) => {
                let prev = mem::take(&mut self.type_lookup);
                let id = self.alloc_interface(name.span);
                self.resolve_interface(id, items, docs, attrs)?;
                self.type_lookup = prev;
                let stability = self.interfaces[id].stability.clone();
                Ok(WorldItem::Interface { id, stability })
            }
            ast::ExternKind::Path(path) => {
                let stability = self.stability(attrs)?;
                let (item, name, span) = self.resolve_ast_item_path(path)?;
                let id = self.extract_iface_from_item(&item, &name, span)?;
                Ok(WorldItem::Interface { id, stability })
            }
            ast::ExternKind::Func(name, func) => {
                let func = self.resolve_function(
                    docs,
                    attrs,
                    &name.name,
                    func,
                    if func.async_ {
                        FunctionKind::AsyncFreestanding
                    } else {
                        FunctionKind::Freestanding
                    },
                )?;
                Ok(WorldItem::Function(func))
            }
        }
    }

    fn resolve_interface(
        &mut self,
        interface_id: InterfaceId,
        fields: &[ast::InterfaceItem<'a>],
        docs: &ast::Docs<'a>,
        attrs: &[ast::Attribute<'a>],
    ) -> Result<()> {
        let docs = self.docs(docs);
        self.interfaces[interface_id].docs = docs;
        let stability = self.stability(attrs)?;
        self.interfaces[interface_id].stability = stability;

        self.resolve_types(
            TypeOwner::Interface(interface_id),
            fields.iter().filter_map(|i| match i {
                ast::InterfaceItem::Use(u) => Some(TypeItem::Use(u)),
                ast::InterfaceItem::TypeDef(t) => Some(TypeItem::Def(t)),
                ast::InterfaceItem::Func(_) => None,
            }),
        )?;

        for (name, (ty, _)) in self.type_lookup.iter() {
            match *ty {
                TypeOrItem::Type(id) => {
                    self.interfaces[interface_id]
                        .types
                        .insert(name.to_string(), id);
                }
                TypeOrItem::Item(_) => unreachable!(),
            }
        }

        // Finally process all function definitions now that all types are
        // defined.
        let mut funcs = Vec::new();
        for field in fields {
            match field {
                ast::InterfaceItem::Func(f) => {
                    self.define_interface_name(&f.name, TypeOrItem::Item("function"))?;
                    funcs.push(self.resolve_function(
                        &f.docs,
                        &f.attributes,
                        &f.name.name,
                        &f.func,
                        if f.func.async_ {
                            FunctionKind::AsyncFreestanding
                        } else {
                            FunctionKind::Freestanding
                        },
                    )?);
                    self.interface_spans[interface_id.index()]
                        .funcs
                        .push(f.name.span);
                }
                ast::InterfaceItem::Use(_) => {}
                ast::InterfaceItem::TypeDef(ast::TypeDef {
                    name,
                    ty: ast::Type::Resource(r),
                    ..
                }) => {
                    for func in r.funcs.iter() {
                        funcs.push(self.resolve_resource_func(func, name)?);
                        self.interface_spans[interface_id.index()]
                            .funcs
                            .push(func.named_func().name.span);
                    }
                }
                ast::InterfaceItem::TypeDef(_) => {}
            }
        }
        for func in funcs {
            let prev = self.interfaces[interface_id]
                .functions
                .insert(func.name.clone(), func);
            assert!(prev.is_none());
        }

        let lookup = mem::take(&mut self.type_lookup);
        self.interface_types[interface_id.index()] = lookup;

        Ok(())
    }

    fn resolve_types<'b>(
        &mut self,
        owner: TypeOwner,
        fields: impl Iterator<Item = TypeItem<'a, 'b>> + Clone,
    ) -> Result<()>
    where
        'a: 'b,
    {
        assert!(self.type_lookup.is_empty());

        // First, populate our namespace with `use` statements
        for field in fields.clone() {
            match field {
                TypeItem::Use(u) => {
                    self.resolve_use(owner, u)?;
                }
                TypeItem::Def(_) => {}
            }
        }

        // Next determine dependencies between types, perform a topological
        // sort, and then define all types. This will define types in a
        // topological fashion, forbid cycles, and weed out references to
        // undefined types all in one go.
        let mut type_deps = IndexMap::new();
        let mut type_defs = IndexMap::new();
        for field in fields {
            match field {
                TypeItem::Def(t) => {
                    let prev = type_defs.insert(t.name.name, Some(t));
                    if prev.is_some() {
                        bail!(Error::new(
                            t.name.span,
                            format!("name `{}` is defined more than once", t.name.name),
                        ))
                    }
                    let mut deps = Vec::new();
                    collect_deps(&t.ty, &mut deps);
                    type_deps.insert(t.name.name, deps);
                }
                TypeItem::Use(u) => {
                    for name in u.names.iter() {
                        let name = name.as_.as_ref().unwrap_or(&name.name);
                        type_deps.insert(name.name, Vec::new());
                        type_defs.insert(name.name, None);
                    }
                }
            }
        }
        let order = toposort("type", &type_deps).map_err(attach_old_float_type_context)?;
        for ty in order {
            let def = match type_defs.swap_remove(&ty).unwrap() {
                Some(def) => def,
                None => continue,
            };
            let docs = self.docs(&def.docs);
            let stability = self.stability(&def.attributes)?;
            let kind = self.resolve_type_def(&def.ty, &stability)?;
            let id = self.types.alloc(TypeDef {
                docs,
                stability,
                kind,
                name: Some(def.name.name.to_string()),
                owner,
            });
            self.type_spans.push(def.name.span);
            self.define_interface_name(&def.name, TypeOrItem::Type(id))?;
        }
        return Ok(());

        fn attach_old_float_type_context(err: ast::toposort::Error) -> anyhow::Error {
            let name = match &err {
                ast::toposort::Error::NonexistentDep { name, .. } => name,
                _ => return err.into(),
            };
            let new = match name.as_str() {
                "float32" => "f32",
                "float64" => "f64",
                _ => return err.into(),
            };

            let context = format!(
                "the `{name}` type has been renamed to `{new}` and is \
                 no longer accepted, but the `WIT_REQUIRE_F32_F64=0` \
                 environment variable can be used to temporarily \
                 disable this error"
            );
            anyhow::Error::from(err).context(context)
        }
    }

    fn resolve_use(&mut self, owner: TypeOwner, u: &ast::Use<'a>) -> Result<()> {
        let (item, name, span) = self.resolve_ast_item_path(&u.from)?;
        let use_from = self.extract_iface_from_item(&item, &name, span)?;
        let stability = self.stability(&u.attributes)?;

        for name in u.names.iter() {
            let lookup = &self.interface_types[use_from.index()];
            let id = match lookup.get(name.name.name) {
                Some((TypeOrItem::Type(id), _)) => *id,
                Some((TypeOrItem::Item(s), _)) => {
                    bail!(Error::new(
                        name.name.span,
                        format!("cannot import {s} `{}`", name.name.name),
                    ))
                }
                None => bail!(Error::new(
                    name.name.span,
                    format!("name `{}` is not defined", name.name.name),
                )),
            };
            self.type_spans.push(name.name.span);
            let name = name.as_.as_ref().unwrap_or(&name.name);
            let id = self.types.alloc(TypeDef {
                docs: Docs::default(),
                stability: stability.clone(),
                kind: TypeDefKind::Type(Type::Id(id)),
                name: Some(name.name.to_string()),
                owner,
            });
            self.define_interface_name(name, TypeOrItem::Type(id))?;
        }
        Ok(())
    }

    /// For each name in the `include`, resolve the path of the include, add it to the self.includes
    fn resolve_include(&mut self, world_id: WorldId, i: &ast::Include<'a>) -> Result<()> {
        let stability = self.stability(&i.attributes)?;
        let (item, name, span) = self.resolve_ast_item_path(&i.from)?;
        let include_from = self.extract_world_from_item(&item, &name, span)?;
        self.worlds[world_id]
            .includes
            .push((stability, include_from));
        self.worlds[world_id].include_names.push(
            i.names
                .iter()
                .map(|n| IncludeName {
                    name: n.name.name.to_string(),
                    as_: n.as_.name.to_string(),
                })
                .collect(),
        );
        self.world_spans[world_id.index()].includes.push(span);
        Ok(())
    }

    fn resolve_resource_func(
        &mut self,
        func: &ast::ResourceFunc<'_>,
        resource: &ast::Id<'_>,
    ) -> Result<Function> {
        let resource_id = match self.type_lookup.get(resource.name) {
            Some((TypeOrItem::Type(id), _)) => *id,
            _ => panic!("type lookup for resource failed"),
        };
        let (name, kind);
        let named_func = func.named_func();
        let async_ = named_func.func.async_;
        match func {
            ast::ResourceFunc::Method(f) => {
                name = format!("[method]{}.{}", resource.name, f.name.name);
                kind = if async_ {
                    FunctionKind::AsyncMethod(resource_id)
                } else {
                    FunctionKind::Method(resource_id)
                };
            }
            ast::ResourceFunc::Static(f) => {
                name = format!("[static]{}.{}", resource.name, f.name.name);
                kind = if async_ {
                    FunctionKind::AsyncStatic(resource_id)
                } else {
                    FunctionKind::Static(resource_id)
                };
            }
            ast::ResourceFunc::Constructor(_) => {
                assert!(!async_); // should not be possible to parse
                name = format!("[constructor]{}", resource.name);
                kind = FunctionKind::Constructor(resource_id);
            }
        }
        self.resolve_function(
            &named_func.docs,
            &named_func.attributes,
            &name,
            &named_func.func,
            kind,
        )
    }

    fn resolve_function(
        &mut self,
        docs: &ast::Docs<'_>,
        attrs: &[ast::Attribute<'_>],
        name: &str,
        func: &ast::Func,
        kind: FunctionKind,
    ) -> Result<Function> {
        let docs = self.docs(docs);
        let stability = self.stability(attrs)?;
        let params = self.resolve_params(&func.params, &kind, func.span)?;
        let result = self.resolve_result(&func.result, &kind, func.span)?;
        Ok(Function {
            docs,
            stability,
            name: name.to_string(),
            kind,
            params,
            result,
        })
    }

    fn resolve_ast_item_path(&self, path: &ast::UsePath<'a>) -> Result<(AstItem, String, Span)> {
        match path {
            ast::UsePath::Id(id) => {
                let item = self.ast_items[self.cur_ast_index]
                    .get(id.name)
                    .or_else(|| self.package_items.get(id.name));
                match item {
                    Some(item) => Ok((*item, id.name.into(), id.span)),
                    None => {
                        bail!(Error::new(
                            id.span,
                            format!("interface or world `{}` does not exist", id.name),
                        ))
                    }
                }
            }
            ast::UsePath::Package { id, name } => Ok((
                self.foreign_deps[&id.package_name()][name.name].0,
                name.name.into(),
                name.span,
            )),
        }
    }

    fn extract_iface_from_item(
        &self,
        item: &AstItem,
        name: &str,
        span: Span,
    ) -> Result<InterfaceId> {
        match item {
            AstItem::Interface(id) => Ok(*id),
            AstItem::World(_) => {
                bail!(Error::new(
                    span,
                    format!("name `{name}` is defined as a world, not an interface"),
                ))
            }
        }
    }

    fn extract_world_from_item(&self, item: &AstItem, name: &str, span: Span) -> Result<WorldId> {
        match item {
            AstItem::World(id) => Ok(*id),
            AstItem::Interface(_) => {
                bail!(Error::new(
                    span,
                    format!("name `{name}` is defined as an interface, not a world"),
                ))
            }
        }
    }

    fn define_interface_name(&mut self, name: &ast::Id<'a>, item: TypeOrItem) -> Result<()> {
        let prev = self.type_lookup.insert(name.name, (item, name.span));
        if prev.is_some() {
            bail!(Error::new(
                name.span,
                format!("name `{}` is defined more than once", name.name),
            ))
        } else {
            Ok(())
        }
    }

    fn resolve_type_def(
        &mut self,
        ty: &ast::Type<'_>,
        stability: &Stability,
    ) -> Result<TypeDefKind> {
        Ok(match ty {
            ast::Type::Bool(_) => TypeDefKind::Type(Type::Bool),
            ast::Type::U8(_) => TypeDefKind::Type(Type::U8),
            ast::Type::U16(_) => TypeDefKind::Type(Type::U16),
            ast::Type::U32(_) => TypeDefKind::Type(Type::U32),
            ast::Type::U64(_) => TypeDefKind::Type(Type::U64),
            ast::Type::S8(_) => TypeDefKind::Type(Type::S8),
            ast::Type::S16(_) => TypeDefKind::Type(Type::S16),
            ast::Type::S32(_) => TypeDefKind::Type(Type::S32),
            ast::Type::S64(_) => TypeDefKind::Type(Type::S64),
            ast::Type::F32(_) => TypeDefKind::Type(Type::F32),
            ast::Type::F64(_) => TypeDefKind::Type(Type::F64),
            ast::Type::Char(_) => TypeDefKind::Type(Type::Char),
            ast::Type::String(_) => TypeDefKind::Type(Type::String),
            ast::Type::ErrorContext(_) => TypeDefKind::Type(Type::ErrorContext),
            ast::Type::Name(name) => {
                let id = self.resolve_type_name(name)?;
                TypeDefKind::Type(Type::Id(id))
            }
            ast::Type::List(list) => {
                let ty = self.resolve_type(&list.ty, stability)?;
                TypeDefKind::List(ty)
            }
            ast::Type::Map(map) => {
                let key_ty = self.resolve_type(&map.key, stability)?;
                let value_ty = self.resolve_type(&map.value, stability)?;

                match key_ty {
                    Type::Bool
                    | Type::U8
                    | Type::U16
                    | Type::U32
                    | Type::U64
                    | Type::S8
                    | Type::S16
                    | Type::S32
                    | Type::S64
                    | Type::Char
                    | Type::String => {}
                    _ => {
                        bail!(Error::new(
                            map.span,
                            "invalid map key type: map keys must be bool, u8, u16, u32, u64, s8, s16, s32, s64, char, or string",
                        ))
                    }
                }

                TypeDefKind::Map(key_ty, value_ty)
            }
            ast::Type::FixedSizeList(list) => {
                let ty = self.resolve_type(&list.ty, stability)?;
                TypeDefKind::FixedSizeList(ty, list.size)
            }
            ast::Type::Handle(handle) => TypeDefKind::Handle(match handle {
                ast::Handle::Own { resource } => Handle::Own(self.validate_resource(resource)?),
                ast::Handle::Borrow { resource } => {
                    Handle::Borrow(self.validate_resource(resource)?)
                }
            }),
            ast::Type::Resource(resource) => {
                // Validate here that the resource doesn't have any duplicate-ly
                // named methods and that there's at most one constructor.
                let mut ctors = 0;
                let mut names = HashSet::new();
                for func in resource.funcs.iter() {
                    match func {
                        ast::ResourceFunc::Method(f) | ast::ResourceFunc::Static(f) => {
                            if !names.insert(&f.name.name) {
                                bail!(Error::new(
                                    f.name.span,
                                    format!("duplicate function name `{}`", f.name.name),
                                ))
                            }
                        }
                        ast::ResourceFunc::Constructor(f) => {
                            ctors += 1;
                            if ctors > 1 {
                                bail!(Error::new(f.name.span, "duplicate constructors"))
                            }
                        }
                    }
                }

                TypeDefKind::Resource
            }
            ast::Type::Record(record) => {
                let fields = record
                    .fields
                    .iter()
                    .map(|field| {
                        Ok(Field {
                            docs: self.docs(&field.docs),
                            name: field.name.name.to_string(),
                            ty: self.resolve_type(&field.ty, stability)?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                TypeDefKind::Record(Record { fields })
            }
            ast::Type::Flags(flags) => {
                let flags = flags
                    .flags
                    .iter()
                    .map(|flag| Flag {
                        docs: self.docs(&flag.docs),
                        name: flag.name.name.to_string(),
                    })
                    .collect::<Vec<_>>();
                TypeDefKind::Flags(Flags { flags })
            }
            ast::Type::Tuple(t) => {
                let types = t
                    .types
                    .iter()
                    .map(|ty| self.resolve_type(ty, stability))
                    .collect::<Result<Vec<_>>>()?;
                TypeDefKind::Tuple(Tuple { types })
            }
            ast::Type::Variant(variant) => {
                if variant.cases.is_empty() {
                    bail!(Error::new(variant.span, "empty variant"))
                }
                let cases = variant
                    .cases
                    .iter()
                    .map(|case| {
                        Ok(Case {
                            docs: self.docs(&case.docs),
                            name: case.name.name.to_string(),
                            ty: self.resolve_optional_type(case.ty.as_ref(), stability)?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                TypeDefKind::Variant(Variant { cases })
            }
            ast::Type::Enum(e) => {
                if e.cases.is_empty() {
                    bail!(Error::new(e.span, "empty enum"))
                }
                let cases = e
                    .cases
                    .iter()
                    .map(|case| {
                        Ok(EnumCase {
                            docs: self.docs(&case.docs),
                            name: case.name.name.to_string(),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                TypeDefKind::Enum(Enum { cases })
            }
            ast::Type::Option(ty) => TypeDefKind::Option(self.resolve_type(&ty.ty, stability)?),
            ast::Type::Result(r) => TypeDefKind::Result(Result_ {
                ok: self.resolve_optional_type(r.ok.as_deref(), stability)?,
                err: self.resolve_optional_type(r.err.as_deref(), stability)?,
            }),
            ast::Type::Future(t) => {
                TypeDefKind::Future(self.resolve_optional_type(t.ty.as_deref(), stability)?)
            }
            ast::Type::Stream(s) => {
                TypeDefKind::Stream(self.resolve_optional_type(s.ty.as_deref(), stability)?)
            }
        })
    }

    fn resolve_type_name(&mut self, name: &ast::Id<'_>) -> Result<TypeId> {
        match self.type_lookup.get(name.name) {
            Some((TypeOrItem::Type(id), _)) => Ok(*id),
            Some((TypeOrItem::Item(s), _)) => bail!(Error::new(
                name.span,
                format!("cannot use {s} `{name}` as a type", name = name.name),
            )),
            None => bail!(Error::new(
                name.span,
                format!("name `{name}` is not defined", name = name.name),
            )),
        }
    }

    fn validate_resource(&mut self, name: &ast::Id<'_>) -> Result<TypeId> {
        let id = self.resolve_type_name(name)?;
        let mut cur = id;
        loop {
            match self.types[cur].kind {
                TypeDefKind::Resource => break Ok(id),
                TypeDefKind::Type(Type::Id(ty)) => cur = ty,
                TypeDefKind::Unknown => {
                    self.required_resource_types.push((cur, name.span));
                    break Ok(id);
                }
                _ => bail!(Error::new(
                    name.span,
                    format!("type `{}` used in a handle must be a resource", name.name),
                )),
            }
        }
    }

    /// If `stability` is `Stability::Unknown`, recursively inspect the
    /// specified `kind` until we either bottom out or find a type which has a
    /// stability that's _not_ unknown.  If we find such a type, return a clone
    /// of its stability; otherwise return `Stability::Unknown`.
    ///
    /// The idea here is that e.g. `option<T>` should inherit `T`'s stability.
    /// This gets a little ambiguous in the case of e.g. `tuple<T, U, V>`; for
    /// now, we just pick the first one has a known stability, if any.
    fn find_stability(&self, kind: &TypeDefKind, stability: &Stability) -> Stability {
        fn find_in_type(types: &Arena<TypeDef>, ty: Type) -> Option<&Stability> {
            if let Type::Id(id) = ty {
                let ty = &types[id];
                if !matches!(&ty.stability, Stability::Unknown) {
                    Some(&ty.stability)
                } else {
                    // Note that this type isn't searched recursively since the
                    // creation of `id` should already have searched its
                    // recursive edges, so there's no need to search again.
                    None
                }
            } else {
                None
            }
        }

        fn find_in_kind<'a>(
            types: &'a Arena<TypeDef>,
            kind: &TypeDefKind,
        ) -> Option<&'a Stability> {
            match kind {
                TypeDefKind::Type(ty) => find_in_type(types, *ty),
                TypeDefKind::Handle(Handle::Borrow(id) | Handle::Own(id)) => {
                    find_in_type(types, Type::Id(*id))
                }
                TypeDefKind::Tuple(t) => t.types.iter().find_map(|ty| find_in_type(types, *ty)),
                TypeDefKind::List(ty)
                | TypeDefKind::FixedSizeList(ty, _)
                | TypeDefKind::Option(ty) => find_in_type(types, *ty),
                TypeDefKind::Map(k, v) => {
                    find_in_type(types, *k).or_else(|| find_in_type(types, *v))
                }
                TypeDefKind::Future(ty) | TypeDefKind::Stream(ty) => {
                    ty.as_ref().and_then(|ty| find_in_type(types, *ty))
                }
                TypeDefKind::Result(r) => {
                    r.ok.as_ref()
                        .and_then(|ty| find_in_type(types, *ty))
                        .or_else(|| r.err.as_ref().and_then(|ty| find_in_type(types, *ty)))
                }
                // Assume these are named types which will be annotated with an
                // explicit stability if applicable:
                TypeDefKind::Resource
                | TypeDefKind::Variant(_)
                | TypeDefKind::Record(_)
                | TypeDefKind::Flags(_)
                | TypeDefKind::Enum(_)
                | TypeDefKind::Unknown => None,
            }
        }

        if let Stability::Unknown = stability {
            find_in_kind(&self.types, kind)
                .cloned()
                .unwrap_or(Stability::Unknown)
        } else {
            stability.clone()
        }
    }

    fn resolve_type(&mut self, ty: &super::Type<'_>, stability: &Stability) -> Result<Type> {
        // Resources must be declared at the top level to have their methods
        // processed appropriately, but resources also shouldn't show up
        // recursively so assert that's not happening here.
        match ty {
            ast::Type::Resource(_) => unreachable!(),
            _ => {}
        }
        let kind = self.resolve_type_def(ty, stability)?;
        let stability = self.find_stability(&kind, stability);
        Ok(self.anon_type_def(
            TypeDef {
                kind,
                name: None,
                docs: Docs::default(),
                stability,
                owner: TypeOwner::None,
            },
            ty.span(),
        ))
    }

    fn resolve_optional_type(
        &mut self,
        ty: Option<&super::Type<'_>>,
        stability: &Stability,
    ) -> Result<Option<Type>> {
        match ty {
            Some(ty) => Ok(Some(self.resolve_type(ty, stability)?)),
            None => Ok(None),
        }
    }

    fn anon_type_def(&mut self, ty: TypeDef, span: Span) -> Type {
        let key = match &ty.kind {
            TypeDefKind::Type(t) => return *t,
            TypeDefKind::Variant(v) => Key::Variant(
                v.cases
                    .iter()
                    .map(|case| (case.name.clone(), case.ty))
                    .collect::<Vec<_>>(),
            ),
            TypeDefKind::Handle(Handle::Borrow(h)) => Key::BorrowHandle(*h),
            // An anonymous `own<T>` type is the same as a reference to the type
            // `T`, so avoid creating anonymous type and return that here
            // directly. Note that this additionally avoids creating distinct
            // anonymous types for `list<T>` and `list<own<T>>` for example.
            TypeDefKind::Handle(Handle::Own(id)) => return Type::Id(*id),
            TypeDefKind::Resource => unreachable!("anonymous resources aren't supported"),
            TypeDefKind::Record(r) => Key::Record(
                r.fields
                    .iter()
                    .map(|case| (case.name.clone(), case.ty))
                    .collect::<Vec<_>>(),
            ),
            TypeDefKind::Flags(r) => {
                Key::Flags(r.flags.iter().map(|f| f.name.clone()).collect::<Vec<_>>())
            }
            TypeDefKind::Tuple(t) => Key::Tuple(t.types.clone()),
            TypeDefKind::Enum(r) => {
                Key::Enum(r.cases.iter().map(|f| f.name.clone()).collect::<Vec<_>>())
            }
            TypeDefKind::List(ty) => Key::List(*ty),
            TypeDefKind::Map(k, v) => Key::Map(*k, *v),
            TypeDefKind::FixedSizeList(ty, size) => Key::FixedSizeList(*ty, *size),
            TypeDefKind::Option(t) => Key::Option(*t),
            TypeDefKind::Result(r) => Key::Result(r.ok, r.err),
            TypeDefKind::Future(ty) => Key::Future(*ty),
            TypeDefKind::Stream(ty) => Key::Stream(*ty),
            TypeDefKind::Unknown => unreachable!(),
        };
        let id = self.anon_types.entry(key).or_insert_with(|| {
            self.type_spans.push(span);
            self.types.alloc(ty)
        });
        Type::Id(*id)
    }

    fn docs(&mut self, doc: &super::Docs<'_>) -> Docs {
        let mut docs = vec![];

        for doc in doc.docs.iter() {
            let contents = match doc.strip_prefix("/**") {
                Some(doc) => doc.strip_suffix("*/").unwrap(),
                None => doc.trim_start_matches('/'),
            };

            docs.push(contents.trim_end());
        }

        // Scan the (non-empty) doc lines to find the minimum amount of leading whitespace.
        // This amount of whitespace will be removed from the start of all doc lines,
        // normalizing the output while retaining intentional spacing added by the original authors.
        let min_leading_ws = docs
            .iter()
            .filter(|doc| !doc.is_empty())
            .map(|doc| doc.bytes().take_while(|c| c.is_ascii_whitespace()).count())
            .min()
            .unwrap_or(0);

        if min_leading_ws > 0 {
            let leading_ws_pattern = " ".repeat(min_leading_ws);
            docs = docs
                .iter()
                .map(|doc| doc.strip_prefix(&leading_ws_pattern).unwrap_or(doc))
                .collect();
        }

        let contents = if docs.is_empty() {
            None
        } else {
            // NB: this notably, through the use of `lines`, normalizes `\r\n`
            // to `\n`.
            let mut contents = String::new();
            for doc in docs {
                if doc.is_empty() {
                    contents.push_str("\n");
                } else {
                    for line in doc.lines() {
                        contents.push_str(line);
                        contents.push_str("\n");
                    }
                }
            }
            while contents.ends_with("\n") {
                contents.pop();
            }
            Some(contents)
        };
        Docs { contents }
    }

    fn stability(&mut self, attrs: &[ast::Attribute<'_>]) -> Result<Stability> {
        match attrs {
            [] => Ok(Stability::Unknown),

            [ast::Attribute::Since { version, .. }] => Ok(Stability::Stable {
                since: version.clone(),
                deprecated: None,
            }),

            [
                ast::Attribute::Since { version, .. },
                ast::Attribute::Deprecated {
                    version: deprecated,
                    ..
                },
            ]
            | [
                ast::Attribute::Deprecated {
                    version: deprecated,
                    ..
                },
                ast::Attribute::Since { version, .. },
            ] => Ok(Stability::Stable {
                since: version.clone(),
                deprecated: Some(deprecated.clone()),
            }),

            [ast::Attribute::Unstable { feature, .. }] => Ok(Stability::Unstable {
                feature: feature.name.to_string(),
                deprecated: None,
            }),

            [
                ast::Attribute::Unstable { feature, .. },
                ast::Attribute::Deprecated { version, .. },
            ]
            | [
                ast::Attribute::Deprecated { version, .. },
                ast::Attribute::Unstable { feature, .. },
            ] => Ok(Stability::Unstable {
                feature: feature.name.to_string(),
                deprecated: Some(version.clone()),
            }),
            [ast::Attribute::Deprecated { span, .. }] => {
                bail!(Error::new(
                    *span,
                    "must pair @deprecated with either @since or @unstable",
                ))
            }
            [_, b, ..] => {
                bail!(Error::new(
                    b.span(),
                    "unsupported combination of attributes",
                ))
            }
        }
    }

    fn resolve_params(
        &mut self,
        params: &ParamList<'_>,
        kind: &FunctionKind,
        span: Span,
    ) -> Result<Vec<(String, Type)>> {
        let mut ret = IndexMap::new();
        match *kind {
            // These kinds of methods don't have any adjustments to the
            // parameters, so do nothing here.
            FunctionKind::Freestanding
            | FunctionKind::AsyncFreestanding
            | FunctionKind::Constructor(_)
            | FunctionKind::Static(_)
            | FunctionKind::AsyncStatic(_) => {}

            // Methods automatically get a `self` initial argument so insert
            // that here before processing the normal parameters.
            FunctionKind::Method(id) | FunctionKind::AsyncMethod(id) => {
                let kind = TypeDefKind::Handle(Handle::Borrow(id));
                let stability = self.find_stability(&kind, &Stability::Unknown);
                let shared = self.anon_type_def(
                    TypeDef {
                        docs: Docs::default(),
                        stability,
                        kind,
                        name: None,
                        owner: TypeOwner::None,
                    },
                    span,
                );
                ret.insert("self".to_string(), shared);
            }
        }
        for (name, ty) in params {
            let prev = ret.insert(
                name.name.to_string(),
                self.resolve_type(ty, &Stability::Unknown)?,
            );
            if prev.is_some() {
                bail!(Error::new(
                    name.span,
                    format!("param `{}` is defined more than once", name.name),
                ))
            }
        }
        Ok(ret.into_iter().collect())
    }

    fn resolve_result(
        &mut self,
        result: &Option<ast::Type<'_>>,
        kind: &FunctionKind,
        _span: Span,
    ) -> Result<Option<Type>> {
        match *kind {
            // These kinds of methods don't have any adjustments to the return
            // values, so plumb them through as-is.
            FunctionKind::Freestanding
            | FunctionKind::AsyncFreestanding
            | FunctionKind::Method(_)
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::Static(_)
            | FunctionKind::AsyncStatic(_) => match result {
                Some(ty) => Ok(Some(self.resolve_type(ty, &Stability::Unknown)?)),
                None => Ok(None),
            },

            FunctionKind::Constructor(id) => match result {
                // When constructors don't define a return type, they're
                // implicitly assumed to return an owned handle to the type
                // they construct.
                None => Ok(Some(Type::Id(id))),

                // If a constructor does define a return type, it must be in the
                // form of `-> result<R, E?>` where `R` is the resource being
                // constructed and `E` is an optional error type.
                Some(ty) => Ok(Some(self.resolve_constructor_result(id, ty)?)),
            },
        }
    }

    fn resolve_constructor_result(
        &mut self,
        resource_id: TypeId,
        result_ast: &ast::Type<'_>,
    ) -> Result<Type> {
        let result = self.resolve_type(result_ast, &Stability::Unknown)?;
        let ok_type = match result {
            Type::Id(id) => match &self.types[id].kind {
                TypeDefKind::Result(r) => Some(r.ok),
                _ => None,
            },
            _ => None,
        };
        let Some(ok_type) = ok_type else {
            bail!(Error::new(
                result_ast.span(),
                "if a constructor return type is declared it must be a `result`",
            ));
        };
        match ok_type {
            Some(Type::Id(ok_id)) if resource_id == ok_id => Ok(result),
            _ => {
                let ok_span =
                    if let ast::Type::Result(ast::Result_ { ok: Some(ok), .. }) = result_ast {
                        ok.span()
                    } else {
                        result_ast.span()
                    };
                bail!(Error::new(
                    ok_span,
                    "the `ok` type must be the resource being constructed",
                ));
            }
        }
    }
}

fn collect_deps<'a>(ty: &ast::Type<'a>, deps: &mut Vec<ast::Id<'a>>) {
    match ty {
        ast::Type::Bool(_)
        | ast::Type::U8(_)
        | ast::Type::U16(_)
        | ast::Type::U32(_)
        | ast::Type::U64(_)
        | ast::Type::S8(_)
        | ast::Type::S16(_)
        | ast::Type::S32(_)
        | ast::Type::S64(_)
        | ast::Type::F32(_)
        | ast::Type::F64(_)
        | ast::Type::Char(_)
        | ast::Type::String(_)
        | ast::Type::Flags(_)
        | ast::Type::Enum(_)
        | ast::Type::ErrorContext(_) => {}
        ast::Type::Name(name) => deps.push(name.clone()),
        ast::Type::Handle(handle) => match handle {
            ast::Handle::Own { resource } => deps.push(resource.clone()),
            ast::Handle::Borrow { resource } => deps.push(resource.clone()),
        },
        ast::Type::Resource(_) => {}
        ast::Type::Record(record) => {
            for field in record.fields.iter() {
                collect_deps(&field.ty, deps);
            }
        }
        ast::Type::Tuple(t) => {
            for ty in t.types.iter() {
                collect_deps(ty, deps);
            }
        }
        ast::Type::Variant(variant) => {
            for case in variant.cases.iter() {
                if let Some(ty) = &case.ty {
                    collect_deps(ty, deps);
                }
            }
        }
        ast::Type::Option(ast::Option_ { ty, .. })
        | ast::Type::List(ast::List { ty, .. })
        | ast::Type::FixedSizeList(ast::FixedSizeList { ty, .. }) => collect_deps(ty, deps),
        ast::Type::Map(ast::Map { key, value, .. }) => {
            collect_deps(key, deps);
            collect_deps(value, deps);
        }
        ast::Type::Result(r) => {
            if let Some(ty) = &r.ok {
                collect_deps(ty, deps);
            }
            if let Some(ty) = &r.err {
                collect_deps(ty, deps);
            }
        }
        ast::Type::Future(t) => {
            if let Some(t) = &t.ty {
                collect_deps(t, deps)
            }
        }
        ast::Type::Stream(s) => {
            if let Some(t) = &s.ty {
                collect_deps(t, deps)
            }
        }
    }
}
