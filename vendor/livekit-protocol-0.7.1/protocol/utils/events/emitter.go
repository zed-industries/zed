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

package events

import (
	"sync"

	"golang.org/x/exp/maps"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/utils/options"
)

const DefaultQueueSize = 16

type Options struct {
	QueueSize int
	Blocking  bool
	Logger    logger.Logger
}

type Option func(o *Options)

func WithQueueSize(size int) Option {
	return func(o *Options) {
		o.QueueSize = size
	}
}

func WithBlocking() Option {
	return func(o *Options) {
		o.Blocking = true
	}
}

func WithLogger(l logger.Logger) Option {
	return func(o *Options) {
		o.Logger = l
	}
}

func WithOptions(src Options) Option {
	return func(o *Options) {
		*o = src
	}
}

func DefaultOptions() Options {
	return Options{
		QueueSize: DefaultQueueSize,
		Logger:    logger.GetLogger(),
	}
}

type Emitter[K comparable, V any] struct {
	options   Options
	mu        sync.RWMutex
	observers map[K]*ObserverList[V]
}

func NewEmitter[K comparable, V any](opts ...Option) *Emitter[K, V] {
	o := DefaultOptions()
	options.Apply(&o, opts)
	return &Emitter[K, V]{
		options:   o,
		observers: map[K]*ObserverList[V]{},
	}
}

func (e *Emitter[K, V]) Emit(k K, v V) {
	e.mu.RLock()
	l, ok := e.observers[k]
	e.mu.RUnlock()
	if !ok {
		return
	}

	l.Emit(v)
}

type eventEmitterObserver[K comparable, V any] struct {
	e *Emitter[K, V]
	k K
	Observer[V]
}

func (o *eventEmitterObserver[K, V]) Stop() {
	o.Observer.Stop()
	o.e.cleanUpObserverList(o.k)
}

func (e *Emitter[K, V]) On(k K, f func(V)) func() {
	e.mu.Lock()
	o := e.getOrCreateObserverList(k).on(f)
	e.mu.Unlock()

	return (&eventEmitterObserver[K, V]{e, k, o}).Stop
}

func (e *Emitter[K, V]) Notify(k K, ch chan V) func() {
	return e.observe(k, ch).Stop
}

func (e *Emitter[K, V]) Observe(k K) Observer[V] {
	return e.observe(k, make(chan V, e.options.QueueSize))
}

func (e *Emitter[K, V]) observe(k K, ch chan V) *eventEmitterObserver[K, V] {
	e.mu.Lock()
	o := e.getOrCreateObserverList(k).observe(ch)
	e.mu.Unlock()

	return &eventEmitterObserver[K, V]{e, k, o}
}

func (e *Emitter[K, V]) getOrCreateObserverList(k K) *ObserverList[V] {
	l, ok := e.observers[k]
	if !ok {
		l = newObserverList[V](e.options)
		e.observers[k] = l
	}
	return l
}

func (e *Emitter[K, V]) ObservedKeys() []K {
	e.mu.Lock()
	defer e.mu.Unlock()
	return maps.Keys(e.observers)
}

func (e *Emitter[K, V]) cleanUpObserverList(k K) {
	e.mu.Lock()
	defer e.mu.Unlock()

	l, ok := e.observers[k]
	if ok && l.Len() == 0 {
		delete(e.observers, k)
	}
}

type ObserverList[V any] struct {
	options   Options
	mu        sync.RWMutex
	observers []*eventObserverListObserver[V]
}

func NewObserverList[V any](opts ...Option) *ObserverList[V] {
	o := DefaultOptions()
	options.Apply(&o, opts)
	return newObserverList[V](o)
}

func newObserverList[V any](options Options) *ObserverList[V] {
	return &ObserverList[V]{
		options: options,
	}
}

func (l *ObserverList[V]) Len() int {
	l.mu.RLock()
	defer l.mu.RUnlock()
	return len(l.observers)
}

type eventObserverListObserver[V any] struct {
	l *ObserverList[V]
	Observer[V]
	index int
}

