use super::ReplSettings;

#[test]
fn test_default_settings_are_valid() {
    let mut settings = ReplSettings::default();
    assert!(
        settings.validate().is_ok(),
        "Default settings should be valid"
    );
}

#[test]
fn test_invalid_max_number_of_lines_zero() {
    let mut settings = ReplSettings {
        max_number_of_lines: 0,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_lines to minimum"
    );
    assert_eq!(
        settings.max_number_of_lines, 4,
        "max_number_of_lines should default to 4"
    );
}

#[test]
fn test_invalid_max_number_of_lines_too_large() {
    let mut settings = ReplSettings {
        max_number_of_lines: 257,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_lines to maximum"
    );
    assert_eq!(
        settings.max_number_of_lines, 256,
        "max_number_of_lines should default to 256"
    );
}

#[test]
fn test_invalid_max_number_of_columns_zero() {
    let mut settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 0,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_columns to minimum"
    );
    assert_eq!(
        settings.max_number_of_columns, 20,
        "max_number_of_columns should default to 20"
    );
}

#[test]
fn test_invalid_max_number_of_columns_too_large() {
    let mut settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 513,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_columns to maximum"
    );
    assert_eq!(
        settings.max_number_of_columns, 512,
        "max_number_of_columns should default to 512"
    );
}

#[test]
fn test_minimum_usable_lines() {
    let mut settings = ReplSettings {
        max_number_of_lines: 3,
        max_number_of_columns: 128,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_lines to minimum"
    );
    assert_eq!(
        settings.max_number_of_lines, 4,
        "max_number_of_lines should default to 4"
    );
}

#[test]
fn test_minimum_usable_columns() {
    let mut settings = ReplSettings {
        max_number_of_lines: 32,
        max_number_of_columns: 19,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should adjust max_number_of_columns to minimum"
    );
    assert_eq!(
        settings.max_number_of_columns, 20,
        "max_number_of_columns should default to 20"
    );
}

#[test]
fn test_valid_custom_settings() {
    let mut settings = ReplSettings {
        max_number_of_lines: 100,
        max_number_of_columns: 200,
    };
    assert!(
        settings.validate().is_ok(),
        "Validation should pass for valid custom settings"
    );
    assert_eq!(
        settings.max_number_of_lines, 100,
        "max_number_of_lines should remain unchanged"
    );
    assert_eq!(
        settings.max_number_of_columns, 200,
        "max_number_of_columns should remain unchanged"
    );
}
