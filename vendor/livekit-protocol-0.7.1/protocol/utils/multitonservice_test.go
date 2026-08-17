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
	"go.uber.org/atomic"
)

type testService struct {
	key       string
	killCalls atomic.Int32
}

func newTestService(key string) *testService {
	return &testService{key: key}
}

func (s *testService) Kill() { s.killCalls.Inc() }

func TestMultitonService(t *testing.T) {
	t.Run("start and stop are called", func(t *testing.T) {
		t.Parallel()

		r := MultitonService[string]{}
		svc := newTestService("foo")
		stop := r.Replace("test", svc)

		time.Sleep(time.Millisecond)

		stop()

		time.Sleep(time.Millisecond)

		require.EqualValues(t, 1, svc.killCalls.Load())
	})

	t.Run("stop is idempotent", func(t *testing.T) {
		t.Parallel()

		r := MultitonService[string]{}
		svc := newTestService("foo")
		stop := r.Replace("test", svc)

		for i := 0; i < 3; i++ {
			time.Sleep(time.Millisecond)

			stop()
		}

		require.EqualValues(t, 1, svc.killCalls.Load())
	})

	t.Run("replaced services are stopped", func(t *testing.T) {
		t.Parallel()

		r := MultitonService[string]{}
		svc0 := newTestService("foo")
		r.Replace("test", svc0)

		time.Sleep(time.Millisecond)

		svc1 := newTestService("foo")
		r.Replace("test", svc1)

		time.Sleep(time.Millisecond)

		require.EqualValues(t, 1, svc0.killCalls.Load())
		require.EqualValues(t, 0, svc1.killCalls.Load())
	})

	t.Run("stop funcs for replaced services are neutered", func(t *testing.T) {
		t.Parallel()

		r := MultitonService[string]{}
		svc0 := newTestService("foo")
		stop0 := r.Replace("test", svc0)

		time.Sleep(time.Millisecond)

		svc1 := newTestService("foo")
		r.Replace("test", svc1)

		time.Sleep(time.Millisecond)

		stop0()

		time.Sleep(time.Millisecond)

		require.EqualValues(t, 1, svc0.killCalls.Load())
		require.EqualValues(t, 0, svc1.killCalls.Load())
	})
}
