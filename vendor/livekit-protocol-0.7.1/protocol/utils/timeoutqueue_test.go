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
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func debugTimeoutQueueItems[T any](q *TimeoutQueue[T]) []T {
	var items []T
	for i := q.head; i != nil; i = i.next {
		items = append(items, i.Value)
	}
	return items
}

func TestTimeoutQueue(t *testing.T) {
	t.Run("queue order matches insert order", func(t *testing.T) {
		t.Parallel()
		var q TimeoutQueue[int]
		for i := 0; i < 5; i++ {
			q.Reset(&TimeoutQueueItem[int]{Value: i})
		}
		require.EqualValues(t, []int{0, 1, 2, 3, 4}, debugTimeoutQueueItems(&q))
	})

	t.Run("reset items do not affect order of other entries", func(t *testing.T) {
		t.Parallel()
		var q TimeoutQueue[int]
		var its []*TimeoutQueueItem[int]
		for i := 0; i < 5; i++ {
			it := &TimeoutQueueItem[int]{Value: i}
			its = append(its, it)
			q.Reset(it)
		}

		q.Reset(its[1])
		require.EqualValues(t, []int{0, 2, 3, 4, 1}, debugTimeoutQueueItems(&q))
		q.Reset(its[3])
		require.EqualValues(t, []int{0, 2, 4, 1, 3}, debugTimeoutQueueItems(&q))
		q.Reset(its[2])
		require.EqualValues(t, []int{0, 4, 1, 3, 2}, debugTimeoutQueueItems(&q))
	})

	t.Run("popBefore removes items", func(t *testing.T) {
		t.Parallel()
		var q TimeoutQueue[int]
		for i := 0; i < 5; i++ {
			it := &TimeoutQueueItem[int]{Value: i}
			q.Reset(it)
		}

		ts := time.Now().UnixNano()
		q.popBefore(ts, true)
		require.EqualValues(t, []int{1, 2, 3, 4}, debugTimeoutQueueItems(&q))
		q.popBefore(ts, true)
		require.EqualValues(t, []int{2, 3, 4}, debugTimeoutQueueItems(&q))
	})
}
