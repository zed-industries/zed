/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn::ext::IdentExt;

use crate::bindgen::config::{Config, LayoutConfig};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, Documentation, Field, GenericArgument, GenericParams, Item, ItemContainer,
    Path, Repr, ReprAlign, ReprStyle,
};
use crate::bindgen::library::Library;
use crate::bindgen::mangle;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::utilities::IterHelpers;

#[derive(Debug, Clone)]
pub struct Union {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<Field>,
    pub tuple_union: bool,
    pub alignment: Option<ReprAlign>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Union {
    pub fn load(
        layout_config: &LayoutConfig,
        item: &syn::ItemUnion,
        mod_cfg: Option<&Cfg>,
    ) -> Result<Union, String> {
        let repr = Repr::load(&item.attrs)?;
        if repr.style != ReprStyle::C {
            return Err("Union is not marked #[repr(C)].".to_owned());
        }

        // Ensure we can safely represent the union given the configuration.
        if let Some(align) = repr.align {
            layout_config.ensure_safe_to_represent(&align)?;
        }

        let path = Path::new(item.ident.unraw().to_string());

        let (fields, tuple_union) = {
            let out = item
                .fields
                .named
                .iter()
                .try_skip_map(|field| Field::load(field, &path))?;
            (out, false)
        };

        Ok(Union::new(
            path,
            GenericParams::load(&item.generics)?,
            fields,
            repr.align,
            tuple_union,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            AnnotationSet::load(&item.attrs)?,
            Documentation::load(&item.attrs),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: Path,
        generic_params: GenericParams,
        fields: Vec<Field>,
        alignment: Option<ReprAlign>,
        tuple_union: bool,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            fields,
            tuple_union,
            alignment,
            cfg,
            annotations,
            documentation,
        }
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        for field in &mut self.fields {
            field.ty.simplify_standard_types(config);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        // Generic unions can instantiate monomorphs only once they've been
        // instantiated. See `instantiate_monomorph` for more details.
        if self.is_generic() {
            return;
        }

        for field in &self.fields {
            field.ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for field in &mut self.fields {
            field.ty.mangle_paths(monomorphs);
        }
    }
}

impl Item for Union {
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
        ItemContainer::Union(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_union(&self.path);
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for field in &mut self.fields {
            field.ty.resolve_declaration_types(resolver);
        }
    }

    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);
        for field in &mut self.fields {
            field.ty.rename_for_config(config, &self.generic_params);
        }

        let rules = self.annotations.parse_atom::<RenameRule>("rename-all");
        let rules = rules.as_ref().unwrap_or(&config.structure.rename_fields);

        if let Some(o) = self.annotations.list("field-names") {
            let mut overriden_fields = Vec::new();

            for (i, field) in self.fields.iter().enumerate() {
                if i >= o.len() {
                    overriden_fields.push(field.clone());
                } else {
                    overriden_fields.push(Field {
                        name: o[i].clone(),
                        ty: field.ty.clone(),
                        cfg: field.cfg.clone(),
                        annotations: field.annotations.clone(),
                        documentation: field.documentation.clone(),
                    });
                }
            }

            self.fields = overriden_fields;
        } else if let Some(r) = rules.not_none() {
            self.fields = self
                .fields
                .iter()
                .map(|field| Field {
                    name: r
                        .apply(&field.name, IdentifierType::StructMember)
                        .into_owned(),
                    ty: field.ty.clone(),
                    cfg: field.cfg.clone(),
                    annotations: field.annotations.clone(),
                    documentation: field.documentation.clone(),
                })
                .collect();
        } else if self.tuple_union {
            // If we don't have any rules for a tuple union, prefix them with
            // an underscore so it still compiles
            for field in &mut self.fields {
                field.name.insert(0, '_');
            }
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        for field in &self.fields {
            field
                .ty
                .add_dependencies_ignoring_generics(&self.generic_params, library, out);
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &[GenericArgument],
        library: &Library,
        out: &mut Monomorphs,
    ) {
        let mappings = self.generic_params.call(self.path.name(), generic_values);

        let mangled_path = mangle::mangle_path(
            &self.path,
            generic_values,
            &library.get_config().export.mangle,
        );

        let monomorph = Union::new(
            mangled_path,
            GenericParams::default(),
            self.fields
                .iter()
                .map(|field| Field {
                    name: field.name.clone(),
                    ty: field.ty.specialize(&mappings),
                    cfg: field.cfg.clone(),
                    annotations: field.annotations.clone(),
                    documentation: field.documentation.clone(),
                })
                .collect(),
            self.alignment,
            self.tuple_union,
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        );

        out.insert_union(library, self, monomorph, generic_values.to_owned());
    }
}
