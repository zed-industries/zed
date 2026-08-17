// Integration tests from https://github.com/reklatsmasters/saslprep (MIT License)
extern crate stringprep;

use stringprep::{saslprep, Error};

fn assert_prohibited_character<T>(result: Result<T, Error>) {
    assert!(result.is_err());
}

fn assert_prohibited_bidirectional_text<T>(result: Result<T, Error>) {
    assert!(result.is_err());
}

#[test]
fn should_work_with_latin_letters() {
    assert_eq!(saslprep("user").unwrap(), "user");
}

#[test]
fn should_preserve_case() {
    assert_eq!(saslprep("USER").unwrap(), "USER");
}

#[test]
fn should_remove_mapped_to_nothing() {
    assert_eq!(saslprep("I\u{00AD}X").unwrap(), "IX");
}

#[test]
fn should_replace_non_ascii_space() {
    assert_eq!(saslprep("a\u{00A0}b").unwrap(), "a\u{0020}b");
}

#[test]
fn should_normalize_as_nfkc() {
    assert_eq!(saslprep("\u{00AA}").unwrap(), "a");
    assert_eq!(saslprep("\u{2168}").unwrap(), "IX");
}

#[test]
fn should_not_allow_prohibited_characters() {
    // C.2.1 ASCII control characters
    assert_prohibited_character(saslprep("a\u{007F}b"));

    // C.2.2 Non-ASCII control characters
    assert_prohibited_character(saslprep("a\u{06DD}b"));

    // C.3 Private use
    assert_prohibited_character(saslprep("a\u{E000}b"));

    // C.4 Non-character code points
    assert_prohibited_character(saslprep("a\u{1FFFE}b"));

    // C.5 Surrogate codes
    // forbidden by rust

    // C.6 Inappropriate for plain text
    assert_prohibited_character(saslprep("a\u{FFF9}b"));

    // C.7 Inappropriate for canonical representation
    assert_prohibited_character(saslprep("a\u{2FF0}b"));

    // C.8 Change display properties or are deprecated
    assert_prohibited_character(saslprep("a\u{200E}b"));

    // C.9 Tagging characters
    assert_prohibited_character(saslprep("a\u{E0001}b"));
}

#[test]
fn randalcat_should_be_first_and_last() {
    assert_eq!(
        saslprep("\u{0627}\u{0031}\u{0628}").unwrap(),
        "\u{0627}\u{0031}\u{0628}"
    );
    assert_prohibited_bidirectional_text(saslprep("\u{0627}\u{0031}"));
}

#[test]
fn should_handle_unassigned_code_points() {
    assert_prohibited_character(saslprep("a\u{0487}"));
}
