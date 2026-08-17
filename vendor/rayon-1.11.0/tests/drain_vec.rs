use rayon::prelude::*;

#[test]
fn drain_vec_yielded() {
    let mut vec_org = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let yielded = vec_org.par_drain(0..5).collect::<Vec<_>>();

    assert_eq!(&yielded, &[0, 1, 2, 3, 4]);
    assert_eq!(&vec_org, &[5, 6, 7, 8, 9]);
}

#[test]
fn drain_vec_dropped() {
    let mut vec_org = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let yielded = vec_org.par_drain(0..5);

    drop(yielded);
    assert_eq!(&vec_org, &[5, 6, 7, 8, 9]);
}

#[test]
fn drain_vec_empty_range_yielded() {
    let mut vec_org = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let yielded = vec_org.par_drain(5..5).collect::<Vec<_>>();

    assert_eq!(&yielded, &[]);
    assert_eq!(&vec_org, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn drain_vec_empty_range_dropped() {
    let mut vec_org = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let yielded = vec_org.par_drain(5..5);

    drop(yielded);
    assert_eq!(&vec_org, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
