use alloc::{vec, vec::Vec};
use core::{iter, ops::Range};

use super::{convert, FallibleIterator, IntoFallible};

#[test]
fn all() {
    assert!(convert([0, 1, 2, 3].iter().map(Ok::<&u32, ()>))
        .all(|&i| Ok(i < 4))
        .unwrap());
    assert!(!convert([0, 1, 2, 4].iter().map(Ok::<&u32, ()>))
        .all(|&i| Ok(i < 4))
        .unwrap());
    assert!(convert([0, 1, 2, 4].iter().map(Ok::<&u32, ()>))
        .all(|_| Err(()))
        .is_err());
}

#[test]
fn any() {
    assert!(convert([0, 1, 2, 3].iter().map(Ok::<&u32, ()>))
        .any(|&i| Ok(i == 3))
        .unwrap());
    assert!(!convert([0, 1, 2, 4].iter().map(Ok::<&u32, ()>))
        .any(|&i| Ok(i == 3))
        .unwrap());
    assert!(convert([0, 1, 2, 4].iter().map(Ok::<&u32, ()>))
        .any(|_| Err(()))
        .is_err());
}

#[test]
fn chain() {
    let a = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, ()>));
    let b = convert(vec![4, 5, 6, 7].into_iter().map(Ok::<u32, ()>));
    let it = a.chain(b);

    assert_eq!(it.collect::<Vec<_>>().unwrap(), [0, 1, 2, 3, 4, 5, 6, 7]);

    let a = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, ()>));
    let b = convert(vec![4, 5, 6, 7].into_iter().map(Ok::<u32, ()>));
    let it = a.chain(b).rev();

    assert_eq!(it.collect::<Vec<_>>().unwrap(), [7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn count() {
    assert_eq!(
        convert([0, 1, 2, 3].iter().map(Ok::<&u32, ()>))
            .count()
            .unwrap(),
        4
    );

    let it = Some(Ok(1)).into_iter().chain(iter::repeat(Err(())));
    assert!(convert(it).count().is_err());
}

#[test]
fn enumerate() {
    let it = convert(vec![5, 6, 7, 8].into_iter().map(Ok::<u32, ()>)).enumerate();

    assert_eq!(
        it.collect::<Vec<_>>().unwrap(),
        [(0, 5), (1, 6), (2, 7), (3, 8)]
    );
}

#[test]
fn filter() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, u32>));
    let it = it.filter(|&x| if x % 2 == 0 { Ok(x % 3 == 0) } else { Err(x) });

    assert_eq!(it.clone().collect::<Vec<_>>(), Err(1));
    assert_eq!(it.rev().collect::<Vec<_>>(), Err(3));

    let it = convert(vec![0, 2, 4, 6].into_iter().map(Ok::<u32, u32>));
    let it = it.filter(|&x| if x % 2 == 0 { Ok(x % 3 == 0) } else { Err(x) });

    assert_eq!(it.clone().collect::<Vec<_>>(), Ok(vec![0, 6]));
    assert_eq!(it.rev().collect::<Vec<_>>(), Ok(vec![6, 0]))
}

#[test]
fn filter_map() {
    fn twos_and_threes(x: u32) -> Result<Option<u32>, u32> {
        if x % 2 == 0 {
            Ok(Some(x + 10))
        } else if x % 3 == 0 {
            Ok(None)
        } else {
            Err(x)
        }
    }

    let it = convert(vec![0, 1, 2, 3, 4, 5, 6].into_iter().map(Ok::<u32, u32>))
        .filter_map(twos_and_threes);

    assert_eq!(it.clone().collect::<Vec<_>>(), Err(1));
    assert_eq!(it.rev().collect::<Vec<_>>(), Err(5));

    let it =
        convert(vec![0, 2, 3, 4, 6].into_iter().map(Ok::<u32, u32>)).filter_map(twos_and_threes);

    assert_eq!(it.clone().collect::<Vec<_>>(), Ok(vec![10, 12, 14, 16]));
    assert_eq!(it.rev().collect::<Vec<_>>(), Ok(vec![16, 14, 12, 10]));
}

