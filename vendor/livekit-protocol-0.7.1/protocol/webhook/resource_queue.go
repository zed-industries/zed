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

package webhook

import (
	"context"
	"errors"
	"sync"
	"time"

	"github.com/gammazero/deque"

	"github.com/livekit/protocol/livekit"
)

var (
	errQueueFull   = errors.New("queue is full")
	errQueueClosed = errors.New("queue is closed")
)

type item struct {
	ctx      context.Context
	queuedAt time.Time
	event    *livekit.WebhookEvent
	params   *ResourceURLNotifierParams
}

type resourceQueueParams struct {
	MaxDepth int

	Poster poster
}

type resourceQueue struct {
	params resourceQueueParams

	mu    sync.Mutex
	items deque.Deque[*item]
	cond  *sync.Cond

	closed bool
	drain  bool
}

func newResourceQueue(params resourceQueueParams) *resourceQueue {
	r := &resourceQueue{
		params: params,
	}
	r.items.SetBaseCap(int(min(params.MaxDepth, 16)))
	r.cond = sync.NewCond(&r.mu)

	go r.worker()
	return r
}

func (r *resourceQueue) Stop(force bool) {
	r.mu.Lock()
	defer r.mu.Unlock()

	if !r.closed {
		r.closed = true
		r.drain = !force

		r.cond.Broadcast()
	}
}

func (r *resourceQueue) Enqueue(ctx context.Context, whEvent *livekit.WebhookEvent, params *ResourceURLNotifierParams) (int, error) {
	return r.EnqueueAt(ctx, time.Now(), whEvent, params)
}

func (r *resourceQueue) EnqueueAt(ctx context.Context, at time.Time, whEvent *livekit.WebhookEvent, params *ResourceURLNotifierParams) (int, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.closed {
		return r.items.Len(), errQueueClosed
	}

	if r.items.Len() >= r.params.MaxDepth {
		return r.items.Len(), errQueueFull
	}

	r.items.PushBack(&item{ctx, at, whEvent, params})
	r.cond.Broadcast()
	return r.items.Len(), nil
}

func (r *resourceQueue) worker() {
	for {
		r.mu.Lock()
		for {
			if r.closed && (!r.drain || r.items.Len() == 0) {
				r.mu.Unlock()
				return
			}

			if r.items.Len() != 0 {
				break
			}
			r.cond.Wait()
		}

		item := r.items.PopFront()
		qLen := r.items.Len()
		r.mu.Unlock()

		r.params.Poster.Process(item.ctx, item.queuedAt, item.event, item.params, qLen)
	}
}
