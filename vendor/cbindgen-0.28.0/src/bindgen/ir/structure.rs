/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn::ext::IdentExt;

use crate::bindgen::config::{Config, Language, LayoutConfig};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, Constant, Documentation, Field, GenericArgument, GenericParams, Item,
    ItemContainer, Path, Repr, ReprAlign, ReprStyle, Type, Typedef,
};
use crate::bindgen::library::Library;
use crate::bindgen::mangle;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::utilities::IterHelpers;
use crate::bindgen::writer::SourceWriter;

#[derive(Debug, Clone)]
pub struct Struct {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub fields: Vec<Field>,
    /// Whether there's a tag field on the body of this struct. When this is
    /// true, is_enum_variant_body is also guaranteed to be true.
    pub has_tag_field: bool,
    /// Whether this is an enum variant body.
    pub is_enum_variant_body: bool,
    pub alignment: Option<ReprAlign>,
    pub is_transparent: bool,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
    pub associated_constants: Vec<Constant>,
}

impl Struct {
    /// Whether this struct can derive operator== / operator!=.
    pub fn can_derive_eq(&self) -> bool {
        !self.fields.is_empty() && self.fields.iter().all(|x| x.ty.can_cmp_eq())
    }

    pub fn add_associated_constant(&mut self, c: Constant) {
        self.associated_constants.push(c);
    }

