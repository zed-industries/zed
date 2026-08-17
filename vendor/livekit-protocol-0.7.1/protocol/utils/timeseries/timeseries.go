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

package timeseries

import (
	"container/list"
	"errors"
	"fmt"
	"math"
	"time"
)

// ------------------------------------------------

var (
	errNotEnoughSamples = errors.New("not enough samples")
)

// ------------------------------------------------

type TimeSeriesUpdateOp int

const (
	TimeSeriesUpdateOpAdd TimeSeriesUpdateOp = iota
	TimeSeriesUpdateOpMax
	TimeSeriesUpdateOpLatest
)

func (t TimeSeriesUpdateOp) String() string {
	switch t {
	case TimeSeriesUpdateOpAdd:
		return "ADD"
	case TimeSeriesUpdateOpMax:
		return "MAX"
	case TimeSeriesUpdateOpLatest:
		return "LATEST"
	default:
		return fmt.Sprintf("%d", int(t))
	}
}

// ------------------------------------------------

type TimeSeriesCompareOp int

const (
	TimeSeriesCompareOpEQ TimeSeriesCompareOp = iota
	TimeSeriesCompareOpNE
	TimeSeriesCompareOpGT
	TimeSeriesCompareOpGTE
	TimeSeriesCompareOpLT
	TimeSeriesCompareOpLTE
)

func (t TimeSeriesCompareOp) String() string {
	switch t {
	case TimeSeriesCompareOpEQ:
		return "EQ"
	case TimeSeriesCompareOpNE:
		return "NE"
	case TimeSeriesCompareOpGT:
		return "GT"
	case TimeSeriesCompareOpGTE:
		return "GTE"
	case TimeSeriesCompareOpLT:
		return "LT"
	case TimeSeriesCompareOpLTE:
		return "LTE"
	default:
		return fmt.Sprintf("%d", int(t))
	}
}

// ------------------------------------------------

type ReverseIterator[T number] struct {
	limit time.Time
	e     *list.Element
	s     TimeSeriesSample[T]
}

func (it *ReverseIterator[T]) Next() bool {
	if it.e == nil {
		return false
	}

	it.s = it.e.Value.(TimeSeriesSample[T])
	it.e = it.e.Prev()
	return it.s.At.After(it.limit)
}

func (it *ReverseIterator[T]) Value() TimeSeriesSample[T] {
	return it.s
}

// ------------------------------------------------

type number interface {
	uint32 | uint64 | int | int32 | int64 | float32 | float64
}

type TimeSeriesSample[T number] struct {
	Value T
	At    time.Time
}

type TimeSeriesParams struct {
	UpdateOp         TimeSeriesUpdateOp
	Window           time.Duration
	CollapseDuration time.Duration
}

type TimeSeries[T number] struct {
	params TimeSeriesParams

	samples        *list.List
	activeSample   T
	isActiveSample bool

	welfordCount int
	welfordM     float64
	welfordS     float64
	welfordStart time.Time
	welfordLast  time.Time
}

func NewTimeSeries[T number](params TimeSeriesParams) *TimeSeries[T] {
	t := &TimeSeries[T]{
		params:  params,
		samples: list.New(),
	}

	t.initSamples()
	return t
}

func (t *TimeSeries[T]) UpdateSample(val T) {
	if !t.isActiveSample {
		t.isActiveSample = true
		t.activeSample = val
		return
	}

	switch t.params.UpdateOp {
	case TimeSeriesUpdateOpAdd:
		t.activeSample += val
	case TimeSeriesUpdateOpMax:
		if val > t.activeSample {
			t.activeSample = val
		}
	case TimeSeriesUpdateOpLatest:
		t.activeSample = val
	}
}

func (t *TimeSeries[T]) CommitActiveSample() {
	t.CommitActiveSampleAt(time.Now())
}

func (t *TimeSeries[T]) CommitActiveSampleAt(at time.Time) {
	if !t.isActiveSample {
		return
	}

	t.addSampleAt(t.activeSample, at)
	t.isActiveSample = false
}

func (t *TimeSeries[T]) AddSample(val T) {
	t.AddSampleAt(val, time.Now())
}

func (t *TimeSeries[T]) AddSampleAt(val T, at time.Time) {
	t.addSampleAt(val, at)
}

func (t *TimeSeries[T]) GetSamples() []TimeSeriesSample[T] {
	t.prune()

	samples := make([]TimeSeriesSample[T], 0, t.samples.Len())
	for e := t.samples.Front(); e != nil; e = e.Next() {
		samples = append(samples, e.Value.(TimeSeriesSample[T]))
	}
	return samples
}

