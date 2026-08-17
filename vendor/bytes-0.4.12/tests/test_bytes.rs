extern crate bytes;

use bytes::{Bytes, BytesMut, BufMut, IntoBuf};

const LONG: &'static [u8] = b"mary had a little lamb, little lamb, little lamb";
const SHORT: &'static [u8] = b"hello world";

fn inline_cap() -> usize {
    use std::mem;
    4 * mem::size_of::<usize>() - 1
}

fn is_sync<T: Sync>() {}
fn is_send<T: Send>() {}

#[test]
fn test_bounds() {
    is_sync::<Bytes>();
    is_sync::<BytesMut>();
    is_send::<Bytes>();
    is_send::<BytesMut>();
}

#[test]
fn from_slice() {
    let a = Bytes::from(&b"abcdefgh"[..]);
    assert_eq!(a, b"abcdefgh"[..]);
    assert_eq!(a, &b"abcdefgh"[..]);
    assert_eq!(a, Vec::from(&b"abcdefgh"[..]));
    assert_eq!(b"abcdefgh"[..], a);
    assert_eq!(&b"abcdefgh"[..], a);
    assert_eq!(Vec::from(&b"abcdefgh"[..]), a);

    let a = BytesMut::from(&b"abcdefgh"[..]);
    assert_eq!(a, b"abcdefgh"[..]);
    assert_eq!(a, &b"abcdefgh"[..]);
    assert_eq!(a, Vec::from(&b"abcdefgh"[..]));
    assert_eq!(b"abcdefgh"[..], a);
    assert_eq!(&b"abcdefgh"[..], a);
    assert_eq!(Vec::from(&b"abcdefgh"[..]), a);
}

#[test]
fn fmt() {
    let a = format!("{:?}", Bytes::from(&b"abcdefg"[..]));
    let b = "b\"abcdefg\"";

    assert_eq!(a, b);

    let a = format!("{:?}", BytesMut::from(&b"abcdefg"[..]));
    assert_eq!(a, b);
}

#[test]
fn fmt_write() {
    use std::fmt::Write;
    use std::iter::FromIterator;
    let s = String::from_iter((0..10).map(|_| "abcdefg"));

    let mut a = BytesMut::with_capacity(64);
    write!(a, "{}", &s[..64]).unwrap();
    assert_eq!(a, s[..64].as_bytes());


    let mut b = BytesMut::with_capacity(64);
    write!(b, "{}", &s[..32]).unwrap();
    write!(b, "{}", &s[32..64]).unwrap();
    assert_eq!(b, s[..64].as_bytes());


    let mut c = BytesMut::with_capacity(64);
    write!(c, "{}", s).unwrap_err();
    assert!(c.is_empty());
}

#[test]
fn len() {
    let a = Bytes::from(&b"abcdefg"[..]);
    assert_eq!(a.len(), 7);

    let a = BytesMut::from(&b"abcdefg"[..]);
    assert_eq!(a.len(), 7);

    let a = Bytes::from(&b""[..]);
    assert!(a.is_empty());

    let a = BytesMut::from(&b""[..]);
    assert!(a.is_empty());
}

#[test]
fn index() {
    let a = Bytes::from(&b"hello world"[..]);
    assert_eq!(a[0..5], *b"hello");
}

#[test]
fn slice() {
    let a = Bytes::from(&b"hello world"[..]);

    let b = a.slice(3, 5);
    assert_eq!(b, b"lo"[..]);

    let b = a.slice(0, 0);
    assert_eq!(b, b""[..]);

    let b = a.slice(3, 3);
    assert_eq!(b, b""[..]);

    let b = a.slice(a.len(), a.len());
    assert_eq!(b, b""[..]);

    let b = a.slice_to(5);
    assert_eq!(b, b"hello"[..]);

    let b = a.slice_from(3);
    assert_eq!(b, b"lo world"[..]);
}

#[test]
#[should_panic]
fn slice_oob_1() {
    let a = Bytes::from(&b"hello world"[..]);
    a.slice(5, inline_cap() + 1);
}

#[test]
#[should_panic]
fn slice_oob_2() {
    let a = Bytes::from(&b"hello world"[..]);
    a.slice(inline_cap() + 1, inline_cap() + 5);
}

