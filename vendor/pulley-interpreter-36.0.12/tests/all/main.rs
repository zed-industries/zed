#[cfg(all(feature = "disas", feature = "encode"))]
mod disas;

#[cfg(feature = "interp")]
mod interp;

// Test the property relied on by `crates/cranelift/src/obj.rs` when filling in
// the `PulleyHostcall` relocation.
#[test]
fn test_call_indirect_host_width() {
    let mut dst = Vec::new();
    pulley_interpreter::encode::call_indirect_host(&mut dst, 1_u8);
    assert_eq!(dst.len(), 4);
    assert_eq!(dst[3], 1);
}
