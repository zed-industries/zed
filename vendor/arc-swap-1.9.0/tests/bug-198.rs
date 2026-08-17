use std:: sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use arc_swap::ArcSwap;

struct SpinBarrier(AtomicUsize);

impl SpinBarrier {
    fn new(n: usize) -> Self {
        Self(AtomicUsize::new(n))
    }

    fn wait(&self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
        while self.0.load(Ordering::Relaxed) != 0 {}
    }

    fn wrap<R: Send, F: FnOnce() -> R + Send>(&self, f: F) -> impl FnOnce() -> R + Send + use<'_, R, F> {
        move || {
            self.wait();
            f()
        }
    }
}

fn main() {
    let arcswap = ArcSwap::<usize>::new(0.into());
    let _guards = std::array::from_fn::<_, 8, _>(|_| arcswap.load());
    let barrier = SpinBarrier::new(3);
    thread::scope(|s| {
        s.spawn(barrier.wrap(|| arcswap.store(1.into())));
        s.spawn(barrier.wrap(|| arcswap.store(2.into())));
        barrier.wait();
        let a1 = arcswap.load();
        let a2 = arcswap.load();
        if **a1 != **a2 && **a1 != 0 {
            assert_eq!(**a2, **arcswap.load());
        }
    });
}
