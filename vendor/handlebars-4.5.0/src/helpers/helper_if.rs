use crate::context::Context;
use crate::error::RenderError;
use crate::helpers::{HelperDef, HelperResult};
use crate::json::value::JsonTruthy;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Helper, RenderContext, Renderable};

#[derive(Clone, Copy)]
pub struct IfHelper {
    positive: bool,
}

impl HelperDef for IfHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h
            .param(0)
            .ok_or_else(|| RenderError::new("Param not found for helper \"if\""))?;
        let include_zero = h
            .hash_get("includeZero")
            .and_then(|v| v.value().as_bool())
            .unwrap_or(false);

        let mut value = param.value().is_truthy(include_zero);

        if !self.positive {
            value = !value;
        }

        let tmpl = if value { h.template() } else { h.inverse() };
        match tmpl {
            Some(t) => t.render(r, ctx, rc, out),
            None => Ok(()),
        }
    }
}

pub static IF_HELPER: IfHelper = IfHelper { positive: true };
pub static UNLESS_HELPER: IfHelper = IfHelper { positive: false };

#[cfg(test)]
mod test {
    use crate::helpers::WITH_HELPER;
    use crate::registry::Registry;
    use serde_json::value::Value as Json;
    use std::str::FromStr;

    #[test]
    fn test_if() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{#if this}}hello{{/if}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{#unless this}}hello{{else}}world{{/unless}}")
            .is_ok());

        let r0 = handlebars.render("t0", &true);
        assert_eq!(r0.ok().unwrap(), "hello".to_string());

        let r1 = handlebars.render("t1", &true);
        assert_eq!(r1.ok().unwrap(), "world".to_string());

        let r2 = handlebars.render("t0", &false);
        assert_eq!(r2.ok().unwrap(), "".to_string());
    }

    #[test]
    fn test_if_context() {
        let json_str = r#"{"a":{"b":99,"c":{"d": true}}}"#;
        let data = Json::from_str(json_str).unwrap();

        let mut handlebars = Registry::new();
        handlebars.register_helper("with", Box::new(WITH_HELPER));
        assert!(handlebars
            .register_template_string("t0", "{{#if a.c.d}}hello {{a.b}}{{/if}}")
            .is_ok());
        assert!(handlebars
            .register_template_string(
                "t1",
                "{{#with a}}{{#if c.d}}hello {{../a.b}}{{/if}}{{/with}}"
            )
            .is_ok());

        let r0 = handlebars.render("t0", &data);
        assert_eq!(r0.unwrap(), "hello 99".to_string());

        let r1 = handlebars.render("t1", &data);
        assert_eq!(r1.unwrap(), "hello 99".to_string());
    }

    #[test]
    fn test_if_include_zero() {
        use std::f64;
        let handlebars = Registry::new();

        assert_eq!(
            "0".to_owned(),
            handlebars
                .render_template("{{#if a}}1{{else}}0{{/if}}", &json!({"a": 0}))
                .unwrap()
        );
        assert_eq!(
            "1".to_owned(),
            handlebars
                .render_template(
                    "{{#if a includeZero=true}}1{{else}}0{{/if}}",
                    &json!({"a": 0})
                )
                .unwrap()
        );
        assert_eq!(
            "0".to_owned(),
            handlebars
                .render_template(
                    "{{#if a includeZero=true}}1{{else}}0{{/if}}",
                    &json!({ "a": f64::NAN })
                )
                .unwrap()
        );
    }

    #[test]
    fn test_invisible_line_stripping() {
        let hbs = Registry::new();
        assert_eq!(
            "yes\n",
            hbs.render_template("{{#if a}}\nyes\n{{/if}}\n", &json!({"a": true}))
                .unwrap()
        );

        assert_eq!(
            "yes\r\n",
            hbs.render_template("{{#if a}}\r\nyes\r\n{{/if}}\r\n", &json!({"a": true}))
                .unwrap()
        );

        assert_eq!(
            "x\ny",
            hbs.render_template("{{#if a}}x{{/if}}\ny", &json!({"a": true}))
                .unwrap()
        );

        assert_eq!(
            "y\nz",
            hbs.render_template("{{#if a}}\nx\n{{^}}\ny\n{{/if}}\nz", &json!({"a": false}))
                .unwrap()
        );

        assert_eq!(
            r#"yes
  foo
  bar
  baz"#,
            hbs.render_template(
                r#"yes
  {{#if true}}
  foo
  bar
  {{/if}}
  baz"#,
                &json!({})
            )
            .unwrap()
        );

        assert_eq!(
            r#"  foo
  bar
  baz"#,
            hbs.render_template(
                r#"  {{#if true}}
  foo
  bar
  {{/if}}
  baz"#,
                &json!({})
            )
            .unwrap()
        );
    }
}
