/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn::ext::IdentExt;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, AnnotationValue, Cfg, ConditionWrite, DeprecatedNoteKind, Documentation, Field,
    GenericArgument, GenericParams, GenericPath, Item, ItemContainer, Literal, Path, Repr,
    ReprStyle, Struct, ToCondition, Type,
};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::library::Library;
use crate::bindgen::mangle;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::writer::{ListType, SourceWriter};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum VariantBody {
    Empty(AnnotationSet),
    Body {
        /// The variant field / export name.
        name: String,
        /// The struct with all the items.
        body: Struct,
        /// A separate named struct is not created for this variant,
        /// an unnamed struct is inlined at the point of use instead.
        /// This is a reasonable thing to do only for tuple variants with a single field.
        inline: bool,
        /// Generated cast methods return the variant's only field instead of the variant itself.
        /// For backward compatibility casts are inlined in a slightly
        /// larger set of cases than whole variants.
        inline_casts: bool,
    },
}

impl VariantBody {
    fn empty() -> Self {
        Self::Empty(AnnotationSet::new())
    }

    pub fn annotations(&self) -> &AnnotationSet {
        match *self {
            Self::Empty(ref anno) => anno,
            Self::Body { ref body, .. } => &body.annotations,
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Self::Empty(..) => true,
            Self::Body { .. } => false,
        }
    }

