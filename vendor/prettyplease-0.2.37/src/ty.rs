use crate::algorithm::Printer;
use crate::fixup::FixupContext;
use crate::iter::IterDelimited;
use crate::path::PathKind;
use crate::INDENT;
use proc_macro2::TokenStream;
use syn::{
    Abi, BareFnArg, BareVariadic, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup,
    TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen, TypePath, TypePtr, TypeReference,
    TypeSlice, TypeTraitObject, TypeTuple,
};

impl Printer {
    pub fn ty(&mut self, ty: &Type) {
        match ty {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Type::Array(ty) => self.type_array(ty),
            Type::BareFn(ty) => self.type_bare_fn(ty),
            Type::Group(ty) => self.type_group(ty),
            Type::ImplTrait(ty) => self.type_impl_trait(ty),
            Type::Infer(ty) => self.type_infer(ty),
            Type::Macro(ty) => self.type_macro(ty),
            Type::Never(ty) => self.type_never(ty),
            Type::Paren(ty) => self.type_paren(ty),
            Type::Path(ty) => self.type_path(ty),
            Type::Ptr(ty) => self.type_ptr(ty),
            Type::Reference(ty) => self.type_reference(ty),
            Type::Slice(ty) => self.type_slice(ty),
            Type::TraitObject(ty) => self.type_trait_object(ty),
            Type::Tuple(ty) => self.type_tuple(ty),
            Type::Verbatim(ty) => self.type_verbatim(ty),
            _ => unimplemented!("unknown Type"),
        }
    }

    fn type_array(&mut self, ty: &TypeArray) {
        self.word("[");
        self.ty(&ty.elem);
        self.word("; ");
        self.expr(&ty.len, FixupContext::NONE);
        self.word("]");
    }

