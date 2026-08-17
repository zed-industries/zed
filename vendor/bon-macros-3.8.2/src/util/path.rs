use crate::util::prelude::*;

pub(crate) trait PathExt {
    fn starts_with_segment(&self, desired_segment: &str) -> bool;
    fn ends_with_segment(&self, desired_segment: &str) -> bool;

    /// Returns an error if this path has some generic arguments.
    fn require_mod_style(&self) -> Result;
}

impl PathExt for syn::Path {
    fn starts_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .first()
            .map(|first| first.ident == desired_segment)
            .unwrap_or(false)
    }

    fn ends_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .last()
            .map(|first| first.ident == desired_segment)
            .unwrap_or(false)
    }

    fn require_mod_style(&self) -> Result {
        if self
            .segments
            .iter()
            .any(|seg| seg.arguments != syn::PathArguments::None)
        {
            bail!(self, "expected a simple path e.g. `foo::bar`");
        }

        Ok(())
    }
}