    fn specialize(
        &self,
        generic_values: &[GenericArgument],
        mappings: &[(&Path, &GenericArgument)],
        config: &Config,
    ) -> Self {
        match *self {
            Self::Empty(ref annos) => Self::Empty(annos.clone()),
            Self::Body {
                ref name,
                ref body,
                inline,
                inline_casts,
            } => Self::Body {
                name: name.clone(),
                body: body.specialize(generic_values, mappings, config),
                inline,
                inline_casts,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub export_name: String,
    pub discriminant: Option<Literal>,
    pub body: VariantBody,
    pub cfg: Option<Cfg>,
    pub documentation: Documentation,
}

impl EnumVariant {
    fn load(
        inline_tag_field: bool,
        variant: &syn::Variant,
        generic_params: GenericParams,
        mod_cfg: Option<&Cfg>,
        self_path: &Path,
        enum_annotations: &AnnotationSet,
        config: &Config,
    ) -> Result<Self, String> {
        let discriminant = match variant.discriminant {
            Some((_, ref expr)) => Some(Literal::load(expr)?),
            None => None,
        };

        fn parse_fields(
            inline_tag_field: bool,
            fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
            self_path: &Path,
            inline_name: Option<&str>,
        ) -> Result<Vec<Field>, String> {
            let mut res = Vec::new();

            if inline_tag_field {
                res.push(Field::from_name_and_type(
                    inline_name.map_or_else(|| "tag".to_string(), |name| format!("{}_tag", name)),
                    Type::Path(GenericPath::new(Path::new("Tag"), vec![])),
                ));
            }

            for (i, field) in fields.iter().enumerate() {
                if let Some(mut ty) = Type::load(&field.ty)? {
                    ty.replace_self_with(self_path);
                    res.push(Field {
                        name: inline_name.map_or_else(
                            || match field.ident {
                                Some(ref ident) => ident.unraw().to_string(),
                                None => i.to_string(),
                            },
                            |name| name.to_string(),
                        ),
                        ty,
                        cfg: Cfg::load(&field.attrs),
                        annotations: AnnotationSet::load(&field.attrs)?,
                        documentation: Documentation::load(&field.attrs),
                    });
                }
            }

            Ok(res)
        }

        let variant_cfg = Cfg::append(mod_cfg, Cfg::load(&variant.attrs));
        let mut annotations = AnnotationSet::load(&variant.attrs)?;
        if let Some(b) = enum_annotations.bool("derive-ostream") {
            annotations.add_default("derive-ostream", AnnotationValue::Bool(b));
        }

        let body_rule = enum_annotations.parse_atom::<RenameRule>("rename-variant-name-fields");
        let body_rule = body_rule
            .as_ref()
            .unwrap_or(&config.enumeration.rename_variant_name_fields);

        let body = match variant.fields {
            syn::Fields::Unit => VariantBody::Empty(annotations),
            syn::Fields::Named(ref fields) => {
                let path = Path::new(format!("{}_Body", variant.ident));
                let name = body_rule
                    .apply(
                        &variant.ident.unraw().to_string(),
                        IdentifierType::StructMember,
                    )
                    .into_owned();
                VariantBody::Body {
                    body: Struct::new(
                        path,
                        generic_params,
                        parse_fields(inline_tag_field, &fields.named, self_path, None)?,
                        inline_tag_field,
                        true,
                        None,
                        false,
                        None,
                        annotations,
                        Documentation::none(),
                    ),
                    name,
                    inline: false,
                    inline_casts: false,
                }
            }
            syn::Fields::Unnamed(ref fields) => {
                let path = Path::new(format!("{}_Body", variant.ident));
                let name = body_rule
                    .apply(
                        &variant.ident.unraw().to_string(),
                        IdentifierType::StructMember,
                    )
                    .into_owned();
                let inline_casts = fields.unnamed.len() == 1;
                // In C++ types with destructors cannot be put into unnamed structs like the
                // inlining requires, and it's hard to detect such types.
                // Besides that for C++ we generate casts/getters that can be used instead of
                // direct field accesses and also have a benefit of being checked.
                // As a result we don't currently inline variant definitions in C++ mode at all.
                let inline = inline_casts && config.language != Language::Cxx;
                let inline_name = if inline { Some(&*name) } else { None };
                VariantBody::Body {
                    body: Struct::new(
                        path,
                        generic_params,
                        parse_fields(inline_tag_field, &fields.unnamed, self_path, inline_name)?,
                        inline_tag_field,
                        true,
                        None,
                        false,
                        None,
                        annotations,
                        Documentation::none(),
                    ),
                    name,
                    inline,
                    inline_casts,
                }
            }
        };

        Ok(EnumVariant::new(
            variant.ident.unraw().to_string(),
            discriminant,
            body,
            variant_cfg,
            Documentation::load(&variant.attrs),
        ))
    }

    pub fn new(
        name: String,
        discriminant: Option<Literal>,
        body: VariantBody,
        cfg: Option<Cfg>,
        documentation: Documentation,
    ) -> Self {
        let export_name = name.clone();
        Self {
            name,
            export_name,
            discriminant,
            body,
            cfg,
            documentation,
        }
    }

    fn simplify_standard_types(&mut self, config: &Config) {
        if let VariantBody::Body { ref mut body, .. } = self.body {
            body.simplify_standard_types(config);
        }
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        if let VariantBody::Body { ref body, .. } = self.body {
            body.add_dependencies(library, out);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        if let VariantBody::Body { ref mut body, .. } = self.body {
            body.resolve_declaration_types(resolver);
        }
    }

    fn specialize(
        &self,
        generic_values: &[GenericArgument],
        mappings: &[(&Path, &GenericArgument)],
        config: &Config,
    ) -> Self {
        Self::new(
            mangle::mangle_name(&self.name, generic_values, &config.export.mangle),
            self.discriminant.clone(),
            self.body.specialize(generic_values, mappings, config),
            self.cfg.clone(),
            self.documentation.clone(),
        )
    }

    fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        if let VariantBody::Body { ref body, .. } = self.body {
            body.add_monomorphs(library, out);
        }
    }

    fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        if let VariantBody::Body { ref mut body, .. } = self.body {
            body.mangle_paths(monomorphs);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub repr: Repr,
    pub variants: Vec<EnumVariant>,
    pub tag: Option<String>,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Enum {
    /// Name of the generated tag enum.
    pub(crate) fn tag_name(&self) -> &str {
        self.tag.as_deref().unwrap_or_else(|| self.export_name())
    }

    /// Enum with data turns into a union of structs with each struct having its own tag field.
    pub(crate) fn inline_tag_field(repr: &Repr) -> bool {
        repr.style != ReprStyle::C
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        if self.is_generic() {
            return;
        }

        for v in &self.variants {
            v.add_monomorphs(library, out);
        }
    }

    fn can_derive_eq(&self) -> bool {
        if self.tag.is_none() {
            return false;
        }

        self.variants.iter().all(|variant| match variant.body {
            VariantBody::Empty(..) => true,
            VariantBody::Body { ref body, .. } => body.can_derive_eq(),
        })
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        for variant in &mut self.variants {
            variant.mangle_paths(monomorphs);
        }
    }

    pub fn load(
        item: &syn::ItemEnum,
        mod_cfg: Option<&Cfg>,
        config: &Config,
    ) -> Result<Enum, String> {
        let repr = Repr::load(&item.attrs)?;
        if repr.style == ReprStyle::Rust && repr.ty.is_none() {
            return Err("Enum is not marked with a valid #[repr(prim)] or #[repr(C)].".to_owned());
        }
        // TODO: Implement translation of aligned enums.
        if repr.align.is_some() {
            return Err("Enum is marked with #[repr(align(...))] or #[repr(packed)].".to_owned());
        }

        let path = Path::new(item.ident.unraw().to_string());
        let generic_params = GenericParams::load(&item.generics)?;

        let mut variants = Vec::new();
        let mut has_data = false;

        let annotations = AnnotationSet::load(&item.attrs)?;

        for variant in item.variants.iter() {
            let variant = EnumVariant::load(
                Self::inline_tag_field(&repr),
                variant,
                generic_params.clone(),
                mod_cfg,
                &path,
                &annotations,
                config,
            )?;
            has_data = has_data || !variant.body.is_empty();
            variants.push(variant);
        }

        if let Some(names) = annotations.list("enum-trailing-values") {
            for name in names {
                variants.push(EnumVariant::new(
                    name,
                    None,
                    VariantBody::empty(),
                    None,
                    Documentation::none(),
                ));
            }
        }

        if config.enumeration.add_sentinel(&annotations) {
            variants.push(EnumVariant::new(
                "Sentinel".to_owned(),
                None,
                VariantBody::empty(),
                None,
                Documentation::simple(" Must be last for serialization purposes"),
            ));
        }

        let tag = if has_data {
            Some("Tag".to_string())
        } else {
            None
        };

        Ok(Enum::new(
            path,
            generic_params,
            repr,
            variants,
            tag,
            Cfg::append(mod_cfg, Cfg::load(&item.attrs)),
            annotations,
            Documentation::load(&item.attrs),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: Path,
        generic_params: GenericParams,
        repr: Repr,
        variants: Vec<EnumVariant>,
        tag: Option<String>,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            repr,
            variants,
            tag,
            cfg,
            annotations,
            documentation,
        }
    }
}

impl Item for Enum {
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
        ItemContainer::Enum(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        if self.tag.is_some() {
            if self.repr.style == ReprStyle::C {
                resolver.add_struct(&self.path);
            } else {
                resolver.add_union(&self.path);
            }
        } else if self.repr.style == ReprStyle::C {
            resolver.add_enum(&self.path);
        } else {
            // This is important to handle conflicting names with opaque items.
            resolver.add_none(&self.path);
        }
    }

    fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        for &mut ref mut var in &mut self.variants {
            var.resolve_declaration_types(resolver);
        }
    }

    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);

        if config.language != Language::Cxx && self.tag.is_some() {
            // it makes sense to always prefix Tag with type name in C
            let new_tag = format!("{}_Tag", self.export_name);
            if self.repr.style == ReprStyle::Rust {
                for variant in &mut self.variants {
                    if let VariantBody::Body { ref mut body, .. } = variant.body {
                        let path = Path::new(new_tag.clone());
                        let generic_path = GenericPath::new(path, vec![]);
                        body.fields[0].ty = Type::Path(generic_path);
                    }
                }
            }
            self.tag = Some(new_tag);
        }

        for variant in &mut self.variants {
            reserved::escape(&mut variant.export_name);
            if let Some(discriminant) = &mut variant.discriminant {
                discriminant.rename_for_config(config);
            }
            if let VariantBody::Body {
                ref mut name,
                ref mut body,
                ..
            } = variant.body
            {
                body.rename_for_config(config);
                reserved::escape(name);
            }
        }

        if self
            .annotations
            .bool("prefix-with-name")
            .unwrap_or(config.enumeration.prefix_with_name)
        {
            let separator = if config.export.mangle.remove_underscores {
                ""
            } else {
                "_"
            };

            for variant in &mut self.variants {
                variant.export_name =
                    format!("{}{}{}", self.export_name, separator, variant.export_name);
                if let VariantBody::Body { ref mut body, .. } = variant.body {
                    body.export_name =
                        format!("{}{}{}", self.export_name, separator, body.export_name());
                }
            }
        }

        let rules = self.annotations.parse_atom::<RenameRule>("rename-all");
        let rules = rules
            .as_ref()
            .unwrap_or(&config.enumeration.rename_variants);

        if let Some(r) = rules.not_none() {
            self.variants = self
                .variants
                .iter()
                .map(|variant| {
                    EnumVariant::new(
                        r.apply(
                            &variant.export_name,
                            IdentifierType::EnumVariant {
                                prefix: &self.export_name,
                            },
                        )
                        .into_owned(),
                        variant.discriminant.clone(),
                        match variant.body {
                            VariantBody::Empty(..) => variant.body.clone(),
                            VariantBody::Body {
                                ref name,
                                ref body,
                                inline,
                                inline_casts,
                            } => VariantBody::Body {
                                name: r.apply(name, IdentifierType::StructMember).into_owned(),
                                body: body.clone(),
                                inline,
                                inline_casts,
                            },
                        },
                        variant.cfg.clone(),
                        variant.documentation.clone(),
                    )
                })
                .collect();
        }
    }

    fn instantiate_monomorph(
        &self,
        generic_values: &[GenericArgument],
        library: &Library,
        out: &mut Monomorphs,
    ) {
        let mappings = self.generic_params.call(self.path.name(), generic_values);

        for variant in &self.variants {
            if let VariantBody::Body { ref body, .. } = variant.body {
                body.instantiate_monomorph(generic_values, library, out);
            }
        }

        let mangled_path = mangle::mangle_path(
            &self.path,
            generic_values,
            &library.get_config().export.mangle,
        );

        let monomorph = Enum::new(
            mangled_path,
            GenericParams::default(),
            self.repr,
            self.variants
                .iter()
                .map(|v| v.specialize(generic_values, &mappings, library.get_config()))
                .collect(),
            self.tag.clone(),
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        );

        out.insert_enum(library, self, monomorph, generic_values.to_owned());
    }

    fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        for variant in &self.variants {
            variant.add_dependencies(library, out);
        }
    }
}

impl Enum {
    /// Emit the tag enum and convenience methods for it.
    /// For enums with data this is only a part of the output,
    /// but for enums without data it's the whole output (modulo doc comments etc.).
    pub(crate) fn write_tag_enum<
        F: Write,
        LB: LanguageBackend,
        WV: Fn(&mut LB, &mut SourceWriter<F>, &EnumVariant),
    >(
        &self,
        config: &Config,
        language_backend: &mut LB,
        out: &mut SourceWriter<F>,
        size: Option<&str>,
        write_variant: WV,
    ) {
        let tag_name = self.tag_name();
        // Open the tag enum.
        match config.language {
            Language::C => {
                if let Some(prim) = size {
                    // If we need to specify size, then we have no choice but to create a typedef,
                    // so `config.style` is not respected.
                    write!(out, "enum");
                    if let Some(note) = self
                        .annotations
                        .deprecated_note(config, DeprecatedNoteKind::Enum)
                    {
                        write!(out, " {}", note);
                    }
                    write!(out, " {}", tag_name);

                    if config.cpp_compatible_c() {
                        out.new_line();
                        out.write("#ifdef __cplusplus");
                        out.new_line();
                        write!(out, "  : {}", prim);
                        out.new_line();
                        out.write("#endif // __cplusplus");
                        out.new_line();
                    }
                } else {
                    if config.style.generate_typedef() {
                        out.write("typedef ");
                    }
                    out.write("enum");
                    if let Some(note) = self
                        .annotations
                        .deprecated_note(config, DeprecatedNoteKind::Enum)
                    {
                        write!(out, " {}", note);
                    }
                    if config.style.generate_tag() {
                        write!(out, " {}", tag_name);
                    }
                }
            }
            Language::Cxx => {
                if config.enumeration.enum_class(&self.annotations) {
                    out.write("enum class");
                } else {
                    out.write("enum");
                }

                if self.annotations.must_use(config) {
                    if let Some(ref anno) = config.enumeration.must_use {
                        write!(out, " {}", anno)
                    }
                }

                if let Some(note) = self
                    .annotations
                    .deprecated_note(config, DeprecatedNoteKind::Enum)
                {
                    write!(out, " {}", note);
                }

                write!(out, " {}", tag_name);
                if let Some(prim) = size {
                    write!(out, " : {}", prim);
                }
            }
            Language::Cython => {
                if size.is_some() {
                    // If we need to specify size, then we have no choice but to create a typedef,
                    // so `config.style` is not respected.
                    write!(out, "cdef enum");
                } else {
                    write!(out, "{}enum {}", config.style.cython_def(), tag_name);
                }
            }
        }
        out.open_brace();

        // Emit enumerators for the tag enum.
        for (i, variant) in self.variants.iter().enumerate() {
            if i != 0 {
                out.new_line()
            }
            write_variant(language_backend, out, variant);
        }

        // Close the tag enum.
        if config.language == Language::C && size.is_none() && config.style.generate_typedef() {
            out.close_brace(false);
            write!(out, " {};", tag_name);
        } else {
            out.close_brace(true);
        }

        // Emit typedef specifying the tag enum's size if necessary.
        // In C++ enums can "inherit" from numeric types (`enum E: uint8_t { ... }`),
        // but in C `typedef uint8_t E` is the only way to give a fixed size to `E`.
        if let Some(prim) = size {
            if config.cpp_compatible_c() {
                out.new_line_if_not_start();
                out.write("#ifndef __cplusplus");
            }

            if config.language != Language::Cxx {
                out.new_line();
                write!(out, "{} {} {};", config.language.typedef(), prim, tag_name);
            }

            if config.cpp_compatible_c() {
                out.new_line_if_not_start();
                out.write("#endif // __cplusplus");
            }
        }

        // Emit convenience methods for the tag enum.
        self.write_derived_functions_enum(config, language_backend, out);
    }

    /// The code here mirrors the beginning of `Struct::write` and `Union::write`.
    pub(crate) fn open_struct_or_union<F: Write>(
        &self,
        config: &Config,
        out: &mut SourceWriter<F>,
        inline_tag_field: bool,
    ) {
        match config.language {
            Language::C if config.style.generate_typedef() => out.write("typedef "),
            Language::C | Language::Cxx => {}
            Language::Cython => out.write(config.style.cython_def()),
        }

        out.write(if inline_tag_field { "union" } else { "struct" });

        if self.annotations.must_use(config) {
            if let Some(ref anno) = config.structure.must_use {
                write!(out, " {}", anno);
            }
        }

        if let Some(note) = self
            .annotations
            .deprecated_note(config, DeprecatedNoteKind::Struct)
        {
            write!(out, " {} ", note);
        }

        if config.language != Language::C || config.style.generate_tag() {
            write!(out, " {}", self.export_name());
        }

        out.open_brace();

        // Emit the pre_body section, if relevant.
        if let Some(body) = config.export.pre_body(&self.path) {
            out.write_raw_block(body);
            out.new_line();
        }
    }

    /// Emit struct definitions for variants having data.
    pub(crate) fn write_variant_defs<F: Write, LB: LanguageBackend>(
        &self,
        config: &Config,
        language_backend: &mut LB, // TODO probably need only one of Config/LanguageBackend
        out: &mut SourceWriter<F>,
    ) {
        for variant in &self.variants {
            if let VariantBody::Body {
                ref body,
                inline: false,
                ..
            } = variant.body
            {
                out.new_line();
                out.new_line();
                let condition = variant.cfg.to_condition(config);
                // Cython doesn't support conditional enum variants.
                if config.language != Language::Cython {
                    condition.write_before(config, out);
                }
                language_backend.write_struct(out, body);
                if config.language != Language::Cython {
                    condition.write_after(config, out);
                }
            }
        }
    }

    /// Emit tag field that is separate from all variants.
    /// For non-inline tag scenario this is *the* tag field, and it does not exist in the variants.
    /// For the inline tag scenario this is just a convenience and another way
    /// to refer to the same tag that exist in all the variants.
    pub(crate) fn write_tag_field<F: Write>(
        &self,
        config: &Config,
        out: &mut SourceWriter<F>,
        size: Option<&str>,
        inline_tag_field: bool,
        tag_name: &str,
    ) {
        // C++ allows accessing only common initial sequence of union
        // fields so we have to wrap the tag field into an anonymous struct.
        let wrap_tag = inline_tag_field && config.language == Language::Cxx;

        if wrap_tag {
            out.write("struct");
            out.open_brace();
        }

        if config.language == Language::C && size.is_none() && !config.style.generate_typedef() {
            out.write("enum ");
        }

        write!(out, "{} tag;", tag_name);

        if wrap_tag {
            out.close_brace(true);
        }
    }

    /// Emit fields for all variants with data.
    pub(crate) fn write_variant_fields<
        F: Write,
        LB: LanguageBackend,
        WF: Fn(&mut LB, &mut SourceWriter<F>, &Field),
    >(
        &self,
        config: &Config,
        language_backend: &mut LB,
        out: &mut SourceWriter<F>,
        inline_tag_field: bool,
        write_field: WF,
    ) {
        let mut first = true;
        for variant in &self.variants {
            if let VariantBody::Body {
                name, body, inline, ..
            } = &variant.body
            {
                if !first {
                    out.new_line();
                }
                first = false;
                let condition = variant.cfg.to_condition(config);
                // Cython doesn't support conditional enum variants.
                if config.language != Language::Cython {
                    condition.write_before(config, out);
                }
                if *inline {
                    // Write definition of an inlined variant with data.
                    // Cython extern declarations don't manage layouts, layouts are defined entierly
                    // by the corresponding C code. So we can inline the unnamed struct and get the
                    // same observable result. Moreother we have to do it because Cython doesn't
                    // support unnamed structs.
                    // For the same reason with Cython we can omit per-variant tags (the first
                    // field) to avoid extra noise, the main `tag` is enough in this case.
                    if config.language != Language::Cython {
                        out.write("struct");
                        out.open_brace();
                    }
                    let start_field =
                        usize::from(inline_tag_field && config.language == Language::Cython);
                    out.write_vertical_source_list(
                        language_backend,
                        &body.fields[start_field..],
                        ListType::Cap(";"),
                        &write_field,
                    );
                    if config.language != Language::Cython {
                        out.close_brace(true);
                    }
                } else if config.style.generate_typedef() || config.language == Language::Cython {
                    write!(out, "{} {};", body.export_name(), name);
                } else {
                    write!(out, "struct {} {};", body.export_name(), name);
                }
                if config.language != Language::Cython {
                    condition.write_after(config, out);
                }
            }
        }
    }

    // Emit convenience methods for enums themselves.
    fn write_derived_functions_enum<F: Write, LB: LanguageBackend>(
        &self,
        config: &Config,
        language_backend: &mut LB,
        out: &mut SourceWriter<F>,
    ) {
        let has_data = self.tag.is_some();
        let tag_name = self.tag_name();
        if config.language != Language::Cxx {
            return;
        }

        // Emit an ostream function if required.
        if config.enumeration.derive_ostream(&self.annotations) {
            // For enums without data, this emits the serializer function for the
            // enum. For enums with data, this emits the serializer function for
            // the tag enum. In the latter case we need a couple of minor changes
            // due to the function living inside the top-level struct or enum.
            let stream = config
                .function
                .rename_args
                .apply("stream", IdentifierType::FunctionArg);
            let instance = config
                .function
                .rename_args
                .apply("instance", IdentifierType::FunctionArg);

            out.new_line();
            out.new_line();
            // For enums without data, we mark the function inline because the
            // header might get included into multiple compilation units that
            // get linked together, and not marking it inline would result in
            // multiply-defined symbol errors. For enums with data we don't have
            // the same problem, but mark it as a friend function of the
            // containing union/struct.
            // Note also that for enums with data, the case labels for switch
            // statements apparently need to be qualified to the top-level
            // generated struct or union. This is why the generated case labels
            // below use the A::B::C format for enums with data, with A being
            // self.export_name(). Failure to have that qualification results
            // in a surprising compilation failure for the generated header.
            write!(
                out,
                "{} std::ostream& operator<<(std::ostream& {}, const {}& {})",
                if has_data { "friend" } else { "inline" },
                stream,
                tag_name,
                instance,
            );

            out.open_brace();
            if has_data {
                // C++ name resolution rules are weird.
                write!(
                    out,
                    "using {} = {}::{};",
                    tag_name,
                    self.export_name(),
                    tag_name
                );
                out.new_line();
            }
            write!(out, "switch ({})", instance);
            out.open_brace();
            let vec: Vec<_> = self
                .variants
                .iter()
                .map(|x| {
                    format!(
                        "case {}::{}: {} << \"{}\"; break;",
                        tag_name, x.export_name, stream, x.export_name
                    )
                })
                .collect();
            out.write_vertical_source_list(
                language_backend,
                &vec[..],
                ListType::Join(""),
                |_, out, s| write!(out, "{}", s),
            );
            out.close_brace(false);
            out.new_line();

            write!(out, "return {};", stream);
            out.close_brace(false);

            if has_data {
                // For enums with data, this emits the serializer function for
                // the top-level union or struct.
                out.new_line();
                out.new_line();
                write!(
                    out,
                    "friend std::ostream& operator<<(std::ostream& {}, const {}& {})",
                    stream,
                    self.export_name(),
                    instance,
                );

                out.open_brace();

                // C++ name resolution rules are weird.
                write!(
                    out,
                    "using {} = {}::{};",
                    tag_name,
                    self.export_name(),
                    tag_name
                );
                out.new_line();

                write!(out, "switch ({}.tag)", instance);
                out.open_brace();
                let vec: Vec<_> = self
                    .variants
                    .iter()
                    .map(|x| {
                        let tag_str = format!("\"{}\"", x.export_name);
                        if let VariantBody::Body {
                            ref name, ref body, ..
                        } = x.body
                        {
                            format!(
                                "case {}::{}: {} << {}{}{}.{}; break;",
                                tag_name,
                                x.export_name,
                                stream,
                                if body.has_tag_field { "" } else { &tag_str },
                                if body.has_tag_field { "" } else { " << " },
                                instance,
                                name,
                            )
                        } else {
                            format!(
                                "case {}::{}: {} << {}; break;",
                                tag_name, x.export_name, stream, tag_str,
                            )
                        }
                    })
                    .collect();
                out.write_vertical_source_list(
                    language_backend,
                    &vec[..],
                    ListType::Join(""),
                    |_, out, s| write!(out, "{}", s),
                );
                out.close_brace(false);
                out.new_line();

                write!(out, "return {};", stream);
                out.close_brace(false);
            }
        }
    }

    // Emit convenience methods for structs or unions produced for enums with data.
    pub(crate) fn write_derived_functions_data<
        F: Write,
        LB: LanguageBackend,
        WF: Fn(&mut LB, &mut SourceWriter<F>, &Field),
    >(
        &self,
        config: &Config,
        language_backend: &mut LB,
        out: &mut SourceWriter<F>,
        tag_name: &str,
        write_field: WF,
    ) {
        if config.language != Language::Cxx {
            return;
        }

        if config.enumeration.derive_helper_methods(&self.annotations) {
            for variant in &self.variants {
                out.new_line();
                out.new_line();

                let condition = variant.cfg.to_condition(config);
                condition.write_before(config, out);

                let arg_renamer = |name: &str| {
                    config
                        .function
                        .rename_args
                        .apply(name, IdentifierType::FunctionArg)
                        .into_owned()
                };

                macro_rules! write_attrs {
                    ($op:expr) => {{
                        if let Some(Some(attrs)) =
                            variant
                                .body
                                .annotations()
                                .atom(concat!("variant-", $op, "-attributes"))
                        {
                            write!(out, "{} ", attrs);
                        }
                    }};
                }

                write_attrs!("constructor");
                write!(out, "static {} {}(", self.export_name, variant.export_name);

                if let VariantBody::Body { ref body, .. } = variant.body {
                    let skip_fields = body.has_tag_field as usize;
                    let vec: Vec<_> = body
                        .fields
                        .iter()
                        .skip(skip_fields)
                        .map(|field| {
                            Field::from_name_and_type(
                                // const-ref args to constructor
                                arg_renamer(&field.name),
                                Type::const_ref_to(&field.ty),
                            )
                        })
                        .collect();
                    out.write_vertical_source_list(
                        language_backend,
                        &vec[..],
                        ListType::Join(","),
                        &write_field,
                    );
                }

                write!(out, ")");
                out.open_brace();

                write!(out, "{} result;", self.export_name);

                if let VariantBody::Body {
                    name: ref variant_name,
                    ref body,
                    ..
                } = variant.body
                {
                    let skip_fields = body.has_tag_field as usize;
                    for field in body.fields.iter().skip(skip_fields) {
                        out.new_line();
                        match field.ty {
                            Type::Array(ref ty, ref length) => {
                                // arrays are not assignable in C++ so we
                                // need to manually copy the elements
                                write!(out, "for (int i = 0; i < {}; i++)", length.as_str());
                                out.open_brace();
                                write!(out, "::new (&result.{}.{}[i]) (", variant_name, field.name);
                                language_backend.write_type(out, ty);
                                write!(out, ")({}[i]);", arg_renamer(&field.name));
                                out.close_brace(false);
                            }
                            ref ty => {
                                write!(out, "::new (&result.{}.{}) (", variant_name, field.name);
                                language_backend.write_type(out, ty);
                                write!(out, ")({});", arg_renamer(&field.name));
                            }
                        }
                    }
                }

                out.new_line();
                write!(out, "result.tag = {}::{};", tag_name, variant.export_name);
                out.new_line();
                write!(out, "return result;");
                out.close_brace(false);

                out.new_line();
                out.new_line();

                write_attrs!("is");
                // FIXME: create a config for method case
                write!(out, "bool Is{}() const", variant.export_name);
                out.open_brace();
                write!(out, "return tag == {}::{};", tag_name, variant.export_name);
                out.close_brace(false);

                let assert_name = match config.enumeration.cast_assert_name {
                    Some(ref n) => &**n,
                    None => "assert",
                };

                let mut derive_casts = |const_casts: bool| {
                    let (member_name, body, inline_casts) = match variant.body {
                        VariantBody::Body {
                            ref name,
                            ref body,
                            inline_casts,
                            ..
                        } => (name, body, inline_casts),
                        VariantBody::Empty(..) => return,
                    };

                    let skip_fields = body.has_tag_field as usize;
                    let field_count = body.fields.len() - skip_fields;
                    if field_count == 0 {
                        return;
                    }

                    out.new_line();
                    out.new_line();

                    if const_casts {
                        write_attrs!("const-cast");
                    } else {
                        write_attrs!("mut-cast");
                    }
                    if inline_casts {
                        let field = body.fields.last().unwrap();
                        let return_type = field.ty.clone();
                        let return_type = Type::Ptr {
                            ty: Box::new(return_type),
                            is_const: const_casts,
                            is_ref: true,
                            is_nullable: false,
                        };
                        language_backend.write_type(out, &return_type);
                    } else if const_casts {
                        write!(out, "const {}&", body.export_name());
                    } else {
                        write!(out, "{}&", body.export_name());
                    }

                    write!(out, " As{}()", variant.export_name);
                    if const_casts {
                        write!(out, " const");
                    }
                    out.open_brace();
                    write!(out, "{}(Is{}());", assert_name, variant.export_name);
                    out.new_line();
                    write!(out, "return {}", member_name);
                    if inline_casts {
                        write!(out, "._0");
                    }
                    write!(out, ";");
                    out.close_brace(false);
                };

                if config.enumeration.derive_const_casts(&self.annotations) {
                    derive_casts(true)
                }

                if config.enumeration.derive_mut_casts(&self.annotations) {
                    derive_casts(false)
                }

                condition.write_after(config, out);
            }
        }

        let other = config
            .function
            .rename_args
            .apply("other", IdentifierType::FunctionArg);

        macro_rules! write_attrs {
            ($op:expr) => {{
                if let Some(Some(attrs)) = self.annotations.atom(concat!($op, "-attributes")) {
                    write!(out, "{} ", attrs);
                }
            }};
        }

        if self.can_derive_eq() && config.structure.derive_eq(&self.annotations) {
            out.new_line();
            out.new_line();
            write_attrs!("eq");
            write!(
                out,
                "bool operator==(const {}& {}) const",
                self.export_name, other
            );
            out.open_brace();
            write!(out, "if (tag != {}.tag)", other);
            out.open_brace();
            write!(out, "return false;");
            out.close_brace(false);
            out.new_line();
            write!(out, "switch (tag)");
            out.open_brace();
            let mut exhaustive = true;
            for variant in &self.variants {
                if let VariantBody::Body {
                    name: ref variant_name,
                    ..
                } = variant.body
                {
                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);
                    write!(
                        out,
                        "case {}::{}: return {} == {}.{};",
                        self.tag.as_ref().unwrap(),
                        variant.export_name,
                        variant_name,
                        other,
                        variant_name
                    );
                    condition.write_after(config, out);
                    out.new_line();
                } else {
                    exhaustive = false;
                }
            }
            if !exhaustive {
                write!(out, "default: break;");
            }
            out.close_brace(false);

            out.new_line();
            write!(out, "return true;");

            out.close_brace(false);

            if config.structure.derive_neq(&self.annotations) {
                out.new_line();
                out.new_line();
                write_attrs!("neq");
                write!(
                    out,
                    "bool operator!=(const {}& {}) const",
                    self.export_name, other
                );
                out.open_brace();
                write!(out, "return !(*this == {});", other);
                out.close_brace(false);
            }
        }

        if config
            .enumeration
            .private_default_tagged_enum_constructor(&self.annotations)
        {
            out.new_line();
            out.new_line();
            write!(out, "private:");
            out.new_line();
            write!(out, "{}()", self.export_name);
            out.open_brace();
            out.close_brace(false);
            out.new_line();
            write!(out, "public:");
            out.new_line();
        }

        if config
            .enumeration
            .derive_tagged_enum_destructor(&self.annotations)
        {
            out.new_line();
            out.new_line();
            write_attrs!("destructor");
            write!(out, "~{}()", self.export_name);
            out.open_brace();
            write!(out, "switch (tag)");
            out.open_brace();
            let mut exhaustive = true;
            for variant in &self.variants {
                if let VariantBody::Body {
                    ref name, ref body, ..
                } = variant.body
                {
                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);
                    write!(
                        out,
                        "case {}::{}: {}.~{}(); break;",
                        self.tag.as_ref().unwrap(),
                        variant.export_name,
                        name,
                        body.export_name(),
                    );
                    condition.write_after(config, out);
                    out.new_line();
                } else {
                    exhaustive = false;
                }
            }
            if !exhaustive {
                write!(out, "default: break;");
            }
            out.close_brace(false);
            out.close_brace(false);
        }

        if config
            .enumeration
            .derive_tagged_enum_copy_constructor(&self.annotations)
        {
            out.new_line();
            out.new_line();
            write_attrs!("copy-constructor");
            write!(
                out,
                "{}(const {}& {})",
                self.export_name, self.export_name, other
            );
            out.new_line();
            write!(out, " : tag({}.tag)", other);
            out.open_brace();
            write!(out, "switch (tag)");
            out.open_brace();
            let mut exhaustive = true;
            for variant in &self.variants {
                if let VariantBody::Body {
                    ref name, ref body, ..
                } = variant.body
                {
                    let condition = variant.cfg.to_condition(config);
                    condition.write_before(config, out);
                    write!(
                        out,
                        "case {}::{}: ::new (&{}) ({})({}.{}); break;",
                        self.tag.as_ref().unwrap(),
                        variant.export_name,
                        name,
                        body.export_name(),
                        other,
                        name,
                    );
                    condition.write_after(config, out);
                    out.new_line();
                } else {
                    exhaustive = false;
                }
            }
            if !exhaustive {
                write!(out, "default: break;");
            }
            out.close_brace(false);
            out.close_brace(false);

            if config
                .enumeration
                .derive_tagged_enum_copy_assignment(&self.annotations)
            {
                out.new_line();
                write_attrs!("copy-assignment");
                write!(
                    out,
                    "{}& operator=(const {}& {})",
                    self.export_name, self.export_name, other
                );
                out.open_brace();
                write!(out, "if (this != &{})", other);
                out.open_brace();
                write!(out, "this->~{}();", self.export_name);
                out.new_line();
                write!(out, "new (this) {}({});", self.export_name, other);
                out.close_brace(false);
                out.new_line();
                write!(out, "return *this;");
                out.close_brace(false);
            }
        }
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        for variant in &mut self.variants {
            variant.simplify_standard_types(config);
        }
    }
}