#[test]
fn split_off() {
    let mut hello = Bytes::from(&b"helloworld"[..]);
    let world = hello.split_off(5);

    assert_eq!(hello, &b"hello"[..]);
    assert_eq!(world, &b"world"[..]);

    let mut hello = BytesMut::from(&b"helloworld"[..]);
    let world = hello.split_off(5);

    assert_eq!(hello, &b"hello"[..]);
    assert_eq!(world, &b"world"[..]);
}

#[test]
#[should_panic]
fn split_off_oob() {
    let mut hello = Bytes::from(&b"helloworld"[..]);
    hello.split_off(inline_cap() + 1);
}

#[test]
fn split_off_uninitialized() {
    let mut bytes = BytesMut::with_capacity(1024);
    let other = bytes.split_off(128);

    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), 128);

    assert_eq!(other.len(), 0);
    assert_eq!(other.capacity(), 896);
}

#[test]
fn split_off_to_loop() {
    let s = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for i in 0..(s.len() + 1) {
        {
            let mut bytes = Bytes::from(&s[..]);
            let off = bytes.split_off(i);
            assert_eq!(i, bytes.len());
            let mut sum = Vec::new();
            sum.extend(&bytes);
            sum.extend(&off);
            assert_eq!(&s[..], &sum[..]);
        }
        {
            let mut bytes = BytesMut::from(&s[..]);
            let off = bytes.split_off(i);
            assert_eq!(i, bytes.len());
            let mut sum = Vec::new();
            sum.extend(&bytes);
            sum.extend(&off);
            assert_eq!(&s[..], &sum[..]);
        }
        {
            let mut bytes = Bytes::from(&s[..]);
            let off = bytes.split_to(i);
            assert_eq!(i, off.len());
            let mut sum = Vec::new();
            sum.extend(&off);
            sum.extend(&bytes);
            assert_eq!(&s[..], &sum[..]);
        }
        {
            let mut bytes = BytesMut::from(&s[..]);
            let off = bytes.split_to(i);
            assert_eq!(i, off.len());
            let mut sum = Vec::new();
            sum.extend(&off);
            sum.extend(&bytes);
            assert_eq!(&s[..], &sum[..]);
        }
    }
}

#[test]
fn split_to_1() {
    // Inline
    let mut a = Bytes::from(SHORT);
    let b = a.split_to(4);

    assert_eq!(SHORT[4..], a);
    assert_eq!(SHORT[..4], b);

    // Allocated
    let mut a = Bytes::from(LONG);
    let b = a.split_to(4);

    assert_eq!(LONG[4..], a);
    assert_eq!(LONG[..4], b);

    let mut a = Bytes::from(LONG);
    let b = a.split_to(30);

    assert_eq!(LONG[30..], a);
    assert_eq!(LONG[..30], b);
}

#[test]
fn split_to_2() {
    let mut a = Bytes::from(LONG);
    assert_eq!(LONG, a);

    let b = a.split_to(1);

    assert_eq!(LONG[1..], a);
    drop(b);
}

#[test]
#[should_panic]
fn split_to_oob() {
    let mut hello = Bytes::from(&b"helloworld"[..]);
    hello.split_to(inline_cap() + 1);
}

#[test]
#[should_panic]
fn split_to_oob_mut() {
    let mut hello = BytesMut::from(&b"helloworld"[..]);
    hello.split_to(inline_cap() + 1);
}

#[test]
fn split_to_uninitialized() {
    let mut bytes = BytesMut::with_capacity(1024);
    let other = bytes.split_to(128);

    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), 896);

    assert_eq!(other.len(), 0);
    assert_eq!(other.capacity(), 128);
}

#[test]
fn split_off_to_at_gt_len() {
    fn make_bytes() -> Bytes {
        let mut bytes = BytesMut::with_capacity(100);
        bytes.put_slice(&[10, 20, 30, 40]);
        bytes.freeze()
    }

    use std::panic;

    make_bytes().split_to(4);
    make_bytes().split_off(4);

    assert!(panic::catch_unwind(move || {
        make_bytes().split_to(5);
    }).is_err());

    assert!(panic::catch_unwind(move || {
        make_bytes().split_off(5);
    }).is_err());
}

