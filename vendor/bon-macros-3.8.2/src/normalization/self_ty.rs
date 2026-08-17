use crate::util::prelude::*;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

pub(crate) struct NormalizeSelfTy<'a> {
    pub(crate) self_ty: &'a syn::Type,
}

impl VisitMut for NormalizeSelfTy<'_> {
    fn visit_item_mut(&mut self, _item: &mut syn::Item) {
        // Don't recurse into nested items because `Self` isn't available there.
    }

    fn visit_receiver_mut(&mut self, _receiver: &mut syn::Receiver) {
        // Don't recurse into the receiver. Keep it at `Self` as it was.
        // We normalize `Self` at a later stage. This is to keep the code
        // generated via `quote!()` using the simpler syntax of `self` without
        // an explicit type annotation, which requires that receiver's type
        // is left intact during normalization here.
    }

    fn visit_impl_item_fn_mut(&mut self, fn_item: &mut syn::ImplItemFn) {
        // We are interested only in signatures of functions. Don't recurse
        // into the function's block.
        self.visit_signature_mut(&mut fn_item.sig);
    }

    // TODO: this isn't all. We need to replace `Self` references in const generics
    // expressions as well.

    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        syn::visit_mut::visit_type_mut(self, ty);

        let ty_path = match ty {
            syn::Type::Path(ty_path) => ty_path,
            _ => return,
        };

        if !ty_path.path.is_ident("Self") {
            return;
        }

        *ty = (*self.self_ty).clone();
    }

    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        syn::visit_mut::visit_type_path_mut(self, type_path);

        let syn::TypePath { qself, path } = type_path;

        let is_self_projection =
            qself.is_none() && path.starts_with_segment("Self") && path.segments.len() > 1;

        if !is_self_projection {
            return;
        }

        // There is no `.remove()` method in `Punctuated`
        // https://github.com/dtolnay/syn/issues/1314
        path.segments = std::mem::take(&mut path.segments)
            .into_iter()
            .skip(1)
            .collect();

        let span = type_path.span();

        // QSelf doesn't implement `Parse` trait
        type_path.qself = Some(syn::QSelf {
            lt_token: syn::Token![<](span),
            ty: Box::new(self.self_ty.clone()),
            position: 0,
            as_token: None,
            gt_token: syn::Token![>](span),
        });
    }
}
