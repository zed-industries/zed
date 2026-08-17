use alloc::string::ToString;

#[test]
fn test_ref_cell_try_borrow_error() {
    let rcell = core::cell::RefCell::new("str");

    let _active_borrow = rcell.try_borrow_mut().unwrap();

    assert_eq!(
        borsh::to_vec(&rcell).unwrap_err().to_string(),
        "already mutably borrowed"
    );
}
