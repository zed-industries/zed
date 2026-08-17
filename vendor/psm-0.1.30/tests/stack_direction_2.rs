extern crate psm;

#[inline(never)]
fn test_direction(previous_sp: *mut u8) {
    let current_sp = psm::stack_pointer();
    match psm::StackDirection::new() {
        psm::StackDirection::Ascending => {
            assert!(
                current_sp > previous_sp,
                "the stack pointer is not ascending! current = {:p}, previous = {:p}",
                current_sp,
                previous_sp
            );
        }
        psm::StackDirection::Descending => {
            assert!(
                current_sp < previous_sp,
                "the stack pointer is not descending! current = {:p}, previous = {:p}",
                current_sp,
                previous_sp
            );
        }
    }
}

#[test]
fn direction_right() {
    test_direction(psm::stack_pointer());
}
