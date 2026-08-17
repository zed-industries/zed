/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::io::Write;

use crate::bindgen::cargo::cargo_metadata::Dependency;
use crate::bindgen::config::{Config, Language};
use crate::bindgen::writer::SourceWriter;

#[derive(PartialEq, Eq)]
enum DefineKey<'a> {
    Boolean(&'a str),
    Named(&'a str, &'a str),
}

impl DefineKey<'_> {
    fn load(key: &str) -> DefineKey {
        // TODO: dirty parser
        if !key.contains('=') {
            return DefineKey::Boolean(key);
        }

        let mut splits = key.trim().split('=');

        let name = match splits.next() {
            Some(n) => n.trim(),
            None => return DefineKey::Boolean(key),
        };

        let value = match splits.next() {
            Some(v) => v.trim(),
            None => return DefineKey::Boolean(key),
        };

        if splits.next().is_some() {
            return DefineKey::Boolean(key);
        }

        DefineKey::Named(name, value)
    }
}

#[derive(Debug, Clone)]
pub enum Cfg {
    Boolean(String),
    Named(String, String),
    Any(Vec<Cfg>),
    All(Vec<Cfg>),
    Not(Box<Cfg>),
}

impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cfg::Boolean(key) => write!(f, "{}", key),
            Cfg::Named(key, value) => write!(f, "{} = {:?}", key, value),
            Cfg::Any(cfgs) => {
                write!(f, "any(")?;
                for (index, cfg) in cfgs.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", cfg)?;
                }
                write!(f, ")")
            }
            Cfg::All(cfgs) => {
                write!(f, "all(")?;
                for (index, cfg) in cfgs.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", cfg)?;
                }
                write!(f, ")")
            }
            Cfg::Not(cfg) => write!(f, "not({})", cfg),
        }
    }
}

impl syn::parse::Parse for Cfg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arg: syn::Meta = input.parse()?;

        match arg {
            syn::Meta::Path(path) => path
                .get_ident()
                .map(|ident| Cfg::Boolean(ident.to_string()))
                .ok_or_else(|| input.error("path must be identifier")),
            syn::Meta::NameValue(syn::MetaNameValue {
                path,
                value:
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit),
                        ..
                    }),
                ..
            }) => path
                .get_ident()
                .map(|ident| Cfg::Named(ident.to_string(), lit.value()))
                .ok_or_else(|| input.error("path must be identifier")),
            syn::Meta::List(meta) => {
                if meta.path.is_ident("not") {
                    let cfg = meta.parse_args()?;
                    Ok(Cfg::Not(Box::new(cfg)))
                } else if meta.path.is_ident("all") {
                    let cfgs = meta.parse_args_with(
                        syn::punctuated::Punctuated::<Cfg, syn::Token![,]>::parse_terminated,
                    )?;

                    Ok(Cfg::All(cfgs.into_iter().collect()))
                } else if meta.path.is_ident("any") {
                    let cfgs = meta.parse_args_with(
                        syn::punctuated::Punctuated::<Cfg, syn::Token![,]>::parse_terminated,
                    )?;

                    Ok(Cfg::Any(cfgs.into_iter().collect()))
                } else {
                    Err(input.error("invalid list argument"))
                }
            }
            _ => Err(input.error("Failed to parse cfg")),
        }
    }
}

impl Cfg {
    pub fn join(cfgs: &[Cfg]) -> Option<Cfg> {
        if cfgs.is_empty() {
            None
        } else {
            Some(Cfg::All(cfgs.to_owned()))
        }
    }

    pub fn append(parent: Option<&Cfg>, child: Option<Cfg>) -> Option<Cfg> {
        match (parent, child) {
            (None, None) => None,
            (None, Some(child)) => Some(child),
            (Some(parent), None) => Some(parent.clone()),
            (Some(parent), Some(child)) => Some(Cfg::All(vec![parent.clone(), child])),
        }
    }

    pub fn load(attrs: &[syn::Attribute]) -> Option<Cfg> {
        let mut configs = Vec::new();

        for attr in attrs {
            if let syn::Meta::List(meta @ syn::MetaList { path, .. }) = &attr.meta {
                if !path.is_ident("cfg") {
                    continue;
                }

                let cfg = meta.parse_args().ok();

                if let Some(config) = cfg {
                    configs.push(config);
                }
            }
        }

        match configs.len() {
            0 => None,
            1 => Some(configs.pop().unwrap()),
            _ => Some(Cfg::All(configs)),
        }
    }

