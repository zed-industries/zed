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

package zaputil

import (
	"testing"

	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"

	"github.com/livekit/protocol/logger/testutil"
)

func TestDeferredLogger(t *testing.T) {
	t.Run("logs are deferred until logger resolves", func(t *testing.T) {
		c := &testCore{Core: zap.NewExample().Core()}
		d := &Deferrer{}
		dc := NewDeferredValueCore(c, d)
		s := zap.New(dc).Sugar()

		s.Infow("test")
		require.Equal(t, 0, c.WriteCount())

		s.With("a", "1").Infow("test")
		require.Equal(t, 0, c.WriteCount())

		d.Resolve("b", "2")
		require.Equal(t, 2, c.WriteCount())

		s.With("c", "3").Infow("test")
		require.Equal(t, 3, c.WriteCount())
	})

	type testLog struct {
		A, B string
	}

	t.Run("resolved values can be overwritten", func(t *testing.T) {
		ws := &testutil.BufferedWriteSyncer{}
		we := NewWriteEnabler(ws, zapcore.DebugLevel)
		enc := zapcore.NewJSONEncoder(zap.NewProductionEncoderConfig())
		c := NewEncoderCore(enc, we)
		d := &Deferrer{}
		dc := NewDeferredValueCore(c, d)
		s := zap.New(dc).Sugar()

		cases := []testLog{
			{"foo", "bar"},
			{"baz", "qux"},
		}
		for _, c := range cases {
			d.Resolve("a", c.A, "b", c.B)
			s.Infow("test")
			s.Sync()

			var log testLog
			require.NoError(t, ws.Unmarshal(&log))
			ws.Reset()

			require.Equal(t, c.A, log.A)
			require.Equal(t, c.B, log.B)
		}
	})

	t.Run("resolved values merge with previous resolutions", func(t *testing.T) {
		ws := &testutil.BufferedWriteSyncer{}
		we := NewWriteEnabler(ws, zapcore.DebugLevel)
		enc := zapcore.NewJSONEncoder(zap.NewProductionEncoderConfig())
		c := NewEncoderCore(enc, we)
		d := &Deferrer{}
		dc := NewDeferredValueCore(c, d)
		s := zap.New(dc).Sugar()

		d.Resolve("a", "foo")
		d.Resolve("b", "bar")
		s.Infow("test")
		s.Sync()

		var log testLog
		require.NoError(t, ws.Unmarshal(&log))
		ws.Reset()

		require.Equal(t, "foo", log.A)
		require.Equal(t, "bar", log.B)
	})

	t.Run("re-resolve", func(t *testing.T) {
		ws := &testutil.BufferedWriteSyncer{}
		we := NewWriteEnabler(ws, zapcore.DebugLevel)
		enc := zapcore.NewJSONEncoder(zap.NewProductionEncoderConfig())
		c := NewEncoderCore(enc, we)
		d := &Deferrer{}
		dc := NewDeferredValueCore(c, d)
		s := zap.New(dc).Sugar()

		d.Resolve("a", "foo")
		d.Resolve("b", "bar")
		s.Infow("test")
		s.Sync()

		var log testLog
		require.NoError(t, ws.Unmarshal(&log))
		ws.Reset()

		require.Equal(t, "foo", log.A)
		require.Equal(t, "bar", log.B)

		d.Reset()
		d.Resolve("a", "car")
		d.Resolve("b", "dog")
		s.Infow("test")
		s.Sync()

		require.NoError(t, ws.Unmarshal(&log))
		ws.Reset()

		require.Equal(t, "car", log.A)
		require.Equal(t, "dog", log.B)
	})
}
