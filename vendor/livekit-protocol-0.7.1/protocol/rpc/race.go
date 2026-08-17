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

package rpc

import (
	"context"
	"sync"
)

type raceResult[T any] struct {
	i   int
	val *T
	err error
}

type Race[T any] struct {
	ctx       context.Context
	cancel    context.CancelFunc
	nextIndex int

	resultLock sync.Mutex
	result     *raceResult[T]
}

// NewRace creates a race to yield the result from one or more candidate
// functions
func NewRace[T any](ctx context.Context) *Race[T] {
	ctx, cancel := context.WithCancel(ctx)
	return &Race[T]{
		ctx:    ctx,
		cancel: cancel,
	}
}

// Go adds a candidate function to the race by running it in a new goroutine
func (r *Race[T]) Go(fn func(ctx context.Context) (*T, error)) {
	i := r.nextIndex
	r.nextIndex++

	go func() {
		val, err := fn(r.ctx)

		r.resultLock.Lock()
		if r.result == nil {
			r.result = &raceResult[T]{i, val, err}
		}
		r.resultLock.Unlock()

		r.cancel()
	}()
}

// Wait awaits the first complete function and returns the index and results
// or -1 if the context is cancelled before any candidate finishes.
func (r *Race[T]) Wait() (int, *T, error) {
	<-r.ctx.Done()

	r.resultLock.Lock()
	res := r.result
	r.resultLock.Unlock()
	if res != nil {
		return res.i, res.val, res.err
	}
	return -1, nil, r.ctx.Err()
}