    fn type_bare_fn(&mut self, ty: &TypeBareFn) {
        if let Some(bound_lifetimes) = &ty.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        if ty.unsafety.is_some() {
            self.word("unsafe ");
        }
        if let Some(abi) = &ty.abi {
            self.abi(abi);
        }
        self.word("fn(");
        self.cbox(INDENT);
        self.zerobreak();
        for bare_fn_arg in ty.inputs.iter().delimited() {
            self.bare_fn_arg(&bare_fn_arg);
            self.trailing_comma(bare_fn_arg.is_last && ty.variadic.is_none());
        }
        if let Some(variadic) = &ty.variadic {
            self.bare_variadic(variadic);
            self.zerobreak();
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
        self.return_type(&ty.output);
    }

    fn type_group(&mut self, ty: &TypeGroup) {
        self.ty(&ty.elem);
    }

    fn type_impl_trait(&mut self, ty: &TypeImplTrait) {
        self.word("impl ");
        for type_param_bound in ty.bounds.iter().delimited() {
            if !type_param_bound.is_first {
                self.word(" + ");
            }
            self.type_param_bound(&type_param_bound);
        }
    }

    fn type_infer(&mut self, ty: &TypeInfer) {
        let _ = ty;
        self.word("_");
    }

    fn type_macro(&mut self, ty: &TypeMacro) {
        let semicolon = false;
        self.mac(&ty.mac, None, semicolon);
    }

    fn type_never(&mut self, ty: &TypeNever) {
        let _ = ty;
        self.word("!");
    }

    fn type_paren(&mut self, ty: &TypeParen) {
        self.word("(");
        self.ty(&ty.elem);
        self.word(")");
    }

    fn type_path(&mut self, ty: &TypePath) {
        self.qpath(&ty.qself, &ty.path, PathKind::Type);
    }

    fn type_ptr(&mut self, ty: &TypePtr) {
        self.word("*");
        if ty.mutability.is_some() {
            self.word("mut ");
        } else {
            self.word("const ");
        }
        self.ty(&ty.elem);
    }

    fn type_reference(&mut self, ty: &TypeReference) {
        self.word("&");
        if let Some(lifetime) = &ty.lifetime {
            self.lifetime(lifetime);
            self.nbsp();
        }
        if ty.mutability.is_some() {
            self.word("mut ");
        }
        self.ty(&ty.elem);
    }

    fn type_slice(&mut self, ty: &TypeSlice) {
        self.word("[");
        self.ty(&ty.elem);
        self.word("]");
    }

    fn type_trait_object(&mut self, ty: &TypeTraitObject) {
        self.word("dyn ");
        for type_param_bound in ty.bounds.iter().delimited() {
            if !type_param_bound.is_first {
                self.word(" + ");
            }
            self.type_param_bound(&type_param_bound);
        }
    }

    fn type_tuple(&mut self, ty: &TypeTuple) {
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for elem in ty.elems.iter().delimited() {
            self.ty(&elem);
            if ty.elems.len() == 1 {
                self.word(",");
                self.zerobreak();
            } else {
                self.trailing_comma(elem.is_last);
            }
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    #[cfg(not(feature = "verbatim"))]
    fn type_verbatim(&mut self, ty: &TokenStream) {
        unimplemented!("Type::Verbatim `{}`", ty);
    }

    #[cfg(feature = "verbatim")]
    fn type_verbatim(&mut self, tokens: &TokenStream) {
        use syn::parse::{Parse, ParseStream, Result};
        use syn::punctuated::Punctuated;
        use syn::{token, FieldsNamed, Token, TypeParamBound};

        enum TypeVerbatim {
            Ellipsis,
            AnonStruct(AnonStruct),
            AnonUnion(AnonUnion),
            DynStar(DynStar),
            MutSelf(MutSelf),
        }

        struct AnonStruct {
            fields: FieldsNamed,
        }

        struct AnonUnion {
            fields: FieldsNamed,
        }

        struct DynStar {
            bounds: Punctuated<TypeParamBound, Token![+]>,
        }

        struct MutSelf {
            ty: Option<Type>,
        }

        impl Parse for TypeVerbatim {
            fn parse(input: ParseStream) -> Result<Self> {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![struct]) {
                    input.parse::<Token![struct]>()?;
                    let fields: FieldsNamed = input.parse()?;
                    Ok(TypeVerbatim::AnonStruct(AnonStruct { fields }))
                } else if lookahead.peek(Token![union]) && input.peek2(token::Brace) {
                    input.parse::<Token![union]>()?;
                    let fields: FieldsNamed = input.parse()?;
                    Ok(TypeVerbatim::AnonUnion(AnonUnion { fields }))
                } else if lookahead.peek(Token![dyn]) {
                    input.parse::<Token![dyn]>()?;
                    input.parse::<Token![*]>()?;
                    let bounds = input.parse_terminated(TypeParamBound::parse, Token![+])?;
                    Ok(TypeVerbatim::DynStar(DynStar { bounds }))
                } else if lookahead.peek(Token![mut]) {
                    input.parse::<Token![mut]>()?;
                    input.parse::<Token![self]>()?;
                    let ty = if input.is_empty() {
                        None
                    } else {
                        input.parse::<Token![:]>()?;
                        let ty: Type = input.parse()?;
                        Some(ty)
                    };
                    Ok(TypeVerbatim::MutSelf(MutSelf { ty }))
                } else if lookahead.peek(Token![...]) {
                    input.parse::<Token![...]>()?;
                    Ok(TypeVerbatim::Ellipsis)
                } else {
                    Err(lookahead.error())
                }
            }
        }

        let ty: TypeVerbatim = match syn::parse2(tokens.clone()) {
            Ok(ty) => ty,
            Err(_) => unimplemented!("Type::Verbatim `{}`", tokens),
        };

        match ty {
            TypeVerbatim::Ellipsis => {
                self.word("...");
            }
            TypeVerbatim::AnonStruct(ty) => {
                self.cbox(INDENT);
                self.word("struct {");
                self.hardbreak_if_nonempty();
                for field in &ty.fields.named {
                    self.field(field);
                    self.word(",");
                    self.hardbreak();
                }
                self.offset(-INDENT);
                self.end();
                self.word("}");
            }
            TypeVerbatim::AnonUnion(ty) => {
                self.cbox(INDENT);
                self.word("union {");
                self.hardbreak_if_nonempty();
                for field in &ty.fields.named {
                    self.field(field);
                    self.word(",");
                    self.hardbreak();
                }
                self.offset(-INDENT);
                self.end();
                self.word("}");
            }
            TypeVerbatim::DynStar(ty) => {
                self.word("dyn* ");
                for type_param_bound in ty.bounds.iter().delimited() {
                    if !type_param_bound.is_first {
                        self.word(" + ");
                    }
                    self.type_param_bound(&type_param_bound);
                }
            }
            TypeVerbatim::MutSelf(bare_fn_arg) => {
                self.word("mut self");
                if let Some(ty) = &bare_fn_arg.ty {
                    self.word(": ");
                    self.ty(ty);
                }
            }
        }
    }

    pub fn return_type(&mut self, ty: &ReturnType) {
        match ty {
            ReturnType::Default => {}
            ReturnType::Type(_arrow, ty) => {
                self.word(" -> ");
                self.ty(ty);
            }
        }
    }

    fn bare_fn_arg(&mut self, bare_fn_arg: &BareFnArg) {
        self.outer_attrs(&bare_fn_arg.attrs);
        if let Some((name, _colon)) = &bare_fn_arg.name {
            self.ident(name);
            self.word(": ");
        }
        self.ty(&bare_fn_arg.ty);
    }

    fn bare_variadic(&mut self, variadic: &BareVariadic) {
        self.outer_attrs(&variadic.attrs);
        if let Some((name, _colon)) = &variadic.name {
            self.ident(name);
            self.word(": ");
        }
        self.word("...");
    }

    pub fn abi(&mut self, abi: &Abi) {
        self.word("extern ");
        if let Some(name) = &abi.name {
            self.lit_str(name);
            self.nbsp();
        }
    }
}