    pub fn load_metadata(dependency: &Dependency) -> Option<Cfg> {
        let target = dependency.target.as_ref()?;
        match syn::parse_str::<syn::Meta>(target) {
            Ok(target) => {
                // Parsing succeeded using the #[cfg] syntax
                if let syn::Meta::List(meta) = target {
                    if !meta.path.is_ident("cfg") {
                        return None;
                    }
                    meta.parse_args().ok()
                } else {
                    None
                }
            }
            Err(_) => {
                // Parsing failed using #[cfg], this may be a literal target
                // name
                Some(Cfg::Boolean(target.clone()))
            }
        }
    }
}

pub trait ToCondition: Sized {
    fn to_condition(&self, config: &Config) -> Option<Condition>;
}

impl ToCondition for Option<Cfg> {
    fn to_condition(&self, config: &Config) -> Option<Condition> {
        self.as_ref()?.to_condition(config)
    }
}

impl ToCondition for Cfg {
    fn to_condition(&self, config: &Config) -> Option<Condition> {
        match *self {
            Cfg::Boolean(ref cfg_name) => {
                let define = config
                    .defines
                    .iter()
                    .find(|(key, ..)| DefineKey::Boolean(cfg_name) == DefineKey::load(key));
                if let Some((_, define)) = define {
                    Some(Condition::Define(define.to_owned()))
                } else {
                    warn!(
                        "Missing `[defines]` entry for `{}` in cbindgen config.",
                        self,
                    );
                    None
                }
            }
            Cfg::Named(ref cfg_name, ref cfg_value) => {
                let define = config.defines.iter().find(|(key, ..)| {
                    DefineKey::Named(cfg_name, cfg_value) == DefineKey::load(key)
                });
                if let Some((_, define)) = define {
                    Some(Condition::Define(define.to_owned()))
                } else {
                    warn!(
                        "Missing `[defines]` entry for `{}` in cbindgen config.",
                        self,
                    );
                    None
                }
            }
            Cfg::Any(ref children) => {
                let conditions: Vec<_> = children
                    .iter()
                    .filter_map(|x| x.to_condition(config))
                    .collect();
                match conditions.len() {
                    0 => None,
                    1 => conditions.into_iter().next(),
                    _ => Some(Condition::Any(conditions)),
                }
            }
            Cfg::All(ref children) => {
                let cfgs: Vec<_> = children
                    .iter()
                    .filter_map(|x| x.to_condition(config))
                    .collect();
                match cfgs.len() {
                    0 => None,
                    1 => cfgs.into_iter().next(),
                    _ => Some(Condition::All(cfgs)),
                }
            }
            Cfg::Not(ref child) => child
                .to_condition(config)
                .map(|cfg| Condition::Not(Box::new(cfg))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Define(String),
    Any(Vec<Condition>),
    All(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        match *self {
            Condition::Define(ref define) => {
                if config.language == Language::Cython {
                    write!(out, "{}", define);
                } else {
                    out.write("defined(");
                    write!(out, "{}", define);
                    out.write(")");
                }
            }
            Condition::Any(ref conditions) => {
                out.write("(");
                for (i, condition) in conditions.iter().enumerate() {
                    if i != 0 {
                        out.write(if config.language == Language::Cython {
                            " or "
                        } else {
                            " || "
                        });
                    }
                    condition.write(config, out);
                }
                out.write(")");
            }
            Condition::All(ref conditions) => {
                out.write("(");
                for (i, condition) in conditions.iter().enumerate() {
                    if i != 0 {
                        out.write(if config.language == Language::Cython {
                            " and "
                        } else {
                            " && "
                        });
                    }
                    condition.write(config, out);
                }
                out.write(")");
            }
            Condition::Not(ref condition) => {
                out.write(if config.language == Language::Cython {
                    "not "
                } else {
                    "!"
                });
                condition.write(config, out);
            }
        }
    }
}

pub trait ConditionWrite {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);
    fn write_after<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>);
}

impl ConditionWrite for Option<Condition> {
    fn write_before<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if let Some(ref cfg) = *self {
            if config.language == Language::Cython {
                out.write("IF ");
                cfg.write(config, out);
                out.open_brace();
            } else {
                out.push_set_spaces(0);
                out.write("#if ");
                cfg.write(config, out);
                out.pop_set_spaces();
                out.new_line();
            }
        }
    }

    fn write_after<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if self.is_some() {
            if config.language == Language::Cython {
                out.close_brace(false);
            } else {
                out.new_line();
                out.push_set_spaces(0);
                out.write("#endif");
                out.pop_set_spaces();
            }
        }
    }
}
