extern crate psm;

#[test]
fn always_equal() {
    assert_eq!(psm::StackDirection::new(), psm::StackDirection::new());
}
