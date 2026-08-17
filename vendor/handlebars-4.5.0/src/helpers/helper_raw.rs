use crate::context::Context;
use crate::helpers::{HelperDef, HelperResult};
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Helper, RenderContext, Renderable};

#[derive(Clone, Copy)]
pub struct RawHelper;

impl HelperDef for RawHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let tpl = h.template();
        if let Some(t) = tpl {
            t.render(r, ctx, rc, out)
        } else {
            Ok(())
        }
    }
}

pub static RAW_HELPER: RawHelper = RawHelper;

#[cfg(test)]
mod test {
    use crate::registry::Registry;

    #[test]
    fn test_raw_helper() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "a{{{{raw}}}}{{content}}{{else}}hello{{{{/raw}}}}")
            .is_ok());

        let r = handlebars.render("t0", &());
        assert_eq!(r.ok().unwrap(), "a{{content}}{{else}}hello");
    }
}
