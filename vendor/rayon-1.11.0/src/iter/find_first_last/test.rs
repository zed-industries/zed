use super::*;

#[test]
fn same_range_first_consumers_return_correct_answer() {
    let find_op = |x: &i32| x % 2 == 0;
    let first_found = AtomicUsize::new(usize::MAX);
    let far_right_consumer = FindConsumer::new(&find_op, MatchPosition::Leftmost, &first_found);

    // We save a consumer that will be far to the right of the main consumer (and therefore not
    // sharing an index range with that consumer) for fullness testing
    let consumer = far_right_consumer.split_off_left();

    // split until we have an indivisible range
    for _ in 0..usize::BITS {
        consumer.split_off_left();
    }

    let reducer = consumer.to_reducer();
    // the left and right folders should now have the same range, having
    // exhausted the resolution of usize
    let left_folder = consumer.split_off_left().into_folder();
    let right_folder = consumer.into_folder();

    let left_folder = left_folder.consume(0).consume(1);
    assert_eq!(left_folder.boundary, right_folder.boundary);
    // expect not full even though a better match has been found because the
    // ranges are the same
    assert!(!right_folder.full());
    assert!(far_right_consumer.full());
    let right_folder = right_folder.consume(2).consume(3);
    assert_eq!(
        reducer.reduce(left_folder.complete(), right_folder.complete()),
        Some(0)
    );
}

#[test]
fn same_range_last_consumers_return_correct_answer() {
    let find_op = |x: &i32| x % 2 == 0;
    let last_found = AtomicUsize::new(0);
    let consumer = FindConsumer::new(&find_op, MatchPosition::Rightmost, &last_found);

    // We save a consumer that will be far to the left of the main consumer (and therefore not
    // sharing an index range with that consumer) for fullness testing
    let far_left_consumer = consumer.split_off_left();

    // split until we have an indivisible range
    for _ in 0..usize::BITS {
        consumer.split_off_left();
    }

    let reducer = consumer.to_reducer();
    // due to the exact calculation in split_off_left, the very last consumer has a
    // range of width 2, so we use the second-to-last consumer instead to get
    // the same boundary on both folders
    let consumer = consumer.split_off_left();
    let left_folder = consumer.split_off_left().into_folder();
    let right_folder = consumer.into_folder();
    let right_folder = right_folder.consume(2).consume(3);
    assert_eq!(left_folder.boundary, right_folder.boundary);
    // expect not full even though a better match has been found because the
    // ranges are the same
    assert!(!left_folder.full());
    assert!(far_left_consumer.full());
    let left_folder = left_folder.consume(0).consume(1);
    assert_eq!(
        reducer.reduce(left_folder.complete(), right_folder.complete()),
        Some(2)
    );
}

// These tests requires that a folder be assigned to an iterator with more than
// one element. We can't necessarily determine when that will happen for a given
// input to find_first/find_last, so we test the folder directly here instead.
#[test]
fn find_first_folder_does_not_clobber_first_found() {
    let best_found = AtomicUsize::new(usize::MAX);
    let f = FindFolder {
        find_op: &(|&_: &i32| -> bool { true }),
        boundary: 0,
        match_position: MatchPosition::Leftmost,
        best_found: &best_found,
        item: None,
    };
    let f = f.consume(0_i32).consume(1_i32).consume(2_i32);
    assert!(f.full());
    assert_eq!(f.complete(), Some(0_i32));
}

#[test]
fn find_last_folder_yields_last_match() {
    let best_found = AtomicUsize::new(0);
    let f = FindFolder {
        find_op: &(|&_: &i32| -> bool { true }),
        boundary: 0,
        match_position: MatchPosition::Rightmost,
        best_found: &best_found,
        item: None,
    };
    let f = f.consume(0_i32).consume(1_i32).consume(2_i32);
    assert_eq!(f.complete(), Some(2_i32));
}
