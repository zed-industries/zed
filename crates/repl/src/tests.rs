zed/crates/repl/src/repl_settings_test.rs
use super::ReplSettings;

#[test]
fn test_default_settings_are_valid() {
    let settings = ReplSettings::default();
    assert!(settings.validate().is_ok(), "Default settings should be valid");
}

#[test]
fn test_invalid_max_number_of_lines_zero() {
    let settings = ReplSettings {
        max_number_of_lines: 0,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_lines is 0"
    );
}

#[test]
fn test_invalid_max_number_of_lines_too_large() {
    let settings = ReplSettings {
        max_number_of_lines: 257,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_lines exceeds 256"
    );
}

#[test]
fn test_invalid_max_number_of_columns_zero() {
    let settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 0,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_columns is 0"
    );
}

#[test]
fn test_invalid_max_number_of_columns_too_large() {
    let settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 513,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_columns exceeds 512"
    );
}

#[test]
fn test_minimum_usable_lines() {
    let settings = ReplSettings {
        max_number_of_lines: 3,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_lines is less than 4"
    );
}

#[test]
fn test_minimum_usable_columns() {
    let settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 19,
    };
    assert!(
        settings.validate().is_err(),
        "Validation should fail when max_number_of_columns is less than 20"
    );
}

#[test]
fn test_valid_custom_settings() {
    let settings = ReplSettings {
        max_number_of_lines: 100,
        max_number_of_columns: 200,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should pass for valid custom settings"
    );
}
