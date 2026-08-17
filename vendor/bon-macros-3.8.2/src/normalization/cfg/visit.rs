use crate::util::prelude::*;

pub(crate) fn visit_attrs(
    item: &mut syn::Item,
    visit: impl FnMut(&mut Vec<syn::Attribute>) -> Result<bool>,
) -> Result {
    VisitAttrs { visit }.visit_item(item)
}

struct VisitAttrs<T> {
    visit: T,
}

impl<T> VisitAttrs<T>
where
    T: FnMut(&mut Vec<syn::Attribute>) -> Result<bool>,
{
    fn visit(&mut self, attrs: &mut Vec<syn::Attribute>) -> Result<bool> {
        (self.visit)(attrs)
    }

    fn visit_item(&mut self, item: &mut syn::Item) -> Result {
        match item {
            syn::Item::Fn(fn_item) => self.visit_item_fn(fn_item),
            syn::Item::Impl(impl_item) => self.visit_item_impl(impl_item),
            _ => Ok(()),
        }
    }

    fn visit_item_fn(&mut self, fn_item: &mut syn::ItemFn) -> Result {
        if !self.visit(&mut fn_item.attrs)? {
            bail!(
                fn_item,
                "This code should never be executed if there is a `#[cfg(...)]` attribute \
                on the function itself, because if that cfg evaluates to `false`, \
                no other proc macro on the item should be called."
            )
        }

        self.visit_signature(&mut fn_item.sig)
    }

    fn visit_impl_item_fn(&mut self, fn_item: &mut syn::ImplItemFn) -> Result<bool> {
        if !self.visit(&mut fn_item.attrs)? {
            return Ok(false);
        }

        self.visit_signature(&mut fn_item.sig)?;

        Ok(true)
    }

    fn visit_signature(&mut self, sig: &mut syn::Signature) -> Result {
        sig.inputs.try_retain_mut(|arg| match arg {
            syn::FnArg::Typed(arg) => self.visit(&mut arg.attrs),
            syn::FnArg::Receiver(arg) => self.visit(&mut arg.attrs),
        })?;

        self.visit_generics(&mut sig.generics)?;

        if let Some(variadic) = &mut sig.variadic {
            if !self.visit(&mut variadic.attrs)? {
                sig.variadic = None;
            }
        }

        // Where clause doesn't support attributes yet.
        // There is an issue in rust-lang/rust:
        // https://github.com/rust-lang/rust/issues/115590

        Ok(())
    }

    fn visit_generics(&mut self, generics: &mut syn::Generics) -> Result {
        generics.params.try_retain_mut(|param| match param {
            syn::GenericParam::Type(param) => self.visit(&mut param.attrs),
            syn::GenericParam::Lifetime(param) => self.visit(&mut param.attrs),
            syn::GenericParam::Const(param) => self.visit(&mut param.attrs),
        })
    }

    fn visit_item_impl(&mut self, impl_item: &mut syn::ItemImpl) -> Result {
        if !self.visit(&mut impl_item.attrs)? {
            bail!(
                impl_item,
                "This code should never be executed if there is a `#[cfg(...)]` attribute \
                on the impl itself, because if that cfg evaluates to `false`, \
                no other proc macro on the item should be called."
            )
        }

        self.visit_generics(&mut impl_item.generics)?;

        impl_item.items.try_retain_mut(|item| match item {
            syn::ImplItem::Fn(fn_item) => self.visit_impl_item_fn(fn_item),
            _ => Ok(true),
        })
    }
}
