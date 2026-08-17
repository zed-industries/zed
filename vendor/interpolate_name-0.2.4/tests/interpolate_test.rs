use interpolate_name::interpolate_test;

#[interpolate_test(foo, "foo")]
#[interpolate_test(bar, "bar")]
#[interpolate_test(baz, "baz")]
#[interpolate_test(124, "test_literal")]
fn testme(f: &str) {
    println!("testing {}", f);
}

#[interpolate_test(foo, "foo")]
#[interpolate_test(bar, "bar")]
#[interpolate_test(baz, "baz")]
#[interpolate_test(124, "test_literal")]
#[ignore]
fn testme_ignore(f: &str) {
    println!("testing {}", f);
}

#[cfg_attr(not(feature = "a"), interpolate_name::interpolate_test(a, "a"))]
#[cfg_attr(feature = "b", interpolate_name::interpolate_test(b, "b"))]
fn testme_variants(f: &str) {
    println!("testing {}", f);
}
