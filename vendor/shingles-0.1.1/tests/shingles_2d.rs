extern crate shingles;

use shingles::*;

#[test]
fn shingles_2d_empty_str() {
    let s = ["", ""];
    let actual: Vec<_> = s.as_shingles_2d([2, 2]).collect();
    assert!(actual.is_empty());
}

#[test]
fn shingles_2d_num() {
    let a: Vec<_> = (1..9).collect();
    let v: Vec<_> = a.chunks(4).collect();

    let expected = vec![
        vec![&v[0][0..2], &v[1][0..2]],
        vec![&v[0][1..3], &v[1][1..3]],
        vec![&v[0][2..4], &v[1][2..4]],
    ];
    let actual: Vec<_> = v[..].as_shingles_2d([2, 2]).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingles_2d_num_with_step() {
    let a: Vec<_> = (1..9).collect();
    let v: Vec<_> = a.chunks(4).collect();

    let expected = vec![
        vec![&v[0][0..2], &v[1][0..2]],
        vec![&v[0][2..4], &v[1][2..4]],
    ];
    let actual: Vec<_> = v[..].as_shingles_2d_with_step([2, 2], [2, 2]).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingles_2d_str() {
    let v: Vec<_> = "abcd\n\
                     efgh\n\
                     ijkl"
        .split_terminator("\n")
        .collect();
    let expected = vec![
        vec![&v[0][0..3], &v[1][0..3], &v[2][0..3]],
        vec![&v[0][1..4], &v[1][1..4], &v[2][1..4]],
    ];
    let actual: Vec<_> = v[..].as_shingles_2d([3, 3]).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingles_2d_str_with_step() {
    let v: Vec<_> = "abcd\n\
                     efgh"
        .split_terminator("\n")
        .collect();
    let expected = vec![
        vec![&v[0][0..2], &v[1][0..2]],
        vec![&v[0][2..4], &v[1][2..4]],
    ];
    let actual: Vec<_> = v[..].as_shingles_2d_with_step([2, 2], [2, 2]).collect();
    assert_eq!(expected, actual);
}