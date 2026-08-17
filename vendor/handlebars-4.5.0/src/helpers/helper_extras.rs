//! Helpers for boolean operations
use serde_json::Value as Json;

use crate::json::value::JsonTruthy;

handlebars_helper!(eq: |x: Json, y: Json| x == y);
handlebars_helper!(ne: |x: Json, y: Json| x != y);
handlebars_helper!(gt: |x: i64, y: i64| x > y);
handlebars_helper!(gte: |x: i64, y: i64| x >= y);
handlebars_helper!(lt: |x: i64, y: i64| x < y);
handlebars_helper!(lte: |x: i64, y: i64| x <= y);
handlebars_helper!(and: |x: Json, y: Json| x.is_truthy(false) && y.is_truthy(false));
handlebars_helper!(or: |x: Json, y: Json| x.is_truthy(false) || y.is_truthy(false));
handlebars_helper!(not: |x: Json| !x.is_truthy(false));
handlebars_helper!(len: |x: Json| {
    match x {
        Json::Array(a) => a.len(),
        Json::Object(m) => m.len(),
        Json::String(s) => s.len(),
        _ => 0
    }
});

#[cfg(test)]
mod test_conditions {
    fn test_condition(condition: &str, expected: bool) {
        let handlebars = crate::Handlebars::new();

        let result = handlebars
            .render_template(
                &format!(
                    "{{{{#if {condition}}}}}lorem{{{{else}}}}ipsum{{{{/if}}}}",
                    condition = condition
                ),
                &json!({}),
            )
            .unwrap();
        assert_eq!(&result, if expected { "lorem" } else { "ipsum" });
    }

    #[test]
    fn foo() {
        test_condition("(gt 5 3)", true);
        test_condition("(gt 3 5)", false);
        test_condition("(or (gt 3 5) (gt 5 3))", true);
        test_condition("(not [])", true);
        test_condition("(and null 4)", false);
    }

    #[test]
    fn test_eq() {
        test_condition("(eq 5 5)", true);
        test_condition("(eq 5 6)", false);
        test_condition(r#"(eq "foo" "foo")"#, true);
        test_condition(r#"(eq "foo" "Foo")"#, false);
        test_condition(r#"(eq [5] [5])"#, true);
        test_condition(r#"(eq [5] [4])"#, false);
        test_condition(r#"(eq 5 "5")"#, false);
        test_condition(r#"(eq 5 [5])"#, false);
    }

    #[test]
    fn test_ne() {
        test_condition("(ne 5 6)", true);
        test_condition("(ne 5 5)", false);
        test_condition(r#"(ne "foo" "foo")"#, false);
        test_condition(r#"(ne "foo" "Foo")"#, true);
    }

    #[test]
    fn nested_conditions() {
        let handlebars = crate::Handlebars::new();

        let result = handlebars
            .render_template("{{#if (gt 5 3)}}lorem{{else}}ipsum{{/if}}", &json!({}))
            .unwrap();
        assert_eq!(&result, "lorem");

        let result = handlebars
            .render_template(
                "{{#if (not (gt 5 3))}}lorem{{else}}ipsum{{/if}}",
                &json!({}),
            )
            .unwrap();
        assert_eq!(&result, "ipsum");
    }

    #[test]
    fn test_len() {
        let handlebars = crate::Handlebars::new();

        let result = handlebars
            .render_template("{{len value}}", &json!({"value": [1,2,3]}))
            .unwrap();
        assert_eq!(&result, "3");

        let result = handlebars
            .render_template("{{len value}}", &json!({"value": {"a" :1, "b": 2}}))
            .unwrap();
        assert_eq!(&result, "2");

        let result = handlebars
            .render_template("{{len value}}", &json!({"value": "tomcat"}))
            .unwrap();
        assert_eq!(&result, "6");

        let result = handlebars
            .render_template("{{len value}}", &json!({"value": 3}))
            .unwrap();
        assert_eq!(&result, "0");
    }
}
