mod countdown_futures;
mod countdown_streams;

mod prioritized_waker {
    use std::{cmp::Ordering, task::Waker};

    // PrioritizedWaker(index, waker).
    // Lowest index gets popped off the BinaryHeap first.
    pub struct PrioritizedWaker(pub usize, pub Waker);
    impl PartialEq for PrioritizedWaker {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for PrioritizedWaker {
        fn assert_receiver_is_total_eq(&self) {}
    }
    impl PartialOrd for PrioritizedWaker {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for PrioritizedWaker {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0).reverse()
        }
    }
}
use prioritized_waker::PrioritizedWaker;

#[derive(Clone, Copy)]
enum State {
    Init,
    Polled,
    Done,
}

fn shuffle<T>(slice: &mut [T]) {
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    slice.shuffle(&mut rng);
}

pub use countdown_futures::*;
pub use countdown_streams::*;
