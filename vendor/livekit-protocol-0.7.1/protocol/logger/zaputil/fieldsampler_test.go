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
	"fmt"
	"math"
	"testing"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"

	"github.com/stretchr/testify/require"
)

func TestRoomSampleRate(t *testing.T) {
	expectedRate := 0.50
	n := 10000

	c := &testCore{Core: zap.NewExample().Core()}
	s := zap.New(NewFieldSampler(c, FieldSamplerConfig{
		FieldName: "field",
		Rate:      NewAtomicFieldSampleRate(expectedRate),
	})).Sugar()

	for i := 0; i < n; i++ {
		s.Debugw("test", "field", fmt.Sprintf("field_%d", i))
	}

	rate := float64(c.WriteCount()) / float64(n)
	require.Greater(t, 0.01, math.Abs(rate-expectedRate))
}

func TestRoomSamplerWith(t *testing.T) {
	c := &testCore{Core: zap.NewExample().Core()}
	s := zap.New(NewFieldSampler(c, FieldSamplerConfig{
		FieldName: "field",
		Rate:      NewAtomicFieldSampleRate(0),
	}).(*fieldSampler)).Sugar()

	s.Debugw("test", "field", "test")
	require.Equal(t, 0, c.WriteCount())

	s.With("field", "test").Debugw("test")
	require.Equal(t, 0, c.WriteCount())
}

func TestRoomSampleFindSampleField(t *testing.T) {
	s := NewFieldSampler(nil, FieldSamplerConfig{
		FieldName: "field",
		Rate:      NewAtomicFieldSampleRate(1),
	}).(*fieldSampler)

	v, ok := s.findSampleField([]zapcore.Field{zap.String("field", "test")})
	require.Equal(t, "test", v)
	require.True(t, ok)

	v, ok = s.findSampleField([]zapcore.Field{zap.Reflect("field", testStringLike("test"))})
	require.Equal(t, "test", v)
	require.True(t, ok)

	v, ok = s.findSampleField([]zapcore.Field{zap.Stringer("field", testStringer("test"))})
	require.Equal(t, "test", v)
	require.True(t, ok)

	_, ok = s.findSampleField([]zapcore.Field{zap.Int("field", 123)})
	require.False(t, ok)

	_, ok = s.findSampleField([]zapcore.Field{})
	require.False(t, ok)
}
