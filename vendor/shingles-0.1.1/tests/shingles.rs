extern crate shingles;

use shingles::*;

#[test]
fn shingles_empty_num() {
    let v = vec![1];
    let res: Vec<_> = v[..].as_shingles(2).collect();
    assert!(res.is_empty());
}

#[test]
fn shingles_num() {
    let v: Vec<_> = (0..10).collect();
    let expected: Vec<_> = v.windows(2).collect();
    let actual: Vec<_> = v.as_shingles(2).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingled_num_with_step() {
    let v: Vec<_> = (0..10).collect();
    let expected: Vec<_> = v.windows(2).enumerate()
        .filter(|&(i, _)| i % 3 == 0)
        .map(|(_, s)| s)
        .collect();
    let actual: Vec<_> = v.as_shingles_with_step(2, 3).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingles_str() {
    let expected = vec!["Прив", "риве", "ивет", "вет!"];
    let actual: Vec<_> = Shingles::new("Привет!", 4).collect();
    assert_eq!(expected, actual);
}

#[test]
fn shingles_str_with_step() {
    let expected = vec!["hell", "llo!"];
    let actual: Vec<_> = Shingles::new_with_step("hello!", 4, 2).collect();
    assert_eq!(expected, actual);
}