use handlebars::*;
use serde_json::json;

struct HelperWithBorrowedData<'a>(&'a String);

impl<'a> HelperDef for HelperWithBorrowedData<'a> {
    fn call<'_reg: '_rc, '_rc>(
        &self,
        _: &Helper<'_reg, '_rc>,
        _: &'_reg Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        out.write(self.0).map_err(RenderError::from)
    }
}

#[test]
fn test_helper_with_ref_data() {
    let s = "hello helper".to_owned();
    let the_helper = HelperWithBorrowedData(&s);

    let mut r = Handlebars::new();
    r.register_helper("hello", Box::new(the_helper));

    let s = r.render_template("Output: {{hello}}", &json!({})).unwrap();
    assert_eq!(s, "Output: hello helper".to_owned());
}
