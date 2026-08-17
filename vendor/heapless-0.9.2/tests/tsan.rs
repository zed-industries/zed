#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]

use std::{ptr::addr_of_mut, thread};

use heapless::spsc;

#[test]
fn once() {
    static mut RB: spsc::Queue<i32, 4> = spsc::Queue::new();

    let rb = unsafe { &mut *addr_of_mut!(RB) };

    rb.enqueue(0).unwrap();

    let (mut p, mut c) = rb.split();

    p.enqueue(1).unwrap();

    thread::spawn(move || {
        p.enqueue(1).unwrap();
    });

    thread::spawn(move || {
        c.dequeue().unwrap();
    });
}

#[test]
fn twice() {
    static mut RB: spsc::Queue<i32, 5> = spsc::Queue::new();

    let rb = unsafe { &mut *addr_of_mut!(RB) };

    rb.enqueue(0).unwrap();
    rb.enqueue(1).unwrap();

    let (mut p, mut c) = rb.split();

    thread::spawn(move || {
        p.enqueue(2).unwrap();
        p.enqueue(3).unwrap();
    });

    thread::spawn(move || {
        c.dequeue().unwrap();
        c.dequeue().unwrap();
    });
}

#[test]
fn scoped() {
    let mut rb: spsc::Queue<i32, 5> = spsc::Queue::new();

    rb.enqueue(0).unwrap();

    {
        let (mut p, mut c) = rb.split();

        thread::scope(move |scope| {
            scope.spawn(move || {
                p.enqueue(1).unwrap();
            });

            scope.spawn(move || {
                c.dequeue().unwrap();
            });
        });
    }

    rb.dequeue().unwrap();
}

#[test]
#[cfg_attr(miri, ignore)] // too slow
fn contention() {
    const N: usize = 1024;

    let mut rb: spsc::Queue<u8, 4> = spsc::Queue::new();

    {
        let (mut p, mut c) = rb.split();

        thread::scope(move |scope| {
            scope.spawn(move || {
                let mut sum: u32 = 0;

                for i in 0..(2 * N) {
                    sum = sum.wrapping_add(i as u32);
                    while p.enqueue(i as u8).is_err() {}
                }

                println!("producer: {sum}");
            });

            scope.spawn(move || {
                let mut sum: u32 = 0;

                for _ in 0..(2 * N) {
                    loop {
                        if let Some(v) = c.dequeue() {
                            sum = sum.wrapping_add(v as u32);
                            break;
                        }
                    }
                }

                println!("consumer: {sum}");
            });
        });
    }

    assert!(rb.is_empty());
}

#[test]
#[cfg_attr(miri, ignore)] // too slow
fn mpmc_contention() {
    use std::sync::mpsc;

    use heapless::mpmc::Queue;

    const N: u32 = 64;

    #[expect(deprecated)]
    static Q: Queue<u32, 64> = Queue::new();

    let (s, r) = mpsc::channel();
    thread::scope(|scope| {
        let s1 = s.clone();
        scope.spawn(move || {
            let mut sum: u32 = 0;

            for i in 0..(16 * N) {
                sum = sum.wrapping_add(i);
                println!("enqueue {i}");
                while Q.enqueue(i).is_err() {}
            }

            s1.send(sum).unwrap();
        });

        let s2 = s.clone();
        scope.spawn(move || {
            let mut sum: u32 = 0;

            for _ in 0..(16 * N) {
                loop {
                    if let Some(v) = Q.dequeue() {
                        sum = sum.wrapping_add(v);
                        println!("dequeue {v}");
                        break;
                    }
                }
            }

            s2.send(sum).unwrap();
        });
    });

    assert_eq!(r.recv().unwrap(), r.recv().unwrap());
}

#[test]
#[cfg_attr(miri, ignore)] // too slow
fn unchecked() {
    const N: usize = 1024;

    let mut rb: spsc::Queue<u8, N> = spsc::Queue::new();

    for _ in 0..N / 2 - 1 {
        rb.enqueue(1).unwrap();
    }

    {
        let (mut p, mut c) = rb.split();

        thread::scope(move |scope| {
            scope.spawn(move || {
                for _ in 0..N / 2 - 1 {
                    p.enqueue(2).unwrap();
                }
            });

            scope.spawn(move || {
                let mut sum: usize = 0;

                for _ in 0..N / 2 - 1 {
                    sum = sum.wrapping_add(usize::from(c.dequeue().unwrap()));
                }

                assert_eq!(sum, N / 2 - 1);
            });
        });
    }

    assert_eq!(rb.len(), N / 2 - 1);
}

#[test]
fn len_properly_wraps() {
    const N: usize = 4;
    let mut rb: spsc::Queue<u8, N> = spsc::Queue::new();

    rb.enqueue(1).unwrap();
    assert_eq!(rb.len(), 1);
    rb.dequeue();
    assert_eq!(rb.len(), 0);
    rb.enqueue(2).unwrap();
    assert_eq!(rb.len(), 1);
    rb.enqueue(3).unwrap();
    assert_eq!(rb.len(), 2);
    rb.enqueue(4).unwrap();
    assert_eq!(rb.len(), 3);
}

#[test]
fn iterator_properly_wraps() {
    const N: usize = 4;
    let mut rb: spsc::Queue<u8, N> = spsc::Queue::new();

    rb.enqueue(1).unwrap();
    rb.dequeue();
    rb.enqueue(2).unwrap();
    rb.enqueue(3).unwrap();
    rb.enqueue(4).unwrap();
    let expected = [2, 3, 4];
    let mut actual = [0, 0, 0];
    for (idx, el) in rb.iter().enumerate() {
        actual[idx] = *el;
    }
    assert_eq!(expected, actual);
}
