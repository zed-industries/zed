#[cfg(feature = "net")]
use core::net::{Ipv4Addr, Ipv6Addr};

use fluent_uri::{component::Host, pct_enc::EStr, ParseErrorKind, Uri, UriRef};

#[test]
fn parse_absolute() {
    let r = UriRef::parse("file:///etc/hosts").unwrap();
    assert_eq!(r.as_str(), "file:///etc/hosts");
    assert_eq!(r.scheme().unwrap().as_str(), "file");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "");
    assert!(matches!(a.host_parsed(), Host::RegName(n) if n.is_empty()));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "/etc/hosts");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("ftp://ftp.is.co.za/rfc/rfc1808.txt").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "ftp");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "ftp.is.co.za");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "ftp.is.co.za");
    assert!(matches!(a.host_parsed(), Host::RegName(name) if name == "ftp.is.co.za"));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "/rfc/rfc1808.txt");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("http://www.ietf.org/rfc/rfc2396.txt").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "http");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "www.ietf.org");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "www.ietf.org");
    assert!(matches!(a.host_parsed(), Host::RegName(name) if name == "www.ietf.org"));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "/rfc/rfc2396.txt");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("ldap://[2001:db8::7]/c=GB?objectClass?one").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "ldap");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "[2001:db8::7]");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "[2001:db8::7]");
    #[cfg(feature = "net")]
    assert!(matches!(
        a.host_parsed(),
        Host::Ipv6(addr) if addr == Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x7)
    ));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "/c=GB");
    assert_eq!(r.query(), Some(EStr::new_or_panic("objectClass?one")));
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("mailto:John.Doe@example.com").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "mailto");
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "John.Doe@example.com");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("news:comp.infosystems.www.servers.unix").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "news");
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "comp.infosystems.www.servers.unix");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("tel:+1-816-555-1212").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "tel");
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "+1-816-555-1212");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("telnet://192.0.2.16:80/").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "telnet");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "192.0.2.16:80");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "192.0.2.16");
    #[cfg(feature = "net")]
    assert!(matches!(a.host_parsed(), Host::Ipv4(addr) if addr == Ipv4Addr::new(192, 0, 2, 16)));
    assert_eq!(a.port(), Some(EStr::new_or_panic("80")));
    assert_eq!(r.path(), "/");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("urn:oasis:names:specification:docbook:dtd:xml:4.1.2").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "urn");
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "oasis:names:specification:docbook:dtd:xml:4.1.2");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("foo://example.com:8042/over/there?name=ferret#nose").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "foo");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "example.com:8042");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "example.com");
    assert!(matches!(a.host_parsed(), Host::RegName(name) if name == "example.com"));
    assert_eq!(a.port(), Some(EStr::new_or_panic("8042")));
    assert_eq!(r.path(), "/over/there");
    assert_eq!(r.query(), Some(EStr::new_or_panic("name=ferret")));
    assert_eq!(r.fragment(), Some(EStr::new_or_panic("nose")));

    let r =
        UriRef::parse("ftp://cnn.example.com&story=breaking_news@10.0.0.1/top_story.htm").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "ftp");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "cnn.example.com&story=breaking_news@10.0.0.1");
    assert_eq!(
        a.userinfo(),
        Some(EStr::new_or_panic("cnn.example.com&story=breaking_news"))
    );
    assert_eq!(a.host(), "10.0.0.1");
    #[cfg(feature = "net")]
    assert!(matches!(a.host_parsed(), Host::Ipv4(addr) if addr == Ipv4Addr::new(10, 0, 0, 1)));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "/top_story.htm");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("http://[vFe.foo.bar]").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "http");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "[vFe.foo.bar]");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "[vFe.foo.bar]");
    assert!(matches!(a.host_parsed(), Host::IpvFuture { .. }));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("http://127.0.0.1:/").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "http");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "127.0.0.1:");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "127.0.0.1");
    #[cfg(feature = "net")]
    assert!(matches!(a.host_parsed(), Host::Ipv4(addr) if addr == Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(a.port(), Some(EStr::EMPTY));
    assert_eq!(r.path(), "/");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("http://127.0.0.1:8080/").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "http");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "127.0.0.1:8080");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "127.0.0.1");
    #[cfg(feature = "net")]
    assert!(matches!(a.host_parsed(), Host::Ipv4(addr) if addr == Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(a.port(), Some(EStr::new_or_panic("8080")));
    assert_eq!(r.path(), "/");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("http://127.0.0.1:80808/").unwrap();
    assert_eq!(r.scheme().unwrap().as_str(), "http");
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "127.0.0.1:80808");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "127.0.0.1");
    #[cfg(feature = "net")]
    assert!(matches!(a.host_parsed(), Host::Ipv4(addr) if addr == Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(a.port(), Some(EStr::new_or_panic("80808")));
    assert_eq!(r.path(), "/");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);
}

