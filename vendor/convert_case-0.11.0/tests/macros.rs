use convert_case::ccase;
use convert_case::{separator, split};

#[test]
fn ccase_to_case() {
    assert_eq!(ccase!(snake, "my_Var_Name"), "my_var_name");
    assert_eq!(ccase!(constant, "my_Var_Name"), "MY_VAR_NAME");
    assert_eq!(ccase!(kebab, "my_Var_Name"), "my-var-name");
    assert_eq!(ccase!(kebab, String::from("my_Var_Name")), "my-var-name");
}

#[test]
fn ccase_from_to_case() {
    assert_eq!(ccase!(kebab -> camel, "myVar-name_var"), "myvarName_var");
    assert_eq!(ccase!(snake -> pascal, "my-var_name-var"), "My-varName-var");
}

#[test]
fn separator_custom_delimiters() {
    let dot = separator!(".");
    assert_eq!(
        split(&"lower.Upper.Upper", &[dot]),
        vec!["lower", "Upper", "Upper"]
    );

    let double_colon = separator!("::");
    assert_eq!(
        split(&"lower::lowerUpper::Upper", &[double_colon]),
        vec!["lower", "lowerUpper", "Upper"]
    );
}
