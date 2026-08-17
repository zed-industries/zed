#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

#[test]
#[cfg(feature = "rust-embed")]
fn test_embed() {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "tests/templates/"]
    #[include = "*.hbs"]
    struct Templates;

    let mut hbs = Handlebars::new();
    hbs.register_embed_templates::<Templates>().unwrap();

    assert_eq!(1, hbs.get_templates().len());

    let data = json!({
        "name": "Andy"
    });

    assert_eq!(
        hbs.render("hello.hbs", &data).unwrap().trim(),
        "Hello, Andy"
    );
}
