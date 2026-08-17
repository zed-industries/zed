// Copyright 2023 LiveKit, Inc.
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

package utils

import (
	"sync"
	"time"
)

type TimeoutQueueItem[T any] struct {
	Value      T
	time       int64
	next, prev *TimeoutQueueItem[T]
	removed    bool
}

type TimeoutQueue[T any] struct {
	mu         sync.Mutex
	head, tail *TimeoutQueueItem[T]
}

func (q *TimeoutQueue[T]) Reset(i *TimeoutQueueItem[T]) bool {
	t := time.Now().UnixNano()

	q.mu.Lock()
	defer q.mu.Unlock()

	if !i.removed {
		i.time = t
		q.remove(i)
		q.push(i)
	}
	return !i.removed
}

func (q *TimeoutQueue[T]) Remove(i *TimeoutQueueItem[T]) {
	q.mu.Lock()
	defer q.mu.Unlock()

	i.removed = true
	q.remove(i)
}

func (q *TimeoutQueue[T]) popBefore(t int64, remove bool) *TimeoutQueueItem[T] {
	q.mu.Lock()
	defer q.mu.Unlock()

	i := q.head
	if i == nil || i.time > t {
		return nil
	}

	i.removed = i.removed || remove
	i.time = 0
	q.remove(i)

	return i
}

func (q *TimeoutQueue[T]) push(i *TimeoutQueueItem[T]) {
	i.prev = q.tail
	i.next = nil

	if i.prev != nil {
		i.prev.next = i
	}

	q.tail = i
	if q.head == nil {
		q.head = i
	}
}

func (q *TimeoutQueue[T]) remove(i *TimeoutQueueItem[T]) {
	if q.tail == i {
		q.tail = i.prev
	}
	if q.head == i {
		q.head = i.next
	}
	if i.prev != nil {
		i.prev.next = i.next
	}
	if i.next != nil {
		i.next.prev = i.prev
	}
	i.next = nil
	i.prev = nil
}

func (q *TimeoutQueue[T]) IterateAfter(timeout time.Duration) TimeoutQueueIterator[T] {
	return newTimeoutQueueIterator(q, timeout, false)
}

func (q *TimeoutQueue[T]) IterateRemoveAfter(timeout time.Duration) TimeoutQueueIterator[T] {
	return newTimeoutQueueIterator(q, timeout, true)
}

type TimeoutQueueIterator[T any] struct {
	q      *TimeoutQueue[T]
	time   int64
	remove bool
	item   *TimeoutQueueItem[T]
}

func newTimeoutQueueIterator[T any](q *TimeoutQueue[T], timeout time.Duration, remove bool) TimeoutQueueIterator[T] {
	return TimeoutQueueIterator[T]{
		q:      q,
		time:   time.Now().Add(-timeout).UnixNano(),
		remove: remove,
	}
}

func (i *TimeoutQueueIterator[T]) Next() bool {
	i.item = i.q.popBefore(i.time, i.remove)
	return i.item != nil
}

func (i *TimeoutQueueIterator[T]) Item() *TimeoutQueueItem[T] {
	return i.item
}