#[test]
fn fns_defined_for_bytes_mut() {
    let mut bytes = BytesMut::from(&b"hello world"[..]);

    bytes.as_ptr();
    bytes.as_mut_ptr();

    // Iterator
    let v: Vec<u8> = bytes.iter().map(|b| *b).collect();
    assert_eq!(&v[..], bytes);
}

#[test]
fn mut_into_buf() {
    let mut v = vec![0, 0, 0, 0];
    let s = &mut v[..];
    s.into_buf().put_u32_le(42);
}

#[test]
fn reserve_convert() {
    // Inline -> Vec
    let mut bytes = BytesMut::with_capacity(8);
    bytes.put("hello");
    bytes.reserve(40);
    assert_eq!(bytes.capacity(), 45);
    assert_eq!(bytes, "hello");

    // Inline -> Inline
    let mut bytes = BytesMut::with_capacity(inline_cap());
    bytes.put("abcdefghijkl");

    let a = bytes.split_to(10);
    bytes.reserve(inline_cap() - 3);
    assert_eq!(inline_cap(), bytes.capacity());

    assert_eq!(bytes, "kl");
    assert_eq!(a, "abcdefghij");

    // Vec -> Vec
    let mut bytes = BytesMut::from(LONG);
    bytes.reserve(64);
    assert_eq!(bytes.capacity(), LONG.len() + 64);

    // Arc -> Vec
    let mut bytes = BytesMut::from(LONG);
    let a = bytes.split_to(30);

    bytes.reserve(128);
    assert!(bytes.capacity() >= bytes.len() + 128);

    drop(a);
}

#[test]
fn reserve_growth() {
    let mut bytes = BytesMut::with_capacity(64);
    bytes.put("hello world");
    let _ = bytes.take();

    bytes.reserve(65);
    assert_eq!(bytes.capacity(), 128);
}

#[test]
fn reserve_allocates_at_least_original_capacity() {
    let mut bytes = BytesMut::with_capacity(1024);

    for i in 0..1020 {
        bytes.put(i as u8);
    }

    let _other = bytes.take();

    bytes.reserve(16);
    assert_eq!(bytes.capacity(), 1024);
}

#[test]
fn reserve_max_original_capacity_value() {
    const SIZE: usize = 128 * 1024;

    let mut bytes = BytesMut::with_capacity(SIZE);

    for _ in 0..SIZE {
        bytes.put(0u8);
    }

    let _other = bytes.take();

    bytes.reserve(16);
    assert_eq!(bytes.capacity(), 64 * 1024);
}

// Without either looking at the internals of the BytesMut or doing weird stuff
// with the memory allocator, there's no good way to automatically verify from
// within the program that this actually recycles memory. Instead, just exercise
// the code path to ensure that the results are correct.
#[test]
fn reserve_vec_recycling() {
    let mut bytes = BytesMut::from(Vec::with_capacity(16));
    assert_eq!(bytes.capacity(), 16);
    bytes.put("0123456789012345");
    bytes.advance(10);
    assert_eq!(bytes.capacity(), 6);
    bytes.reserve(8);
    assert_eq!(bytes.capacity(), 16);
}

#[test]
fn reserve_in_arc_unique_does_not_overallocate() {
    let mut bytes = BytesMut::with_capacity(1000);
    bytes.take();

    // now bytes is Arc and refcount == 1

    assert_eq!(1000, bytes.capacity());
    bytes.reserve(2001);
    assert_eq!(2001, bytes.capacity());
}

#[test]
fn reserve_in_arc_unique_doubles() {
    let mut bytes = BytesMut::with_capacity(1000);
    bytes.take();

    // now bytes is Arc and refcount == 1

    assert_eq!(1000, bytes.capacity());
    bytes.reserve(1001);
    assert_eq!(2000, bytes.capacity());
}

#[test]
fn reserve_in_arc_nonunique_does_not_overallocate() {
    let mut bytes = BytesMut::with_capacity(1000);
    let _copy = bytes.take();

    // now bytes is Arc and refcount == 2

    assert_eq!(1000, bytes.capacity());
    bytes.reserve(2001);
    assert_eq!(2001, bytes.capacity());
}

#[test]
fn inline_storage() {
    let mut bytes = BytesMut::with_capacity(inline_cap());
    let zero = [0u8; 64];

    bytes.put(&zero[0..inline_cap()]);
    assert_eq!(*bytes, zero[0..inline_cap()]);
}

