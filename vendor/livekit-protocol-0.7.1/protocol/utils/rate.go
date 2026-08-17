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
// SOURCE: https://github.com/uber-go/ratelimit/blob/main/limiter_mutexbased.go
// EDIT: slight modification to allow setting rate limit on the fly
// SCOPE: LeakyBucket
package utils

import (
	"sync"
	"time"

	"go.uber.org/atomic"
)

type LeakyBucket struct {
	mutex    sync.Mutex
	last     time.Time
	sleepFor time.Duration
	cfg      atomic.Pointer[leakyBucketConfig]
	clock    Clock
}

type leakyBucketConfig struct {
	perRequest time.Duration
	maxSlack   time.Duration
}

// NewLeakyBucket initiates LeakyBucket with rateLimit, slack, and clock.
//
// rateLimit is defined as the number of request per second.
//
// slack is defined as the number of allowed requests before limiting.
// e.g. when slack=5, LeakyBucket will allow 5 requests to pass through Take
// without a sleep as long as these requests are under perRequest duration.
func NewLeakyBucket(rateLimit int, slack int, clock Clock) *LeakyBucket {
	var lb LeakyBucket
	lb.clock = clock
	lb.Update(rateLimit, slack)
	return &lb
}

// Update sets the underlying rate limit and slack.
// The setting may not be applied immediately.
//
// Update is THREAD SAFE and NON-BLOCKING.
func (lb *LeakyBucket) Update(rateLimit int, slack int) {
	perRequest := time.Second / time.Duration(rateLimit)
	maxSlack := -1 * time.Duration(slack) * perRequest
	cfg := leakyBucketConfig{
		perRequest: perRequest,
		maxSlack:   maxSlack,
	}
	lb.cfg.Store(&cfg)
}

// Take blocks to ensure that the time spent between multiple Take calls
// is on average time.Second/rate.
//
// Take is THREAD SAFE and BLOCKING.
func (lb *LeakyBucket) Take() time.Time {
	lb.mutex.Lock()
	defer lb.mutex.Unlock()

	cfg := lb.cfg.Load()
	now := lb.clock.Now()

	// If this is our first request, then we allow it.
	if lb.last.IsZero() {
		lb.last = now
		return lb.last
	}

	// sleepFor calculates how much time we should sleep based on
	// the perRequest budget and how long the last request took.
	// Since the request may take longer than the budget, this number
	// can get negative, and is summed across requests.
	lb.sleepFor += cfg.perRequest - now.Sub(lb.last)

	// We shouldn't allow sleepFor to get too negative, since it would mean that
	// a service that slowed down a lot for a short period of time would get
	// a much higher RPS following that.
	if lb.sleepFor < cfg.maxSlack {
		lb.sleepFor = cfg.maxSlack
	}

	// If sleepFor is positive, then we should sleep now.
	if lb.sleepFor > 0 {
		lb.clock.Sleep(lb.sleepFor)
		lb.last = now.Add(lb.sleepFor)
		lb.sleepFor = 0
	} else {
		lb.last = now
	}

	return lb.last
}
