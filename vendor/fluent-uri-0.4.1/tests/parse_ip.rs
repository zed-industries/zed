#![cfg(feature = "net")]

use core::net::{Ipv4Addr, Ipv6Addr};

use fluent_uri::{component::Host, UriRef};

fn parse_v4(s: &str) -> Option<Ipv4Addr> {
    let s = format!("//{s}");
    match UriRef::parse(&*s).ok()?.authority()?.host_parsed() {
        Host::Ipv4(addr) => Some(addr),
        _ => None,
    }
}

fn parse_v6(s: &str) -> Option<Ipv6Addr> {
    let s = format!("//[{s}]");
    match UriRef::parse(&*s).ok()?.authority()?.host_parsed() {
        Host::Ipv6(addr) => Some(addr),
        _ => None,
    }
}

#[test]
fn test_parse_v4() {
    assert_eq!(Some(Ipv4Addr::new(127, 0, 0, 1)), parse_v4("127.0.0.1"));
    assert_eq!(
        Some(Ipv4Addr::new(255, 255, 255, 255)),
        parse_v4("255.255.255.255")
    );
    assert_eq!(Some(Ipv4Addr::new(0, 0, 0, 0)), parse_v4("0.0.0.0"));

    // out of range
    assert!(parse_v4("256.0.0.1").is_none());
    // too short
    assert!(parse_v4("255.0.0").is_none());
    // too long
    assert!(parse_v4("255.0.0.1.2").is_none());
    // no number between dots
    assert!(parse_v4("255.0..1").is_none());
    // octal
    assert!(parse_v4("255.0.0.01").is_none());
    // octal zero
    assert!(parse_v4("255.0.0.00").is_none());
    assert!(parse_v4("255.0.00.0").is_none());
    // leading dot
    assert!(parse_v4(".0.0.0.0").is_none());
    // trailing dot
    assert!(parse_v4("0.0.0.0.").is_none());
}

#[test]
fn test_parse_v6() {
    assert_eq!(
        Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        parse_v6("0:0:0:0:0:0:0:0")
    );
    assert_eq!(
        Some(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8)),
        parse_v6("1:02:003:0004:0005:006:07:8")
    );

    assert_eq!(Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), parse_v6("::1"));
    assert_eq!(Some(Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0)), parse_v6("1::"));
    assert_eq!(Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), parse_v6("::"));

    assert_eq!(
        Some(Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11)),
        parse_v6("2a02:6b8::11:11")
    );

    assert_eq!(
        Some(Ipv6Addr::new(0, 2, 3, 4, 5, 6, 7, 8)),
        parse_v6("::2:3:4:5:6:7:8")
    );
    assert_eq!(
        Some(Ipv6Addr::new(1, 2, 3, 4, 0, 6, 7, 8)),
        parse_v6("1:2:3:4::6:7:8")
    );
    assert_eq!(
        Some(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 0)),
        parse_v6("1:2:3:4:5:6:7::")
    );

    // only a colon
    assert!(parse_v6(":").is_none());
    // too long group
    assert!(parse_v6("::00000").is_none());
    // too short
    assert!(parse_v6("1:2:3:4:5:6:7").is_none());
    // too long
    assert!(parse_v6("1:2:3:4:5:6:7:8:9").is_none());
    // triple colon
    assert!(parse_v6("1:2:::6:7:8").is_none());
    assert!(parse_v6("1:2:::").is_none());
    assert!(parse_v6(":::6:7:8").is_none());
    assert!(parse_v6(":::").is_none());
    // two double colons
    assert!(parse_v6("1:2::6::8").is_none());
    assert!(parse_v6("::6::8").is_none());
    assert!(parse_v6("1:2::6::").is_none());
    assert!(parse_v6("::2:6::").is_none());
    // `::` indicating zero groups of zeros
    assert!(parse_v6("::1:2:3:4:5:6:7:8").is_none());
    assert!(parse_v6("1:2:3:4::5:6:7:8").is_none());
    assert!(parse_v6("1:2:3:4:5:6:7:8::").is_none());
    // leading colon
    assert!(parse_v6(":1:2:3:4:5:6:7:8").is_none());
    assert!(parse_v6(":1::1").is_none());
    assert!(parse_v6(":1").is_none());
    // trailing colon
    assert!(parse_v6("1:2:3:4:5:6:7:8:").is_none());
    assert!(parse_v6("1::1:").is_none());
    assert!(parse_v6("1:").is_none());
}

#[test]
fn test_parse_v4_in_v6() {
    assert_eq!(
        Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 49152, 545)),
        parse_v6("::192.0.2.33")
    );
    assert_eq!(
        Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0xFFFF, 49152, 545)),
        parse_v6("::FFFF:192.0.2.33")
    );
    assert_eq!(
        Some(Ipv6Addr::new(0x64, 0xff9b, 0, 0, 0, 0, 49152, 545)),
        parse_v6("64:ff9b::192.0.2.33")
    );
    assert_eq!(
        Some(Ipv6Addr::new(
            0x2001, 0xdb8, 0x122, 0xc000, 0x2, 0x2100, 49152, 545
        )),
        parse_v6("2001:db8:122:c000:2:2100:192.0.2.33")
    );

    // colon after v4
    assert!(parse_v6("::127.0.0.1:").is_none());
    // not enough groups
    assert!(parse_v6("1:2:3:4:5:127.0.0.1").is_none());
    // too many groups
    assert!(parse_v6("1:2:3:4:5:6:7:127.0.0.1").is_none());
    // triple colons before v4
    assert!(parse_v6(":::4.4.4.4").is_none());
    // no colon before v4
    assert!(parse_v6("::ffff4.4.4.4").is_none());
}
