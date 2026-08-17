use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("ID {} denied", id))]
    OnlyPositionalArguments { id: i32 },

    #[snafu(display("Person {name} with ID {id} denied"))]
    OnlyShorthandArguments { id: i32, name: &'static str },

    #[snafu(display("Person {} with ID {id} denied", name))]
    PositionalAndShorthandArguments { id: i32, name: &'static str },

    #[snafu(display("Person {name:?} with ID {id:X} denied"))]
    ShorthandArgumentsWithSpecs { id: i32, name: &'static str },

    #[snafu(display("ID {id} denied"))]
    FieldNotUsedAsNamedArgument { id: i32, name: &'static str },

    #[snafu(display("ID {id} denied at {time}", time = 1 + 1))]
    AdditionalNamedArguments { id: i32 },

    #[snafu(display("Person {name} with ID {id} denied", id = 99))]
    RedefinedNamedArguments { id: i32, name: &'static str },

    /// Person {name} with ID {id} denied
    ShorthandArgumentsInDocComments { id: i32, name: &'static str },
}

#[test]
fn supports_positional_formatting() {
    let error = OnlyPositionalArgumentsSnafu { id: 42 }.build();
    assert_eq!(error.to_string(), "ID 42 denied");
}

#[test]
fn supports_shorthand_formatting() {
    let error = OnlyShorthandArgumentsSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), "Person Anna with ID 42 denied");
}

#[test]
fn supports_positional_and_shorthand_formatting() {
    let error = PositionalAndShorthandArgumentsSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), "Person Anna with ID 42 denied");
}

#[test]
fn supports_format_specs() {
    let error = ShorthandArgumentsWithSpecsSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), r#"Person "Anna" with ID 2A denied"#);
}

#[test]
fn ignores_unused_fields() {
    let error = FieldNotUsedAsNamedArgumentSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), "ID 42 denied");
}

#[test]
fn allows_additional_named_arguments() {
    let error = AdditionalNamedArgumentsSnafu { id: 42 }.build();
    assert_eq!(error.to_string(), "ID 42 denied at 2");
}

#[test]
fn does_not_redefine_user_provided_named_arguments() {
    let error = RedefinedNamedArgumentsSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), "Person Anna with ID 99 denied");
}

#[test]
fn allows_shorthand_in_doc_comments() {
    let error = ShorthandArgumentsInDocCommentsSnafu {
        id: 42,
        name: "Anna",
    }
    .build();
    assert_eq!(error.to_string(), "Person Anna with ID 42 denied");
}
