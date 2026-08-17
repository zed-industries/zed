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
	"sort"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestEventEmitter(t *testing.T) {
	t.Run("emitter", func(t *testing.T) {
		emitter := NewEmitter[string, int]()
		ao0 := emitter.Observe("a")
		ao1 := emitter.Observe("a")
		bo := emitter.Observe("b")

		emitter.Emit("a", 1)
		emitter.Emit("b", 2)
		require.Equal(t, 1, <-ao0.Events())
		require.Equal(t, 1, <-ao1.Events())
		require.Equal(t, 2, <-bo.Events())

		ao1.Stop()
		emitter.Emit("a", 3)
		require.Equal(t, 3, <-ao0.Events())
		select {
		case <-ao1.Events():
			require.Fail(t, "expected no event from stopped observer")
		default:
		}

		ao0.Stop()

		keys := emitter.ObservedKeys()
		sort.Strings(keys)
		require.Equal(t, []string{"b"}, keys)
	})

	t.Run("observer", func(t *testing.T) {
		var closeCalled bool
		o, emit := NewObserver[int](func() { closeCalled = true })

		emit(1)
		require.Equal(t, 1, <-o.Events())

		o.Stop()
		require.True(t, closeCalled)
	})

	t.Run("notify", func(t *testing.T) {
		emitter := NewEmitter[string, int]()

		as := make(chan int, 1)
		stop := emitter.Notify("a", as)

		emitter.Emit("a", 0)
		select {
		case v := <-as:
			require.Equal(t, 0, v)
		default:
			require.FailNow(t, "expected event in channel")
		}

		stop()

		emitter.Emit("a", 0)
		select {
		case <-as:
			require.FailNow(t, "expected no event in channel after stop")
		default:
		}
	})

	t.Run("on", func(t *testing.T) {
		emitter := NewEmitter[string, int]()

		as := make(chan int, 1)
		stop := emitter.On("a", func(i int) {
			as <- i
		})

		emitter.Emit("a", 0)
		select {
		case v := <-as:
			require.Equal(t, 0, v)
		case <-time.After(100 * time.Millisecond):
			require.FailNow(t, "expected event in channel")
		}

		stop()

		emitter.Emit("a", 0)
		select {
		case <-as:
			require.FailNow(t, "expected no event in channel after stop")
		case <-time.After(100 * time.Millisecond):
		}
	})

	t.Run("stop unblocks blocking observers", func(t *testing.T) {
		observer, emit := NewObserver[int](func() {})

		list := NewObserverList[int](WithBlocking())

		emitter := NewEmitter[int, int](WithBlocking())

		cases := []struct {
			label    string
			emit     func()
			observer Observer[int]
		}{
			{
				label:    "observer",
				emit:     func() { emit(0) },
				observer: observer,
			},
			{
				label:    "list",
				emit:     func() { list.Emit(0) },
				observer: list.Observe(),
			},
			{
				label:    "emitter",
				emit:     func() { emitter.Emit(0, 0) },
				observer: emitter.Observe(0),
			},
		}

		for _, c := range cases {
			t.Run(c.label, func(t *testing.T) {
				emitDone := make(chan struct{})
				stopDone := make(chan struct{})
				ready := make(chan struct{})

				go func() {
					for i := 0; i < DefaultQueueSize; i++ {
						c.emit()
					}
					close(ready)
					c.emit()
					close(emitDone)
				}()

				go func() {
					<-ready
					time.Sleep(100 * time.Millisecond)
					c.observer.Stop()
					close(stopDone)
				}()

				select {
				case <-emitDone:
				case <-time.After(time.Second):
					require.FailNow(t, "timeout waiting for emit to unblock")
				}

				select {
				case <-stopDone:
				case <-time.After(time.Second):
					require.FailNow(t, "timeout waiting for stop to unblock")
				}
			})
		}
	})
}

func BenchmarkEventEmitter(b *testing.B) {
	e := NewEmitter[int, int]()
	for i := 0; i < b.N; i++ {
		o := e.Observe(i)
		e.Emit(i, i)
		<-o.Events()
		o.Stop()
	}
}

func BenchmarkEventObserverList(b *testing.B) {
	l := NewObserverList[int]()
	for i := 0; i < b.N; i++ {
		o := l.Observe()
		l.Emit(i)
		<-o.Events()
		o.Stop()
	}
}