#[test]
fn find() {
    let mut it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, u32>));

    assert_eq!(it.find(|x| Ok(x % 2 == 1)), Ok(Some(1)));
    assert_eq!(it.next(), Ok(Some(2)));

    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, u32>));
    assert_eq!(
        it.clone()
            .find(|&x| if x == 2 { Err(29) } else { Ok(false) }),
        Err(29)
    );
    assert_eq!(
        it.clone()
            .find(|&x| if x == 2 { Err(29) } else { Ok(true) }),
        Ok(Some(0))
    );
    assert_eq!(
        it.clone()
            .rev()
            .find(|&x| if x == 2 { Err(29) } else { Ok(false) }),
        Err(29)
    );
    assert_eq!(
        it.rev().find(|&x| if x == 2 { Err(29) } else { Ok(true) }),
        Ok(Some(3))
    );
}

#[test]
fn fold() {
    fn add_smol(a: u32, b: u32) -> Result<u32, u32> {
        if b <= 2 {
            Ok(a + b)
        } else {
            Err(b)
        }
    }

    let it = convert(vec![0, 1, 3, 2].into_iter().map(Ok::<u32, u32>));
    assert_eq!(it.fold(0, add_smol), Err(3));

    let it = convert(vec![0, 1, 2, 1].into_iter().map(Ok::<u32, u32>));
    assert_eq!(it.fold(0, add_smol), Ok(4));
}

#[test]
fn for_each() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, ()>));

    let mut acc = vec![];
    it.for_each(|n| {
        acc.push(n);
        Ok(())
    })
    .unwrap();
    assert_eq!(acc, vec![0, 1, 2, 3]);
}

#[test]
fn iterator() {
    let it = convert(
        "ab cd"
            .chars()
            .map(|c| if c.is_whitespace() { Err(()) } else { Ok(c) }),
    );

    assert!(it.clone().count().is_err());
    assert!(it.clone().rev().count().is_err());
    assert_eq!(it.clone().iterator().count(), 5);
    assert_eq!(it.clone().iterator().rev().count(), 5);
}

#[test]
fn last() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<u32, ()>));
    assert_eq!(it.last().unwrap(), Some(3));
}

#[test]
fn map() {
    let it = convert(vec![0, 1, 2, 3, 4].into_iter().map(Ok::<u32, ()>)).map(|n| Ok(n * 2));
    fn assert_debug(_: &impl core::fmt::Debug) {}
    assert_debug(&it);
    assert_eq!(it.clone().collect::<Vec<_>>().unwrap(), [0, 2, 4, 6, 8]);
    assert_eq!(it.rev().collect::<Vec<_>>().unwrap(), [8, 6, 4, 2, 0]);

    let it = convert(vec![0, 1, 2, 3, 4].into_iter().map(Ok::<u32, ()>)).map(|n| {
        if n == 2 {
            Err(())
        } else {
            Ok(n * 2)
        }
    });

    {
        let mut it = it.clone();
        assert_eq!(it.next(), Ok(Some(0)));
        assert_eq!(it.next(), Ok(Some(2)));
        assert_eq!(it.next(), Err(()));
    }

    {
        let mut it = it.rev();
        assert_eq!(it.next(), Ok(Some(8)));
        assert_eq!(it.next(), Ok(Some(6)));
        assert_eq!(it.next(), Err(()));
    }
}

#[test]
fn map_err() {
    let it = convert(
        vec![0, 1, 2, 3]
            .into_iter()
            .map(|n| if n % 2 == 0 { Ok(n) } else { Err(n) }),
    );

    assert_eq!(it.clone().collect::<Vec<_>>(), Err(1));
    assert_eq!(it.rev().collect::<Vec<_>>(), Err(3));
}

#[test]
fn max() {
    let it = convert(vec![0, 3, 1, -10].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.max().unwrap(), Some(3));
}

