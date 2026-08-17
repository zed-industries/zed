use error_graph::{ErrorList, WriteErrorList};

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum UpperError {
    Upper,
    Middle(ErrorList<MiddleError>),
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum MiddleError {
    Middle,
    Lower(ErrorList<LowerError>),
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum LowerError {
    Lower,
}

fn upper(mut errors: impl WriteErrorList<UpperError>) {
    errors.push(UpperError::Upper);
    middle(errors.subwriter(UpperError::Middle));
    errors.push(UpperError::Upper);
}

fn middle(mut errors: impl WriteErrorList<MiddleError>) {
    errors.push(MiddleError::Middle);
    lower(errors.subwriter(MiddleError::Lower));
    errors.push(MiddleError::Middle);
}

fn lower(mut errors: impl WriteErrorList<LowerError>) {
    errors.push(LowerError::Lower);
}

fn lower_errors_only(mut errors: impl WriteErrorList<MiddleError>) {
    lower(errors.subwriter(MiddleError::Lower));
}

fn no_errors(mut errors: impl WriteErrorList<UpperError>) {
    no_errors_middle(errors.subwriter(UpperError::Middle));
}

fn no_errors_middle(mut errors: impl WriteErrorList<MiddleError>) {
    no_errors_lower(errors.subwriter(MiddleError::Lower));
}

fn no_errors_lower(mut _errors: impl WriteErrorList<LowerError>) {}

#[test]
fn empty() {
    let mut errors = ErrorList::default();
    errors.push(UpperError::Upper);
    no_errors_middle(errors.subwriter(UpperError::Middle));
    assert_eq!(errors.len(), 1);
}

#[test]
fn basic() {
    let mut errors = ErrorList::default();
    upper(&mut errors);

    let mut upper_it = errors.into_iter();
    assert!(matches!(upper_it.next(), Some(UpperError::Upper)));
    let Some(UpperError::Middle(middle)) = upper_it.next() else {
        panic!();
    };
    assert!(matches!(upper_it.next(), Some(UpperError::Upper)));

    let mut middle_it = middle.into_iter();
    assert!(matches!(middle_it.next(), Some(MiddleError::Middle)));
    let Some(MiddleError::Lower(lower)) = middle_it.next() else {
        panic!();
    };
    assert!(matches!(middle_it.next(), Some(MiddleError::Middle)));

    let mut lower_it = lower.into_iter();
    assert!(matches!(lower_it.next(), Some(LowerError::Lower)));
}

#[test]
fn sublist() {
    let mut errors = ErrorList::default();
    errors.push(UpperError::Upper);
    let mut sublist = errors.sublist(UpperError::Middle);
    middle(&mut sublist);
    assert_eq!(sublist.len(), 3);
    drop(sublist);
    assert_eq!(errors.len(), 2);
}

#[test]
fn dont_care() {
    let mut errors = error_graph::strategy::DontCare;
    errors.push(UpperError::Upper);
    let mut sublist = errors.sublist(UpperError::Middle);
    middle(&mut sublist);
    assert_eq!(sublist.len(), 3);
    sublist.finish();
    middle(errors.subwriter(UpperError::Middle));
}

#[test]
fn error_occurred() {
    let mut error_occurred = error_graph::strategy::ErrorOccurred::default();
    no_errors(&mut error_occurred);
    assert!(!error_occurred.as_bool());
    lower_errors_only(&mut error_occurred);
    assert!(error_occurred.as_bool());
    let mut error_occurred = error_graph::strategy::ErrorOccurred::default();
    middle(&mut error_occurred);
    assert!(error_occurred.as_bool());
}

#[cfg(feature = "serde")]
#[test]
fn serde() {
    const EXPECTED_JSON: &str = r#"[
  "Upper",
  {
    "Middle": [
      "Middle",
      {
        "Lower": [
          "Lower"
        ]
      },
      "Middle"
    ]
  },
  "Upper"
]"#;

    let mut errors = ErrorList::default();
    upper(&mut errors);
    let json = serde_json::to_string_pretty(&errors).unwrap();
    assert_eq!(json.as_str(), EXPECTED_JSON);

    let recreated_errors: ErrorList<UpperError> = serde_json::from_str(&json).unwrap();

    for (left, right) in errors.iter().zip(recreated_errors.iter()) {
        assert_eq!(left, right);
    }
}
