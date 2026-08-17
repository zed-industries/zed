extern crate handlebars;
#[macro_use]
extern crate serde_json;

use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};

#[test]
fn test_subexpression() {
    let hbs = Handlebars::new();

    let data = json!({"a": 1, "b": 0, "c": 2});

    assert_eq!(
        hbs.render_template("{{#if (gt a b)}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Success"
    );

    assert_eq!(
        hbs.render_template("{{#if (gt a c)}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Failed"
    );

    assert_eq!(
        hbs.render_template("{{#if (not (gt a c))}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Success"
    );

    assert_eq!(
        hbs.render_template("{{#if (not (gt a b))}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Failed"
    );

    // no argument provided for not
    assert!(hbs
        .render_template("{{#if (not)}}Success{{else}}Failed{{/if}}", &data)
        .is_err());

    // json literal
    assert_eq!(
        hbs.render_template("{{#if (not true)}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Failed"
    );
    assert_eq!(
        hbs.render_template("{{#if (not false)}}Success{{else}}Failed{{/if}}", &data)
            .unwrap(),
        "Success"
    );
}

#[test]
fn test_strict_mode() {
    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);

    let data = json!({"a": 1});

    assert!(hbs
        .render_template("{{#if (eq a 1)}}Success{{else}}Failed{{/if}}", &data)
        .is_ok());
    assert!(hbs
        .render_template("{{#if (eq z 1)}}Success{{else}}Failed{{/if}}", &data)
        .is_err())
}

#[test]
fn invalid_json_path() {
    // The data here is not important
    let data = &Vec::<()>::new();

    let hbs = Handlebars::new();

    let error = hbs.render_template("{{x[]@this}}", &data).unwrap_err();

    let expected = "Error rendering \"Unnamed template\" line 1, col 1: Helper not defined: \"x\"";

    assert_eq!(format!("{}", error), expected);
}

struct MyHelper;

impl HelperDef for MyHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Ok(ScopedJson::Derived(json!({
            "a": 1,
            "b": 2,
        })))
    }
}

#[test]
fn test_lookup_with_subexpression() {
    let mut registry = Handlebars::new();
    registry.register_helper("myhelper", Box::new(MyHelper {}));
    registry
        .register_template_string("t", "{{ lookup (myhelper) \"a\" }}")
        .unwrap();

    let result = registry.render("t", &json!({})).unwrap();

    assert_eq!("1", result);
}

struct CallCounterHelper {
    pub(crate) c: Arc<AtomicU16>,
}

impl HelperDef for CallCounterHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        // inc counter
        self.c.fetch_add(1, Ordering::SeqCst);

        if let Some(_) = h.param(0) {
            Ok(json!({
                "a": 1,
            })
            .into())
        } else {
            Ok(json!(null).into())
        }
    }
}

#[test]
fn test_helper_call_count() {
    let mut registry = Handlebars::new();

    let counter = Arc::new(AtomicU16::new(0));
    let helper = Box::new(CallCounterHelper { c: counter.clone() });

    registry.register_helper("myhelper", helper);

    registry
        .render_template(
            "{{#if (myhelper a)}}something{{else}}nothing{{/if}}",
            &json!(null),
        ) // If returns true
        .unwrap();

    assert_eq!(1, counter.load(Ordering::SeqCst));

    registry
        .render_template(
            "{{#if (myhelper)}}something{{else}}nothing{{/if}}",
            &json!(null),
        ) // If returns false
        .unwrap();

    assert_eq!(2, counter.load(Ordering::SeqCst));
}
