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

// Package mono enforces use of monotonic time when creating/parsing time.Time from external sources.
//
// Using time.Now produces monotonic time values that correctly measure time difference in the presence of clock resets.
//
// On the other hand, time produce by time.Unix or time.Parse doesn't have this property. Clock reset may lead to incorrect
// durations computed from these timestamps. To fix this, prefer using Unix and Parse provided by this package.
//
// Monotonic time could also be erased when using functions like Truncate, Round, In, UTC. Be careful when using these.
//
// More details: https://go.googlesource.com/proposal/+/master/design/12914-monotonic.md
package mono

import "time"

var (
	epoch     = time.Now()
	epochNano = epoch.UnixNano()
)

// resetClock resets the reference timestamp.
// Used in tests only.
func resetClock() {
	epoch = time.Now()
	epochNano = epoch.UnixNano()
}

// jumpClock adjusts reference timestamp by a given duration emulating a clock reset/jump.
// Used in tests only.
func jumpClock(dt time.Duration) {
	epoch = epoch.Add(-dt) // we pretend time.Now() jumps, not the reference
	epochNano = epoch.UnixNano()
}

// FromTime ensures that time.Time value uses monotonic clock.
//
// Deprecated: You should probably use Unix or Parse instead.
func FromTime(t time.Time) time.Time {
	return fromTime(t)
}

func fromTime(t time.Time) time.Time {
	if t.IsZero() {
		return time.Time{}
	}
	return epoch.Add(t.Sub(epoch))
}

// Now is a wrapper for time.Time.
//
// Deprecated: time.Now always uses monotonic clock.
func Now() time.Time {
	return time.Now()
}

// Unix is an analog of time.Unix that produces monotonic time.
func Unix(sec, nsec int64) time.Time {
	return fromTime(time.Unix(sec, nsec))
}

// Parse is an analog of time.Parse that produces monotonic time.
func Parse(layout, value string) (time.Time, error) {
	t, err := time.Parse(layout, value)
	if err != nil {
		return time.Time{}, err
	}
	return fromTime(t), nil
}

// UnixNano returns the number of nanoseconds elapsed, based on the application start time.
// This value may be different from time.Now().UnixNano() in the presence of time resets.
func UnixNano() int64 {
	return epochNano + int64(time.Since(epoch))
}

// UnixMicro returns the number of microseconds elapsed, based on the application start time.
// This value may be different from time.Now().UnixMicro() in the presence of time resets.
func UnixMicro() int64 {
	return UnixNano() / 1000
}
