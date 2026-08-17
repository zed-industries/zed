extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

#[test]
fn test_walk_dir_template_name() {
    let mut hbs = Handlebars::new();

    let data = json!({
        "a": [1, 2, 3, 4],
        "b": "top"
    });

    hbs.register_template_string("foo/bar", "{{@root/b}}")
        .unwrap();
    assert_eq!(hbs.render_template("{{> foo/bar }}", &data).unwrap(), "top");
}

#[test]
fn test_walk_dir_template_name_with_args() {
    let mut hbs = Handlebars::new();

    let data = json!({
        "a": [1, 2, 3, 4],
        "b": "top"
    });

    hbs.register_template_string("foo/bar", "{{this}}").unwrap();
    assert_eq!(
        hbs.render_template("{{> foo/bar b }}", &data).unwrap(),
        "top"
    );
}
