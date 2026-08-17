use crate::config::{AsyncConf, ErrorConf, ErrorConfField, TracingConf};
use anyhow::{Error, anyhow};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::rc::Rc;
use witx::{Document, Id, InterfaceFunc, Module, NamedType, TypeRef};

pub use crate::config::Asyncness;

pub struct CodegenSettings {
    pub errors: ErrorTransform,
    pub async_: AsyncConf,
    pub wasmtime: bool,
    /// Disabling this feature makes it possible to remove all of the tracing
    /// code emitted in the Wiggle-generated code; this can be helpful while
    /// inspecting the code (e.g., with `cargo expand`).
    pub tracing: TracingConf,
    /// Determine whether the context structure will use `&mut self` (true) or
    /// simply `&self`.
    pub mutable: bool,
}
impl CodegenSettings {
    pub fn new(
        error_conf: &ErrorConf,
        async_: &AsyncConf,
        doc: &Document,
        wasmtime: bool,
        tracing: &TracingConf,
        mutable: bool,
    ) -> Result<Self, Error> {
        let errors = ErrorTransform::new(error_conf, doc)?;
        Ok(Self {
            errors,
            async_: async_.clone(),
            wasmtime,
            tracing: tracing.clone(),
            mutable,
        })
    }
    pub fn get_async(&self, module: &Module, func: &InterfaceFunc) -> Asyncness {
        self.async_.get(module.name.as_str(), func.name.as_str())
    }
}

pub struct ErrorTransform {
    m: Vec<ErrorType>,
}

impl ErrorTransform {
    pub fn empty() -> Self {
        Self { m: Vec::new() }
    }
    pub fn new(conf: &ErrorConf, doc: &Document) -> Result<Self, Error> {
        let mut richtype_identifiers = HashMap::new();
        let m = conf.iter().map(|(ident, field)|
            match field {
                ErrorConfField::Trappable(field) => if let Some(abi_type) = doc.typename(&Id::new(ident.to_string())) {
                    Ok(ErrorType::Generated(TrappableErrorType { abi_type, rich_type: field.rich_error.clone() }))
                } else {
                    Err(anyhow!("No witx typename \"{}\" found", ident.to_string()))
                },
                ErrorConfField::User(field) => if let Some(abi_type) = doc.typename(&Id::new(ident.to_string())) {
                    if let Some(ident) = field.rich_error.get_ident() {
                        if let Some(prior_def) = richtype_identifiers.insert(ident.clone(), field.err_loc)
                         {
                            return Err(anyhow!(
                                    "duplicate rich type identifier of {:?} not allowed. prior definition at {:?}",
                                    ident, prior_def
                                ));
                        }
                        Ok(ErrorType::User(UserErrorType {
                            abi_type,
                            rich_type: field.rich_error.clone(),
                            method_fragment: ident.to_string()
                        }))
                    } else {
                        return Err(anyhow!(
                            "rich error type must be identifier for now - TODO add ability to provide a corresponding identifier: {:?}",
                            field.err_loc
                        ))
                    }
                }
                else { Err(anyhow!("No witx typename \"{}\" found", ident.to_string())) }
            }
        ).collect::<Result<Vec<_>, Error>>()?;
        Ok(Self { m })
    }

    pub fn iter(&self) -> impl Iterator<Item = &ErrorType> {
        self.m.iter()
    }

    pub fn for_abi_error(&self, tref: &TypeRef) -> Option<&ErrorType> {
        match tref {
            TypeRef::Name(nt) => self.for_name(nt),
            TypeRef::Value { .. } => None,
        }
    }

    pub fn for_name(&self, nt: &NamedType) -> Option<&ErrorType> {
        self.m.iter().find(|e| e.abi_type().name == nt.name)
    }
}

pub enum ErrorType {
    User(UserErrorType),
    Generated(TrappableErrorType),
}
impl ErrorType {
    pub fn abi_type(&self) -> &NamedType {
        match self {
            Self::User(u) => &u.abi_type,
            Self::Generated(r) => &r.abi_type,
        }
    }
}

pub struct TrappableErrorType {
    abi_type: Rc<NamedType>,
    rich_type: Ident,
}

impl TrappableErrorType {
    pub fn abi_type(&self) -> TypeRef {
        TypeRef::Name(self.abi_type.clone())
    }
    pub fn typename(&self) -> TokenStream {
        let richtype = &self.rich_type;
        quote!(#richtype)
    }
}

pub struct UserErrorType {
    abi_type: Rc<NamedType>,
    rich_type: syn::Path,
    method_fragment: String,
}

impl UserErrorType {
    pub fn abi_type(&self) -> TypeRef {
        TypeRef::Name(self.abi_type.clone())
    }
    pub fn typename(&self) -> TokenStream {
        let t = &self.rich_type;
        quote!(#t)
    }
    pub fn method_fragment(&self) -> &str {
        &self.method_fragment
    }
}