func (t *TimeSeries[T]) GetSamplesAfter(at time.Time) []TimeSeriesSample[T] {
	t.prune()

	samples := make([]TimeSeriesSample[T], 0, t.samples.Len())
	for e := t.samples.Front(); e != nil; e = e.Next() {
		s := e.Value.(TimeSeriesSample[T])
		if s.At.After(at) {
			samples = append(samples, s)
		}
	}
	return samples
}

func (t *TimeSeries[T]) ReverseIterateSamplesAfter(at time.Time) ReverseIterator[T] {
	t.prune()

	return ReverseIterator[T]{
		limit: at,
		e:     t.samples.Back(),
	}
}

func (t *TimeSeries[T]) ClearSamples() {
	t.initSamples()
}

func (t *TimeSeries[T]) Sum() float64 {
	t.prune()

	sum := float64(0.0)
	for e := t.samples.Front(); e != nil; e = e.Next() {
		s := e.Value.(TimeSeriesSample[T])
		sum += float64(s.Value)
	}

	return sum
}

func (t *TimeSeries[T]) HasSamplesAfter(at time.Time) bool {
	t.prune()

	if e := t.samples.Back(); e != nil {
		return e.Value.(TimeSeriesSample[T]).At.After(at)
	}
	return false
}

func (t *TimeSeries[T]) Back() TimeSeriesSample[T] {
	t.prune()

	if e := t.samples.Back(); e != nil {
		return e.Value.(TimeSeriesSample[T])
	}
	return TimeSeriesSample[T]{}
}

func (t *TimeSeries[T]) Min() T {
	t.prune()

	return t.minLocked(t.samples.Len())
}

func (t *TimeSeries[T]) minLocked(numSamples int) T {
	min := T(0)
	for e, samplesSeen := t.samples.Back(), 0; e != nil && samplesSeen < numSamples; e, samplesSeen = e.Prev(), samplesSeen+1 {
		s := e.Value.(TimeSeriesSample[T])
		if min == T(0) || min > s.Value {
			min = s.Value
		}
	}

	return min
}

func (t *TimeSeries[T]) Max() T {
	t.prune()

	return t.maxLocked(t.samples.Len())
}

func (t *TimeSeries[T]) maxLocked(numSamples int) T {
	max := T(0)
	for e, samplesSeen := t.samples.Back(), 0; e != nil && samplesSeen < numSamples; e, samplesSeen = e.Prev(), samplesSeen+1 {
		s := e.Value.(TimeSeriesSample[T])
		if max < s.Value {
			max = s.Value
		}
	}

	return max
}

func (t *TimeSeries[T]) CurrentRun(threshold T, op TimeSeriesCompareOp) time.Duration {
	t.prune()

	start := time.Time{}
	end := time.Time{}

	for e := t.samples.Back(); e != nil; e = e.Prev() {
		cond := false
		s := e.Value.(TimeSeriesSample[T])
		switch op {
		case TimeSeriesCompareOpEQ:
			cond = s.Value == threshold
		case TimeSeriesCompareOpNE:
			cond = s.Value != threshold
		case TimeSeriesCompareOpGT:
			cond = s.Value > threshold
		case TimeSeriesCompareOpGTE:
			cond = s.Value >= threshold
		case TimeSeriesCompareOpLT:
			cond = s.Value < threshold
		case TimeSeriesCompareOpLTE:
			cond = s.Value <= threshold
		}
		if !cond {
			break
		}
		if end.IsZero() {
			end = s.At
		}
		start = s.At
	}

	if end.IsZero() || start.IsZero() {
		return 0
	}

	return end.Sub(start)
}

func (t *TimeSeries[T]) OnlineAverage() float64 {
	return t.welfordM
}

func (t *TimeSeries[T]) OnlineVariance() float64 {
	return t.onlineVarianceLocked()
}

func (t *TimeSeries[T]) onlineVarianceLocked() float64 {
	if t.welfordCount > 1 {
		return t.welfordS / float64(t.welfordCount-1)
	}

	return 0.0
}

func (t *TimeSeries[T]) OnlineStdDev() float64 {
	return t.onlineStdDevLocked()
}

func (t *TimeSeries[T]) onlineStdDevLocked() float64 {
	return math.Sqrt(t.onlineVarianceLocked())
}

func (t *TimeSeries[T]) ZScore(val T) float64 {
	onlineStdDev := t.onlineStdDevLocked()
	if onlineStdDev != 0.0 {
		return (float64(val) - t.welfordM) / onlineStdDev
	}

	return 0.0
}

func (t *TimeSeries[T]) Slope() float64 {
	t.prune()

	numSamples := t.samples.Len()
	slope, _, _, _ := t.linearFitLocked(numSamples)

	// convert to angle to normalize between -90deg to +90deg
	return math.Atan(slope) * 180 / math.Pi
}