func (o *eventObserverListObserver[V]) Stop() {
	o.Observer.Stop()
	o.l.stopObserving(o)
}

func (l *ObserverList[V]) On(f func(V)) func() {
	return l.on(f).Stop
}

func (l *ObserverList[V]) on(f func(V)) *eventObserverListObserver[V] {
	o := &eventObserverListObserver[V]{l: l}

	if l.options.Blocking {
		o.Observer = blockingCallback[V](f)
	} else {
		o.Observer = nonblockingCallback[V](f)
	}

	l.startObserving(o)
	return o
}

func (l *ObserverList[V]) Notify(ch chan V) func() {
	return l.observe(ch).Stop
}

func (l *ObserverList[V]) Observe() Observer[V] {
	return l.observe(make(chan V, l.options.QueueSize))
}

func (l *ObserverList[V]) observe(ch chan V) *eventObserverListObserver[V] {
	o := &eventObserverListObserver[V]{l: l}

	if l.options.Blocking {
		o.Observer = &blockingObserver[V]{
			done: make(chan struct{}),
			ch:   ch,
		}
	} else {
		o.Observer = &nonblockingObserver[V]{
			logger: l.options.Logger,
			ch:     ch,
		}
	}

	l.startObserving(o)
	return o
}

func (l *ObserverList[V]) Emit(v V) {
	l.mu.RLock()
	defer l.mu.RUnlock()
	for _, o := range l.observers {
		o.emit(v)
	}
}

func (l *ObserverList[V]) startObserving(o *eventObserverListObserver[V]) {
	l.mu.Lock()
	defer l.mu.Unlock()
	o.index = len(l.observers)
	l.observers = append(l.observers, o)
}

func (l *ObserverList[V]) stopObserving(o *eventObserverListObserver[V]) {
	l.mu.Lock()
	defer l.mu.Unlock()
	l.observers[o.index] = l.observers[len(l.observers)-1]
	l.observers[o.index].index = o.index
	l.observers[len(l.observers)-1] = nil
	l.observers = l.observers[:len(l.observers)-1]
}

type Observer[V any] interface {
	emit(v V)
	Stop()
	Events() <-chan V
}

type eventObserver[V any] struct {
	stopFunc func()
	Observer[V]
}

func (o *eventObserver[V]) Stop() {
	o.stopFunc()
	o.Observer.Stop()
}

func NewObserver[V any](stopFunc func()) (Observer[V], func(v V)) {
	o := &nonblockingObserver[V]{
		logger: logger.GetLogger(),
		ch:     make(chan V, DefaultQueueSize),
	}
	return &eventObserver[V]{stopFunc, o}, o.emit
}

type nonblockingObserver[V any] struct {
	logger logger.Logger
	ch     chan V
}

func (o *nonblockingObserver[V]) emit(v V) {
	select {
	case o.ch <- v:
	default:
		o.logger.Warnw("could not add event to observer queue", nil)
	}
}

func (o *nonblockingObserver[V]) Stop() {}

func (o *nonblockingObserver[V]) Events() <-chan V {
	return o.ch
}

type blockingObserver[V any] struct {
	done chan struct{}
	ch   chan V
}

func (o *blockingObserver[V]) emit(v V) {
	select {
	case o.ch <- v:
	case <-o.done:
	}
}

func (o *blockingObserver[V]) Stop() {
	close(o.done)
}

func (o *blockingObserver[V]) Events() <-chan V {
	return o.ch
}

type nonblockingCallback[V any] func(V)

func (o nonblockingCallback[V]) emit(v V) {
	go o(v)
}

func (o nonblockingCallback[V]) Stop() {}

func (o nonblockingCallback[V]) Events() <-chan V { return nil }

type blockingCallback[V any] func(V)

func (o blockingCallback[V]) emit(v V) {
	o(v)
}

func (o blockingCallback[V]) Stop() {}

func (o blockingCallback[V]) Events() <-chan V { return nil }
