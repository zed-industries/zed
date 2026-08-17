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

package hwstats

import (
	"runtime"

	"github.com/mackerelio/go-osstat/cpu"
)

type osStatCPUMonitor struct {
	prev *cpu.Stats
}

func newOSStatCPUMonitor() (*osStatCPUMonitor, error) {
	stats, err := cpu.Get()
	if err != nil {
		return nil, err
	}

	return &osStatCPUMonitor{
		prev: stats,
	}, nil
}

func (p *osStatCPUMonitor) getCPUIdle() (float64, error) {
	next, err := cpu.Get()
	if err != nil {
		return 0, err
	}
	idleRatio := float64(next.Idle-p.prev.Idle) / float64(next.Total-p.prev.Total)
	p.prev = next

	return p.numCPU() * idleRatio, nil
}

func (p *osStatCPUMonitor) numCPU() float64 {
	return float64(runtime.NumCPU())
}
