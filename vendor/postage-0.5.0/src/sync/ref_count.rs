use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct RefCount {
    count: AtomicUsize,
}

pub enum TryDecrement {
    Alive(usize),
    Dead,
}

impl TryDecrement {
    #[allow(dead_code)]
    #[track_caller]
    pub fn expect_dead(&self, message: &str) {
        if let Self::Alive(_) = self {
            panic!("TryDecrement unwrapped on an Alive value: {}", message);
        }
    }
}

impl RefCount {
    pub fn new(count: usize) -> Self {
        Self {
            count: AtomicUsize::new(count),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.count.load(Ordering::Acquire) > 0
    }

    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::AcqRel);
    }

    pub fn decrement(&self) -> TryDecrement {
        loop {
            let state = self.count.load(Ordering::Acquire);

            if state == 0 {
                return TryDecrement::Dead;
            }

            if let Ok(_prev) =
                self.count
                    .compare_exchange(state, state - 1, Ordering::AcqRel, Ordering::Relaxed)
            {
                if state == 1 {
                    return TryDecrement::Dead;
                } else {
                    return TryDecrement::Alive(state - 1);
                }
            }
        }
    }
}
