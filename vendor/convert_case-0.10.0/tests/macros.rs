use convert_case::ccase;
use convert_case::{delim_boundary, split};

#[test]
fn ccase_snake() {
    assert_eq!("my_var_name", ccase!(snake, "my_Var_Name"));
}

#[test]
fn ccase_constant() {
    assert_eq!("MY_VAR_NAME", ccase!(constant, "my_Var_Name"));
}

#[test]
fn ccase_kebab() {
    assert_eq!("my-var-name", ccase!(kebab, "my_Var_Name"));
}

#[test]
fn ccase_kebab_string() {
    assert_eq!("my-var-name", ccase!(kebab, String::from("my_Var_Name")));
}

#[test]
fn ccase_from_kebab_to_camel() {
    assert_eq!("myvarName_var", ccase!(kebab -> camel, "myVar-name_var"));
}

#[test]
fn ccase_from_snake_to_pascal() {
    assert_eq!("My-varName-var", ccase!(snake -> pascal, "my-var_name-var"));
}

#[test]
fn delim_boundary_dot() {
    let boundary = delim_boundary!(".");
    let s = "lower.Upper.Upper";
    let v = split(&s, &[boundary]);
    assert_eq!(vec!["lower", "Upper", "Upper"], v)
}

#[test]
fn delim_boundary_double_colon() {
    let boundary = delim_boundary!("::");
    let s = "lower::lowerUpper::Upper";
    let v = split(&s, &[boundary]);
    assert_eq!(vec!["lower", "lowerUpper", "Upper"], v)
}
