/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path as FilePath, PathBuf as FilePathBuf};

use syn::ext::IdentExt;

use crate::bindgen::bitflags;
use crate::bindgen::cargo::{Cargo, PackageRef};
use crate::bindgen::config::{Config, ParseConfig};
use crate::bindgen::error::Error;
use crate::bindgen::ir::{
    AnnotationSet, AnnotationValue, Cfg, Constant, Documentation, Enum, Function, GenericParam,
    GenericParams, ItemMap, OpaqueItem, Path, Static, Struct, Type, Typedef, Union,
};
use crate::bindgen::utilities::{SynAbiHelpers, SynAttributeHelpers, SynItemHelpers};

const STD_CRATES: &[&str] = &[
    "std",
    "std_unicode",
    "alloc",
    "collections",
    "core",
    "proc_macro",
];

type ParseResult = Result<Parse, Error>;

/// Parses a single rust source file, not following `mod` or `extern crate`.
pub fn parse_src(src_file: &FilePath, config: &Config) -> ParseResult {
    let mod_name = src_file.file_stem().unwrap().to_str().unwrap();
    let mut config = config.clone();
    config.parse = ParseConfig {
        parse_deps: true,
        ..ParseConfig::default()
    };

    let mut context = Parser {
        binding_crate_name: mod_name.to_owned(),
        config: &config,
        lib: None,
        parsed_crates: HashSet::new(),
        cache_src: HashMap::new(),
        cache_expanded_crate: HashMap::new(),
        cfg_stack: Vec::new(),
        out: Parse::new(),
    };

    let pkg_ref = PackageRef {
        name: mod_name.to_owned(),
        version: None,
    };

    context.parse_mod(&pkg_ref, src_file, 0)?;
    context.out.source_files = context.cache_src.keys().map(|k| k.to_owned()).collect();
    Ok(context.out)
}

/// Recursively parses a rust library starting at the root crate's directory.
///
/// Inside a crate, `mod` and `extern crate` declarations are followed
/// and parsed. To find an external crate, the parser uses the `cargo metadata`
/// command to find the location of dependencies.
pub(crate) fn parse_lib(lib: Cargo, config: &Config) -> ParseResult {
    let mut context = Parser {
        binding_crate_name: lib.binding_crate_name().to_owned(),
        config,
        lib: Some(lib),
        parsed_crates: HashSet::new(),
        cache_src: HashMap::new(),
        cache_expanded_crate: HashMap::new(),
        cfg_stack: Vec::new(),
        out: Parse::new(),
    };

    let binding_crate = context.lib.as_ref().unwrap().binding_crate_ref();
    context.parse_crate(&binding_crate)?;
    context.out.source_files = context.cache_src.keys().map(|k| k.to_owned()).collect();
    context.out.package_version = context
        .lib
        .as_ref()
        .unwrap()
        .binding_crate_ref()
        .version
        .unwrap();
    Ok(context.out)
}

#[derive(Debug, Clone)]
struct Parser<'a> {
    binding_crate_name: String,
    lib: Option<Cargo>,
    config: &'a Config,

    parsed_crates: HashSet<String>,
    cache_src: HashMap<FilePathBuf, Vec<syn::Item>>,
    cache_expanded_crate: HashMap<String, Vec<syn::Item>>,

    cfg_stack: Vec<Cfg>,

    out: Parse,
}

