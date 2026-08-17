/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path;

use crate::bindgen::bindings::Bindings;
use crate::bindgen::cargo::Cargo;
use crate::bindgen::config::{Braces, Config, Language, Profile, Style};
use crate::bindgen::error::Error;
use crate::bindgen::library::Library;
use crate::bindgen::parser::{self, Parse};

/// A builder for generating a bindings header.
#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
    srcs: Vec<path::PathBuf>,
    lib: Option<(path::PathBuf, Option<String>)>,
    lib_cargo: Option<Cargo>,
    std_types: bool,
    lockfile: Option<path::PathBuf>,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            srcs: Vec::new(),
            lib: None,
            lib_cargo: None,
            std_types: true,
            lockfile: None,
        }
    }

    #[allow(unused)]
    pub fn with_header<S: AsRef<str>>(mut self, header: S) -> Builder {
        self.config.header = Some(String::from(header.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_no_includes(mut self) -> Builder {
        self.config.no_includes = true;
        self
    }

    #[allow(unused)]
    pub fn with_include<S: AsRef<str>>(mut self, include: S) -> Builder {
        self.config.includes.push(String::from(include.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_sys_include<S: AsRef<str>>(mut self, include: S) -> Builder {
        self.config
            .sys_includes
            .push(String::from(include.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_after_include<S: AsRef<str>>(mut self, line: S) -> Builder {
        self.config.after_includes = Some(String::from(line.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_trailer<S: AsRef<str>>(mut self, trailer: S) -> Builder {
        self.config.trailer = Some(String::from(trailer.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_include_guard<S: AsRef<str>>(mut self, include_guard: S) -> Builder {
        self.config.include_guard = Some(String::from(include_guard.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_pragma_once(mut self, pragma_once: bool) -> Builder {
        self.config.pragma_once = pragma_once;
        self
    }

    #[allow(unused)]
    pub fn with_autogen_warning<S: AsRef<str>>(mut self, autogen_warning: S) -> Builder {
        self.config.autogen_warning = Some(String::from(autogen_warning.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_include_version(mut self, include_version: bool) -> Builder {
        self.config.include_version = include_version;
        self
    }

    #[allow(unused)]
    pub fn with_namespace<S: AsRef<str>>(mut self, namespace: S) -> Builder {
        self.config.namespace = Some(String::from(namespace.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_namespaces<S: AsRef<str>>(mut self, namespaces: &[S]) -> Builder {
        self.config.namespaces = Some(
            namespaces
                .iter()
                .map(|x| String::from(x.as_ref()))
                .collect(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_using_namespaces<S: AsRef<str>>(mut self, namespaces: &[S]) -> Builder {
        self.config.using_namespaces = Some(
            namespaces
                .iter()
                .map(|x| String::from(x.as_ref()))
                .collect(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_braces(mut self, braces: Braces) -> Builder {
        self.config.braces = braces;
        self
    }

    #[allow(unused)]
    pub fn with_line_length(mut self, line_length: usize) -> Builder {
        self.config.line_length = line_length;
        self
    }

    #[allow(unused)]
    pub fn with_tab_width(mut self, tab_width: usize) -> Builder {
        self.config.tab_width = tab_width;
        self
    }

    #[allow(unused)]
    pub fn with_language(mut self, language: Language) -> Builder {
        self.config.language = language;
        self
    }

    #[allow(unused)]
    pub fn with_cpp_compat(mut self, cpp_compat: bool) -> Builder {
        self.config.cpp_compat = cpp_compat;
        self
    }

    #[allow(unused)]
    pub fn with_style(mut self, style: Style) -> Builder {
        self.config.style = style;
        self
    }

    #[allow(unused)]
    pub fn include_item<S: AsRef<str>>(mut self, item_name: S) -> Builder {
        self.config
            .export
            .include
            .push(String::from(item_name.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn exclude_item<S: AsRef<str>>(mut self, item_name: S) -> Builder {
        self.config
            .export
            .exclude
            .push(String::from(item_name.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn rename_item<S: AsRef<str>>(mut self, from: S, to: S) -> Builder {
        self.config
            .export
            .rename
            .insert(String::from(from.as_ref()), String::from(to.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_item_prefix<S: AsRef<str>>(mut self, prefix: S) -> Builder {
        self.config.export.prefix = Some(String::from(prefix.as_ref()));
        self
    }

    #[allow(unused)]
    pub fn with_parse_deps(mut self, parse_deps: bool) -> Builder {
        self.config.parse.parse_deps = parse_deps;
        self
    }

    #[allow(unused)]
    pub fn with_parse_include<S: AsRef<str>>(mut self, include: &[S]) -> Builder {
        self.config.parse.include =
            Some(include.iter().map(|x| String::from(x.as_ref())).collect());
        self
    }

    #[allow(unused)]
    pub fn with_parse_exclude<S: AsRef<str>>(mut self, exclude: &[S]) -> Builder {
        self.config.parse.exclude = exclude.iter().map(|x| String::from(x.as_ref())).collect();
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand<S: AsRef<str>>(mut self, expand: &[S]) -> Builder {
        self.config.parse.expand.crates = expand.iter().map(|x| String::from(x.as_ref())).collect();
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand_all_features(mut self, expand_all_features: bool) -> Builder {
        self.config.parse.expand.all_features = expand_all_features;
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand_default_features(mut self, expand_default_features: bool) -> Builder {
        self.config.parse.expand.default_features = expand_default_features;
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand_features<S: AsRef<str>>(mut self, expand_features: &[S]) -> Builder {
        self.config.parse.expand.features = Some(
            expand_features
                .iter()
                .map(|x| String::from(x.as_ref()))
                .collect(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_parse_expand_profile(mut self, profile: Profile) -> Builder {
        self.config.parse.expand.profile = profile;
        self
    }

    #[allow(unused)]
    pub fn with_parse_extra_bindings<S: AsRef<str>>(mut self, extra_bindings: &[S]) -> Builder {
        self.config.parse.extra_bindings = extra_bindings
            .iter()
            .map(|x| String::from(x.as_ref()))
            .collect();
        self
    }

    #[allow(unused)]
    pub fn with_only_target_dependencies(mut self, only_target_dependencies: bool) -> Builder {
        self.config.only_target_dependencies = only_target_dependencies;
        self
    }

    #[allow(unused)]
    pub fn with_documentation(mut self, documentation: bool) -> Builder {
        self.config.documentation = documentation;
        self
    }

    #[allow(unused)]
    pub fn with_target_os_define(mut self, platform: &str, preprocessor_define: &str) -> Builder {
        self.config.defines.insert(
            format!("target_os = {}", platform),
            preprocessor_define.to_owned(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_define(mut self, key: &str, value: &str, preprocessor_define: &str) -> Builder {
        self.config.defines.insert(
            format!("{} = {}", key, value),
            preprocessor_define.to_owned(),
        );
        self
    }

    #[allow(unused)]
    pub fn with_config(mut self, config: Config) -> Builder {
        self.config = config;
        self
    }

    #[allow(unused)]
    pub fn with_std_types(mut self, std_types: bool) -> Builder {
        self.std_types = std_types;
        self
    }

    #[allow(unused)]
    pub fn with_src<P: AsRef<path::Path>>(mut self, src: P) -> Builder {
        self.srcs.push(src.as_ref().to_owned());
        self
    }

    #[allow(unused)]
    pub fn with_crate<P: AsRef<path::Path>>(mut self, lib_dir: P) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib = Some((path::PathBuf::from(lib_dir.as_ref()), None));
        self
    }

    #[allow(unused)]
    pub fn with_crate_and_name<P: AsRef<path::Path>, S: AsRef<str>>(
        mut self,
        lib_dir: P,
        binding_lib_name: S,
    ) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib = Some((
            path::PathBuf::from(lib_dir.as_ref()),
            Some(String::from(binding_lib_name.as_ref())),
        ));
        self
    }

    #[allow(unused)]
    pub(crate) fn with_cargo(mut self, lib: Cargo) -> Builder {
        debug_assert!(self.lib.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lib_cargo = Some(lib);
        self
    }

    #[allow(unused)]
    pub fn with_lockfile<P: AsRef<path::Path>>(mut self, lockfile: P) -> Builder {
        debug_assert!(self.lockfile.is_none());
        debug_assert!(self.lib_cargo.is_none());
        self.lockfile = Some(path::PathBuf::from(lockfile.as_ref()));
        self
    }

    pub fn generate(self) -> Result<Bindings, Error> {
        // If macro expansion is enabled, then cbindgen will attempt to build the crate
        // and will run its build script which may run cbindgen again. That second run may start
        // infinite recursion, or overwrite previously written files with bindings.
        // So if we are called recursively, we are skipping the whole generation
        // and produce "noop" bindings that won't be able to overwrite anything.
        if std::env::var("_CBINDGEN_IS_RUNNING").is_ok() {
            return Ok(Bindings::new(
                self.config,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                true,
                String::new(),
            ));
        }

        let mut result = Parse::new();

        if self.std_types {
            result.add_std_types();
        }

        for x in &self.srcs {
            result.extend_with(&parser::parse_src(x, &self.config)?);
        }

        if let Some((lib_dir, binding_lib_name)) = self.lib.clone() {
            let lockfile = self.lockfile.as_deref();

            let cargo = Cargo::load(
                &lib_dir,
                lockfile,
                binding_lib_name.as_deref(),
                self.config.parse.parse_deps,
                self.config.parse.clean,
                self.config.only_target_dependencies,
                /* existing_metadata = */ None,
            )?;

            result.extend_with(&parser::parse_lib(cargo, &self.config)?);
        } else if let Some(cargo) = self.lib_cargo.clone() {
            result.extend_with(&parser::parse_lib(cargo, &self.config)?);
        }

        result.source_files.extend_from_slice(self.srcs.as_slice());

        Library::new(
            self.config,
            result.constants,
            result.globals,
            result.enums,
            result.structs,
            result.unions,
            result.opaque_items,
            result.typedefs,
            result.functions,
            result.source_files,
            result.package_version,
        )
        .generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_style() {
        assert_eq!(
            Style::Tag,
            Builder::new().with_style(Style::Tag).config.style
        );
    }
}
