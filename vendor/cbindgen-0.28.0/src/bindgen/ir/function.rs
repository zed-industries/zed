/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use syn::ext::IdentExt;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{AnnotationSet, Cfg, Documentation, GenericPath, Path, Type};
use crate::bindgen::library::Library;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::utilities::IterHelpers;

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Option<String>,
    pub ty: Type,
    pub array_length: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub path: Path,
    /// Path to the self-type of the function
    /// If the function is a method, this will contain the path of the type in the impl block
    pub self_type_path: Option<Path>,
    pub ret: Type,
    pub args: Vec<FunctionArgument>,
    pub extern_decl: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
    pub never_return: bool,
}

impl Function {
    pub fn load(
        path: Path,
        self_type_path: Option<&Path>,
        sig: &syn::Signature,
        extern_decl: bool,
        attrs: &[syn::Attribute],
        mod_cfg: Option<&Cfg>,
    ) -> Result<Function, String> {
        let mut args = sig.inputs.iter().try_skip_map(|x| x.as_argument())?;
        if sig.variadic.is_some() {
            args.push(FunctionArgument {
                name: None,
                ty: Type::Primitive(super::PrimitiveType::VaList),
                array_length: None,
            })
        }

        let (mut ret, never_return) = Type::load_from_output(&sig.output)?;

        if let Some(self_path) = self_type_path {
            for arg in &mut args {
                arg.ty.replace_self_with(self_path);
            }
            ret.replace_self_with(self_path);
        }

        Ok(Function {
            path,
            self_type_path: self_type_path.cloned(),
            ret,
            args,
            extern_decl,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
            never_return,
        })
    }

    pub fn swift_name(&self, config: &Config) -> Option<String> {
        if config.language == Language::Cython {
            return None;
        }
        // If the symbol name starts with the type name, separate the two components with '.'
        // so that Swift recognises the association between the method and the type
        let (ref type_prefix, ref type_name) = match self.self_type_path {
            Some(ref type_name) => {
                let type_name = type_name.to_string();
                if !self.path.name().starts_with(&type_name) {
                    return Some(self.path.to_string());
                }
                (format!("{}.", type_name), type_name)
            }
            None => ("".to_string(), "".to_string()),
        };

        let item_name = self
            .path
            .name()
            .trim_start_matches(type_name)
            .trim_start_matches('_');

        let item_args = {
            let mut items = Vec::with_capacity(self.args.len());
            for arg in self.args.iter() {
                items.push(format!("{}:", arg.name.as_ref()?.as_str()));
            }
            items.join("")
        };
        Some(format!("{}{}({})", type_prefix, item_name, item_args))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        self.ret.simplify_standard_types(config);
        for arg in &mut self.args {
            arg.ty.simplify_standard_types(config);
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ret.add_dependencies(library, out);
        for arg in &self.args {
            arg.ty.add_dependencies(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for arg in &self.args {
            arg.ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.ret.mangle_paths(monomorphs);
        for arg in &mut self.args {
            arg.ty.mangle_paths(monomorphs);
        }
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ret.resolve_declaration_types(resolver);
        for arg in &mut self.args {
            arg.ty.resolve_declaration_types(resolver);
        }
    }

    pub fn rename_for_config(&mut self, config: &Config) {
        // Rename the types used in arguments
        let generic_params = Default::default();
        self.ret.rename_for_config(config, &generic_params);

        // Apply rename rules to argument names
        let rules = self.annotations.parse_atom::<RenameRule>("rename-all");
        let rules = rules.as_ref().unwrap_or(&config.function.rename_args);

        if let Some(r) = rules.not_none() {
            let args = std::mem::take(&mut self.args);
            self.args = args
                .into_iter()
                .map(|arg| {
                    let name = arg
                        .name
                        .map(|n| r.apply(&n, IdentifierType::FunctionArg).into_owned());
                    FunctionArgument {
                        name,
                        ty: arg.ty,
                        array_length: None,
                    }
                })
                .collect()
        }

        // Escape C/C++ reserved keywords used in argument names, and
        // recursively rename argument types.
        for arg in &mut self.args {
            arg.ty.rename_for_config(config, &generic_params);
            if let Some(ref mut name) = arg.name {
                reserved::escape(name);
            }
        }

        // Save the array length of the pointer arguments which need to use
        // the C-array notation
        if let Some(tuples) = self.annotations.list("ptrs-as-arrays") {
            let mut ptrs_as_arrays: HashMap<String, String> = HashMap::new();
            for str_tuple in tuples {
                let parts: Vec<&str> = str_tuple[1..str_tuple.len() - 1]
                    .split(';')
                    .map(|x| x.trim())
                    .collect();
                if parts.len() != 2 {
                    warn!(
                        "{:?} does not follow the correct syntax, so the annotation is being ignored",
                        parts
                    );
                    continue;
                }
                ptrs_as_arrays.insert(parts[0].to_string(), parts[1].to_string());
            }

            for arg in &mut self.args {
                match arg.ty {
                    Type::Ptr { .. } => {}
                    _ => continue,
                }
                let name = match arg.name {
                    Some(ref name) => name,
                    None => continue,
                };
                arg.array_length = ptrs_as_arrays.get(name).cloned();
            }
        }
    }
}

trait SynFnArgHelpers {
    fn as_argument(&self) -> Result<Option<FunctionArgument>, String>;
}

fn gen_self_type(receiver: &syn::Receiver) -> Result<Type, String> {
    let mut self_ty = Type::Path(GenericPath::self_path());

    // Custom self type
    if receiver.colon_token.is_some() {
        self_ty = Type::load(receiver.ty.as_ref())?.unwrap_or(self_ty);
    }

    if receiver.reference.is_none() {
        return Ok(self_ty);
    }

    let is_const = receiver.mutability.is_none();
    Ok(Type::Ptr {
        ty: Box::new(self_ty),
        is_const,
        is_nullable: false,
        is_ref: false,
    })
}

impl SynFnArgHelpers for syn::FnArg {
    fn as_argument(&self) -> Result<Option<FunctionArgument>, String> {
        match *self {
            syn::FnArg::Typed(syn::PatType {
                ref pat, ref ty, ..
            }) => {
                let ty = match Type::load(ty)? {
                    Some(x) => x,
                    None => return Ok(None),
                };
                let name = match **pat {
                    syn::Pat::Wild(..) => None,
                    syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => {
                        if ty == Type::Primitive(super::PrimitiveType::VaList) {
                            None
                        } else {
                            Some(ident.unraw().to_string())
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Parameter has an unsupported argument name: {:?}",
                            pat
                        ))
                    }
                };
                if let Type::Array(..) = ty {
                    return Err("Array as function arguments are not supported".to_owned());
                }
                Ok(Some(FunctionArgument {
                    name,
                    ty,
                    array_length: None,
                }))
            }
            syn::FnArg::Receiver(ref receiver) => Ok(Some(FunctionArgument {
                name: Some("self".to_string()),
                ty: gen_self_type(receiver)?,
                array_length: None,
            })),
        }
    }
}
