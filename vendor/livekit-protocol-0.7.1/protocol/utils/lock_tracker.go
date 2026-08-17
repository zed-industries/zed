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
	"math"
	"runtime"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"time"
	"unsafe"

	"golang.org/x/exp/slices"
)

const lockTrackerMaxStackDepth = 16

var lockTrackerEnabled = false
var enableLockTrackerOnce sync.Once
var lowResTime uint32 = uint32(time.Now().Unix())
var enableLockTrackerStackTrace uint32

// EnableLockTracker enable lock tracking background worker. This should be
// called during init
func EnableLockTracker() {
	enableLockTrackerOnce.Do(func() {
		lockTrackerEnabled = true
		go updateLowResTime()
	})
}

func ToggleLockTrackerStackTraces(enable bool) {
	var v uint32
	if enable {
		v = 1
	}
	atomic.StoreUint32(&enableLockTrackerStackTrace, v)
}

func updateLowResTime() {
	ticker := time.NewTicker(time.Second)
	for t := range ticker.C {
		atomic.StoreUint32(&lowResTime, uint32(t.Unix()))
	}
}

var weakRefs []uintptr
var weakRefFree []int
var weakRefLock sync.Mutex

func NumMutexes() int {
	weakRefLock.Lock()
	defer weakRefLock.Unlock()
	return len(weakRefs) - len(weakRefFree)
}

// ScanTrackedLocks check all lock trackers
func ScanTrackedLocks(threshold time.Duration) []*StuckLock {
	minTS := uint32(time.Now().Add(-threshold).Unix())

	weakRefLock.Lock()
	defer weakRefLock.Unlock()
	return scanTrackedLocks(weakRefs, minTS)
}

var nextScanMin int

// ScanTrackedLocksI check lock trackers incrementally n at a time
func ScanTrackedLocksI(threshold time.Duration, n int) []*StuckLock {
	minTS := uint32(time.Now().Add(-threshold).Unix())
	if n <= 0 {
		n = 10000
	}

	weakRefLock.Lock()
	defer weakRefLock.Unlock()

	min := nextScanMin
	max := nextScanMin + n
	if rl := len(weakRefs); rl <= max {
		max = rl
		nextScanMin = 0
	} else {
		nextScanMin = max
	}

	return scanTrackedLocks(weakRefs[min:max], minTS)
}

//go:norace
//go:nosplit
func scanTrackedLocks(refs []uintptr, minTS uint32) []*StuckLock {
	var stuck []*StuckLock
	for _, ref := range refs {
		if ref != 0 {
			t := (*lockTracker)(unsafe.Pointer(ref))
			ts := atomic.LoadUint32(&t.ts)
			waiting := atomic.LoadInt32(&t.waiting)
			if ts <= minTS && waiting > 0 {
				stuck = append(stuck, &StuckLock{
					stack:   slices.Clone(t.stack),
					ts:      ts,
					waiting: waiting,
					held:    atomic.LoadInt32(&t.held),
				})
			}
		}
	}
	return stuck
}

type StuckLock struct {
	stack   []uintptr
	ts      uint32
	waiting int32
	held    int32
}

func (d *StuckLock) FirstLockedAtStack() string {
	fs := runtime.CallersFrames(d.stack)
	var b strings.Builder
	for {
		f, ok := fs.Next()
		if !ok {
			break
		}
		if f.Function != "" {
			b.WriteString(f.Function)
			b.WriteByte('\n')
		}
		if f.File != "" {
			b.WriteByte('\t')
			b.WriteString(f.File)
			b.WriteByte(':')
			b.WriteString(strconv.Itoa(f.Line))
			b.WriteByte('\n')
		}
	}
	return b.String()
}

func (d *StuckLock) HeldSince() time.Time {
	return time.Unix(int64(d.ts), 0)
}

func (d *StuckLock) NumGoroutineHeld() int {
	return int(d.held)
}

func (d *StuckLock) NumGoroutineWaiting() int {
	return int(d.waiting)
}

type lockTracker struct {
	stack   []uintptr
	ts      uint32
	waiting int32
	held    int32
	ref     int
}

