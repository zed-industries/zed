#![cfg(feature = "NSData")]

use crate::{NSData, NSMutableData};

#[test]
fn test_bytes_mut() {
    let data = NSMutableData::with_bytes(&[7, 16]);
    unsafe { data.as_mut_bytes_unchecked()[0] = 3 };
    assert_eq!(data.to_vec(), [3, 16]);
}

#[test]
fn test_set_len() {
    let data = NSMutableData::with_bytes(&[7, 16]);
    data.setLength(4);
    assert_eq!(data.len(), 4);
    assert_eq!(data.to_vec(), [7, 16, 0, 0]);

    data.setLength(1);
    assert_eq!(data.len(), 1);
    assert_eq!(data.to_vec(), [7]);
}

#[test]
fn test_append() {
    let data = NSMutableData::with_bytes(&[7, 16]);
    data.extend_from_slice(&[3, 52]);
    assert_eq!(data.len(), 4);
    assert_eq!(data.to_vec(), [7, 16, 3, 52]);
}

#[test]
#[cfg(feature = "NSRange")]
fn test_replace() {
    let data = NSMutableData::with_bytes(&[7, 16]);
    data.replace_range(0..0, &[3]);
    assert_eq!(data.to_vec(), [3, 7, 16]);

    data.replace_range(1..2, &[52, 13]);
    assert_eq!(data.to_vec(), [3, 52, 13, 16]);

    data.replace_range(2..4, &[6]);
    assert_eq!(data.to_vec(), [3, 52, 6]);

    data.set_bytes(&[8, 17]);
    assert_eq!(data.to_vec(), [8, 17]);
}

#[test]
fn test_from_data() {
    let data = NSData::with_bytes(&[1, 2]);
    let mut_data = NSMutableData::dataWithData(&data);
    assert_eq!(&*data, &**mut_data);
}

#[test]
fn test_with_capacity() {
    let data = NSMutableData::dataWithCapacity(5).unwrap();
    assert_eq!(data.to_vec(), &[]);
    data.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(data.to_vec(), &[1, 2, 3, 4, 5]);
    data.extend_from_slice(&[6, 7]);
    assert_eq!(data.to_vec(), &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_extend() {
    let mut data = NSMutableData::with_bytes(&[1, 2]);
    data.extend(3..=5);
    assert_eq!(data.to_vec(), &[1, 2, 3, 4, 5]);
    (&data).extend(&*NSData::with_bytes(&[6, 7]));
    assert_eq!(data.to_vec(), &[1, 2, 3, 4, 5, 6, 7]);
}
