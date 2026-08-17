use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::value::Value as Json;

use crate::block::BlockContext;
use crate::context::{merge_json, Context};
use crate::error::RenderError;
use crate::json::path::Path;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Decorator, Evaluable, RenderContext, Renderable};
use crate::template::Template;

pub(crate) const PARTIAL_BLOCK: &str = "@partial-block";

fn find_partial<'reg: 'rc, 'rc: 'a, 'a>(
    rc: &'a RenderContext<'reg, 'rc>,
    r: &'reg Registry<'reg>,
    d: &Decorator<'reg, 'rc>,
    name: &str,
) -> Result<Option<Cow<'a, Template>>, RenderError> {
    if let Some(partial) = rc.get_partial(name) {
        return Ok(Some(Cow::Borrowed(partial)));
    }

    if let Some(tpl) = r.get_or_load_template_optional(name) {
        return tpl.map(Option::Some);
    }

    if let Some(tpl) = d.template() {
        return Ok(Some(Cow::Borrowed(tpl)));
    }

    Ok(None)
}

pub fn expand_partial<'reg: 'rc, 'rc>(
    d: &Decorator<'reg, 'rc>,
    r: &'reg Registry<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // try eval inline partials first
    if let Some(t) = d.template() {
        t.eval(r, ctx, rc)?;
    }

    let tname = d.name();
    if rc.is_current_template(tname) {
        return Err(RenderError::new("Cannot include self in >"));
    }

    let partial = find_partial(rc, r, d, tname)?;

    if let Some(t) = partial {
        // clone to avoid lifetime issue
        // FIXME refactor this to avoid
        let mut local_rc = rc.clone();

        // if tname == PARTIAL_BLOCK
        let is_partial_block = tname == PARTIAL_BLOCK;

        // add partial block depth there are consecutive partial
        // blocks in the stack.
        if is_partial_block {
            local_rc.inc_partial_block_depth();
        } else {
            // depth cannot be lower than 0, which is guaranted in the
            // `dec_partial_block_depth` method
            local_rc.dec_partial_block_depth();
        }

        let mut block_created = false;

        // create context if param given
        if let Some(base_path) = d.param(0).and_then(|p| p.context_path()) {
            // path given, update base_path
            let mut block_inner = BlockContext::new();
            *block_inner.base_path_mut() = base_path.to_vec();

            // because block is moved here, we need another bool variable to track
            // its status for later cleanup
            block_created = true;
            // clear blocks to prevent block params from parent
            // template to be leaked into partials
            // see `test_partial_context_issue_495` for the case.
            local_rc.clear_blocks();
            local_rc.push_block(block_inner);
        }

        if !d.hash().is_empty() {
            // hash given, update base_value
            let hash_ctx = d
                .hash()
                .iter()
                .map(|(k, v)| (*k, v.value()))
                .collect::<HashMap<&str, &Json>>();

            // create block if we didn't (no param provided for partial expression)
            if !block_created {
                let block_inner = if let Some(block) = local_rc.block() {
                    // reuse current block information, including base_path and
                    // base_value if any
                    block.clone()
                } else {
                    BlockContext::new()
                };

                local_rc.clear_blocks();
                local_rc.push_block(block_inner);
            }

            // evaluate context within current block, this includes block
            // context provided by partial expression parameter
            let merged_context = merge_json(
                local_rc.evaluate2(ctx, &Path::current())?.as_json(),
                &hash_ctx,
            );

            // update the base value, there must be a block for this so it's
            // also safe to unwrap.
            if let Some(block) = local_rc.block_mut() {
                block.set_base_value(merged_context);
            }
        }

        // @partial-block
        if let Some(pb) = d.template() {
            local_rc.push_partial_block(pb);
        }

        // indent
        local_rc.set_indent_string(d.indent());

        let result = t.render(r, ctx, &mut local_rc, out);

        // cleanup
        if block_created {
            local_rc.pop_block();
        }

        if d.template().is_some() {
            local_rc.pop_partial_block();
        }

        result
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;
    use crate::error::RenderError;
    use crate::output::Output;
    use crate::registry::Registry;
    use crate::render::{Helper, RenderContext};

    #[test]
    fn test() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{> t1}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{this}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t2", "{{#> t99}}not there{{/t99}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t3", "{{#*inline \"t31\"}}{{this}}{{/inline}}{{> t31}}")
            .is_ok());
        assert!(handlebars
            .register_template_string(
                "t4",
                "{{#> t5}}{{#*inline \"nav\"}}navbar{{/inline}}{{/t5}}"
            )
            .is_ok());
        assert!(handlebars
            .register_template_string("t5", "include {{> nav}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t6", "{{> t1 a}}")
            .is_ok());
        assert!(handlebars
            .register_template_string(
                "t7",
                "{{#*inline \"t71\"}}{{a}}{{/inline}}{{> t71 a=\"world\"}}"
            )
            .is_ok());
        assert!(handlebars.register_template_string("t8", "{{a}}").is_ok());
        assert!(handlebars
            .register_template_string("t9", "{{> t8 a=2}}")
            .is_ok());

        assert_eq!(handlebars.render("t0", &1).ok().unwrap(), "1".to_string());
        assert_eq!(
            handlebars.render("t2", &1).ok().unwrap(),
            "not there".to_string()
        );
        assert_eq!(handlebars.render("t3", &1).ok().unwrap(), "1".to_string());
        assert_eq!(
            handlebars.render("t4", &1).ok().unwrap(),
            "include navbar".to_string()
        );
        assert_eq!(
            handlebars.render("t6", &json!({"a": "2"})).ok().unwrap(),
            "2".to_string()
        );
        assert_eq!(
            handlebars.render("t7", &1).ok().unwrap(),
            "world".to_string()
        );
        assert_eq!(handlebars.render("t9", &1).ok().unwrap(), "2".to_string());
    }

    #[test]
    fn test_include_partial_block() {
        let t0 = "hello {{> @partial-block}}";
        let t1 = "{{#> t0}}inner {{this}}{{/t0}}";

        let mut handlebars = Registry::new();
        assert!(handlebars.register_template_string("t0", t0).is_ok());
        assert!(handlebars.register_template_string("t1", t1).is_ok());

        let r0 = handlebars.render("t1", &true);
        assert_eq!(r0.ok().unwrap(), "hello inner true".to_string());
    }

    #[test]
    fn test_self_inclusion() {
        let t0 = "hello {{> t1}} {{> t0}}";
        let t1 = "some template";
        let mut handlebars = Registry::new();
        assert!(handlebars.register_template_string("t0", t0).is_ok());
        assert!(handlebars.register_template_string("t1", t1).is_ok());

        let r0 = handlebars.render("t0", &true);
        assert!(r0.is_err());
    }

    #[test]
    fn test_issue_143() {
        let main_template = "one{{> two }}three{{> two }}";
        let two_partial = "--- two ---";

        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("template", main_template)
            .is_ok());
        assert!(handlebars
            .register_template_string("two", two_partial)
            .is_ok());

        let r0 = handlebars.render("template", &true);
        assert_eq!(r0.ok().unwrap(), "one--- two ---three--- two ---");
    }

    #[test]
    fn test_hash_context_outscope() {
        let main_template = "In: {{> p a=2}} Out: {{a}}";
        let p_partial = "{{a}}";

        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("template", main_template)
            .is_ok());
        assert!(handlebars.register_template_string("p", p_partial).is_ok());

        let r0 = handlebars.render("template", &true);
        assert_eq!(r0.ok().unwrap(), "In: 2 Out: ");
    }

    #[test]
    fn test_partial_context_hash() {
        let mut hbs = Registry::new();
        hbs.register_template_string("one", "This is a test. {{> two name=\"fred\" }}")
            .unwrap();
        hbs.register_template_string("two", "Lets test {{name}}")
            .unwrap();
        assert_eq!(
            "This is a test. Lets test fred",
            hbs.render("one", &0).unwrap()
        );
    }

    #[test]
    fn teset_partial_context_with_both_hash_and_param() {
        let mut hbs = Registry::new();
        hbs.register_template_string("one", "This is a test. {{> two this name=\"fred\" }}")
            .unwrap();
        hbs.register_template_string("two", "Lets test {{name}} and {{root_name}}")
            .unwrap();
        assert_eq!(
            "This is a test. Lets test fred and tom",
            hbs.render("one", &json!({"root_name": "tom"})).unwrap()
        );
    }

    #[test]
    fn test_partial_subexpression_context_hash() {
        let mut hbs = Registry::new();
        hbs.register_template_string("one", "This is a test. {{> (x @root) name=\"fred\" }}")
            .unwrap();
        hbs.register_template_string("two", "Lets test {{name}}")
            .unwrap();

        hbs.register_helper(
            "x",
            Box::new(
                |_: &Helper<'_, '_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    out.write("two")?;
                    Ok(())
                },
            ),
        );
        assert_eq!(
            "This is a test. Lets test fred",
            hbs.render("one", &0).unwrap()
        );
    }

    #[test]
    fn test_nested_partial_scope() {
        let t = "{{#*inline \"pp\"}}{{a}} {{b}}{{/inline}}{{#each c}}{{> pp a=2}}{{/each}}";
        let data = json!({"c": [{"b": true}, {"b": false}]});

        let mut handlebars = Registry::new();
        assert!(handlebars.register_template_string("t", t).is_ok());
        let r0 = handlebars.render("t", &data);
        assert_eq!(r0.ok().unwrap(), "2 true2 false");
    }

    #[test]
    fn test_nested_partial_block() {
        let mut handlebars = Registry::new();
        let template1 = "<outer>{{> @partial-block }}</outer>";
        let template2 = "{{#> t1 }}<inner>{{> @partial-block }}</inner>{{/ t1 }}";
        let template3 = "{{#> t2 }}Hello{{/ t2 }}";

        handlebars
            .register_template_string("t1", &template1)
            .unwrap();
        handlebars
            .register_template_string("t2", &template2)
            .unwrap();

        let page = handlebars.render_template(&template3, &json!({})).unwrap();
        assert_eq!("<outer><inner>Hello</inner></outer>", page);
    }

    #[test]
    fn test_up_to_partial_level() {
        let outer = r#"{{>inner name="fruit:" vegetables=fruits}}"#;
        let inner = "{{#each vegetables}}{{../name}} {{this}},{{/each}}";

        let data = json!({ "fruits": ["carrot", "tomato"] });

        let mut handlebars = Registry::new();
        handlebars.register_template_string("outer", outer).unwrap();
        handlebars.register_template_string("inner", inner).unwrap();

        assert_eq!(
            handlebars.render("outer", &data).unwrap(),
            "fruit: carrot,fruit: tomato,"
        );
    }

    #[test]
    fn line_stripping_with_inline_and_partial() {
        let tpl0 = r#"{{#*inline "foo"}}foo
{{/inline}}
{{> foo}}
{{> foo}}
{{> foo}}"#;
        let tpl1 = r#"{{#*inline "foo"}}foo{{/inline}}
{{> foo}}
{{> foo}}
{{> foo}}"#;

        let hbs = Registry::new();
        assert_eq!(
            r#"foo
foo
foo
"#,
            hbs.render_template(tpl0, &json!({})).unwrap()
        );
        assert_eq!(
            r#"
foofoofoo"#,
            hbs.render_template(tpl1, &json!({})).unwrap()
        );
    }

    #[test]
    fn test_partial_indent() {
        let outer = r#"                {{> inner inner_solo}}

{{#each inners}}
                {{> inner}}
{{/each}}

        {{#each inners}}
        {{> inner}}
        {{/each}}
"#;
        let inner = r#"name: {{name}}
"#;

        let mut hbs = Registry::new();

        hbs.register_template_string("inner", inner).unwrap();
        hbs.register_template_string("outer", outer).unwrap();

        let result = hbs
            .render(
                "outer",
                &json!({
                    "inner_solo": {"name": "inner_solo"},
                    "inners": [
                        {"name": "hello"},
                        {"name": "there"}
                    ]
                }),
            )
            .unwrap();

        assert_eq!(
            result,
            r#"                name: inner_solo

                name: hello
                name: there

        name: hello
        name: there
"#
        );
    }
    // Rule::partial_expression should not trim leading indent  by default

    #[test]
    fn test_partial_prevent_indent() {
        let outer = r#"                {{> inner inner_solo}}

{{#each inners}}
                {{> inner}}
{{/each}}

        {{#each inners}}
        {{> inner}}
        {{/each}}
"#;
        let inner = r#"name: {{name}}
"#;

        let mut hbs = Registry::new();
        hbs.set_prevent_indent(true);

        hbs.register_template_string("inner", inner).unwrap();
        hbs.register_template_string("outer", outer).unwrap();

        let result = hbs
            .render(
                "outer",
                &json!({
                    "inner_solo": {"name": "inner_solo"},
                    "inners": [
                        {"name": "hello"},
                        {"name": "there"}
                    ]
                }),
            )
            .unwrap();

        assert_eq!(
            result,
            r#"                name: inner_solo

                name: hello
                name: there

        name: hello
        name: there
"#
        );
    }

    #[test]
    fn test_nested_partials() {
        let mut hb = Registry::new();
        hb.register_template_string("partial", "{{> @partial-block}}")
            .unwrap();
        hb.register_template_string(
            "index",
            r#"{{#>partial}}
    Yo
    {{#>partial}}
    Yo 2
    {{/partial}}
{{/partial}}"#,
        )
        .unwrap();
        assert_eq!(
            r#"    Yo
    Yo 2
"#,
            hb.render("index", &()).unwrap()
        );

        hb.register_template_string("partial2", "{{> @partial-block}}")
            .unwrap();
        let r2 = hb
            .render_template(
                r#"{{#> partial}}
{{#> partial2}}
:(
{{/partial2}}
{{/partial}}"#,
                &(),
            )
            .unwrap();
        assert_eq!(":(\n", r2);
    }

    #[test]
    fn test_partial_context_issue_495() {
        let mut hb = Registry::new();
        hb.register_template_string(
            "t1",
            r#"{{~#*inline "displayName"~}}
Template:{{name}}
{{/inline}}
{{#each data as |name|}}
Name:{{name}}
{{>displayName name="aaaa"}}
{{/each}}"#,
        )
        .unwrap();

        hb.register_template_string(
            "t1",
            r#"{{~#*inline "displayName"~}}
Template:{{this}}
{{/inline}}
{{#each data as |name|}}
Name:{{name}}
{{>displayName}}
{{/each}}"#,
        )
        .unwrap();

        let data = json!({
            "data": ["hudel", "test"]
        });

        assert_eq!(
            r#"Name:hudel
Template:hudel
Name:test
Template:test
"#,
            hb.render("t1", &data).unwrap()
        );
    }

    #[test]
    fn test_multiline_partial_indent() {
        let mut hb = Registry::new();

        hb.register_template_string(
            "t1",
            r#"{{#*inline "thepartial"}}
  inner first line
  inner second line
{{/inline}}
  {{> thepartial}}
outer third line"#,
        )
        .unwrap();
        assert_eq!(
            r#"    inner first line
    inner second line
outer third line"#,
            hb.render("t1", &()).unwrap()
        );

        hb.register_template_string(
            "t2",
            r#"{{#*inline "thepartial"}}inner first line
inner second line
{{/inline}}
  {{> thepartial}}
outer third line"#,
        )
        .unwrap();
        assert_eq!(
            r#"  inner first line
  inner second line
outer third line"#,
            hb.render("t2", &()).unwrap()
        );

        hb.register_template_string(
            "t3",
            r#"{{#*inline "thepartial"}}{{a}}{{/inline}}
  {{> thepartial}}
outer third line"#,
        )
        .unwrap();
        assert_eq!(
            r#"
  inner first line
  inner second lineouter third line"#,
            hb.render("t3", &json!({"a": "inner first line\ninner second line"}))
                .unwrap()
        );

        hb.register_template_string(
            "t4",
            r#"{{#*inline "thepartial"}}
  inner first line
  inner second line
{{/inline}}
  {{~> thepartial}}
outer third line"#,
        )
        .unwrap();
        assert_eq!(
            r#"  inner first line
  inner second line
outer third line"#,
            hb.render("t4", &()).unwrap()
        );

        let mut hb2 = Registry::new();
        hb2.set_prevent_indent(true);

        hb2.register_template_string(
            "t1",
            r#"{{#*inline "thepartial"}}
  inner first line
  inner second line
{{/inline}}
  {{> thepartial}}
outer third line"#,
        )
        .unwrap();
        assert_eq!(
            r#"    inner first line
  inner second line
outer third line"#,
            hb2.render("t1", &()).unwrap()
        )
    }
}

#[test]
fn test_issue_534() {
    let t1 = "{{title}}";
    let t2 = "{{#each modules}}{{> (lookup this \"module\") content name=0}}{{/each}}";

    let data = json!({
      "modules": [
        {"module": "t1", "content": {"title": "foo"}},
        {"module": "t1", "content": {"title": "bar"}},
      ]
    });

    let mut hbs = Registry::new();
    hbs.register_template_string("t1", t1).unwrap();
    hbs.register_template_string("t2", t2).unwrap();

    assert_eq!("foobar", hbs.render("t2", &data).unwrap());
}
