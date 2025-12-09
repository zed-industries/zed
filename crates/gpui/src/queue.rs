use rand::{Rng, SeedableRng, rand_core::block, rngs::SmallRng};

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
            rand: SmallRng::from_os_rng(),
            high_priority: Vec::new(),
            medium_priority: Vec::new(),
            low_priority: Vec::new(),
            disconnected: self.disconnected,
        }
    }
}

pub(crate) struct ReceiverDisconnected;

impl<T> PriorityQueueReceiver<T> {
    const TICKET_COUNT: usize = 100;

    pub(crate) fn new() -> (PriorityQueueSender<T>, Self) {
        let (tx, rx) = flume::unbounded();

        let sender = PriorityQueueSender::new(tx);

        let receiver = PriorityQueueReceiver {
            receiver: rx,
            rand: SmallRng::from_os_rng(),
            high_priority: Vec::new(),
            medium_priority: Vec::new(),
            low_priority: Vec::new(),
            disconnected: false,
        };

        (sender, receiver)
    }

    /// Tries to pop as many elements from the priority queue as possible
    /// and returns them in the order of priorities [High, Medium, Low].
    ///
    /// This will early return if there are no elements in the queue.
    ///
    /// # Errors
    ///
    /// If the sender was dropped
    pub(crate) fn try_pop(&mut self) -> Result<T, ReceiverDisconnected> {
        self.pop_inner(false)
    }

    /// Tries to pop as many elements from the priority queue as possible
    /// and returns them in the order of priorities [High, Medium, Low]
    ///
    /// # Errors
    ///
    /// If the sender was dropped
    pub(crate) fn pop(&mut self) -> Result<T, ReceiverDisconnected> {
        self.pop_inner(true)
    }

    fn collect_new(&mut self, block: bool) {
        let mut add_element = |(priority, item): (Priority, T)| match priority {
            Priority::High => self.high_priority.push(item),
            Priority::Medium => self.medium_priority.push(item),
            Priority::Low => self.low_priority.push(item),
        };

        let mut max_count = Self::TICKET_COUNT;
        if block && self.is_empty() {
            match self.receiver.recv() {
                Ok(e) => add_element(e),
                Err(flume::RecvError::Disconnected) => {
                    self.disconnected = true;
                }
            };
            max_count -= 1;
        }

        loop {
            match self.receiver.try_recv() {
                Ok(e) => {
                    max_count -= 1;
                    add_element(e);
                    if max_count == 0 {
                        break;
                    }
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
    fn pop_inner(&mut self, block: bool) -> Result<Option<T>, ReceiverDisconnected> {
        if self.disconnected {
            return Err(ReceiverDisconnected);
        }

        self.collect_new(block);
        // let total_items =
        //     self.high_priority.len() + self.medium_priority.len() + self.low_priority.len();

        // let value = self.rand.random_range::<u32, _>(0..total_tickets);

        let mut value = self.rand.random_range::<u32, _>(0..100);
        loop {
            match value {
                0..10 if !self.low_priority.is_empty() => ,
                0..10 if => value = 10,
                10..30 if !self.medium_priority.is_empty() => ();
            10..30 => value = 30
                30..100 if !self.high_priority.is_empty() => ();
                30..100 => return Ok(None)
            }
        }



        let mut ticket_count = Self::TICKET_COUNT;

        let high_tickets = Priority::High.ticket_count() * !self.high_priority.is_empty() as u32;
        let medium_tickets =
            Priority::Medium.ticket_count() * !self.medium_priority.is_empty() as u32;
        let low_tickets = Priority::High.ticket_count() * !self.low_priority.is_empty() as u32;
        let total_tickets = high_tickets + medium_tickets + low_tickets;

        let value = self.rand.random_range::<u32, _>(0..total_tickets);
        if value < low_tickets && low_tickets > 0 {
            return self.low_priority.pop().unwrap();
        } else if value < medium_tickets && medium_tickets > 0 {
            return self.medium_priority.pop().unwrap();
        } else if value < high_tickets && high_tickets > 0 {
            return self.high_priority.pop().unwrap();
        }

        return None;


        let high_percentage = Priority::High.ticket_percentage();
        let medium_percentage = Priority::Medium.ticket_percentage() / (1.0f32 - high_percentage);
        let low_percentage = (Priority::Low.ticket_percentage() / (1.0f32 - high_percentage))
            / (1.0f32 - medium_percentage);

        let high_taken = (ticket_count as f32 * high_percentage).ceil() as usize;
        ticket_count -= high_taken;

        let medium_taken = (ticket_count as f32 * medium_percentage).ceil() as usize;
        ticket_count -= medium_taken;

        let low_taken = (ticket_count as f32 * low_percentage).ceil() as usize;

        let high_priority = self
            .high_priority
            .drain(..high_taken.min(self.high_priority.len()));
        let medium_priority = self
            .medium_priority
            .drain(..medium_taken.min(self.medium_priority.len()));
        let low_priority = self
            .low_priority
            .drain(..low_taken.min(self.low_priority.len()));

        Ok(high_priority
            .chain(medium_priority)
            .chain(low_priority)
            .take(100))
    }

    fn is_empty(&self) -> bool {
        self.high_priority.is_empty()
            && self.medium_priority.is_empty()
            && self.low_priority.is_empty()
    }
}

/// If None is returned the sender disconnected
struct Iter<T>(PriorityQueueReceiver<T>);
impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop().ok()
    }
}

/// If None is returned the sender disconnected
struct TryIter<T>(PriorityQueueReceiver<T>);
impl<T> Iterator for TryIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.try_pop().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tasks_get_yielded() {
        let (tx, mut rx) = PriorityQueueReceiver::new();
        tx.send(Priority::Medium, 2);
        tx.send(Priority::High, 3);
        tx.send(Priority::Low, 1);
        tx.send(Priority::Medium, 2);
        tx.send(Priority::High, 3);

        assert_eq!(rx.pop().unwrap(), 3);
        assert_eq!(rx.pop().unwrap(), 3);
        assert_eq!(rx.pop().unwrap(), 2);
        assert_eq!(rx.pop().unwrap(), 2);
        assert_eq!(rx.pop().unwrap(), 1);
    }

    #[test]
    fn new_high_prio_task_get_scheduled_quickly() {
        let (tx, mut rx) = PriorityQueueReceiver::new();
        for _ in 0..100 {
            tx.send(Priority::Low, 1);
        }

        assert_eq!(rx.pop().unwrap(), 1);
        tx.send(Priority::High, 3);
        assert_eq!(rx.pop().unwrap(), 3);
        assert_eq!(rx.pop().unwrap(), 1);
    }
}
