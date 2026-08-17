//! A Set of string utilities often required during code generation.
//!
//! See also: [`heck`](https://docs.rs/heck/latest/heck)
//!
//! `lowerCamelCase`: Convert a string to lowerCamelCase
//! `upperCamelCase`: Convert a string to UpperCamelCase
//! `snakeCase`: Convert a string to snake_case
//! `kebabCase`: Conver a string to kebab-case
//! `shoutySnakeCase`: Convert a string to SHOUTY_SNAKE_CASE
//! `shoutyKebabCase`: Convert a string to SHOUTY-KEBAB-CASE
//! `titleCase`: Convert a string to Title Case
//! `trainCase`: Convert a string to Train-Case

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};

macro_rules! define_case_helper {
    ($helper_fn_name: ident, $heck_fn_name:ident) => {
        pub(crate) fn $helper_fn_name(
            h: &crate::render::Helper<'_, '_>,
            _: &crate::Handlebars<'_>,
            _: &crate::context::Context,
            _rc: &mut crate::render::RenderContext<'_, '_>,
            out: &mut dyn crate::output::Output,
        ) -> crate::helpers::HelperResult {
            let param = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
                crate::error::RenderError::new(format!("{} required string parameter", h.name()))
            })?;
            out.write(param.$heck_fn_name().as_ref())?;
            Ok(())
        }
    };
}

define_case_helper!(lower_camel_case, to_lower_camel_case);
define_case_helper!(upper_camel_case, to_upper_camel_case);
define_case_helper!(snake_case, to_snake_case);
define_case_helper!(shouty_snake_case, to_shouty_snake_case);
define_case_helper!(kebab_case, to_kebab_case);
define_case_helper!(shouty_kebab_case, to_shouty_kebab_case);
define_case_helper!(title_case, to_title_case);
define_case_helper!(train_case, to_train_case);

#[cfg(test)]
mod tests {
    macro_rules! define_case_helpers_test_cases {
    ($template_fn_name:literal, $helper_tc_fn_name:ident, $(($tc_input:literal, $tc_expected:literal),)+) => {

        #[test]
        fn $helper_tc_fn_name() {
            let hbs = crate::registry::Registry::new();
            let test_cases = vec![$(($tc_input, $tc_expected)),+];
            for tc in test_cases {
                let result =
                    hbs.render_template(
                        concat!("{{", $template_fn_name, " data}}"),
                        &json!({"data": tc.0}));
                assert!(result.is_ok(), "{}", result.err().unwrap());
                assert_eq!(result.unwrap(), tc.1.to_string());
            }
        }
    }
}

    define_case_helpers_test_cases!(
        "lowerCamelCase",
        test_lower_camel_case,
        ("lower camel case", "lowerCamelCase"),
        ("lower-camel-case", "lowerCamelCase"),
        ("lower_camel_case", "lowerCamelCase"),
    );

    define_case_helpers_test_cases!(
        "upperCamelCase",
        test_upper_camel_case,
        ("upper camel case", "UpperCamelCase"),
        ("upper-camel-case", "UpperCamelCase"),
        ("upper_camel_case", "UpperCamelCase"),
    );

    define_case_helpers_test_cases!(
        "snakeCase",
        test_snake_case,
        ("snake case", "snake_case"),
        ("snake-case", "snake_case"),
    );

    define_case_helpers_test_cases!(
        "kebabCase",
        test_kebab_case,
        ("kebab case", "kebab-case"),
        ("kebab_case", "kebab-case"),
    );

    define_case_helpers_test_cases!(
        "shoutySnakeCase",
        test_shouty_snake_case,
        ("shouty snake case", "SHOUTY_SNAKE_CASE"),
        ("shouty snake-case", "SHOUTY_SNAKE_CASE"),
    );

    define_case_helpers_test_cases!(
        "shoutyKebabCase",
        test_shouty_kebab_case,
        ("shouty kebab case", "SHOUTY-KEBAB-CASE"),
        ("shouty_kebab_case", "SHOUTY-KEBAB-CASE"),
    );

    define_case_helpers_test_cases!("titleCase", test_title_case, ("title case", "Title Case"),);

    define_case_helpers_test_cases!("trainCase", test_train_case, ("train case", "Train-Case"),);

    #[test]
    fn test_invlaid_input() {
        let hbs = crate::registry::Registry::new();
        assert!(hbs.render_template("{{snakeCase 1}}", &json!({})).is_err());
    }
}
