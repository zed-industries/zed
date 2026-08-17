// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::VecDeque;

#[derive(Debug)]
pub struct TxQueue<T> {
    inner: VecDeque<T>,
    buffered_size: usize,
}

impl<T: TxQueueItem> TxQueue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self { inner: VecDeque::new(), buffered_size: 0 }
    }

    /// Number of elements in the queue.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Total size in bytes of all items currently in the queue.
    pub fn buffered_size(&self) -> usize {
        self.buffered_size
    }

    /// Provides a reference to the front element, or `None if the queue is empty.
    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    /// Appends an item to the back of the queue.
    pub fn enqueue(&mut self, item: T) {
        let size = item.buffered_size();
        self.inner.push_back(item);
        self.buffered_size += size;
    }

    /// Removes the first item and returns it, or `None` if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        let item = self.inner.pop_front()?;
        let size = item.buffered_size();
        self.buffered_size -= size;
        return Some(item);
    }

    /// Dequeue and discard items until the buffered size is less than or
    /// equal to the given target.
    pub fn trim(&mut self, target_buffer_size: usize) {
        while self.buffered_size > target_buffer_size {
            _ = self.dequeue()
        }
    }
}

/// Item in a [`TxQueue`].
pub trait TxQueueItem {
    /// Amount in bytes this item adds to the [`TxQueue`]'s buffered size.
    fn buffered_size(&self) -> usize;
}

impl TxQueueItem for Vec<u8> {
    fn buffered_size(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::TxQueue;

    #[test]
    fn test_buffered_size() {
        let mut queue = TxQueue::new();

        queue.enqueue(vec![0xFF, 0xFA]);
        assert_eq!(queue.buffered_size(), 2);

        queue.enqueue(vec![0x0F, 0xFC, 0xAF]);
        assert_eq!(queue.buffered_size(), 5);

        assert_eq!(queue.dequeue(), Some(vec![0xFF, 0xFA]));
        assert_eq!(queue.buffered_size, 3);

        assert_eq!(queue.dequeue(), Some(vec![0x0F, 0xFC, 0xAF]));
        assert_eq!(queue.buffered_size, 0);

        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_trim() {
        let mut queue = TxQueue::new();
        queue.enqueue(vec![0xFF, 0xFA]);
        queue.enqueue(vec![0x0F, 0xFC, 0xAF]);
        queue.enqueue(vec![0xAA]);

        queue.trim(1);
        assert_eq!(queue.buffered_size(), 1);
    }
}
