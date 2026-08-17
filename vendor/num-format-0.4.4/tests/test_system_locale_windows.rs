#![cfg(all(feature = "with-system-locale", windows))]

use num_format::SystemLocale;

#[test]
fn test_windows() {
    let names = SystemLocale::available_names().unwrap();
    assert!(!names.is_empty());
    for name in &names {
        let _ = SystemLocale::from_name(name).unwrap();
    }
}