#[test]
fn extend_mut() {
    let mut bytes = BytesMut::with_capacity(0);
    bytes.extend(LONG);
    assert_eq!(*bytes, LONG[..]);
}

#[test]
fn extend_shr() {
    let mut bytes = Bytes::new();
    bytes.extend(LONG);
    assert_eq!(*bytes, LONG[..]);
}

#[test]
fn extend_from_slice_mut() {
    for &i in &[3, 34] {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&LONG[..i]);
        bytes.extend_from_slice(&LONG[i..]);
        assert_eq!(LONG[..], *bytes);
    }
}

#[test]
fn extend_from_slice_shr() {
    for &i in &[3, 34] {
        let mut bytes = Bytes::new();
        bytes.extend_from_slice(&LONG[..i]);
        bytes.extend_from_slice(&LONG[i..]);
        assert_eq!(LONG[..], *bytes);
    }
}

#[test]
fn from_static() {
    let mut a = Bytes::from_static(b"ab");
    let b = a.split_off(1);

    assert_eq!(a, b"a"[..]);
    assert_eq!(b, b"b"[..]);
}

#[test]
fn advance_inline() {
    let mut a = Bytes::from(&b"hello world"[..]);
    a.advance(6);
    assert_eq!(a, &b"world"[..]);
}

#[test]
fn advance_static() {
    let mut a = Bytes::from_static(b"hello world");
    a.advance(6);
    assert_eq!(a, &b"world"[..]);
}

#[test]
fn advance_vec() {
    let mut a = BytesMut::from(b"hello world boooo yah world zomg wat wat".to_vec());
    a.advance(16);
    assert_eq!(a, b"o yah world zomg wat wat"[..]);

    a.advance(4);
    assert_eq!(a, b"h world zomg wat wat"[..]);

    // Reserve some space.
    a.reserve(1024);
    assert_eq!(a, b"h world zomg wat wat"[..]);

    a.advance(6);
    assert_eq!(a, b"d zomg wat wat"[..]);
}

#[test]
#[should_panic]
fn advance_past_len() {
    let mut a = BytesMut::from(b"hello world".to_vec());
    a.advance(20);
}

#[test]
// Only run these tests on little endian systems. CI uses qemu for testing
// little endian... and qemu doesn't really support threading all that well.
#[cfg(target_endian = "little")]
fn stress() {
    // Tests promoting a buffer from a vec -> shared in a concurrent situation
    use std::sync::{Arc, Barrier};
    use std::thread;

    const THREADS: usize = 8;
    const ITERS: usize = 1_000;

    for i in 0..ITERS {
        let data = [i as u8; 256];
        let buf = Arc::new(Bytes::from(&data[..]));

        let barrier = Arc::new(Barrier::new(THREADS));
        let mut joins = Vec::with_capacity(THREADS);

        for _ in 0..THREADS {
            let c = barrier.clone();
            let buf = buf.clone();

            joins.push(thread::spawn(move || {
                c.wait();
                let buf: Bytes = (*buf).clone();
                drop(buf);
            }));
        }

        for th in joins {
            th.join().unwrap();
        }

        assert_eq!(*buf, data[..]);
    }
}

#[test]
fn partial_eq_bytesmut() {
    let bytes = Bytes::from(&b"The quick red fox"[..]);
    let bytesmut = BytesMut::from(&b"The quick red fox"[..]);
    assert!(bytes == bytesmut);
    assert!(bytesmut == bytes);
    let bytes2 = Bytes::from(&b"Jumped over the lazy brown dog"[..]);
    assert!(bytes2 != bytesmut);
    assert!(bytesmut != bytes2);
}

#[test]
fn unsplit_basic() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaabbbcccddd");

    let splitted = buf.split_off(6);
    assert_eq!(b"aaabbb", &buf[..]);
    assert_eq!(b"cccddd", &splitted[..]);

    buf.unsplit(splitted);
    assert_eq!(b"aaabbbcccddd", &buf[..]);
}

#[test]
fn unsplit_empty_other() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaabbbcccddd");

    // empty other
    let other = BytesMut::new();

    buf.unsplit(other);
    assert_eq!(b"aaabbbcccddd", &buf[..]);
}

