// Copyright (c) 2016,2020 Uber Technologies, Inc.
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
//
// SOURCE: https://github.com/uber-go/ratelimit/blob/main/ratelimit_test.go
// EDIT: slight modification to allow setting rate limit on the fly
// SCOPE: LeakyBucket
package utils

import (
	"sync"
	"testing"
	"time"

	"go.uber.org/atomic"

	"github.com/benbjohnson/clock"
	"github.com/stretchr/testify/require"
)

const UnstableTest = "UNSTABLE TEST"

// options from upstream, but stripped these
// Note: This file is inspired by:
// https://github.com/prashantv/go-bench/blob/master/ratelimit

// Limiter is used to rate-limit some process, possibly across goroutines.
// The process is expected to call Take() before every iteration, which
// may block to throttle the goroutine.
type Limiter interface {
	// Take should block to make sure that the RPS is met.
	Take() time.Time
}

// config configures a limiter.
type config struct {
	clock Clock
	slack int
	per   time.Duration
}

// buildConfig combines defaults with options.
func buildConfig(opts []Option) config {
	c := config{
		clock: clock.New(),
		slack: 10,
		per:   time.Second,
	}

	for _, opt := range opts {
		opt.apply(&c)
	}
	return c
}

// Option configures a Limiter.
type Option interface {
	apply(*config)
}

type clockOption struct {
	clock Clock
}

func (o clockOption) apply(c *config) {
	c.clock = o.clock
}

// WithClock returns an option for ratelimit.New that provides an alternate
// Clock implementation, typically a mock Clock for testing.
func WithClock(clock Clock) Option {
	return clockOption{clock: clock}
}

type slackOption int

func (o slackOption) apply(c *config) {
	c.slack = int(o)
}

// WithoutSlack configures the limiter to be strict and not to accumulate
// previously "unspent" requests for future bursts of traffic.
var WithoutSlack Option = slackOption(0)

// WithSlack configures custom slack.
// Slack allows the limiter to accumulate "unspent" requests
// for future bursts of traffic.
func WithSlack(slack int) Option {
	return slackOption(slack)
}

type perOption time.Duration

func (p perOption) apply(c *config) {
	c.per = time.Duration(p)
}

// Per allows configuring limits for different time windows.
//
// The default window is one second, so New(100) produces a one hundred per
// second (100 Hz) rate limiter.
//
// New(2, Per(60*time.Second)) creates a 2 per minute rate limiter.
func Per(per time.Duration) Option {
	return perOption(per)
}

type testRunner interface {
	// createLimiter builds a limiter with given options.
	createLimiter(int, ...Option) Limiter
	// takeOnceAfter attempts to Take at a specific time.
	takeOnceAfter(time.Duration, Limiter)
	// startTaking tries to Take() on passed in limiters in a loop/goroutine.
	startTaking(rls ...Limiter)
	// assertCountAt asserts the limiters have Taken() a number of times at the given time.
	// It's a thin wrapper around afterFunc to reduce boilerplate code.
	assertCountAt(d time.Duration, count int)
	assertCountAtWithNoise(d time.Duration, count int, noise int)
	// afterFunc executes a func at a given time.
	// not using clock.AfterFunc because andres-erbsen/clock misses a nap there.
	afterFunc(d time.Duration, fn func())
	// some tests want raw access to the clock.
	getClock() *clock.Mock
}

type runnerImpl struct {
	t *testing.T

	clock       *clock.Mock
	constructor func(int, ...Option) Limiter
	count       atomic.Int32
	// maxDuration is the time we need to move into the future for a test.
	// It's populated automatically based on assertCountAt/afterFunc.
	maxDuration time.Duration
	doneCh      chan struct{}
	wg          sync.WaitGroup
}

func runTest(t *testing.T, fn func(testRunner)) {
	impls := []struct {
		name        string
		constructor func(int, ...Option) Limiter
	}{
		{
			name: "mutex",
			constructor: func(rate int, opts ...Option) Limiter {
				config := buildConfig(opts)
				perRequest := config.per / time.Duration(rate)
				cfg := leakyBucketConfig{
					perRequest: perRequest,
					maxSlack:   -1 * time.Duration(config.slack) * perRequest,
				}
				l := &LeakyBucket{
					clock: config.clock,
				}
				l.cfg.Store(&cfg)
				return l
			},
		},
	}

	for _, tt := range impls {
		t.Run(tt.name, func(t *testing.T) {
			// Set a non-default time.Time since some limiters (int64 in particular) use
			// the default value as "non-initialized" state.
			clockMock := clock.NewMock()
			clockMock.Set(time.Now())
			r := runnerImpl{
				t:           t,
				clock:       clockMock,
				constructor: tt.constructor,
				doneCh:      make(chan struct{}),
			}
			defer close(r.doneCh)
			defer r.wg.Wait()

			fn(&r)

			// it's possible that there are some goroutines still waiting
			// in taking the bandwidth. We need to keep moving the clock forward
			// until all goroutines are finished
			go func() {
				ticker := time.NewTicker(5 * time.Millisecond)
				defer ticker.Stop()

				for {
					select {
					case <-ticker.C:
						r.clock.Add(r.maxDuration)
					case <-r.doneCh:
					}
				}
			}()
		})
	}
}

