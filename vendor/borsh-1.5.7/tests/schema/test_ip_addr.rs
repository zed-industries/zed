use crate::common_macro::schema_imports::*;
use std::net::IpAddr;

#[test]
fn ip_addr_schema() {
    let actual_name = IpAddr::declaration();
    assert_eq!("IpAddr", actual_name);
    let mut defs = Default::default();
    IpAddr::add_definitions_recursively(&mut defs);
    insta::assert_snapshot!(format!("{:#?}", defs));
}
