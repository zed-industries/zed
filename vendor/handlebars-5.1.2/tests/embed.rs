#[macro_use]
extern crate serde_json;

#[test]
#[cfg(feature = "rust-embed")]
fn test_embed() {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "tests/templates/"]
    #[include = "*.hbs"]
    struct Templates;

    let mut hbs = handlebars::Handlebars::new();
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

#[test]
#[cfg(feature = "rust-embed")]
fn test_embed_with_extension() {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "tests/templates/"]
    struct Templates;

    let mut hbs = handlebars::Handlebars::new();
    hbs.register_embed_templates_with_extension::<Templates>(".hbs")
        .unwrap();

    assert_eq!(1, hbs.get_templates().len());

    let data = json!({
        "name": "Andy"
    });

    assert_eq!(hbs.render("hello", &data).unwrap().trim(), "Hello, Andy");
}

#[test]
#[cfg(feature = "rust-embed")]
fn test_embed_with_extension_and_tests_struct_root() {
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "tests/"]
    struct Templates;

    let mut hbs = handlebars::Handlebars::new();
    hbs.register_embed_templates_with_extension::<Templates>(".hbs")
        .unwrap();

    assert_eq!(1, hbs.get_templates().len());

    let data = json!({
        "name": "Andy"
    });

    assert_eq!(
        hbs.render("templates/hello", &data).unwrap().trim(),
        "Hello, Andy"
    );
}
