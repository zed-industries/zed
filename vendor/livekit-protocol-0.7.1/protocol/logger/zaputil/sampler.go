// Copyright (c) 2016-2022 Uber Technologies, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

package zaputil

import (
	"sync/atomic"
	"time"

	"github.com/zeebo/xxh3"
	"go.uber.org/zap/zapcore"
)

// this is lifted almost entirely from zap but with one significant change -
// the state is injected into the sampler core to allow rebuilding the logger
// without resetting the counts.

const (
	minLevel = zapcore.DebugLevel
	maxLevel = zapcore.FatalLevel

	numLevels        = maxLevel - minLevel + 1
	countersPerLevel = 4096
)

type counter struct {
	resetAt atomic.Int64
	counter atomic.Uint64
}

type counters [numLevels][countersPerLevel]counter

func newCounters() *counters {
	return &counters{}
}

func (cs *counters) get(lvl zapcore.Level, key string) *counter {
	i := lvl - minLevel
	j := xxh3.HashString(key) % countersPerLevel
	return &cs[i][j]
}

func (c *counter) IncCheckReset(t time.Time, tick time.Duration) uint64 {
	tn := t.UnixNano()
	resetAfter := c.resetAt.Load()
	if resetAfter > tn {
		return c.counter.Add(1)
	}

	c.counter.Store(1)

	newResetAfter := tn + tick.Nanoseconds()
	if !c.resetAt.CompareAndSwap(resetAfter, newResetAfter) {
		// We raced with another goroutine trying to reset, and it also reset
		// the counter to 1, so we need to reincrement the counter.
		return c.counter.Add(1)
	}

	return 1
}

type Sampler struct {
	counts            *counters
	tick              time.Duration
	first, thereafter uint64
}

func NewSampler(tick time.Duration, first, thereafter int) *Sampler {
	return &Sampler{
		tick:       tick,
		counts:     newCounters(),
		first:      uint64(first),
		thereafter: uint64(thereafter),
	}
}

func NewSamplerCore(core zapcore.Core, s *Sampler) zapcore.Core {
	return &sampler{
		Core: core,
		s:    s,
	}
}

type sampler struct {
	zapcore.Core
	s *Sampler
}

func (s *sampler) Level() zapcore.Level {
	return zapcore.LevelOf(s.Core)
}

func (s *sampler) With(fields []zapcore.Field) zapcore.Core {
	return &sampler{
		Core: s.Core.With(fields),
		s:    s.s,
	}
}

func (s *sampler) Check(ent zapcore.Entry, ce *zapcore.CheckedEntry) *zapcore.CheckedEntry {
	if !s.Enabled(ent.Level) {
		return ce
	}

	if ent.Level >= minLevel && ent.Level <= maxLevel {
		counter := s.s.counts.get(ent.Level, ent.Message)
		n := counter.IncCheckReset(ent.Time, s.s.tick)
		if n > s.s.first && (s.s.thereafter == 0 || (n-s.s.first)%s.s.thereafter != 0) {
			return ce
		}
	}
	return s.Core.Check(ent, ce)
}
