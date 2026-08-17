#![cfg(not(target_os = "unknown"))]

use async_std::{future::ready, task::block_on};

#[test]
fn smoke() {
    let res = block_on(async { 1 + 2 });
    assert_eq!(res, 3);
}

#[test]
#[should_panic = "boom"]
fn panic() {
    block_on(async {
        // This panic should get propagated into the parent thread.
        panic!("boom");
    });
}

#[cfg(feature = "unstable")]
#[test]
fn nested_block_on_local() {
    use async_std::task::spawn_local;

    let x = block_on(async {
        let a = block_on(async { block_on(async { ready(3).await }) });
        let b = spawn_local(async { block_on(async { ready(2).await }) }).await;
        let c = block_on(async { block_on(async { ready(1).await }) });
        a + b + c
    });

    assert_eq!(x, 3 + 2 + 1);

    let y = block_on(async {
        let a = block_on(async { block_on(async { ready(3).await }) });
        let b = spawn_local(async { block_on(async { ready(2).await }) }).await;
        let c = block_on(async { block_on(async { ready(1).await }) });
        a + b + c
    });

    assert_eq!(y, 3 + 2 + 1);
}

#[test]
fn nested_block_on() {
    let x = block_on(async {
        let a = block_on(async { block_on(async { ready(3).await }) });
        let b = block_on(async { block_on(async { ready(2).await }) });
        let c = block_on(async { block_on(async { ready(1).await }) });
        a + b + c
    });

    assert_eq!(x, 3 + 2 + 1);

    let y = block_on(async {
        let a = block_on(async { block_on(async { ready(3).await }) });
        let b = block_on(async { block_on(async { ready(2).await }) });
        let c = block_on(async { block_on(async { ready(1).await }) });
        a + b + c
    });

    assert_eq!(y, 3 + 2 + 1);
}