func (t *TimeSeries[T]) linearFitLocked(numSamples int) (slope float64, intercept float64, startedAt time.Time, endedAt time.Time) {
	// go back numSamples first
	e := t.samples.Back()
	for i := 1; i < numSamples && e != nil; i++ {
		e = e.Prev()
	}

	if e == nil {
		// not enough samples
		return
	}

	sx := float64(0.0)
	sxsq := float64(0.0)
	sy := float64(0.0)
	sysq := float64(0.0)
	sxy := float64(0.0)

	for ; e != nil; e = e.Next() {
		s := e.Value.(TimeSeriesSample[T])
		if startedAt.IsZero() {
			startedAt = s.At
		}
		if endedAt.IsZero() || s.At.After(endedAt) {
			endedAt = s.At
		}

		x := s.At.Sub(startedAt).Seconds()
		y := float64(s.Value)

		sx += x
		sxsq += x * x

		sy += y
		sysq += y * y

		sxy += x * y
	}

	N := float64(numSamples)
	sxwsq := sx * sx
	denom := N*sxsq - sxwsq
	if denom != 0.0 {
		slope = (N*sxy - sx*sy) / denom
	}
	intercept = (sy - slope*sx) / N
	return
}

func (t *TimeSeries[T]) LinearExtrapolateTo(numSamplesToUse int, after time.Duration) (float64, error) {
	t.prune()

	slope, intercept, startedAt, endedAt := t.linearFitLocked(numSamplesToUse)
	if startedAt.IsZero() {
		return 0, errNotEnoughSamples
	}

	x := endedAt.Add(after).Sub(startedAt).Seconds()
	y := slope*x + intercept
	return y, nil
}

func (t *TimeSeries[T]) KendallsTau(numSamplesToUse int) (float64, error) {
	t.prune()

	if t.samples.Len() < numSamplesToUse {
		return 0.0, errNotEnoughSamples
	}

	values := make([]T, numSamplesToUse)
	idx := numSamplesToUse - 1
	for e := t.samples.Back(); e != nil; e = e.Prev() {
		if idx < 0 {
			break
		}

		s := e.Value.(TimeSeriesSample[T])
		values[idx] = s.Value
		idx--
	}

	concordantPairs := 0
	discordantPairs := 0
	for i := 0; i < len(values)-1; i++ {
		for j := i + 1; j < len(values); j++ {
			if values[i] < values[j] {
				concordantPairs++
			} else if values[i] > values[j] {
				discordantPairs++
			}
		}
	}

	if (concordantPairs + discordantPairs) == 0 {
		return 0.0, nil
	}

	return (float64(concordantPairs) - float64(discordantPairs)) / (float64(concordantPairs) + float64(discordantPairs)), nil
}

func (t *TimeSeries[T]) initSamples() {
	t.samples = t.samples.Init()
}

func (t *TimeSeries[T]) addSampleAt(val T, at time.Time) {
	// insert in time order
	e := t.samples.Back()
	if e != nil {
		lastSample := e.Value.(TimeSeriesSample[T])
		if val == lastSample.Value && at.Sub(lastSample.At) < t.params.CollapseDuration {
			// repeated value within collapse duration
			t.prune()
			return
		}
	}
	for e = t.samples.Back(); e != nil; e = e.Prev() {
		s := e.Value.(TimeSeriesSample[T])
		if at.After(s.At) {
			break
		}
	}

	sample := TimeSeriesSample[T]{
		Value: val,
		At:    at,
	}
	switch {
	case e != nil: // in the middle
		t.samples.InsertAfter(sample, e)

	case t.samples.Front() != nil: // in the front
		t.samples.PushFront(sample)

	default: // at the end
		t.samples.PushBack(sample)
	}

	t.updateWelfordStats(val, at)

	t.prune()
}

func (t *TimeSeries[T]) updateWelfordStats(val T, at time.Time) {
	t.welfordCount++
	mLast := t.welfordM
	t.welfordM += (float64(val) - t.welfordM) / float64(t.welfordCount)
	t.welfordS += (float64(val) - mLast) * (float64(val) - t.welfordM)

	if t.welfordStart.IsZero() {
		t.welfordStart = at
	}
	t.welfordLast = at
}

func (t *TimeSeries[T]) prune() {
	thresh := t.welfordLast.Add(-t.params.Window)
	//thresh := time.Now().Add(-t.params.Window)

	for next := t.samples.Front(); next != nil; {
		e := next
		s := e.Value.(TimeSeriesSample[T])
		if s.At.After(thresh) {
			break
		}
		next = e.Next()

		t.samples.Remove(e)
	}
}

// TODO - a bunch of stats
// - sum
// - moving average
// - EWMA
// - min
// - max
// - average
// - median
// - variance
// - stddev
// - trend
// - run
// - z-score
