use super::block_util::create_block;
use crate::block::BlockParams;
use crate::context::Context;
use crate::error::RenderError;
use crate::helpers::{HelperDef, HelperResult};
use crate::json::value::JsonTruthy;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Helper, RenderContext, Renderable};

#[derive(Clone, Copy)]
pub struct WithHelper;

impl HelperDef for WithHelper {
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
            .ok_or_else(|| RenderError::new("Param not found for helper \"with\""))?;

        if param.value().is_truthy(false) {
            let mut block = create_block(param);

            if let Some(block_param) = h.block_param() {
                let mut params = BlockParams::new();
                if param.context_path().is_some() {
                    params.add_path(block_param, Vec::with_capacity(0))?;
                } else {
                    params.add_value(block_param, param.value().clone())?;
                }

                block.set_block_params(params);
            }

            rc.push_block(block);

            if let Some(t) = h.template() {
                t.render(r, ctx, rc, out)?;
            };

            rc.pop_block();
            Ok(())
        } else if let Some(t) = h.inverse() {
            t.render(r, ctx, rc, out)
        } else if r.strict_mode() {
            Err(RenderError::strict_error(param.relative_path()))
        } else {
            Ok(())
        }
    }
}

pub static WITH_HELPER: WithHelper = WithHelper;

#[cfg(test)]
mod test {
    use crate::registry::Registry;

    #[derive(Serialize)]
    struct Address {
        city: String,
        country: String,
    }

    #[derive(Serialize)]
    struct Person {
        name: String,
        age: i16,
        addr: Address,
        titles: Vec<String>,
    }

    #[test]
    fn test_with() {
        let addr = Address {
            city: "Beijing".to_string(),
            country: "China".to_string(),
        };

        let person = Person {
            name: "Ning Sun".to_string(),
            age: 27,
            addr,
            titles: vec!["programmer".to_string(), "cartographier".to_string()],
        };

        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{#with addr}}{{city}}{{/with}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{#with notfound}}hello{{else}}world{{/with}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t2", "{{#with addr/country}}{{this}}{{/with}}")
            .is_ok());

        let r0 = handlebars.render("t0", &person);
        assert_eq!(r0.ok().unwrap(), "Beijing".to_string());

        let r1 = handlebars.render("t1", &person);
        assert_eq!(r1.ok().unwrap(), "world".to_string());

        let r2 = handlebars.render("t2", &person);
        assert_eq!(r2.ok().unwrap(), "China".to_string());
    }

    #[test]
    fn test_with_block_param() {
        let addr = Address {
            city: "Beijing".to_string(),
            country: "China".to_string(),
        };

        let person = Person {
            name: "Ning Sun".to_string(),
            age: 27,
            addr,
            titles: vec!["programmer".to_string(), "cartographier".to_string()],
        };

        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{#with addr as |a|}}{{a.city}}{{/with}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{#with notfound as |c|}}hello{{else}}world{{/with}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t2", "{{#with addr/country as |t|}}{{t}}{{/with}}")
            .is_ok());

        let r0 = handlebars.render("t0", &person);
        assert_eq!(r0.ok().unwrap(), "Beijing".to_string());

        let r1 = handlebars.render("t1", &person);
        assert_eq!(r1.ok().unwrap(), "world".to_string());

        let r2 = handlebars.render("t2", &person);
        assert_eq!(r2.ok().unwrap(), "China".to_string());
    }

    #[test]
    fn test_with_in_each() {
        let addr = Address {
            city: "Beijing".to_string(),
            country: "China".to_string(),
        };

        let person = Person {
            name: "Ning Sun".to_string(),
            age: 27,
            addr,
            titles: vec!["programmer".to_string(), "cartographier".to_string()],
        };

        let addr2 = Address {
            city: "Beijing".to_string(),
            country: "China".to_string(),
        };

        let person2 = Person {
            name: "Ning Sun".to_string(),
            age: 27,
            addr: addr2,
            titles: vec!["programmer".to_string(), "cartographier".to_string()],
        };

        let people = vec![person, person2];

        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string(
                "t0",
                "{{#each this}}{{#with addr}}{{city}}{{/with}}{{/each}}"
            )
            .is_ok());
        assert!(handlebars
            .register_template_string(
                "t1",
                "{{#each this}}{{#with addr}}{{../age}}{{/with}}{{/each}}"
            )
            .is_ok());
        assert!(handlebars
            .register_template_string(
                "t2",
                "{{#each this}}{{#with addr}}{{@../index}}{{/with}}{{/each}}"
            )
            .is_ok());

        let r0 = handlebars.render("t0", &people);
        assert_eq!(r0.ok().unwrap(), "BeijingBeijing".to_string());

        let r1 = handlebars.render("t1", &people);
        assert_eq!(r1.ok().unwrap(), "2727".to_string());

        let r2 = handlebars.render("t2", &people);
        assert_eq!(r2.ok().unwrap(), "01".to_string());
    }

    #[test]
    fn test_path_up() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{#with a}}{{#with b}}{{../../d}}{{/with}}{{/with}}")
            .is_ok());
        let data = json!({
            "a": {
                "b": [{"c": [1]}]
            },
            "d": 1
        });

        let r0 = handlebars.render("t0", &data);
        assert_eq!(r0.ok().unwrap(), "1".to_string());
    }

    #[test]
    fn test_else_context() {
        let reg = Registry::new();
        let template = "{{#with list}}A{{else}}{{foo}}{{/with}}";
        let input = json!({"list": [], "foo": "bar"});
        let rendered = reg.render_template(template, &input).unwrap();
        assert_eq!("bar", rendered);
    }

    #[test]
    fn test_derived_value() {
        let hb = Registry::new();
        let data = json!({"a": {"b": {"c": "d"}}});
        let template = "{{#with (lookup a.b \"c\")}}{{this}}{{/with}}";
        assert_eq!("d", hb.render_template(template, &data).unwrap());
    }

    #[test]
    fn test_nested_derived_value() {
        let hb = Registry::new();
        let data = json!({"a": {"b": {"c": "d"}}});
        let template = "{{#with (lookup a \"b\")}}{{#with this}}{{c}}{{/with}}{{/with}}";
        assert_eq!("d", hb.render_template(template, &data).unwrap());
    }

    #[test]
    fn test_strict_with() {
        let mut hb = Registry::new();

        assert_eq!(
            hb.render_template("{{#with name}}yes{{/with}}", &json!({}))
                .unwrap(),
            ""
        );
        assert_eq!(
            hb.render_template("{{#with name}}yes{{else}}no{{/with}}", &json!({}))
                .unwrap(),
            "no"
        );

        hb.set_strict_mode(true);

        assert!(hb
            .render_template("{{#with name}}yes{{/with}}", &json!({}))
            .is_err());
        assert_eq!(
            hb.render_template("{{#with name}}yes{{else}}no{{/with}}", &json!({}))
                .unwrap(),
            "no"
        );
    }
}
