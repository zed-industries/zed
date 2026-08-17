use super::Error;

#[test]
fn test_unknown_log_is_not_fatal() {
    assert!(!Error::UnknownLog.should_be_fatal());
}

#[test]
fn test_unknown_sct_version_is_not_fatal() {
    assert!(!Error::UnsupportedSctVersion.should_be_fatal());
}

#[test]
fn test_other_errors_are_fatal() {
    assert!(Error::MalformedSct.should_be_fatal());
    assert!(Error::InvalidSignature.should_be_fatal());
    assert!(Error::TimestampInFuture.should_be_fatal());
}
