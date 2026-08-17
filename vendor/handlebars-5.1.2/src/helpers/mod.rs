use crate::context::Context;
use crate::error::{RenderError, RenderErrorReason};
use crate::json::value::ScopedJson;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{do_escape, Helper, RenderContext};

pub use self::helper_each::EACH_HELPER;
pub use self::helper_if::{IF_HELPER, UNLESS_HELPER};
pub use self::helper_log::LOG_HELPER;
pub use self::helper_lookup::LOOKUP_HELPER;
pub use self::helper_raw::RAW_HELPER;
pub use self::helper_with::WITH_HELPER;

/// A type alias for `Result<(), RenderError>`
pub type HelperResult = Result<(), RenderError>;

/// Helper Definition
///
/// Implement `HelperDef` to create custom helpers. You can retrieve useful information from these arguments.
///
/// * `&Helper`: current helper template information, contains name, params, hashes and nested template
/// * `&Registry`: the global registry, you can find templates by name from registry
/// * `&Context`: the whole data to render, in most case you can use data from `Helper`
/// * `&mut RenderContext`: you can access data or modify variables (starts with @)/partials in render context, for example, @index of #each. See its document for detail.
/// * `&mut dyn Output`: where you write output to
///
/// By default, you can use a bare function as a helper definition because we have supported unboxed_closure. If you have stateful or configurable helper, you can create a struct to implement `HelperDef`.
///
/// ## Define an inline helper
///
/// ```
/// use handlebars::*;
///
/// fn upper(h: &Helper< '_>, _: &Handlebars<'_>, _: &Context, rc:
/// &mut RenderContext<'_, '_>, out: &mut dyn Output)
///     -> HelperResult {
///    // get parameter from helper or throw an error
///    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
///    out.write(param.to_uppercase().as_ref())?;
///    Ok(())
/// }
/// ```
///
/// ## Define block helper
///
/// Block helper is like `#if` or `#each` which has a inner template and an optional *inverse* template (the template in else branch). You can access the inner template by `helper.template()` and `helper.inverse()`. In most cases you will just call `render` on it.
///
/// ```
/// use handlebars::*;
///
/// fn dummy_block<'reg, 'rc>(
///     h: &Helper<'rc>,
///     r: &'reg Handlebars<'reg>,
///     ctx: &'rc Context,
///     rc: &mut RenderContext<'reg, 'rc>,
///     out: &mut dyn Output,
/// ) -> HelperResult {
///     h.template()
///         .map(|t| t.render(r, ctx, rc, out))
///         .unwrap_or(Ok(()))
/// }
/// ```
///
/// ## Define helper function using macro
///
/// In most cases you just need some simple function to call from templates. We have a `handlebars_helper!` macro to simplify the job.
///
/// ```
/// use handlebars::*;
///
/// handlebars_helper!(plus: |x: i64, y: i64| x + y);
///
/// let mut hbs = Handlebars::new();
/// hbs.register_helper("plus", Box::new(plus));
/// ```
///
pub trait HelperDef {
    /// A simplified api to define helper
    ///
    /// To implement your own `call_inner`, you will return a new `ScopedJson`
    /// which has a JSON value computed from current context.
    ///
    /// ### Calling from subexpression
    ///
    /// When calling the helper as a subexpression, the value and its type can
    /// be received by upper level helpers.
    ///
    /// Note that the value can be `json!(null)` which is treated as `false` in
    /// helpers like `if` and rendered as empty string.
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &Helper<'rc>,
        _: &'reg Registry<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        Err(RenderErrorReason::Unimplemented.into())
    }

    /// A complex version of helper interface.
    ///
    /// This function offers `Output`, which you can write custom string into
    /// and render child template. Helpers like `if` and `each` are implemented
    /// with this. Because the data written into `Output` are typically without
    /// type information. So helpers defined by this function are not composable.
    ///
    /// ### Calling from subexpression
    ///
    /// Although helpers defined by this are not composable, when called from
    /// subexpression, handlebars tries to parse the string output as JSON to
    /// re-build its type. This can be buggy with numrical and other literal values.
    /// So it is not recommended to use these helpers in subexpression.
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        match self.call_inner(h, r, ctx, rc) {
            Ok(result) => {
                if r.strict_mode() && result.is_missing() {
                    Err(RenderError::strict_error(None))
                } else {
                    // auto escape according to settings
                    let output = do_escape(r, rc, result.render());
                    out.write(output.as_ref())?;
                    Ok(())
                }
            }
            Err(e) => {
                if e.is_unimplemented() {
                    // default implementation, do nothing
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}

/// implement HelperDef for bare function so we can use function as helper
impl<
        F: for<'reg, 'rc> Fn(
            &Helper<'rc>,
            &'reg Registry<'reg>,
            &'rc Context,
            &mut RenderContext<'reg, 'rc>,
            &mut dyn Output,
        ) -> HelperResult,
    > HelperDef for F
{
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        (*self)(h, r, ctx, rc, out)
    }
}

mod block_util;
mod helper_each;
pub(crate) mod helper_extras;
mod helper_if;
mod helper_log;
mod helper_lookup;
mod helper_raw;
mod helper_with;
#[cfg(feature = "script_helper")]
pub(crate) mod scripting;

#[cfg(feature = "string_helpers")]
pub(crate) mod string_helpers;

// pub type HelperDef = for <'a, 'b, 'c> Fn<(&'a Context, &'b Helper, &'b Registry, &'c mut RenderContext), Result<String, RenderError>>;
//
// pub fn helper_dummy (ctx: &Context, h: &Helper, r: &Registry, rc: &mut RenderContext) -> Result<String, RenderError> {
// h.template().unwrap().render(ctx, r, rc).unwrap()
// }
//
#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::context::Context;
    use crate::error::RenderError;
    use crate::helpers::HelperDef;
    use crate::json::value::JsonRender;
    use crate::output::Output;
    use crate::registry::Registry;
    use crate::render::{Helper, RenderContext, Renderable};

    #[derive(Clone, Copy)]
    struct MetaHelper;

    impl HelperDef for MetaHelper {
        fn call<'reg: 'rc, 'rc>(
            &self,
            h: &Helper<'rc>,
            r: &'reg Registry<'reg>,
            ctx: &'rc Context,
            rc: &mut RenderContext<'reg, 'rc>,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            let v = h.param(0).unwrap();

            write!(out, "{}:{}", h.name(), v.value().render())?;
            if h.is_block() {
                out.write("->")?;
                h.template().unwrap().render(r, ctx, rc, out)?;
            }
            Ok(())
        }
    }

    #[test]
    fn test_meta_helper() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{foo this}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{#bar this}}nice{{/bar}}")
            .is_ok());

        let meta_helper = MetaHelper;
        handlebars.register_helper("helperMissing", Box::new(meta_helper));
        handlebars.register_helper("blockHelperMissing", Box::new(meta_helper));

        let r0 = handlebars.render("t0", &true);
        assert_eq!(r0.ok().unwrap(), "foo:true".to_string());

        let r1 = handlebars.render("t1", &true);
        assert_eq!(r1.ok().unwrap(), "bar:true->nice".to_string());
    }

    #[test]
    fn test_helper_for_subexpression() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t2", "{{foo value=(bar 0)}}")
            .is_ok());

        handlebars.register_helper(
            "helperMissing",
            Box::new(
                |h: &Helper<'_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    write!(out, "{}{}", h.name(), h.param(0).unwrap().value())?;
                    Ok(())
                },
            ),
        );
        handlebars.register_helper(
            "foo",
            Box::new(
                |h: &Helper<'_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    write!(out, "{}", h.hash_get("value").unwrap().value().render())?;
                    Ok(())
                },
            ),
        );

        let mut data = BTreeMap::new();
        // handlebars should never try to lookup this value because
        // subexpressions are now resolved as string literal
        data.insert("bar0".to_string(), true);

        let r2 = handlebars.render("t2", &data);

        assert_eq!(r2.ok().unwrap(), "bar0".to_string());
    }
}