// createLimiter builds a limiter with given options.
func (r *runnerImpl) createLimiter(rate int, opts ...Option) Limiter {
	opts = append(opts, WithClock(r.clock))
	return r.constructor(rate, opts...)
}

func (r *runnerImpl) getClock() *clock.Mock {
	return r.clock
}

// startTaking tries to Take() on passed in limiters in a loop/goroutine.
func (r *runnerImpl) startTaking(rls ...Limiter) {
	r.goWait(func() {
		for {
			for _, rl := range rls {
				rl.Take()
			}
			r.count.Inc()
			select {
			case <-r.doneCh:
				return
			default:
			}
		}
	})
}

// takeOnceAfter attempts to Take at a specific time.
func (r *runnerImpl) takeOnceAfter(d time.Duration, rl Limiter) {
	r.wg.Add(1)
	r.afterFunc(d, func() {
		rl.Take()
		r.count.Inc()
		r.wg.Done()
	})
}

// assertCountAt asserts the limiters have Taken() a number of times at a given time.
func (r *runnerImpl) assertCountAt(d time.Duration, count int) {
	r.wg.Add(1)
	r.afterFunc(d, func() {
		defer r.wg.Done()
		require.Equal(r.t, int32(count), r.count.Load(), "count not as expected")
	})
}

// assertCountAtWithNoise like assertCountAt but also considers possible noise in CI
func (r *runnerImpl) assertCountAtWithNoise(d time.Duration, count int, noise int) {
	r.wg.Add(1)
	r.afterFunc(d, func() {
		defer r.wg.Done()
		require.InDelta(r.t, count, int(r.count.Load()), float64(noise),
			"expected count to be within noise tolerance")
	})
}

// afterFunc executes a func at a given time.
func (r *runnerImpl) afterFunc(d time.Duration, fn func()) {
	if d > r.maxDuration {
		r.maxDuration = d
	}

	r.goWait(func() {
		select {
		case <-r.doneCh:
			return
		case <-r.clock.After(d):
		}
		fn()
	})
}

// goWait runs a function in a goroutine and makes sure the goroutine was scheduled.
func (r *runnerImpl) goWait(fn func()) {
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		wg.Done()
		fn()
	}()
	wg.Wait()
}

func TestRateLimiter(t *testing.T) {
	runTest(t, func(r testRunner) {
		rl := r.createLimiter(100, WithoutSlack)

		// Create copious counts concurrently.
		r.startTaking(rl)
		r.startTaking(rl)
		r.startTaking(rl)
		r.startTaking(rl)

		r.assertCountAtWithNoise(1*time.Second, 100, 2)
		r.assertCountAtWithNoise(2*time.Second, 200, 2)
		r.assertCountAtWithNoise(3*time.Second, 300, 2)
	})
}

func TestDelayedRateLimiter(t *testing.T) {
	t.Skip(UnstableTest)
	runTest(t, func(r testRunner) {
		slow := r.createLimiter(10, WithoutSlack)
		fast := r.createLimiter(100, WithoutSlack)

		r.startTaking(slow, fast)

		r.afterFunc(20*time.Second, func() {
			r.startTaking(fast)
			r.startTaking(fast)
			r.startTaking(fast)
			r.startTaking(fast)
		})

		r.assertCountAt(30*time.Second, 1200)
	})
}

func TestPer(t *testing.T) {
	runTest(t, func(r testRunner) {
		rl := r.createLimiter(7, WithoutSlack, Per(time.Minute))

		r.startTaking(rl)
		r.startTaking(rl)

		r.assertCountAt(1*time.Second, 1)
		r.assertCountAt(1*time.Minute, 8)
		r.assertCountAt(2*time.Minute, 15)
	})
}

// TestInitial verifies that the initial sequence is scheduled as expected.
func TestInitial(t *testing.T) {
	tests := []struct {
		msg  string
		opts []Option
	}{
		{
			msg: "With Slack",
		},
		{
			msg:  "Without Slack",
			opts: []Option{WithoutSlack},
		},
	}

	for _, tt := range tests {
		t.Run(tt.msg, func(t *testing.T) {
			runTest(t, func(r testRunner) {
				perRequest := 100 * time.Millisecond
				rl := r.createLimiter(10, tt.opts...)

				var (
					clk  = r.getClock()
					prev = clk.Now()

					results = make(chan time.Time, 3)
					have    []time.Duration
				)

				results <- rl.Take()
				clk.Add(perRequest)

				results <- rl.Take()
				clk.Add(perRequest)

				results <- rl.Take()
				clk.Add(perRequest)

				for i := 0; i < 3; i++ {
					ts := <-results
					have = append(have, ts.Sub(prev))
					prev = ts
				}

				require.Equal(t,
					[]time.Duration{
						0,
						perRequest,
						perRequest,
					},
					have,
					"bad timestamps for inital takes",
				)
			})
		})
	}
}

