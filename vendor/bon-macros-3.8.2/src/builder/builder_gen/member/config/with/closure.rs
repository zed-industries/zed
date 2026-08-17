use crate::parsing::SimpleClosure;
use crate::util::prelude::*;
use darling::FromMeta;

const INVALID_RETURN_TYPE_ERROR: &str = "\
expected one of the following:

(1) no return type annotation;
    this means the closure is expected to return a value of the same type
    as the member's underlying type(*);

(2) `-> *Result<_, {{ErrorType}}>` or `-> *Result<_>` return type annotation;
    this means the closure is expected to return a `Result` where the `Ok`
    variant is of the same type as the member's underlying type(*); this syntax
    allows you to define a fallbile setter (one that returns a `Result<Builder>`);

    the `_` placeholder must be spelled literally to mark the underlying type(*)
    of the member; an optional second generic parameter for the error type is allowed;

    the return type doesn't have to be named `Result` exactly, the only requirement is
    that it must have the `Result` suffix; for example if you have a type alias
    `ApiResult<_>`, then it'll work fine;

(*) underlying type is the type of the member stripped from the `Option<T>` wrapper
    if this member is of `Option<T>` type and no `#[builder(required)]` annotation
    is present";

#[derive(Debug)]
pub(crate) struct SetterClosure {
    pub(crate) inputs: Vec<SetterClosureInput>,
    pub(crate) body: Box<syn::Expr>,
    pub(crate) output: Option<SetterClosureOutput>,
}

#[derive(Debug)]
pub(crate) struct SetterClosureOutput {
    pub(crate) result_path: syn::Path,
    pub(crate) err_ty: Option<syn::Type>,
}

#[derive(Debug)]
pub(crate) struct SetterClosureInput {
    pub(crate) pat: syn::PatIdent,
    pub(crate) ty: Box<syn::Type>,
}

impl FromMeta for SetterClosure {
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        let closure = SimpleClosure::from_meta(item)?;

        let inputs = closure
            .inputs
            .into_iter()
            .map(|input| {
                Ok(SetterClosureInput {
                    ty: input.ty.ok_or_else(|| {
                        err!(&input.pat, "expected a type for the setter input parameter")
                    })?,
                    pat: input.pat,
                })
            })
            .collect::<Result<_>>()?;

        let return_type = match closure.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => {
                let err = || err!(&ty, "{INVALID_RETURN_TYPE_ERROR}");

                let ty = ty
                    .as_generic_angle_bracketed_path(|last_segment| {
                        // We allow for arbitrary `Result` type variations
                        // including custom type aliases like `ApiResult<_>`
                        last_segment.to_string().ends_with("Result")
                    })
                    .ok_or_else(err)?;

                if !(1..=2).contains(&ty.args.len()) {
                    return Err(err());
                }

                let mut args = ty.args.iter();
                let ok_ty = args.next().ok_or_else(err)?;

                if !matches!(ok_ty, syn::GenericArgument::Type(syn::Type::Infer(_))) {
                    return Err(err());
                }

                let err_ty = args
                    .next()
                    .map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => Ok(ty.clone()),
                        _ => Err(err()),
                    })
                    .transpose()?;

                let mut result_path = ty.path.clone();

                // We store the error type of the result separately.
                // Strip the generic arguments, because we only care
                // about the path of the `Result` in `result_path` field.
                result_path
                    .segments
                    .last_mut()
                    .expect("BUG: segments can't be empty")
                    .arguments = syn::PathArguments::None;

                Some(SetterClosureOutput {
                    result_path,
                    err_ty,
                })
            }
        };

        Ok(Self {
            inputs,
            body: closure.body,
            output: return_type,
        })
    }
}
