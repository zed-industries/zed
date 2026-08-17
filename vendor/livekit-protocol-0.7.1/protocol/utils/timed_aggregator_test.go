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
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestTimedAggregator(t *testing.T) {
	t.Run("functions", func(t *testing.T) {
		ta := NewTimedAggregator[float64](TimedAggregatorParams{})

		aggregate, aggregateDuration := ta.GetAggregate()
		require.Equal(t, 0.0, aggregate)
		require.Equal(t, time.Duration(0), aggregateDuration)
		require.Equal(t, 0.0, ta.GetAverage())

		now := time.Now()
		require.NoError(t, ta.AddSampleAt(1.0, now))
		// nothing to aggregate with just one sample
		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 0.0, aggregate)
		require.Equal(t, time.Duration(0), aggregateDuration)
		require.Equal(t, 0.0, ta.GetAverage())

		require.NoError(t, ta.AddSampleAt(2.0, now.Add(500*time.Millisecond)))
		// second sample added should make stats available
		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 0.5, aggregate)
		require.Equal(t, 500*time.Millisecond, aggregateDuration)
		require.Equal(t, 1.0, ta.GetAverage())

		// cannot add anachronous sample
		require.Error(t, ErrAnachronousSample, ta.AddSampleAt(10.0, now.Add(200*time.Millisecond)))

		// cannot get aggregate or average at an older time
		_, _, err := ta.GetAggregateAt(now.Add(200 * time.Millisecond))
		require.Error(t, ErrAnachronousSample, err)

		// check a bit later, last value should continue
		// 1.0 (0.5s) +  2.0 (1.0s)
		aggregate, aggregateDuration, err = ta.GetAggregateAt(now.Add(1500 * time.Millisecond))
		require.Equal(t, 2.5, aggregate)
		require.Equal(t, 1500*time.Millisecond, aggregateDuration)
		require.NoError(t, err)

		require.Equal(t, 2.5/1.5, ta.GetAverage())

		// another second out and restart
		// 1.0 (0.5s) +  2.0 (2.0s)
		aggregate, aggregateDuration, err = ta.GetAggregateAndRestartAt(now.Add(2500 * time.Millisecond))
		require.Equal(t, 4.5, aggregate)
		require.Equal(t, 2500*time.Millisecond, aggregateDuration)
		require.NoError(t, err)

		// restart should have reset the aggregates
		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 0.0, aggregate)
		require.Equal(t, time.Duration(0), aggregateDuration)

		require.Equal(t, 0.0, ta.GetAverage())

		// add a sample a bit later and check aggregates
		require.NoError(t, ta.AddSampleAt(20.0, now.Add(2800*time.Millisecond)))

		// 2.0 (0.3s) + 20.0 (0.2s)
		aggregate, aggregateDuration, err = ta.GetAggregateAt(now.Add(3000 * time.Millisecond))
		require.Equal(t, 4.6, aggregate)
		require.Equal(t, 500*time.Millisecond, aggregateDuration)
		require.NoError(t, err)

		require.Equal(t, 4.6/0.5, ta.GetAverage())

		// get average a bit later
		// 2.0 (0.3s) +  20.0 (0.5s)
		average, err := ta.GetAverageAt(now.Add(3300 * time.Millisecond))
		require.Equal(t, float64(10.6)/float64(0.8), average)
		require.NoError(t, err)

		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 10.6, aggregate)
		require.Equal(t, 800*time.Millisecond, aggregateDuration)

		// get average and restart a bit later
		// 2.0 (0.3s) +  20.0 (1.0s)
		average, err = ta.GetAverageAndRestartAt(now.Add(3800 * time.Millisecond))
		require.Equal(t, float64(20.6)/float64(1.3), average)
		require.NoError(t, err)

		// restart should have reset the aggregates
		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 0.0, aggregate)
		require.Equal(t, time.Duration(0), aggregateDuration)

		// add a negative value sample
		require.NoError(t, ta.AddSampleAt(-2.0, now.Add(4000*time.Millisecond)))

		// get average a bit later
		// 20.0 (0.2s) +  -2.0 (0.5s)
		average, err = ta.GetAverageAt(now.Add(4500 * time.Millisecond))
		require.Equal(t, float64(3.0)/float64(0.7), average)
		require.NoError(t, err)

		aggregate, aggregateDuration = ta.GetAggregate()
		require.Equal(t, 3.0, aggregate)
		require.Equal(t, 700*time.Millisecond, aggregateDuration)
	})

	t.Run("negative_values", func(t *testing.T) {
		ta := NewTimedAggregator[int64](TimedAggregatorParams{
			CapNegativeValues: true,
		})

		now := time.Now()
		require.NoError(t, ta.AddSampleAt(1, now))
		require.NoError(t, ta.AddSampleAt(-1, now.Add(time.Second)))
		require.NoError(t, ta.AddSampleAt(1, now.Add(2*time.Second)))

		// 1 (1.0s) + 0 (capped value) (1.0s) + 1 (1.0s)
		aggregate, aggregateDuration, err := ta.GetAggregateAt(now.Add(3 * time.Second))
		require.Equal(t, int64(2), aggregate)
		require.Equal(t, 3*time.Second, aggregateDuration)
		require.NoError(t, err)

		require.Equal(t, float64(2.0)/float64(3.0), ta.GetAverage())
	})
}