#[test]
fn parse_relative() {
    let r = UriRef::parse("").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("foo.txt").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "foo.txt");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse(".").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), ".");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("./this:that").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "./this:that");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("//example.com").unwrap();
    assert!(r.scheme().is_none());
    let a = r.authority().unwrap();
    assert_eq!(a.as_str(), "example.com");
    assert_eq!(a.userinfo(), None);
    assert_eq!(a.host(), "example.com");
    assert!(matches!(a.host_parsed(), Host::RegName(name) if name == "example.com"));
    assert_eq!(a.port(), None);
    assert_eq!(r.path(), "");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("?query").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "");
    assert_eq!(r.query(), Some(EStr::new_or_panic("query")));
    assert_eq!(r.fragment(), None);

    let r = UriRef::parse("#fragment").unwrap();
    assert!(r.scheme().is_none());
    assert!(r.authority().is_none());
    assert_eq!(r.path(), "");
    assert_eq!(r.query(), None);
    assert_eq!(r.fragment(), Some(EStr::new_or_panic("fragment")));
}

use ParseErrorKind::*;

#[test]
fn parse_error_uri() {
    #[track_caller]
    fn fail(input: &str, index: usize, kind: ParseErrorKind) {
        let e = Uri::parse(input).unwrap_err();
        assert_eq!(e.index(), index);
        assert_eq!(e.kind(), kind);
    }

    // No scheme
    fail("foo", 3, UnexpectedChar);

    // Empty scheme
    fail(":hello", 0, UnexpectedChar);

    // Scheme starts with non-letter
    fail("3ttp://a.com", 0, UnexpectedChar);

    // Unexpected char in scheme
    fail("exam=ple:foo", 4, UnexpectedChar);
    fail("(:", 0, UnexpectedChar);

    // Percent-encoded scheme
    fail("a%20:foo", 1, UnexpectedChar);
}

#[track_caller]
fn fail(input: &str, index: usize, kind: ParseErrorKind) {
    let e = UriRef::parse(input).unwrap_err();
    assert_eq!(e.index(), index);
    assert_eq!(e.kind(), kind);
}

#[test]
fn parse_error_uri_ref() {
    // Empty scheme
    fail(":hello", 0, UnexpectedChar);

    // Scheme starts with non-letter
    fail("3ttp://a.com", 0, UnexpectedChar);

    // After rewriting the parser, the following two cases are interpreted as
    // containing colon in the first path segment of a relative reference.

    // Unexpected char in scheme
    fail("exam=ple:foo", 8, UnexpectedChar);
    fail("(:", 1, UnexpectedChar);

    // Percent-encoded scheme
    fail("a%20:foo", 4, UnexpectedChar);

    // Unexpected char in path
    fail("foo\\bar", 3, UnexpectedChar);

    // Non-hexadecimal percent-encoded octet
    fail("foo%xxd", 3, InvalidPctEncodedOctet);

    // Incomplete percent-encoded octet
    fail("text%a", 4, InvalidPctEncodedOctet);

    // A single percent
    fail("%", 0, InvalidPctEncodedOctet);

    // Non-decimal port
    fail("http://example.com:80ab", 21, UnexpectedChar);
    fail("http://user@example.com:80ab", 26, UnexpectedChar);

    // Multiple colons in authority
    fail("http://user:pass:example.com/", 16, UnexpectedChar);

    // Unclosed bracket
    fail("https://[::1/", 12, UnexpectedChar);

    // Not port after IP literal
    fail("https://[::1]wrong", 13, UnexpectedChar);

    // IP literal too short
    fail("http://[:]", 8, InvalidIpv6Addr);
    fail("http://[]", 8, UnexpectedChar);

    // Non-hexadecimal version in IPvFuture
    fail("http://[vG.addr]", 9, UnexpectedChar);

    // Empty version in IPvFuture
    fail("http://[v.addr]", 9, UnexpectedChar);

    // Empty address in IPvFuture
    fail("ftp://[vF.]", 10, UnexpectedChar);

    // Percent-encoded address in IPvFuture
    fail("ftp://[vF.%20]", 10, UnexpectedChar);

    // With zone identifier
    fail("ftp://[fe80::abcd%eth0]", 17, UnexpectedChar);

    // Invalid IPv6 address
    fail("example://[44:55::66::77]", 11, InvalidIpv6Addr);
}

#[test]
fn strict_ip_addr() {
    let r = UriRef::parse("//127.0.0.001").unwrap();
    let a = r.authority().unwrap();
    assert!(matches!(a.host_parsed(), Host::RegName(_)));

    let r = UriRef::parse("//127.1").unwrap();
    let a = r.authority().unwrap();
    assert!(matches!(a.host_parsed(), Host::RegName(_)));

    let r = UriRef::parse("//127.00.00.1").unwrap();
    let a = r.authority().unwrap();
    assert!(matches!(a.host_parsed(), Host::RegName(_)));

    assert!(UriRef::parse("//[::1.1.1.1]").is_ok());
    assert!(UriRef::parse("//[::ffff:1.1.1.1]").is_ok());
    assert!(UriRef::parse("//[0000:0000:0000:0000:0000:0000:255.255.255.255]").is_ok());

    fail("//[::01.1.1.1]", 3, InvalidIpv6Addr);
    fail("//[::00.1.1.1]", 3, InvalidIpv6Addr);
}