func TestMaxSlack(t *testing.T) {
	runTest(t, func(r testRunner) {
		clock := r.getClock()
		rl := r.createLimiter(1, WithSlack(1))
		rl.Take()
		clock.Add(time.Second)
		rl.Take()
		clock.Add(time.Second)
		rl.Take()

		doneCh := make(chan struct{})
		go func() {
			rl.Take()
			close(doneCh)
		}()

		select {
		case <-doneCh:
			require.Fail(t, "expect rate limiter to be waiting")
		case <-time.After(time.Millisecond):
			// clean up ratelimiter waiting for take
			clock.Add(time.Second)
		}
	})
}

func TestSlack(t *testing.T) {
	t.Skip(UnstableTest)

	// To simulate slack, we combine two limiters.
	// - First, we start a single goroutine with both of them,
	//   during this time the slow limiter will dominate,
	//   and allow the fast limiter to accumulate slack.
	// - After 2 seconds, we start another goroutine with
	//   only the faster limiter. This will allow it to max out,
	//   and consume all the slack.
	// - After 3 seconds, we look at the final result, and we expect,
	//   a sum of:
	//   - slower limiter running for 3 seconds
	//   - faster limiter running for 1 second
	//   - slack accumulated by the faster limiter during the two seconds.
	//     it was blocked by slower limiter.
	tests := []struct {
		msg  string
		opt  []Option
		want int
	}{
		{
			msg: "no option, defaults to 10",
			// 2*10 + 1*100 + 1*10 (slack)
			want: 130,
		},
		{
			msg: "slack of 10, like default",
			opt: []Option{WithSlack(10)},
			// 2*10 + 1*100 + 1*10 (slack)
			want: 130,
		},
		{
			msg: "slack of 20",
			opt: []Option{WithSlack(20)},
			// 2*10 + 1*100 + 1*20 (slack)
			want: 140,
		},
		{
			// Note this is bigger then the rate of the limiter.
			msg: "slack of 150",
			opt: []Option{WithSlack(150)},
			// 2*10 + 1*100 + 1*150 (slack)
			want: 270,
		},
		{
			msg: "no option, defaults to 10, with per",
			// 2*(10*2) + 1*(100*2) + 1*10 (slack)
			opt:  []Option{Per(500 * time.Millisecond)},
			want: 230,
		},
		{
			msg: "slack of 10, like default, with per",
			opt: []Option{WithSlack(10), Per(500 * time.Millisecond)},
			// 2*(10*2) + 1*(100*2) + 1*10 (slack)
			want: 230,
		},
		{
			msg: "slack of 20, with per",
			opt: []Option{WithSlack(20), Per(500 * time.Millisecond)},
			// 2*(10*2) + 1*(100*2) + 1*20 (slack)
			want: 240,
		},
		{
			// Note this is bigger then the rate of the limiter.
			msg: "slack of 150, with per",
			opt: []Option{WithSlack(150), Per(500 * time.Millisecond)},
			// 2*(10*2) + 1*(100*2) + 1*150 (slack)
			want: 370,
		},
	}

	for _, tt := range tests {
		t.Run(tt.msg, func(t *testing.T) {
			runTest(t, func(r testRunner) {
				slow := r.createLimiter(10, WithoutSlack)
				fast := r.createLimiter(100, tt.opt...)

				r.startTaking(slow, fast)

				r.afterFunc(2*time.Second, func() {
					r.startTaking(fast)
					r.startTaking(fast)
				})

				// limiter with 10hz dominates here - we're always at 10.
				r.assertCountAtWithNoise(1*time.Second, 10, 2)
				r.assertCountAtWithNoise(3*time.Second, tt.want, 2)
			})
		})
	}
}

func TestSetRateLimitOnTheFly(t *testing.T) {
	t.Skip(UnstableTest)
	runTest(t, func(r testRunner) {
		// Set rate to 1hz
		limiter, ok := r.createLimiter(1, WithoutSlack).(*LeakyBucket)
		if !ok {
			t.Skip("Update is not supported")
		}

		r.startTaking(limiter)
		r.assertCountAt(time.Second, 2)

		r.getClock().Add(time.Second)
		r.assertCountAt(time.Second, 3)

		// increase to 2hz
		limiter.Update(2, 0)
		r.getClock().Add(time.Second)
		r.assertCountAt(time.Second, 4) // <- delayed due to paying sleepFor debt
		r.getClock().Add(time.Second)
		r.assertCountAt(time.Second, 6)

		// reduce to 1hz again
		limiter.Update(1, 0)
		r.getClock().Add(time.Second)
		r.assertCountAt(time.Second, 7)
		r.getClock().Add(time.Second)
		r.assertCountAt(time.Second, 8)

		slack := 3
		require.GreaterOrEqual(t, limiter.sleepFor, time.Duration(0))
		limiter.Update(1, slack)
		r.getClock().Add(time.Second * time.Duration(slack))
		r.assertCountAt(time.Second, 8+slack)
	})
}