#[test]
fn unsplit_empty_self() {
    // empty self
    let mut buf = BytesMut::new();

    let mut other = BytesMut::with_capacity(64);
    other.extend_from_slice(b"aaabbbcccddd");

    buf.unsplit(other);
    assert_eq!(b"aaabbbcccddd", &buf[..]);
}

#[test]
fn unsplit_inline_arc() {
    let mut buf = BytesMut::with_capacity(8); //inline
    buf.extend_from_slice(b"aaaabbbb");

    let mut buf2 = BytesMut::with_capacity(64);
    buf2.extend_from_slice(b"ccccddddeeee");

    buf2.split_off(8); //arc

    buf.unsplit(buf2);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);
}

#[test]
fn unsplit_arc_inline() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaaabbbbeeee");

    buf.split_off(8); //arc

    let mut buf2 = BytesMut::with_capacity(8); //inline
    buf2.extend_from_slice(b"ccccdddd");

    buf.unsplit(buf2);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);

}

#[test]
fn unsplit_both_inline() {
    let mut buf = BytesMut::with_capacity(16); //inline
    buf.extend_from_slice(b"aaaabbbbccccdddd");

    let splitted = buf.split_off(8); // both inline
    assert_eq!(b"aaaabbbb", &buf[..]);
    assert_eq!(b"ccccdddd", &splitted[..]);

    buf.unsplit(splitted);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);
}


#[test]
fn unsplit_arc_different() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaaabbbbeeee");

    buf.split_off(8); //arc

    let mut buf2 = BytesMut::with_capacity(64);
    buf2.extend_from_slice(b"ccccddddeeee");

    buf2.split_off(8); //arc

    buf.unsplit(buf2);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);
}

#[test]
fn unsplit_arc_non_contiguous() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaaabbbbeeeeccccdddd");

    let mut buf2 = buf.split_off(8); //arc

    let buf3 = buf2.split_off(4); //arc

    buf.unsplit(buf3);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);
}

#[test]
fn unsplit_two_split_offs() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"aaaabbbbccccdddd");

    let mut buf2 = buf.split_off(8); //arc
    let buf3 = buf2.split_off(4); //arc

    buf2.unsplit(buf3);
    buf.unsplit(buf2);
    assert_eq!(b"aaaabbbbccccdddd", &buf[..]);
}

#[test]
fn from_iter_no_size_hint() {
    use std::iter;

    let mut expect = vec![];

    let actual: Bytes = iter::repeat(b'x')
        .scan(100, |cnt, item| {
            if *cnt >= 1 {
                *cnt -= 1;
                expect.push(item);
                Some(item)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(&actual[..], &expect[..]);
}

fn test_slice_ref(bytes: &Bytes, start: usize, end: usize, expected: &[u8]) {
    let slice = &(bytes.as_ref()[start..end]);
    let sub = bytes.slice_ref(&slice);
    assert_eq!(&sub[..], expected);
}

#[test]
fn slice_ref_works() {
    let bytes = Bytes::from(&b"012345678"[..]);

    test_slice_ref(&bytes, 0, 0, b"");
    test_slice_ref(&bytes, 0, 3, b"012");
    test_slice_ref(&bytes, 2, 6, b"2345");
    test_slice_ref(&bytes, 7, 9, b"78");
    test_slice_ref(&bytes, 9, 9, b"");
}


#[test]
fn slice_ref_empty() {
    let bytes = Bytes::from(&b""[..]);
    let slice = &(bytes.as_ref()[0..0]);

    let sub = bytes.slice_ref(&slice);
    assert_eq!(&sub[..], b"");
}

#[test]
#[should_panic]
fn slice_ref_catches_not_a_subset() {
    let bytes = Bytes::from(&b"012345678"[..]);
    let slice = &b"012345"[0..4];

    bytes.slice_ref(slice);
}

#[test]
#[should_panic]
fn slice_ref_catches_not_an_empty_subset() {
    let bytes = Bytes::from(&b"012345678"[..]);
    let slice = &b""[0..0];

    bytes.slice_ref(slice);
}

#[test]
#[should_panic]
fn empty_slice_ref_catches_not_an_empty_subset() {
    let bytes = Bytes::from(&b""[..]);
    let slice = &b""[0..0];

    bytes.slice_ref(slice);
}
