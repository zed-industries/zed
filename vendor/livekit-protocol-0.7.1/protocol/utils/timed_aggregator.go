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
	"errors"
	"sync"
	"time"
)

// ------------------------------------------------

var (
	ErrAnachronousSample = errors.New("anachronous sample")
)

// ------------------------------------------------

type timedAggregatorNumber interface {
	int64 | float64
}

type TimedAggregatorParams struct {
	CapNegativeValues bool
}

type TimedAggregator[T timedAggregatorNumber] struct {
	params TimedAggregatorParams

	lock              sync.RWMutex
	lastSample        T
	lastSampleAt      time.Time
	aggregate         T
	aggregateDuration time.Duration
}

func NewTimedAggregator[T timedAggregatorNumber](params TimedAggregatorParams) *TimedAggregator[T] {
	return &TimedAggregator[T]{
		params: params,
	}
}

func (t *TimedAggregator[T]) AddSampleAt(val T, at time.Time) error {
	t.lock.Lock()
	defer t.lock.Unlock()

	if val < 0 && t.params.CapNegativeValues {
		val = 0
	}

	return t.addSampleAtLocked(val, at)
}

func (t *TimedAggregator[T]) AddSample(val T) error {
	return t.AddSampleAt(val, time.Now())
}

func (t *TimedAggregator[T]) addSampleAtLocked(val T, at time.Time) error {
	var sinceLast time.Duration
	if !t.lastSampleAt.IsZero() {
		if t.lastSampleAt.After(at) {
			return ErrAnachronousSample
		}

		sinceLast = at.Sub(t.lastSampleAt)
	}
	lastVal := t.lastSample

	t.lastSample = val
	t.lastSampleAt = at

	t.aggregate += T(sinceLast.Seconds() * float64(lastVal))
	t.aggregateDuration += sinceLast
	return nil
}

func (t *TimedAggregator[T]) GetAggregate() (T, time.Duration) {
	t.lock.RLock()
	defer t.lock.RUnlock()

	return t.aggregate, t.aggregateDuration
}

func (t *TimedAggregator[T]) GetAggregateAt(at time.Time) (T, time.Duration, error) {
	t.lock.Lock()
	defer t.lock.Unlock()

	return t.getAggregateAtLocked(at)
}

func (t *TimedAggregator[T]) GetAggregateAndRestartAt(at time.Time) (T, time.Duration, error) {
	t.lock.Lock()
	defer t.lock.Unlock()

	aggregate, aggregateDuration, err := t.getAggregateAtLocked(at)
	t.restartAtLocked(at)
	return aggregate, aggregateDuration, err
}

func (t *TimedAggregator[T]) getAggregateAtLocked(at time.Time) (T, time.Duration, error) {
	if !t.lastSampleAt.IsZero() {
		// re-add last sample at given time
		if err := t.addSampleAtLocked(t.lastSample, at); err != nil {
			return 0, 0, ErrAnachronousSample
		}
	}

	return t.aggregate, t.aggregateDuration, nil
}

func (t *TimedAggregator[T]) GetAverage() float64 {
	t.lock.RLock()
	defer t.lock.RUnlock()

	return t.getAverageLocked()
}

func (t *TimedAggregator[T]) GetAverageAt(at time.Time) (float64, error) {
	t.lock.Lock()
	defer t.lock.Unlock()

	return t.getAverageAtLocked(at)
}

func (t *TimedAggregator[T]) GetAverageAndRestartAt(at time.Time) (float64, error) {
	t.lock.Lock()
	defer t.lock.Unlock()

	average, err := t.getAverageAtLocked(at)
	t.restartAtLocked(at)
	return average, err
}

func (t *TimedAggregator[T]) getAverageLocked() float64 {
	seconds := t.aggregateDuration.Seconds()
	if seconds == 0.0 {
		return 0.0
	}

	return float64(t.aggregate) / seconds
}

func (t *TimedAggregator[T]) getAverageAtLocked(at time.Time) (float64, error) {
	if !t.lastSampleAt.IsZero() {
		// re-add last sample at given time
		if err := t.addSampleAtLocked(t.lastSample, at); err != nil {
			return 0.0, err
		}
	}

	return t.getAverageLocked(), nil
}

func (t *TimedAggregator[T]) Reset() {
	t.lock.Lock()
	defer t.lock.Unlock()

	t.lastSample = 0
	t.lastSampleAt = time.Time{}
	t.aggregate = 0
	t.aggregateDuration = 0
}

func (t *TimedAggregator[T]) RestartAt(at time.Time) {
	t.lock.Lock()
	defer t.lock.Unlock()

	t.restartAtLocked(at)
}

func (t *TimedAggregator[T]) Restart() {
	t.RestartAt(time.Now())
}

func (t *TimedAggregator[T]) restartAtLocked(at time.Time) {
	if t.lastSampleAt.IsZero() {
		// no samples yet, nothing to restart
		return
	}

	t.lastSampleAt = at
	t.aggregate = 0
	t.aggregateDuration = 0
}
