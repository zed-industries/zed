use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn test_ipv4_addr_roundtrip_enum() {
    let original = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
    let encoded = borsh::to_vec(&original).expect("Serialization failed");
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);
    let decoded = borsh::from_slice::<IpAddr>(&encoded).expect("Deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_ipv4_addr_roundtrip() {
    let original = Ipv4Addr::new(192, 168, 0, 1);
    let encoded = borsh::to_vec(&original).expect("Serialization failed");
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);
    let decoded = borsh::from_slice::<Ipv4Addr>(&encoded).expect("Deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_ipv6_addr_roundtrip_enum() {
    let original = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    let encoded = borsh::to_vec(&original).expect("Serialization failed");
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);
    let decoded = borsh::from_slice::<IpAddr>(&encoded).expect("Deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_ipv6_addr_roundtrip() {
    let original = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    let encoded = borsh::to_vec(&original).expect("Serialization failed");
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);
    let decoded = borsh::from_slice::<Ipv6Addr>(&encoded).expect("Deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_ipaddr_vec_roundtrip() {
    let original = vec![
        IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
    ];
    let encoded = borsh::to_vec(&original).expect("Serialization failed");
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);
    let decoded = borsh::from_slice::<Vec<IpAddr>>(&encoded).expect("Deserialization failed");
    assert_eq!(original, decoded);
}