#[test]
fn max_by_key() {
    let it = convert(vec![0, 3, 1, -10].into_iter().map(Ok::<i32, i32>));
    assert_eq!(it.clone().max_by_key(|&i| Ok(-i)), Ok(Some(-10)));
    // Exercise failure both on the first item, and later.
    assert_eq!(it.clone().max_by_key(|&i| Err::<i32, _>(i)), Err(0));
    assert_eq!(
        it.max_by_key(|&i| if i > 0 { Err(i) } else { Ok(-i) }),
        Err(3)
    );
}

#[test]
fn max_by() {
    let it = convert(vec![0, 3, 1, -10].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.max_by(|a, b| Ok(b.cmp(a))), Ok(Some(-10)));
}

#[test]
fn min() {
    let it = convert(vec![0, 3, -10, 1].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.min().unwrap(), Some(-10));
}

#[test]
fn min_by_key() {
    let it = convert(vec![0, 3, 1, -10].into_iter().map(Ok::<i32, i32>));
    assert_eq!(it.clone().min_by_key(|&i| Ok(-i)), Ok(Some(3)));
    // Exercise failure both on the first item, and later.
    assert_eq!(it.clone().min_by_key(|&i| Err::<i32, _>(i)), Err(0));
    assert_eq!(
        it.min_by_key(|&i| if i > 0 { Err(i) } else { Ok(-i) }),
        Err(3)
    );
}

#[test]
fn min_by() {
    let it = convert(vec![0, 3, 1, -10].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.min_by(|a, b| Ok(b.cmp(a))), Ok(Some(3)));
}

#[test]
fn nth() {
    let mut it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.nth(1).unwrap(), Some(1));
    assert_eq!(it.nth(0).unwrap(), Some(2));
    assert_eq!(it.nth(2).unwrap(), None);
}

#[test]
fn peekable() {
    let mut it = convert(vec![0, 1].into_iter().map(Ok::<i32, ()>)).peekable();
    assert_eq!(it.peek().unwrap(), Some(&0));
    assert_eq!(it.peek().unwrap(), Some(&0));
    assert_eq!(it.next().unwrap(), Some(0));
    assert_eq!(it.next().unwrap(), Some(1));
    assert_eq!(it.peek().unwrap(), None);
    assert_eq!(it.next().unwrap(), None);
}

#[test]
fn position() {
    let mut it = convert(vec![1, 2, 3, 4].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.position(|n| Ok(n == 2)).unwrap(), Some(1));
    assert_eq!(it.position(|n| Ok(n == 3)).unwrap(), Some(0));
    assert_eq!(it.position(|n| Ok(n == 5)).unwrap(), None);

    let mut it = convert(vec![1, 2, 3, 4].into_iter().map(Ok::<i32, i32>));
    assert_eq!(
        it.clone()
            .position(|n| if n == 3 { Err(42) } else { Ok(n == 2) }),
        Ok(Some(1))
    );
    assert_eq!(
        it.position(|n| if n == 3 { Err(42) } else { Ok(n == 4) }),
        Err(42)
    );
}

#[test]
fn scan() {
    let it = convert(vec![1, 2, 3, 4].into_iter().map(Ok::<i32, ()>)).scan(0, |st, v| {
        if v > 3 {
            Ok(None)
        } else {
            *st += v;
            Ok(Some(-*st))
        }
    });
    assert_eq!(it.collect::<Vec<_>>(), Ok(vec![-1, -3, -6]));
}

#[test]
fn skip() {
    let it = convert(vec![1, 2, 3, 4].into_iter().map(Ok::<i32, ()>));
    assert_eq!(it.clone().skip(0).collect::<Vec<_>>(), Ok(vec![1, 2, 3, 4]));
    assert_eq!(it.clone().skip(2).collect::<Vec<_>>(), Ok(vec![3, 4]));
    assert_eq!(it.skip(4).collect::<Vec<_>>(), Ok(vec![]));
}

#[test]
fn skip_while() {
    let it = convert(vec![1, 2, 3, 4, 1].into_iter().map(Ok::<i32, ()>));
    assert_eq!(
        it.clone().skip_while(|x| Ok(*x < 1)).collect::<Vec<_>>(),
        Ok(vec![1, 2, 3, 4, 1])
    );
    assert_eq!(
        it.clone().skip_while(|x| Ok(*x < 3)).collect::<Vec<_>>(),
        Ok(vec![3, 4, 1])
    );
    assert_eq!(
        it.skip_while(|x| Ok(*x < 5)).collect::<Vec<_>>(),
        Ok(vec![])
    );
}

