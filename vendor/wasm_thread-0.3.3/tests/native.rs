//! Trivial tests to ensure that native thread API is unchanged.

use wasm_thread as thread;

#[test]
fn thread_join() {
    let handle = thread::spawn(|| 1234);

    assert_eq!(handle.join().unwrap(), 1234);
}

#[test]
fn thread_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread {:?}", thread::current().id());
            // We can borrow `a` here.
            dbg!(&a)
        });

        s.spawn(|| {
            println!("hello from the second scoped thread {:?}", thread::current().id());
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });

        println!(
            "Hello from scope \"main\" thread {:?} inside scope.",
            thread::current().id()
        );
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
