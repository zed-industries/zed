use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::Priority;

pub(crate) struct PriorityQueueSender<T> {
    sender: flume::Sender<(Priority, T)>,
}

impl<T> PriorityQueueSender<T> {
    pub(crate) fn new(tx: flume::Sender<(Priority, T)>) -> Self {
        Self { sender: tx }
    }

    pub(crate) fn send(
        &self,
        priority: Priority,
        item: T,
    ) -> Result<(), flume::SendError<(Priority, T)>> {
        self.sender.send((priority, item))
    }
}

pub(crate) struct PriorityQueueReceiver<T> {
    receiver: flume::Receiver<(Priority, T)>,
    rand: SmallRng,
    high_priority: Vec<T>,
    medium_priority: Vec<T>,
    low_priority: Vec<T>,
    disconnected: bool,
}

impl<T> Clone for PriorityQueueReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            rand: SmallRng::seed_from_u64(0),
            high_priority: Vec::new(),
            medium_priority: Vec::new(),
            low_priority: Vec::new(),
            disconnected: self.disconnected,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ReceiverDisconnected;

impl<T> PriorityQueueReceiver<T> {
    pub(crate) fn new() -> (PriorityQueueSender<T>, Self) {
        let (tx, rx) = flume::unbounded();

        let sender = PriorityQueueSender::new(tx);

        let receiver = PriorityQueueReceiver {
            receiver: rx,
            rand: SmallRng::seed_from_u64(0),
            high_priority: Vec::new(),
            medium_priority: Vec::new(),
            low_priority: Vec::new(),
            disconnected: false,
        };

        (sender, receiver)
    }

    /// Tries to pop one element from the priority queue without blocking.
    ///
    /// This will early return if there are no elements in the queue.
    ///
    /// This method is best suited if you only intend to pop one element, for better performance
    /// on large queues see [`Self::try_iter`]
    ///
    /// # Errors
    ///
    /// If the sender was dropped
    pub(crate) fn try_pop(&mut self) -> Result<Option<T>, ReceiverDisconnected> {
        self.pop_inner(false, false)
    }

    /// Pops an element from the priority queue blocking if necessary.
    ///
    /// This method is best suited if you only intend to pop one element, for better performance
    /// on large queues see [`Self::iter``]
    ///
    /// # Errors
    ///
    /// If the sender was dropped
    pub(crate) fn pop(&mut self) -> Result<T, ReceiverDisconnected> {
        self.pop_inner(false, true).map(|e| e.unwrap())
    }

    /// Returns an iterator over the elements of the queue
    /// this iterator will end when all elements have been consumed and will not wait for new ones.
    pub(crate) fn try_iter(self) -> TryIter<T> {
        TryIter(self)
    }

    /// Returns an iterator over the elements of the queue
    /// this iterator will wait for new elements if the queue is empty.
    pub(crate) fn iter(self) -> Iter<T> {
        Iter(self)
    }

    fn collect_new(&mut self, pop_many: bool, block: bool) {
        let mut add_element = |this: &mut Self, (priority, item): (Priority, T)| match priority {
            Priority::Realtime(_) => unreachable!(),
            Priority::High => this.high_priority.push(item),
            Priority::Medium => this.medium_priority.push(item),
            Priority::Low => this.low_priority.push(item),
        };

        if block && self.is_empty() {
            match self.receiver.recv() {
                Ok(e) => {
                    add_element(self, e);
                }
                Err(flume::RecvError::Disconnected) => {
                    self.disconnected = true;
                }
            };
        }

        // dont starve by getting stuck here
        let count = if pop_many { 100 } else { 1 };
        for _ in 0..count {
            match self.receiver.try_recv() {
                Ok(e) => {
                    add_element(self, e);
                }
                Err(flume::TryRecvError::Empty) => {
                    break;
                }
                Err(flume::TryRecvError::Disconnected) => {
                    self.disconnected = true;
                    break;
                }
            }
        }
    }

    #[inline(always)]
    // algorithm is the loaded die from biased coin from
    // https://www.keithschwarz.com/darts-dice-coins/
    fn pop_inner(
        &mut self,
        pop_many: bool,
        block: bool,
    ) -> Result<Option<T>, ReceiverDisconnected> {
        use Priority as P;
        if self.disconnected && self.is_empty() {
            return Err(ReceiverDisconnected);
        }

        self.collect_new(pop_many, block);

        let high = P::High.probability() * !self.high_priority.is_empty() as u32;
        let medium = P::Medium.probability() * !self.medium_priority.is_empty() as u32;
        let low = P::Low.probability() * !self.low_priority.is_empty() as u32;
        let mut mass = high + medium + low; //%

        if !self.high_priority.is_empty() {
            let flip = self.rand.random_ratio(P::High.probability(), mass);
            if flip {
                return Ok(self.high_priority.pop());
            }
            mass -= P::High.probability();
        }

        if !self.medium_priority.is_empty() {
            let flip = self.rand.random_ratio(P::Medium.probability(), mass);
            if flip {
                return Ok(self.medium_priority.pop());
            }
            mass -= P::Medium.probability();
        }

        if !self.low_priority.is_empty() {
            let flip = self.rand.random_ratio(P::Low.probability(), mass);
            if flip {
                return Ok(self.low_priority.pop());
            }
        }

        debug_assert!(
            self.is_empty(),
            "Prio::High + Prio::Medium + Prio::low should be 100"
        );
        Ok(None)
    }

    fn is_empty(&self) -> bool {
        self.high_priority.is_empty()
            && self.medium_priority.is_empty()
            && self.low_priority.is_empty()
    }
}

/// If None is returned the sender disconnected
pub(crate) struct Iter<T>(PriorityQueueReceiver<T>);
impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_inner(true, true).ok().flatten()
    }
}

/// If None is returned there are no more elements in the queue
pub(crate) struct TryIter<T>(PriorityQueueReceiver<T>);
impl<T> Iterator for TryIter<T> {
    type Item = Result<T, ReceiverDisconnected>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_inner(true, false).transpose()
    }
}

#[cfg(test)]
mod tests {
    use collections::HashSet;

    use super::*;

    #[test]
    fn all_tasks_get_yielded() {
        let (tx, mut rx) = PriorityQueueReceiver::new();
        tx.send(Priority::Medium, 20).unwrap();
        tx.send(Priority::High, 30).unwrap();
        tx.send(Priority::Low, 10).unwrap();
        tx.send(Priority::Medium, 21).unwrap();
        tx.send(Priority::High, 31).unwrap();

        drop(tx);

        assert_eq!(
            rx.iter().collect::<HashSet<_>>(),
            [30, 31, 20, 21, 10].into_iter().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn new_high_prio_task_get_scheduled_quickly() {
        let (tx, mut rx) = PriorityQueueReceiver::new();
        for _ in 0..100 {
            tx.send(Priority::Low, 1).unwrap();
        }

        assert_eq!(rx.pop().unwrap(), 1);
        tx.send(Priority::High, 3).unwrap();
        assert_eq!(rx.pop().unwrap(), 3);
        assert_eq!(rx.pop().unwrap(), 1);
    }
}
