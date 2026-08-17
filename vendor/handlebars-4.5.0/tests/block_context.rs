use handlebars::Handlebars;
use serde_json::json;

#[test]
fn test_partial_with_blocks() {
    let hbs = Handlebars::new();

    let data = json!({
        "a": [
            {"b": 1},
            {"b": 2},
        ],
    });

    let template = "{{#*inline \"test\"}}{{b}};{{/inline}}{{#each a as |z|}}{{> test z}}{{/each}}";
    assert_eq!(hbs.render_template(template, &data).unwrap(), "1;2;");
}

#[test]
fn test_root_with_blocks() {
    let hbs = Handlebars::new();

    let data = json!({
        "a": [
            {"b": 1},
            {"b": 2},
        ],
        "b": 3,
    });

    let template =
        "{{#*inline \"test\"}}{{b}}:{{@root.b}};{{/inline}}{{#each a}}{{> test}}{{/each}}";
    assert_eq!(hbs.render_template(template, &data).unwrap(), "1:3;2:3;");
}

#[test]
fn test_singular_and_pair_block_params() {
    let hbs = Handlebars::new();

    let data = json!([
        {"value": 11},
        {"value": 22},
    ]);

    let template =
        "{{#each this as |b index|}}{{b.value}}{{#each this as |value key|}}:{{key}},{{/each}}{{/each}}";
    assert_eq!(
        hbs.render_template(template, &data).unwrap(),
        "11:value,22:value,"
    );
}

#[test]
fn test_nested_each() {
    let hbs = Handlebars::new();

    let data = json!({
        "classes": [
            {
                "methods": [
                    {"id": 1},
                    {"id": 2}
                ]
            },
            {
                "methods": [
                    {"id": 3},
                    {"id": 4}
                ]
            },
        ],
    });

    let template = "{{#each classes as |class|}}{{#each class.methods as |method|}}{{method.id}};{{/each}}{{/each}}";
    assert_eq!(hbs.render_template(template, &data).unwrap(), "1;2;3;4;");
}

#[test]
fn test_referencing_block_param_from_upper_scope() {
    let hbs = Handlebars::new();

    let data = json!({
        "classes": [
            {
                "methods": [
                    {"id": 1},
                    {"id": 2}
                ],
                "private": false
            },
            {
                "methods": [
                    {"id": 3},
                    {"id": 4}
                ],
                "private": true
            },
        ],
    });

    let template = "{{#each classes as |class|}}{{#each class.methods as |method|}}{{class.private}}|{{method.id}};{{/each}}{{/each}}";
    assert_eq!(
        hbs.render_template(template, &data).unwrap(),
        "false|1;false|2;true|3;true|4;"
    );
}
