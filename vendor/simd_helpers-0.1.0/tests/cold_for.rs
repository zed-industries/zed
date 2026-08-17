use simd_helpers::cold_for_target_arch;


#[test]
#[cold_for_target_arch("x86", "x86_64")]
fn t() {}