    pub fn load(
        layout_config: &LayoutConfig,
        item: &syn::ItemStruct,
        mod_cfg: Option<&Cfg>,
    ) -> Result<Self, String> {
        let repr = Repr::load(&item.attrs)?;
        let is_transparent = match repr.style {
            ReprStyle::C => false,
            ReprStyle::Transparent => true,
            _ => {
                return Err("Struct is not marked #[repr(C)] or #[repr(transparent)].".to_owned());
            }
        };

        let path = Path::new(item.ident.unraw().to_string());

        // Ensure we can safely represent the struct given the configuration.
        if let Some(align) = repr.align {
            layout_config.ensure_safe_to_represent(&align)?;
        }

        let fields = match item.fields {
            syn::Fields::Unit => Vec::new(),
            syn::Fields::Named(ref fields) => fields
                .named
                .iter()
                .try_skip_map(|field| Field::load(field, &path))?,
            syn::Fields::Unnamed(ref fields) => {
                let mut out = Vec::new();
                let mut current = 0;
                for field in fields.unnamed.iter() {
                    if let Some(mut ty) = Type::load(&field.ty)? {
                        ty.replace_self_with(&path);
                        out.push(Field {
                            name: format!("{}", current),
                            ty,
                            cfg: Cfg::load(&field.attrs),
                            annotations: AnnotationSet::load(&field.attrs)?,
                            documentation: Documentation::load(&field.attrs),
                        });
                        current += 1;
                    }
                }
                out
            }
        };

        let has_tag_field = false;
        let is_enum_variant_body = false;

        Ok(Struct::new(
            path,
            GenericParams::load(&item.generics)?,
            fields,
            has_tag_field,
            is_enum_variant_body,
            repr.align,
            is_transparent,
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
        has_tag_field: bool,
        is_enum_variant_body: bool,
        alignment: Option<ReprAlign>,
        mut is_transparent: bool,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        // WARNING: Zero-sized transparent structs are legal rust [1], but zero-sized types of any
        // repr are "best avoided entirely" [2] because they "will be nonsensical or problematic if
        // passed through the FFI boundary" [3]. Further, because no well-defined underlying native
        // type exists for a ZST, we cannot emit a typedef and must define an empty struct instead.
        //
        // [1] https://github.com/rust-lang/rust/issues/77841#issuecomment-716575747
        // [2] https://github.com/rust-lang/rust/issues/77841#issuecomment-716796313
        // [3] https://doc.rust-lang.org/nomicon/other-reprs.html
        if fields.is_empty() {
            warn!(
                "Passing zero-sized struct {} across the FFI boundary is undefined behavior",
                &path
            );
            is_transparent = false;
        }

        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            fields,
            has_tag_field,
            is_enum_variant_body,
            alignment,
            is_transparent,
            cfg,
            annotations,
            documentation,
            associated_constants: vec![],
        }
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        for field in &mut self.fields {
            field.ty.simplify_standard_types(config);
        }
    }

    /// Attempts to convert this struct to a typedef (only works for transparent structs).
    pub fn as_typedef(&self) -> Option<Typedef> {
        match self.fields.first() {
            Some(field) if self.is_transparent => Some(Typedef::new_from_struct_field(self, field)),
            _ => None,
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        // Generic structs can instantiate monomorphs only once they've been
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

    pub fn specialize(
        &self,
        generic_values: &[GenericArgument],
        mappings: &[(&Path, &GenericArgument)],
        config: &Config,
    ) -> Self {
        let mangled_path = mangle::mangle_path(&self.path, generic_values, &config.export.mangle);
        Struct::new(
            mangled_path,
            GenericParams::default(),
            self.fields
                .iter()
                .map(|field| Field {
                    name: field.name.clone(),
                    ty: field.ty.specialize(mappings),
                    cfg: field.cfg.clone(),
                    annotations: field.annotations.clone(),
                    documentation: field.documentation.clone(),
                })
                .collect(),
            self.has_tag_field,
            self.is_enum_variant_body,
            self.alignment,
            self.is_transparent,
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        )
    }

    pub(crate) fn emit_bitflags_binop<F: Write>(
        &self,
        constexpr_prefix: &str,
        operator: char,
        other: &str,
        out: &mut SourceWriter<F>,
    ) {
        let bits = &self.fields[0].name;
        out.new_line();
        write!(
            out,
            "{}{} operator{}(const {}& {}) const",
            constexpr_prefix,
            self.export_name(),
            operator,
            self.export_name(),
            other
        );
        out.open_brace();
        write!(
            out,
            "return {} {{ static_cast<decltype({bits})>(this->{bits} {operator} {other}.{bits}) }};",
            self.export_name()
        );
        out.close_brace(false);

        out.new_line();
        write!(
            out,
            "{}& operator{}=(const {}& {})",
            self.export_name(),
            operator,
            self.export_name(),
            other
        );
        out.open_brace();
        write!(out, "*this = (*this {} {});", operator, other);
        out.new_line();
        write!(out, "return *this;");
        out.close_brace(false);
    }
}

impl Item for Struct {
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
        ItemContainer::Struct(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        if self.is_transparent {
            resolver.add_none(&self.path);
        } else {
            resolver.add_struct(&self.path);
        }
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
        // Rename the name of the struct
        if !(self.has_tag_field && config.language == Language::Cxx) {
            config.export.rename(&mut self.export_name);
        }

        // Rename the types used in fields
        {
            let fields = self.fields.iter_mut().skip(self.has_tag_field as usize);
            for field in fields {
                field.ty.rename_for_config(config, &self.generic_params);
            }
        }

        // Apply renaming rules to fields in the following order
        //   1. `cbindgen::field-names` annotation
        //   2. `cbindgen::rename-all` annotation
        //   3. config struct rename rule
        // If the struct is a tuple struct and we have not renamed the
        // fields, then prefix each of them with an underscore.
        // If any field is a reserved keyword, then postfix it with an
        // underscore.

        // Scope for mutable borrow of fields
        {
            let names = self.fields.iter_mut().map(|field| &mut field.name);

            let field_rules = self.annotations.parse_atom::<RenameRule>("rename-all");
            let field_rules = field_rules
                .as_ref()
                .unwrap_or(&config.structure.rename_fields);

            if let Some(o) = self.annotations.list("field-names") {
                for (dest, src) in names.zip(o) {
                    *dest = src;
                }
            } else if let Some(r) = field_rules.not_none() {
                for name in names {
                    *name = r.apply(name, IdentifierType::StructMember).into_owned();
                }
            } else {
                // If we don't have any rules for a tuple struct, prefix them with
                // an underscore so it still compiles.
                for name in names {
                    if name.starts_with(|c: char| c.is_ascii_digit()) {
                        name.insert(0, '_');
                    }
                }
            }
        }

        for field in &mut self.fields {
            reserved::escape(&mut field.name);
        }

        for c in self.associated_constants.iter_mut() {
            c.rename_for_config(config);
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        let mut fields = self.fields.iter();

        // If there is a tag field, skip it
        if self.has_tag_field {
            fields.next();
        }

        for field in fields {
            field
                .ty
                .add_dependencies_ignoring_generics(&self.generic_params, library, out);
        }

        for c in &self.associated_constants {
            c.add_dependencies(library, out);
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &[GenericArgument],
        library: &Library,
        out: &mut Monomorphs,
    ) {
        let mappings = self.generic_params.call(self.path.name(), generic_values);
        let monomorph = self.specialize(generic_values, &mappings, library.get_config());
        out.insert_struct(library, self, monomorph, generic_values.to_owned());
    }
}
