// Copyright 2024 LiveKit, Inc.
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

package configutil

import (
	"time"

	"go.uber.org/atomic"
)

type ConfigObserver[T any] interface {
	Observe(cb func(*T)) func()
	Load() *T
}

func NewAtomicBool[C any](config ConfigObserver[C], accessor func(*C) bool) *atomic.Bool {
	return newAtomic(atomic.NewBool, config, accessor)
}

func NewAtomicDuration[C any](config ConfigObserver[C], accessor func(*C) time.Duration) *atomic.Duration {
	return newAtomic(atomic.NewDuration, config, accessor)
}

func NewAtomicFloat32[C any](config ConfigObserver[C], accessor func(*C) float32) *atomic.Float32 {
	return newAtomic(atomic.NewFloat32, config, accessor)
}

func NewAtomicFloat64[C any](config ConfigObserver[C], accessor func(*C) float64) *atomic.Float64 {
	return newAtomic(atomic.NewFloat64, config, accessor)
}

func NewAtomicInt32[C any](config ConfigObserver[C], accessor func(*C) int32) *atomic.Int32 {
	return newAtomic(atomic.NewInt32, config, accessor)
}

func NewAtomicInt64[C any](config ConfigObserver[C], accessor func(*C) int64) *atomic.Int64 {
	return newAtomic(atomic.NewInt64, config, accessor)
}

func NewAtomicString[C any](config ConfigObserver[C], accessor func(*C) string) *atomic.String {
	return newAtomic(atomic.NewString, config, accessor)
}

func NewAtomicTime[C any](config ConfigObserver[C], accessor func(*C) time.Time) *atomic.Time {
	return newAtomic(atomic.NewTime, config, accessor)
}

func NewAtomicUint32[C any](config ConfigObserver[C], accessor func(*C) uint32) *atomic.Uint32 {
	return newAtomic(atomic.NewUint32, config, accessor)
}

func NewAtomicUint64[C any](config ConfigObserver[C], accessor func(*C) uint64) *atomic.Uint64 {
	return newAtomic(atomic.NewUint64, config, accessor)
}

func NewAtomicPointer[T, C any](config ConfigObserver[C], accessor func(*C) *T) *atomic.Pointer[T] {
	return newAtomic(atomic.NewPointer[T], config, accessor)
}

type AtomicValue[T any] struct {
	v atomic.Value
}

func newAtomicValue[T any](_ T) *AtomicValue[T] {
	return &AtomicValue[T]{}
}

func (a *AtomicValue[T]) Store(v T) {
	a.v.Store(v)
}

func (a *AtomicValue[T]) Load() T {
	return a.v.Load().(T)
}

func NewAtomicValue[T, C any](config ConfigObserver[C], accessor func(*C) T) *AtomicValue[T] {
	return newAtomic(newAtomicValue[T], config, accessor)
}

func newAtomic[T, C any, A interface{ Store(T) }](ctor func(T) A, config ConfigObserver[C], accessor func(*C) T) A {
	var zero T
	v := ctor(zero)
	config.Observe(func(cc *C) { v.Store(accessor(cc)) })
	v.Store(accessor(config.Load()))
	return v
}