#[test]
fn step_by() {
    let it = convert(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
            .into_iter()
            .map(Ok::<i32, ()>),
    )
    .step_by(3);
    assert_eq!(it.collect::<Vec<_>>(), Ok(vec![0, 3, 6]));
}

#[test]
fn take() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>)).take(2);
    assert_eq!(it.collect::<Vec<_>>().unwrap(), [0, 1]);
}

#[test]
fn take_while() {
    let it = convert(vec![0, 1, 2, 3, 0].into_iter().map(Ok::<i32, ()>));
    assert_eq!(
        it.clone().take_while(|x| Ok(*x < 0)).collect::<Vec<_>>(),
        Ok(vec![])
    );
    assert_eq!(
        it.clone().take_while(|x| Ok(*x < 2)).collect::<Vec<_>>(),
        Ok(vec![0, 1])
    );
    assert_eq!(
        it.take_while(|x| Ok(*x < 4)).collect::<Vec<_>>(),
        Ok(vec![0, 1, 2, 3, 0])
    );
}

#[test]
fn flat_map() {
    let it = convert(vec![0..1, 0..0, 1..5].into_iter().map(Ok::<Range<i32>, ()>))
        .flat_map(|r| Ok(convert(r.map(Ok::<i32, ()>))));
    assert_eq!(it.collect::<Vec<_>>(), Ok(vec![0, 1, 2, 3, 4]));
}

#[test]
fn flatten() {
    let it = convert(
        vec![0..1, 0..0, 1..5]
            .into_iter()
            .map(|r| convert(r.map(Ok::<i32, ()>)))
            .map(Ok::<_, ()>),
    )
    .flatten();
    assert_eq!(it.collect::<Vec<_>>(), Ok(vec![0, 1, 2, 3, 4]));
}

#[test]
fn inspect() {
    let mut buf = vec![];
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>)).inspect(|&v| {
        buf.push(v);
        Ok(())
    });
    it.count().unwrap();
    assert_eq!(buf, vec![0, 1, 2, 3]);
}

#[test]
fn partition() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>));
    let (even, odd): (Vec<i32>, Vec<i32>) = it.partition(|i| Ok(*i % 2 == 0)).unwrap();
    assert_eq!(even, vec![0, 2]);
    assert_eq!(odd, vec![1, 3]);
}

#[test]
fn find_map() {
    let mut it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>));
    assert_eq!(
        it.find_map(|v| match v {
            2 => Ok(Some("hi")),
            _ => Ok(None),
        }),
        Ok(Some("hi"))
    );
}

#[test]
fn unzip() {
    let it = convert(
        vec![(0, 0), (1, -1), (2, -2), (3, -3)]
            .into_iter()
            .map(Ok::<_, ()>),
    );
    let (pos, neg): (Vec<i32>, Vec<i32>) = it.unzip().unwrap();
    assert_eq!(pos, vec![0, 1, 2, 3]);
    assert_eq!(neg, vec![0, -1, -2, -3]);
}

#[test]
fn cycle() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>)).cycle();
    assert_eq!(it.take(6).collect::<Vec<_>>(), Ok(vec![0, 1, 2, 3, 0, 1]));
}

#[test]
fn unwrap() {
    let it = convert(vec![0, 1, 2, 3].into_iter().map(Ok::<i32, ()>)).unwrap();
    assert_eq!(it.collect::<Vec<_>>(), vec![0, 1, 2, 3]);
}

#[test]
#[should_panic]
fn unwrap_panic() {
    let _ = convert(vec![Ok(0), Err(())].into_iter())
        .unwrap()
        .collect::<Vec<_>>();
}

#[test]
fn wrap_std_iter_into_fallible() {
    let it = IntoFallible::from(vec![0, 1, 2, 3].into_iter());
    assert_eq!(it.collect::<Vec<_>>().unwrap(), vec![0, 1, 2, 3]);
}
