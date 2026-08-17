use handlebars::Handlebars;
use serde_json::json;

#[test]
fn test_whitespaces_elision() {
    let hbs = Handlebars::new();
    assert_eq!(
        "bar",
        hbs.render_template("  {{~ foo ~}}  ", &json!({"foo": "bar"}))
            .unwrap()
    );

    assert_eq!(
        "<bar/>",
        hbs.render_template("  {{{~ foo ~}}}  ", &json!({"foo": "<bar/>"}))
            .unwrap()
    );

    assert_eq!(
        "<bar/>",
        hbs.render_template("  {{~ {foo} ~}}  ", &json!({"foo": "<bar/>"}))
            .unwrap()
    );
}