func (t *lockTracker) trackWait() {
	if t != nil {
		atomic.AddInt32(&t.waiting, 1)
	}
}

func (t *lockTracker) trackLock() {
	if t != nil {
		atomic.AddInt32(&t.waiting, -1)
		if atomic.AddInt32(&t.held, 1) == 1 {
			atomic.StoreUint32(&t.ts, atomic.LoadUint32(&lowResTime))

			if atomic.LoadUint32(&enableLockTrackerStackTrace) == 1 {
				n := runtime.Callers(2, t.stack[:lockTrackerMaxStackDepth])
				t.stack = t.stack[:n]
			}
		}
	}
}

func (t *lockTracker) trackUnlock() {
	if t != nil {
		if atomic.AddInt32(&t.held, -1) == 0 {
			atomic.StoreUint32(&t.ts, math.MaxUint32)
		}
	}
}

func newLockTracker() *lockTracker {
	t := &lockTracker{
		stack: make([]uintptr, lockTrackerMaxStackDepth),
		ts:    math.MaxUint32,
	}

	runtime.SetFinalizer(t, finalizeLockTracker)

	weakRefLock.Lock()
	defer weakRefLock.Unlock()

	if fi := len(weakRefFree) - 1; fi >= 0 {
		t.ref = weakRefFree[fi]
		weakRefs[t.ref] = uintptr((unsafe.Pointer)(t))
		weakRefFree = weakRefFree[:fi]
	} else {
		t.ref = len(weakRefs)
		weakRefs = append(weakRefs, uintptr((unsafe.Pointer)(t)))
	}

	return t
}

func finalizeLockTracker(t *lockTracker) {
	weakRefLock.Lock()
	defer weakRefLock.Unlock()

	weakRefs[t.ref] = 0
	weakRefFree = append(weakRefFree, t.ref)
}

var waiting lockTracker

//go:linkname sync_runtime_canSpin sync.runtime_canSpin
func sync_runtime_canSpin(int) bool

//go:linkname sync_runtime_doSpin sync.runtime_doSpin
func sync_runtime_doSpin()

func lazyInitLockTracker(p **lockTracker) *lockTracker {
	if !lockTrackerEnabled {
		return nil
	}
	up := (*unsafe.Pointer)(unsafe.Pointer(p))
	iter := 0
	for {
		if t := atomic.LoadPointer(up); t == nil {
			if atomic.CompareAndSwapPointer(up, nil, (unsafe.Pointer)(&waiting)) {
				atomic.StorePointer(up, (unsafe.Pointer)(newLockTracker()))
			}
		} else if t == (unsafe.Pointer)(&waiting) {
			if sync_runtime_canSpin(iter) {
				sync_runtime_doSpin()
				iter++
			} else {
				runtime.Gosched()
			}
		} else {
			return (*lockTracker)(t)
		}
	}
}

type Mutex struct {
	sync.Mutex
	t *lockTracker
}

func (m *Mutex) Lock() {
	t := lazyInitLockTracker(&m.t)
	t.trackWait()
	m.Mutex.Lock()
	t.trackLock()
}

func (m *Mutex) Unlock() {
	t := lazyInitLockTracker(&m.t)
	t.trackUnlock()
	m.Mutex.Unlock()
}

type RWMutex struct {
	sync.RWMutex
	t *lockTracker
}

func (m *RWMutex) Lock() {
	t := lazyInitLockTracker(&m.t)
	t.trackWait()
	m.RWMutex.Lock()
	t.trackLock()
}

func (m *RWMutex) Unlock() {
	t := lazyInitLockTracker(&m.t)
	t.trackUnlock()
	m.RWMutex.Unlock()
}

func (m *RWMutex) RLock() {
	t := lazyInitLockTracker(&m.t)
	t.trackWait()
	m.RWMutex.RLock()
	t.trackLock()
}

func (m *RWMutex) RUnlock() {
	t := lazyInitLockTracker(&m.t)
	t.trackUnlock()
	m.RWMutex.RUnlock()
}