impl Parser<'_> {
    fn should_parse_dependency(&self, pkg_name: &str) -> bool {
        if self.parsed_crates.contains(pkg_name) {
            return false;
        }

        if !self.config.parse.parse_deps {
            return false;
        }

        // Skip any whitelist or blacklist for expand
        if self
            .config
            .parse
            .expand
            .crates
            .iter()
            .any(|name| name == pkg_name)
        {
            return true;
        }

        // If we have a whitelist, check it
        if let Some(ref include) = self.config.parse.include {
            if !include.iter().any(|name| name == pkg_name) {
                debug!("Excluding crate {}", pkg_name);
                return false;
            }
        }

        // Check the blacklist
        !STD_CRATES.contains(&pkg_name)
            && !self
                .config
                .parse
                .exclude
                .iter()
                .any(|name| name == pkg_name)
    }

    fn parse_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());
        debug!("Parsing crate {}", pkg.name);
        self.parsed_crates.insert(pkg.name.clone());

        // Check if we should use cargo expand for this crate
        if self.config.parse.expand.crates.contains(&pkg.name) {
            self.parse_expand_crate(pkg)?;
        } else {
            // Parse the crate before the dependencies otherwise the same-named idents we
            // want to generate bindings for would be replaced by the ones provided
            // by the first dependency containing it.
            let crate_src = self.lib.as_ref().unwrap().find_crate_src(pkg);

            match crate_src {
                Some(crate_src) => self.parse_mod(pkg, crate_src.as_path(), 0)?,
                None => {
                    // This should be an error, but is common enough to just elicit a warning
                    warn!(
                        "Parsing crate `{}`: can't find lib.rs with `cargo metadata`. \
                        The crate may be available only on a particular platform, \
                        so consider setting `fetch_all_dependencies` in your cbindgen configuration.",
                        pkg.name
                    );
                }
            }
        }

        for (dep_pkg, cfg) in self.lib.as_ref().unwrap().dependencies(pkg) {
            if !self.should_parse_dependency(&dep_pkg.name) {
                continue;
            }

            if let Some(ref cfg) = cfg {
                self.cfg_stack.push(cfg.clone());
            }

            self.parse_crate(&dep_pkg)?;

            if cfg.is_some() {
                self.cfg_stack.pop();
            }
        }

        Ok(())
    }

    fn parse_expand_crate(&mut self, pkg: &PackageRef) -> Result<(), Error> {
        assert!(self.lib.is_some());

        let mod_items = {
            if !self.cache_expanded_crate.contains_key(&pkg.name) {
                let s = self
                    .lib
                    .as_ref()
                    .unwrap()
                    .expand_crate(
                        pkg,
                        self.config.parse.expand.all_features,
                        self.config.parse.expand.default_features,
                        &self.config.parse.expand.features,
                        self.config.parse.expand.profile,
                    )
                    .map_err(|x| Error::CargoExpand(pkg.name.clone(), x))?;
                let i = syn::parse_file(&s).map_err(|x| Error::ParseSyntaxError {
                    crate_name: pkg.name.clone(),
                    src_path: "".to_owned(),
                    error: x,
                })?;
                self.cache_expanded_crate.insert(pkg.name.clone(), i.items);
            }

            self.cache_expanded_crate.get(&pkg.name).unwrap().clone()
        };

        self.process_mod(
            pkg, None, None, &mod_items, 0, /* is_mod_rs = */ true,
            /* is_inline = */ false,
        )
    }

    fn parse_mod(
        &mut self,
        pkg: &PackageRef,
        mod_path: &FilePath,
        depth: usize,
    ) -> Result<(), Error> {
        let mod_items = match self.cache_src.entry(mod_path.to_path_buf()) {
            Entry::Vacant(vacant_entry) => {
                let mut s = String::new();
                let mut f = File::open(mod_path).map_err(|_| Error::ParseCannotOpenFile {
                    crate_name: pkg.name.clone(),
                    src_path: mod_path.to_str().unwrap().to_owned(),
                })?;

                f.read_to_string(&mut s)
                    .map_err(|_| Error::ParseCannotOpenFile {
                        crate_name: pkg.name.clone(),
                        src_path: mod_path.to_str().unwrap().to_owned(),
                    })?;

                let i = syn::parse_file(&s).map_err(|x| Error::ParseSyntaxError {
                    crate_name: pkg.name.clone(),
                    src_path: mod_path.to_string_lossy().into(),
                    error: x,
                })?;

                vacant_entry.insert(i.items).clone()
            }
            Entry::Occupied(occupied_entry) => occupied_entry.get().clone(),
        };

        // Compute module directory according to Rust 2018 rules
        let submod_dir_2018;

        let mod_dir = mod_path.parent().unwrap();

        let is_mod_rs = depth == 0 || mod_path.ends_with("mod.rs");
        let submod_dir = if is_mod_rs {
            mod_dir
        } else {
            submod_dir_2018 = mod_path
                .parent()
                .unwrap()
                .join(mod_path.file_stem().unwrap());
            &submod_dir_2018
        };

        self.process_mod(
            pkg,
            Some(mod_dir),
            Some(submod_dir),
            &mod_items,
            depth,
            /* is_inline = */ false,
            is_mod_rs,
        )
    }

    /// `mod_dir` is the path to the current directory of the module. It may be
    /// `None` for pre-expanded modules.
    ///
    /// `submod_dir` is the path to search submodules in by default, which might
    /// be different for rust 2018 for example.
    #[allow(clippy::too_many_arguments)]
    fn process_mod(
        &mut self,
        pkg: &PackageRef,
        mod_dir: Option<&FilePath>,
        submod_dir: Option<&FilePath>,
        items: &[syn::Item],
        depth: usize,
        is_inline: bool,
        is_in_mod_rs: bool,
    ) -> Result<(), Error> {
        debug_assert_eq!(mod_dir.is_some(), submod_dir.is_some());
        // We process the items first then the nested modules.
        let nested_modules = self.out.load_syn_crate_mod(
            self.config,
            &self.binding_crate_name,
            &pkg.name,
            Cfg::join(&self.cfg_stack).as_ref(),
            items,
        );

        for item in nested_modules {
            let next_mod_name = item.ident.unraw().to_string();
            let cfg = Cfg::load(&item.attrs);
            if let Some(ref cfg) = cfg {
                self.cfg_stack.push(cfg.clone());
            }

            if let Some((_, ref inline_items)) = item.content {
                // TODO(emilio): This should use #[path] attribute if present,
                // rather than next_mod_name.
                let next_submod_dir = submod_dir.map(|dir| dir.join(&next_mod_name));
                let next_mod_dir = mod_dir.map(|dir| dir.join(&next_mod_name));
                self.process_mod(
                    pkg,
                    next_mod_dir.as_deref(),
                    next_submod_dir.as_deref(),
                    inline_items,
                    depth,
                    /* is_inline = */ true,
                    is_in_mod_rs,
                )?;
            } else if let Some(mod_dir) = mod_dir {
                let submod_dir = submod_dir.unwrap();
                let next_mod_path1 = submod_dir.join(next_mod_name.clone() + ".rs");
                let next_mod_path2 = submod_dir.join(next_mod_name.clone()).join("mod.rs");

                if next_mod_path1.exists() {
                    self.parse_mod(pkg, next_mod_path1.as_path(), depth + 1)?;
                } else if next_mod_path2.exists() {
                    self.parse_mod(pkg, next_mod_path2.as_path(), depth + 1)?;
                } else {
                    // Last chance to find a module path
                    let mut path_attr_found = false;
                    for attr in &item.attrs {
                        if let syn::Meta::NameValue(syn::MetaNameValue {
                            path,
                            value: syn::Expr::Lit(syn::ExprLit { lit, .. }),
                            ..
                        }) = &attr.meta
                        {
                            match lit {
                                syn::Lit::Str(ref path_lit) if path.is_ident("path") => {
                                    path_attr_found = true;
                                    // https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute
                                    //
                                    //     For path attributes on modules not inside inline module blocks, the file path
                                    //     is relative to the directory the source file is located.
                                    //
                                    //     For path attributes inside inline module blocks, the relative location of the
                                    //     file path depends on the kind of source file the path attribute is located
                                    //     in.  "mod-rs" source files are root modules (such as lib.rs or main.rs) and
                                    //     modules with files named mod.rs. "non-mod-rs" source files are all other
                                    //     module files.
                                    //
                                    //     Paths for path attributes inside inline module blocks in a mod-rs file are
                                    //     relative to the directory of the mod-rs file including the inline module
                                    //     components as directories. For non-mod-rs files, it is the same except the
                                    //     path starts with a directory with the name of the non-mod-rs module.
                                    //
                                    let base = if is_inline && !is_in_mod_rs {
                                        submod_dir
                                    } else {
                                        mod_dir
                                    };
                                    self.parse_mod(pkg, &base.join(path_lit.value()), depth + 1)?;
                                    break;
                                }
                                _ => (),
                            }
                        }
                    }

                    // This should be an error, but it's common enough to
                    // just elicit a warning
                    if !path_attr_found {
                        warn!(
                            "Parsing crate `{}`: can't find mod {}`.",
                            pkg.name, next_mod_name
                        );
                    }
                }
            } else {
                warn!(
                    "Parsing expanded crate `{}`: can't find mod {}`.",
                    pkg.name, next_mod_name
                );
            }

            if cfg.is_some() {
                self.cfg_stack.pop();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub constants: ItemMap<Constant>,
    pub globals: ItemMap<Static>,
    pub enums: ItemMap<Enum>,
    pub structs: ItemMap<Struct>,
    pub unions: ItemMap<Union>,
    pub opaque_items: ItemMap<OpaqueItem>,
    pub typedefs: ItemMap<Typedef>,
    pub functions: Vec<Function>,
    pub source_files: Vec<FilePathBuf>,
    pub package_version: String,
}

impl Parse {
    pub fn new() -> Parse {
        Parse {
            constants: ItemMap::default(),
            globals: ItemMap::default(),
            enums: ItemMap::default(),
            structs: ItemMap::default(),
            unions: ItemMap::default(),
            opaque_items: ItemMap::default(),
            typedefs: ItemMap::default(),
            functions: Vec::new(),
            source_files: Vec::new(),
            package_version: String::new(),
        }
    }

    pub fn add_std_types(&mut self) {
        let mut add_opaque = |path: &str, generic_params: Vec<&str>| {
            let path = Path::new(path);
            let generic_params: Vec<_> = generic_params
                .into_iter()
                .map(GenericParam::new_type_param)
                .collect();
            self.opaque_items.try_insert(OpaqueItem::new(
                path,
                GenericParams(generic_params),
                None,
                AnnotationSet::new(),
                Documentation::none(),
            ))
        };

        add_opaque("String", vec![]);
        add_opaque("Box", vec!["T"]);
        add_opaque("RefCell", vec!["T"]);
        add_opaque("Rc", vec!["T"]);
        add_opaque("Arc", vec!["T"]);
        add_opaque("Result", vec!["T", "E"]);
        add_opaque("Option", vec!["T"]);
        add_opaque("NonNull", vec!["T"]);
        add_opaque("Vec", vec!["T"]);
        add_opaque("HashMap", vec!["K", "V", "Hasher"]);
        add_opaque("BTreeMap", vec!["K", "V"]);
        add_opaque("HashSet", vec!["T"]);
        add_opaque("BTreeSet", vec!["T"]);
        add_opaque("LinkedList", vec!["T"]);
        add_opaque("VecDeque", vec!["T"]);
        add_opaque("ManuallyDrop", vec!["T"]);
        add_opaque("MaybeUninit", vec!["T"]);
    }

    pub fn extend_with(&mut self, other: &Parse) {
        self.constants.extend_with(&other.constants);
        self.globals.extend_with(&other.globals);
        self.enums.extend_with(&other.enums);
        self.structs.extend_with(&other.structs);
        self.unions.extend_with(&other.unions);
        self.opaque_items.extend_with(&other.opaque_items);
        self.typedefs.extend_with(&other.typedefs);
        self.functions.extend_from_slice(&other.functions);
        self.source_files.extend_from_slice(&other.source_files);
        self.package_version.clone_from(&other.package_version);
    }

    fn load_syn_crate_mod<'a>(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        items: &'a [syn::Item],
    ) -> Vec<&'a syn::ItemMod> {
        let mut impls_with_assoc_consts = Vec::new();
        let mut nested_modules = Vec::new();

        for item in items {
            if item.should_skip_parsing() {
                continue;
            }
            match item {
                syn::Item::ForeignMod(ref item) => {
                    self.load_syn_foreign_mod(
                        config,
                        binding_crate_name,
                        crate_name,
                        mod_cfg,
                        item,
                    );
                }
                syn::Item::Fn(ref item) => {
                    self.load_syn_fn(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Const(ref item) => {
                    self.load_syn_const(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Static(ref item) => {
                    self.load_syn_static(config, binding_crate_name, crate_name, mod_cfg, item);
                }
                syn::Item::Struct(ref item) => {
                    self.load_syn_struct(config, crate_name, mod_cfg, item);
                }
                syn::Item::Union(ref item) => {
                    self.load_syn_union(config, crate_name, mod_cfg, item);
                }
                syn::Item::Enum(ref item) => {
                    self.load_syn_enum(config, crate_name, mod_cfg, item);
                }
                syn::Item::Type(ref item) => {
                    self.load_syn_ty(crate_name, mod_cfg, item);
                }
                syn::Item::Impl(ref item_impl) => {
                    let has_assoc_const = item_impl
                        .items
                        .iter()
                        .any(|item| matches!(item, syn::ImplItem::Const(_)));
                    if has_assoc_const {
                        impls_with_assoc_consts.push(item_impl);
                    }

                    if let syn::Type::Path(ref path) = *item_impl.self_ty {
                        if let Some(type_name) = path.path.get_ident() {
                            for method in item_impl.items.iter().filter_map(|item| match item {
                                syn::ImplItem::Fn(method) if !method.should_skip_parsing() => {
                                    Some(method)
                                }
                                _ => None,
                            }) {
                                self.load_syn_method(
                                    config,
                                    binding_crate_name,
                                    crate_name,
                                    mod_cfg,
                                    &Path::new(type_name.unraw().to_string()),
                                    method,
                                )
                            }
                        }
                    }
                }
                syn::Item::Macro(ref item) => {
                    self.load_builtin_macro(config, crate_name, mod_cfg, item);
                }
                syn::Item::Mod(ref item) => {
                    nested_modules.push(item);
                }
                _ => {}
            }
        }

        for item_impl in impls_with_assoc_consts {
            self.load_syn_assoc_consts_from_impl(crate_name, mod_cfg, item_impl)
        }

        nested_modules
    }

    fn load_syn_assoc_consts_from_impl(
        &mut self,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item_impl: &syn::ItemImpl,
    ) {
        let associated_constants = item_impl.items.iter().filter_map(|item| match item {
            syn::ImplItem::Const(ref associated_constant)
                if !associated_constant.should_skip_parsing() =>
            {
                Some(associated_constant)
            }
            _ => None,
        });
        self.load_syn_assoc_consts(
            crate_name,
            mod_cfg,
            &item_impl.self_ty,
            associated_constants,
        );
    }

    /// Enters a `extern "C" { }` declaration and loads function declarations.
    fn load_syn_foreign_mod(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemForeignMod,
    ) {
        if !item.abi.is_c() && !item.abi.is_omitted() {
            info!("Skip {} - (extern block must be extern C).", crate_name);
            return;
        }

        let mod_cfg = Cfg::append(mod_cfg, Cfg::load(&item.attrs));
        for foreign_item in &item.items {
            if let syn::ForeignItem::Fn(ref function) = *foreign_item {
                if !config
                    .parse
                    .should_generate_top_level_item(crate_name, binding_crate_name)
                {
                    info!(
                        "Skip {}::{} - (fn's outside of the binding crate are not used).",
                        crate_name, &function.sig.ident
                    );
                    return;
                }
                let path = Path::new(function.sig.ident.unraw().to_string());
                match Function::load(
                    path,
                    None,
                    &function.sig,
                    true,
                    &function.attrs,
                    mod_cfg.as_ref(),
                ) {
                    Ok(func) => {
                        info!("Take {}::{}.", crate_name, &function.sig.ident);

                        self.functions.push(func);
                    }
                    Err(msg) => {
                        error!(
                            "Cannot use fn {}::{} ({}).",
                            crate_name, &function.sig.ident, msg
                        );
                    }
                }
            }
        }
    }

    /// Loads a `fn` declaration inside an `impl` block, if the type is a simple identifier
    fn load_syn_method(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        self_type: &Path,
        item: &syn::ImplItemFn,
    ) {
        self.load_fn_declaration(
            config,
            binding_crate_name,
            crate_name,
            mod_cfg,
            item,
            Some(self_type),
            &item.sig,
            &item.attrs,
        )
    }

    /// Loads a `fn` declaration
    fn load_syn_fn(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemFn,
    ) {
        self.load_fn_declaration(
            config,
            binding_crate_name,
            crate_name,
            mod_cfg,
            item,
            None,
            &item.sig,
            &item.attrs,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn load_fn_declaration(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        named_symbol: &dyn SynItemHelpers,
        self_type: Option<&Path>,
        sig: &syn::Signature,
        attrs: &[syn::Attribute],
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (fn's outside of the binding crate are not used).",
                crate_name, &sig.ident
            );
            return;
        }

        let loggable_item_name = || {
            let mut items = Vec::with_capacity(3);
            items.push(crate_name.to_owned());
            if let Some(ref self_type) = self_type {
                items.push(self_type.to_string());
            }
            items.push(sig.ident.unraw().to_string());
            items.join("::")
        };

        let is_extern_c = sig.abi.is_omitted() || sig.abi.is_c();
        let exported_name = named_symbol.exported_name();

        match (is_extern_c, exported_name) {
            (true, Some(exported_name)) => {
                let path = Path::new(exported_name);
                match Function::load(path, self_type, sig, false, attrs, mod_cfg) {
                    Ok(func) => {
                        info!("Take {}.", loggable_item_name());
                        self.functions.push(func);
                    }
                    Err(msg) => {
                        error!("Cannot use fn {} ({}).", loggable_item_name(), msg);
                    }
                }
            }
            (true, None) => {
                warn!(
                    "Skipping {} - (not `no_mangle`, and has no `export_name` attribute)",
                    loggable_item_name()
                );
            }
            (false, Some(_exported_name)) => {
                warn!("Skipping {} - (not `extern \"C\"`)", loggable_item_name());
            }
            (false, None) => {}
        }
    }

    /// Loads associated `const` declarations
    fn load_syn_assoc_consts<'a, I>(
        &mut self,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        impl_ty: &syn::Type,
        items: I,
    ) where
        I: IntoIterator<Item = &'a syn::ImplItemConst>,
    {
        let ty = match Type::load(impl_ty) {
            Ok(ty) => ty,
            Err(e) => {
                warn!("Skipping associated constants for {:?}: {:?}", impl_ty, e);
                return;
            }
        };

        let ty = match ty {
            Some(ty) => ty,
            None => return,
        };

        let impl_path = match ty.get_root_path() {
            Some(p) => p,
            None => {
                warn!(
                    "Couldn't find path for {:?}, skipping associated constants",
                    ty
                );
                return;
            }
        };

        for item in items.into_iter() {
            if let syn::Visibility::Public(_) = item.vis {
            } else {
                warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
                return;
            }

            let path = Path::new(item.ident.unraw().to_string());
            match Constant::load(
                path,
                mod_cfg,
                &item.ty,
                &item.expr,
                &item.attrs,
                Some(impl_path.clone()),
            ) {
                Ok(constant) => {
                    info!("Take {}::{}::{}.", crate_name, impl_path, &item.ident);
                    let mut any = false;
                    self.structs.for_items_mut(&impl_path, |item| {
                        any = true;
                        item.add_associated_constant(constant.clone());
                    });
                    // Handle associated constants to other item types that are
                    // not structs like enums or such as regular constants.
                    if !any && !self.constants.try_insert(constant) {
                        error!(
                            "Conflicting name for constant {}::{}::{}.",
                            crate_name, impl_path, &item.ident,
                        );
                    }
                }
                Err(msg) => {
                    warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
                }
            }
        }
    }

    /// Loads a `const` declaration
    fn load_syn_const(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemConst,
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (const's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        if let syn::Visibility::Public(_) = item.vis {
        } else {
            warn!("Skip {}::{} - (not `pub`).", crate_name, &item.ident);
            return;
        }

        let path = Path::new(item.ident.unraw().to_string());
        match Constant::load(path, mod_cfg, &item.ty, &item.expr, &item.attrs, None) {
            Ok(constant) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                let full_name = constant.path.clone();
                if !self.constants.try_insert(constant) {
                    error!("Conflicting name for constant {}", full_name);
                }
            }
            Err(msg) => {
                warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
            }
        }
    }

    /// Loads a `static` declaration
    fn load_syn_static(
        &mut self,
        config: &Config,
        binding_crate_name: &str,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemStatic,
    ) {
        if !config
            .parse
            .should_generate_top_level_item(crate_name, binding_crate_name)
        {
            info!(
                "Skip {}::{} - (static's outside of the binding crate are not used).",
                crate_name, &item.ident
            );
            return;
        }

        if let Some(exported_name) = item.exported_name() {
            let path = Path::new(exported_name);
            match Static::load(path, item, mod_cfg) {
                Ok(constant) => {
                    info!("Take {}::{}.", crate_name, &item.ident);
                    self.globals.try_insert(constant);
                }
                Err(msg) => {
                    warn!("Skip {}::{} - ({})", crate_name, &item.ident, msg);
                }
            }
        } else {
            warn!("Skip {}::{} - (not `no_mangle`).", crate_name, &item.ident);
        }
    }

    /// Loads a `struct` declaration
    fn load_syn_struct(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemStruct,
    ) {
        match Struct::load(&config.layout, item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);
                self.structs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.unraw().to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `union` declaration
    fn load_syn_union(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemUnion,
    ) {
        match Union::load(&config.layout, item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.unions.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.unraw().to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `enum` declaration
    fn load_syn_enum(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemEnum,
    ) {
        match Enum::load(item, mod_cfg, config) {
            Ok(en) => {
                info!("Take {}::{}.", crate_name, &item.ident);
                self.enums.try_insert(en);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.unraw().to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    /// Loads a `type` declaration
    fn load_syn_ty(&mut self, crate_name: &str, mod_cfg: Option<&Cfg>, item: &syn::ItemType) {
        match Typedef::load(item, mod_cfg) {
            Ok(st) => {
                info!("Take {}::{}.", crate_name, &item.ident);

                self.typedefs.try_insert(st);
            }
            Err(msg) => {
                info!("Take {}::{} - opaque ({}).", crate_name, &item.ident, msg);
                let path = Path::new(item.ident.unraw().to_string());
                self.opaque_items.try_insert(
                    OpaqueItem::load(path, &item.generics, &item.attrs, mod_cfg).unwrap(),
                );
            }
        }
    }

    fn load_builtin_macro(
        &mut self,
        config: &Config,
        crate_name: &str,
        mod_cfg: Option<&Cfg>,
        item: &syn::ItemMacro,
    ) {
        let name = match item.mac.path.segments.last() {
            Some(n) => n.ident.unraw().to_string(),
            None => return,
        };

        if name != "bitflags" || !config.macro_expansion.bitflags {
            return;
        }

        let bitflags = match bitflags::parse(item.mac.tokens.clone()) {
            Ok(bf) => bf,
            Err(e) => {
                warn!("Failed to parse bitflags invocation: {:?}", e);
                return;
            }
        };

        let (struct_, impl_) = bitflags.expand();
        if let Some(struct_) = struct_ {
            self.load_syn_struct(config, crate_name, mod_cfg, &struct_);
        }
        if let syn::Type::Path(ref path) = *impl_.self_ty {
            if let Some(type_name) = path.path.get_ident() {
                self.structs
                    .for_items_mut(&Path::new(type_name.unraw().to_string()), |item| {
                        item.annotations
                            .add_default("internal-derive-bitflags", AnnotationValue::Bool(true));
                    });
            }
        }
        self.load_syn_assoc_consts_from_impl(crate_name, mod_cfg, &impl_)
    }
}
