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

func TestTimedVersion(t *testing.T) {
	t.Run("timed versions are monotonic and comparable", func(t *testing.T) {
		gen := NewDefaultTimedVersionGenerator()
		tv1 := gen.Next()
		tv2 := gen.Next()
		tv3 := gen.Next()

		require.True(t, tv3.After(tv1))
		require.True(t, tv3.After(tv2))
		require.True(t, tv2.After(tv1))

		tv2.Update(tv3)
		require.True(t, tv2.After(tv1))
		// tv3 and tv2 are equivalent after update
		require.False(t, tv3.After(tv2))

		require.Equal(t, 0, tv1.Compare(tv1))
		require.Equal(t, -1, tv1.Compare(tv2))
		require.Equal(t, 1, tv2.Compare(tv1))
	})

	t.Run("protobuf roundtrip", func(t *testing.T) {
		gen := NewDefaultTimedVersionGenerator()
		tv1 := gen.Next()
		tv2 := TimedVersionFromProto(tv1.ToProto())
		require.Equal(t, tv1, tv2)
	})

	t.Run("from zero time yields epoch version", func(t *testing.T) {
		gen := NewDefaultTimedVersionGenerator()
		tv1 := TimedVersionFromTime(time.Date(0, 0, 0, 0, 0, 0, 0, time.UTC))
		tv2 := gen.Next()
		require.True(t, tv2.After(tv1))
	})

	t.Run("time.Time roundtrip", func(t *testing.T) {
		ts1 := time.Now().Round(time.Microsecond)
		tv1 := TimedVersionFromTime(ts1)
		ts2 := tv1.Time()
		tv2 := TimedVersionFromTime(ts2)
		require.Equal(t, ts1, ts2)
		require.Equal(t, tv1, tv2)
	})

	t.Run("timed version from nil is zero", func(t *testing.T) {
		require.True(t, TimedVersionFromProto(nil).IsZero())
	})
}
