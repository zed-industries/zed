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
	"runtime"
	"sync"

	"go.uber.org/atomic"
)

// ParallelExec will executes the given function with each element of vals, if len(vals) >= parallelThreshold,
// will execute them in parallel, with the given step size. So fn must be thread-safe.
func ParallelExec[T any](vals []T, parallelThreshold, step uint64, fn func(T)) {
	if uint64(len(vals)) < parallelThreshold {
		for _, v := range vals {
			fn(v)
		}
		return
	}

	// parallel - enables much more efficient multi-core utilization
	start := atomic.NewUint64(0)
	end := uint64(len(vals))

	var wg sync.WaitGroup
	numCPU := runtime.NumCPU()
	if numCPU > len(vals) {
		numCPU = len(vals)
	}
	wg.Add(numCPU)
	for p := 0; p < numCPU; p++ {
		go func() {
			defer wg.Done()
			for {
				n := start.Add(step)
				if n >= end+step {
					return
				}

				for i := n - step; i < n && i < end; i++ {
					fn(vals[i])
				}
			}
		}()
	}
	wg.Wait()
}
