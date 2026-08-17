use crate::util::prelude::*;

pub(crate) trait ExprExt {
    fn require_path_mod_style(&self) -> Result<&syn::Path>;
}

impl ExprExt for syn::Expr {
    fn require_path_mod_style(&self) -> Result<&syn::Path> {
        let expr = match self {
            Self::Path(expr) => expr,
            _ => bail!(self, "expected a simple path, like `foo::bar`"),
        };

        crate::parsing::reject_attrs(&expr.attrs)?;
        crate::parsing::reject_syntax("<T as Trait> syntax", &expr.qself)?;

        expr.path.require_mod_style()?;

        Ok(&expr.path)
    }
}
