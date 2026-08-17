/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::config::Config;
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, Documentation, GenericParams, Item, ItemContainer, Path, Type,
};
use crate::bindgen::library::Library;

#[derive(Debug, Clone)]
pub struct Static {
    pub path: Path,
    pub export_name: String,
    pub ty: Type,
    pub mutable: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Static {
    pub fn load(
        path: Path,
        item: &syn::ItemStatic,
        mod_cfg: Option<&Cfg>,
    ) -> Result<Static, String> {
        let ty = Type::load(&item.ty)?;

        if ty.is_none() {
            return Err("Cannot have a zero sized static definition.".to_owned());
        }

        Ok(Static::new(
            path,
            ty.unwrap(),
            matches!(item.mutability, syn::StaticMutability::Mut(_)),
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    pub fn new(
        path: Path,
        ty: Type,
        mutable: bool,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            ty,
            mutable,
            cfg,
            annotations,
            documentation,
        }
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        self.ty.simplify_standard_types(config);
    }
}

impl Item for Static {
    fn path(&self) -> &Path {
        &self.path
    }

    fn export_name(&self) -> &str {
        &self.export_name
    }

    fn cfg(&self) -> Option<&Cfg> {
        self.cfg.as_ref()
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Static(self.clone())
    }

    fn rename_for_config(&mut self, config: &Config) {
        self.ty.rename_for_config(config, &Default::default());
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ty.resolve_declaration_types(resolver);
    }

    fn generic_params(&self) -> &GenericParams {
        GenericParams::empty()
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ty.add_dependencies(library, out);
    }
}
