#![cfg(feature = "NSData")]

use objc2::rc::Retained;
use objc2::runtime::AnyObject;

use crate::Foundation::{NSData, NSMutableData, NSObject};

#[test]
fn test_bytes_mut() {
    let mut data = NSMutableData::with_bytes(&[7, 16]);
    data.bytes_mut()[0] = 3;
    assert_eq!(data.bytes(), [3, 16]);
}

#[test]
fn test_set_len() {
    let mut data = NSMutableData::with_bytes(&[7, 16]);
    data.setLength(4);
    assert_eq!(data.len(), 4);
    assert_eq!(data.bytes(), [7, 16, 0, 0]);

    data.setLength(1);
    assert_eq!(data.len(), 1);
    assert_eq!(data.bytes(), [7]);
}

#[test]
fn test_append() {
    let mut data = NSMutableData::with_bytes(&[7, 16]);
    data.extend_from_slice(&[3, 52]);
    assert_eq!(data.len(), 4);
    assert_eq!(data.bytes(), [7, 16, 3, 52]);
}

#[test]
#[cfg(feature = "NSRange")]
fn test_replace() {
    let mut data = NSMutableData::with_bytes(&[7, 16]);
    data.replace_range(0..0, &[3]);
    assert_eq!(data.bytes(), [3, 7, 16]);

    data.replace_range(1..2, &[52, 13]);
    assert_eq!(data.bytes(), [3, 52, 13, 16]);

    data.replace_range(2..4, &[6]);
    assert_eq!(data.bytes(), [3, 52, 6]);

    data.set_bytes(&[8, 17]);
    assert_eq!(data.bytes(), [8, 17]);
}

#[test]
fn test_from_data() {
    let data = NSData::with_bytes(&[1, 2]);
    let mut_data = NSMutableData::dataWithData(&data);
    assert_eq!(&*data, &**mut_data);
}

#[test]
fn test_with_capacity() {
    let mut data = NSMutableData::dataWithCapacity(5).unwrap();
    assert_eq!(data.bytes(), &[]);
    data.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(data.bytes(), &[1, 2, 3, 4, 5]);
    data.extend_from_slice(&[6, 7]);
    assert_eq!(data.bytes(), &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_extend() {
    let mut data = NSMutableData::with_bytes(&[1, 2]);
    data.extend(3..=5);
    assert_eq!(data.bytes(), &[1, 2, 3, 4, 5]);
    data.extend(&*NSData::with_bytes(&[6, 7]));
    assert_eq!(data.bytes(), &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_as_ref_borrow() {
    use core::borrow::{Borrow, BorrowMut};

    fn impls_borrow<T: AsRef<U> + Borrow<U> + ?Sized, U: ?Sized>(_: &T) {}
    fn impls_borrow_mut<T: AsMut<U> + BorrowMut<U> + ?Sized, U: ?Sized>(_: &mut T) {}

    // TODO: For some reason `new` doesn't work on GNUStep in release mode?
    let mut obj = if cfg!(feature = "gnustep-1-8") {
        NSMutableData::with_bytes(&[])
    } else {
        NSMutableData::new()
    };
    impls_borrow::<Retained<NSMutableData>, NSMutableData>(&obj);
    impls_borrow_mut::<Retained<NSMutableData>, NSMutableData>(&mut obj);

    impls_borrow::<NSMutableData, NSMutableData>(&obj);
    impls_borrow_mut::<NSMutableData, NSMutableData>(&mut obj);
    impls_borrow::<NSMutableData, NSData>(&obj);
    impls_borrow_mut::<NSMutableData, NSData>(&mut obj);
    impls_borrow::<NSMutableData, NSObject>(&obj);
    impls_borrow_mut::<NSMutableData, NSObject>(&mut obj);
    impls_borrow::<NSMutableData, AnyObject>(&obj);
    impls_borrow_mut::<NSMutableData, AnyObject>(&mut obj);

    impls_borrow::<NSData, NSData>(&obj);
    impls_borrow_mut::<NSData, NSData>(&mut obj);
    impls_borrow::<NSData, NSObject>(&obj);
    impls_borrow_mut::<NSData, NSObject>(&mut obj);
    impls_borrow::<NSData, AnyObject>(&obj);
    impls_borrow_mut::<NSData, AnyObject>(&mut obj);

    fn impls_as_ref<T: AsRef<U> + ?Sized, U: ?Sized>(_: &T) {}
    fn impls_as_mut<T: AsMut<U> + ?Sized, U: ?Sized>(_: &mut T) {}

    impls_as_ref::<NSMutableData, [u8]>(&obj);
    impls_as_mut::<NSMutableData, [u8]>(&mut obj);
    impls_as_ref::<NSData, [u8]>(&obj);

    let obj: &mut NSMutableData = &mut obj;
    let _: &[u8] = obj.as_ref();
    let _: &mut [u8] = obj.as_mut();

    let obj: &mut NSData = obj;
    let _: &[u8] = obj.as_ref();
}
