use handlebars::*;

fn ifcond<'reg, 'rc>(
    h: &Helper<'rc>,
    handle: &'reg Handlebars,
    ctx: &'rc Context,
    render_ctx: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let cond = h.param(0).and_then(|ref v| v.value().as_bool()).ok_or(
        RenderErrorReason::ParamTypeMismatchForName("ifcond", "0".to_owned(), "bool".to_owned()),
    )? as bool;
    let temp = if cond { h.template() } else { h.inverse() };
    match temp {
        Some(t) => t.render(handle, ctx, render_ctx, out),
        None => Ok(()),
    }
}

#[test]
fn test_helper() {
    let mut handlebars = Handlebars::new();

    // register some custom helpers
    handlebars.register_helper("ifcond", Box::new(ifcond));

    // make data and render it
    let data = true;
    assert_eq!(
        "yes",
        handlebars
            .render_template("{{#ifcond this}}yes{{/ifcond}}", &data)
            .unwrap()
    );
}
