use mac_notification_sys::*;

#[test]
fn set_application_again() {
    set_application("com.apple.Terminal").unwrap();
    assert!(set_application("com.apple.Terminal").is_err());
}

#[test]
fn get_default_identifier() {
    let bundle = get_bundle_identifier_or_default("thisappdoesnotexist");
    assert_eq!(bundle, "com.apple.Finder");
}
