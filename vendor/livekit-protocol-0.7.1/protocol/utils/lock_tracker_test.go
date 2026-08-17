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

package utils_test

import (
	"fmt"
	"runtime"
	"sync"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/utils"
)

func init() {
	utils.EnableLockTracker()
}

func cleanupTest() {
	runtime.GC()
	time.Sleep(time.Millisecond)
}

func noop() {}

func TestScanTrackedLocks(t *testing.T) {
	t.Cleanup(cleanupTest)
	require.Nil(t, utils.ScanTrackedLocks(time.Millisecond))

	ms := make([]*utils.Mutex, 100)
	for i := range ms {
		m := &utils.Mutex{}
		m.Lock()
		noop()
		m.Unlock()
		ms[i] = m
	}

	go func() {
		ms[50].Lock()
		ms[50].Lock()
	}()

	time.Sleep(100 * time.Millisecond)
	require.NotNil(t, utils.ScanTrackedLocks(time.Millisecond))

	ms[50].Unlock()
}

func TestFirstLockStackTrace(t *testing.T) {
	t.Cleanup(cleanupTest)
	require.Nil(t, utils.ScanTrackedLocks(time.Millisecond))

	utils.ToggleLockTrackerStackTraces(true)
	defer utils.ToggleLockTrackerStackTraces(false)

	m := &utils.Mutex{}

	var deepLock func(n int)
	deepLock = func(n int) {
		if n > 0 {
			deepLock(n - 1)
		} else {
			m.Lock()
		}
	}

	go func() {
		deepLock(5)
		m.Lock()
	}()

	time.Sleep(100 * time.Millisecond)
	locks := utils.ScanTrackedLocks(time.Millisecond)
	require.NotNil(t, locks)
	require.NotEqual(t, "", locks[0].FirstLockedAtStack())

	m.Unlock()
}

func TestMutexFinalizer(t *testing.T) {
	cleanupTest()
	require.Equal(t, 0, utils.NumMutexes())

	{
		m := &utils.Mutex{}
		m.Lock()
		go func() {
			m.Unlock()
		}()
		require.Equal(t, 1, utils.NumMutexes())
	}

	for range 100 {
		cleanupTest()
		if utils.NumMutexes() == 0 {
			break
		}
	}

	require.Equal(t, 0, utils.NumMutexes())
}

func TestEmbeddedMutex(t *testing.T) {
	t.Cleanup(cleanupTest)

	foo := struct{ m utils.Mutex }{}
	foo.m.Lock()
	noop()
	foo.m.Unlock()

	bar := struct{ utils.Mutex }{}
	bar.Lock()
	noop()
	bar.Unlock()
}

func TestContestedGlobalLock(t *testing.T) {
	t.Cleanup(cleanupTest)

	ms := make([]*utils.Mutex, 100)
	for i := range ms {
		m := &utils.Mutex{}
		m.Lock()
		noop()
		m.Unlock()
		ms[i] = m
	}

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		for i := 0; i < 100; i++ {
			wg.Add(1)
			go func() {
				utils.ScanTrackedLocks(time.Minute)
				wg.Done()
			}()
		}
		wg.Done()
	}()

	go func() {
		for i := 0; i < 100; i++ {
			var m utils.Mutex
			wg.Add(3)
			for i := 0; i < 3; i++ {
				go func() {
					m.Lock()
					noop()
					m.Unlock()
					wg.Done()
				}()
			}
		}
		wg.Done()
	}()

	wg.Wait()
}

func TestInitRace(t *testing.T) {
	t.Cleanup(cleanupTest)

	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		var m utils.Mutex
		wg.Add(3)
		done := make(chan struct{})
		for i := 0; i < 3; i++ {
			go func() {
				<-done
				m.Lock()
				noop()
				m.Unlock()
				wg.Done()
			}()
		}
		close(done)
		runtime.Gosched()
	}

	wg.Wait()
}

func BenchmarkLockTracker(b *testing.B) {
	b.Run("wrapped mutex", func(b *testing.B) {
		var m utils.Mutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped rwmutex", func(b *testing.B) {
		var m utils.RWMutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped mutex init", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m utils.Mutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped rwmutex init", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m utils.RWMutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})

	utils.ToggleLockTrackerStackTraces(true)
	b.Run("wrapped mutex + stack trace", func(b *testing.B) {
		var m utils.Mutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped rwmutex + stack trace", func(b *testing.B) {
		var m utils.RWMutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped mutex init + stack trace", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m utils.Mutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("wrapped rwmutex init + stack trace", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m utils.RWMutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	utils.ToggleLockTrackerStackTraces(false)

	b.Run("native mutex", func(b *testing.B) {
		var m sync.Mutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("native rwmutex", func(b *testing.B) {
		var m sync.RWMutex
		for i := 0; i < b.N; i++ {
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("native mutex init", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m sync.Mutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})
	b.Run("native rwmutex init", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			var m sync.RWMutex
			m.Lock()
			noop()
			m.Unlock()
		}
	})
}

func BenchmarkGetBlocked(b *testing.B) {
	for n := 100; n <= 1000000; n *= 100 {
		n := n
		b.Run(fmt.Sprintf("serial/%d", n), func(b *testing.B) {
			cleanupTest()

			ms := make([]*utils.Mutex, n)
			for i := range ms {
				m := &utils.Mutex{}
				m.Lock()
				noop()
				m.Unlock()
				ms[i] = m
			}

			b.ResetTimer()

			for i := 0; i < b.N; i++ {
				utils.ScanTrackedLocks(time.Minute)
			}
		})
	}
}
