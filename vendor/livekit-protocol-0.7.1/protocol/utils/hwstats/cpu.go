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
	"time"

	"github.com/frostbyte73/core"
	"github.com/prometheus/procfs"
	"go.uber.org/atomic"

	"github.com/livekit/protocol/logger"
)

// This object returns cgroup quota aware cpu stats. On other systems than Linux,
// it falls back to full system stats

type platformCPUMonitor interface {
	getCPUIdle() (float64, error)
	numCPU() float64
}

type CPUStats struct {
	idleCPUs atomic.Float64
	platform platformCPUMonitor

	idleCallback    func(idle float64)
	procCallback    func(*ProcStats)
	warningThrottle core.Throttle
	closeChan       chan struct{}
}

type ProcStats struct {
	CpuIdle     float64
	Cpu         map[int]float64
	MemoryTotal int
	Memory      map[int]int
}

func NewCPUStats(idleUpdateCallback func(idle float64)) (*CPUStats, error) {
	p, err := newPlatformCPUMonitor()
	if err != nil {
		return nil, err
	}

	c := &CPUStats{
		platform:        p,
		warningThrottle: core.NewThrottle(time.Minute),
		idleCallback:    idleUpdateCallback,
		closeChan:       make(chan struct{}),
	}

	go c.monitorCPULoad()

	return c, nil
}

func NewProcMonitor(onUpdate func(*ProcStats)) (*CPUStats, error) {
	p, err := newPlatformCPUMonitor()
	if err != nil {
		return nil, err
	}

	c := &CPUStats{
		platform:        p,
		warningThrottle: core.NewThrottle(time.Minute),
		procCallback:    onUpdate,
		closeChan:       make(chan struct{}),
	}

	go c.monitorProcesses()

	return c, nil
}

func (c *CPUStats) GetCPUIdle() float64 {
	return c.idleCPUs.Load()
}

func (c *CPUStats) NumCPU() float64 {
	return c.platform.numCPU()
}

func (c *CPUStats) GetCPULoad() float64 {
	var cpuLoad float64
	cpuIdle := c.GetCPUIdle()
	nCPU := c.NumCPU()
	if nCPU > 0 && cpuIdle > 0 {
		cpuLoad = 1 - (cpuIdle / c.NumCPU())
	}
	return cpuLoad
}

func (c *CPUStats) Stop() {
	close(c.closeChan)
}

func (c *CPUStats) monitorCPULoad() {
	ticker := time.NewTicker(time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-c.closeChan:
			return
		case <-ticker.C:
			idle, err := c.platform.getCPUIdle()
			if err != nil {
				logger.Errorw("failed retrieving CPU idle", err)
				continue
			}

			c.idleCPUs.Store(idle)
			idleRatio := idle / c.platform.numCPU()

			if idleRatio < 0.1 {
				c.warningThrottle(func() { logger.Infow("high cpu load", "load", 1-idleRatio) })
			}

			if c.idleCallback != nil {
				c.idleCallback(idle)
			}
		}
	}
}

func (c *CPUStats) monitorProcesses() {
	numCPU := c.platform.numCPU()
	pageSize := getPageSize()

	fs, err := procfs.NewFS(procfs.DefaultMountPoint)
	if err != nil {
		logger.Errorw("failed to read proc fs", err)
		return
	}
	hostCPU, err := getHostCPUCount(fs)
	if err != nil {
		logger.Errorw("failed to read pod cpu count", err)
		return
	}

	self, err := fs.Self()
	if err != nil {
		logger.Errorw("failed to read self", err)
		return
	}

	ticker := time.NewTicker(time.Second)
	defer ticker.Stop()

	var prevTotalTime float64
	var prevStats map[int]procfs.ProcStat
	for {
		select {
		case <-c.closeChan:
			return
		case <-ticker.C:
			procStats := make(map[int]procfs.ProcStat)
			procs, err := procfs.AllProcs()
			if err != nil {
				logger.Errorw("failed to read processes", err)
				continue
			}

			total, err := fs.Stat()
			if err != nil {
				logger.Errorw("failed to read stats", err)
				continue
			}

			ppids := make(map[int]int)
			for _, proc := range procs {
				stat, err := proc.Stat()
				if err != nil {
					continue
				}

				procStats[proc.PID] = stat
				if proc.PID != self.PID {
					ppids[proc.PID] = stat.PPID
				}
			}

			totalHostTime := total.CPUTotal.Idle + total.CPUTotal.Iowait +
				total.CPUTotal.User + total.CPUTotal.Nice + total.CPUTotal.System +
				total.CPUTotal.IRQ + total.CPUTotal.SoftIRQ + total.CPUTotal.Steal

			stats := &ProcStats{
				CpuIdle:     numCPU,
				Cpu:         make(map[int]float64),
				MemoryTotal: 0,
				Memory:      make(map[int]int),
			}

			for pid, stat := range procStats {
				pidForGroup := pid
				for ppids[pidForGroup] != self.PID && ppids[pidForGroup] != 0 {
					// bundle usage up to first child of main go process
					pidForGroup = ppids[pidForGroup]
				}

				memory := stat.RSS * pageSize
				stats.Memory[pidForGroup] += memory
				stats.MemoryTotal += memory

				// process usage as percent of total host cpu
				procPercentUsage := float64(stat.UTime + stat.STime - prevStats[pid].UTime - prevStats[pid].STime)
				if procPercentUsage == 0 {
					continue
				}

				cpu := hostCPU * procPercentUsage / 100 / (totalHostTime - prevTotalTime)
				stats.Cpu[pidForGroup] += cpu
				stats.CpuIdle -= cpu
			}

			c.idleCPUs.Store(stats.CpuIdle)

			if c.procCallback != nil {
				c.procCallback(stats)
			}

			prevTotalTime = totalHostTime
			prevStats = procStats
		}
	}
}
