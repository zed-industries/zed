use ring::test::{compile_time_assert_send, compile_time_assert_sync};
use webpki::DnsNameRef;

#[test]
fn test_dns_name_ref_traits() {
    compile_time_assert_send::<DnsNameRef>();
    compile_time_assert_sync::<DnsNameRef>();

    let a = DnsNameRef::try_from_ascii(b"example.com").unwrap();

    // `Copy`
    {
        let _b = a;
        let _c = a;
    }

    // `Clone`
    #[allow(clippy::clone_on_copy)]
    let _ = a.clone();
    // TODO: verify the clone is the same as `a`.

    // TODO: Don't require `alloc` for these.
    #[cfg(feature = "alloc")]
    {
        // `Debug`.
        assert_eq!(format!("{:?}", &a), "DnsNameRef(\"example.com\")");
    }
}

#[cfg(feature = "alloc")]
#[test]
fn test_dns_name_traits() {
    use webpki::DnsName;

    fn compile_time_assert_hash<T: core::hash::Hash>() {}

    compile_time_assert_hash::<DnsName>();
    compile_time_assert_send::<DnsName>();
    compile_time_assert_sync::<DnsName>();

    let a_ref = DnsNameRef::try_from_ascii(b"example.com").unwrap();
    let a = a_ref.to_owned();

    // `Clone`, `Debug`, `PartialEq`.
    assert_eq!(&a, &a.clone());

    // `Debug`.
    assert_eq!(format!("{:?}", &a), "DnsName(\"example.com\")");

    // PartialEq is case-insensitive
    assert_eq!(
        a,
        DnsNameRef::try_from_ascii(b"Example.Com")
            .unwrap()
            .to_owned()
    );

    // PartialEq isn't completely wrong.
    assert_ne!(
        a,
        DnsNameRef::try_from_ascii(b"fxample.com")
            .unwrap()
            .to_owned()
    );
    assert_ne!(
        a,
        DnsNameRef::try_from_ascii(b"example.co")
            .unwrap()
            .to_owned()
    );
}
