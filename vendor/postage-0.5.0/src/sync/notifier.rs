use atomic::Ordering;
use crossbeam_queue::SegQueue;
use std::{sync::atomic::AtomicUsize, task::Waker};

#[derive(Debug)]
pub struct Notifier {
    generation: AtomicUsize,
    wakers: SegQueue<Waker>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            generation: AtomicUsize::new(0),
            wakers: SegQueue::new(),
        }
    }

    pub fn guard(&self) -> NotificationGuard {
        let generation = self.generation.load(Ordering::Relaxed);

        NotificationGuard {
            generation,
            stored_generation: &self.generation,
        }
    }

    pub fn notify(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);

        #[cfg(feature = "debug")]
        let mut woken = 0usize;

        while let Some(waker) = self.wakers.pop() {
            #[cfg(feature = "debug")]
            {
                woken += 1;
            }

            waker.wake();
        }

        #[cfg(feature = "debug")]
        if woken > 0 {
            log::info!("Woke {} tasks", woken);
        }
    }

    pub fn subscribe(&self, cx: &crate::Context<'_>) {
        if let Some(waker) = cx.waker() {
            self.wakers.push(waker.clone());
        }
    }
}

pub struct NotificationGuard<'a> {
    generation: usize,
    stored_generation: &'a AtomicUsize,
}

impl<'a> NotificationGuard<'a> {
    pub fn is_expired(&self) -> bool {
        self.stored_generation.load(Ordering::Relaxed) != self.generation
    }
}
