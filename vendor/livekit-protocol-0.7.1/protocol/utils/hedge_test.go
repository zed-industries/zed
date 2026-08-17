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
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
	"go.uber.org/atomic"
)

func TestHedgeCall(t *testing.T) {
	t.Run("success", func(t *testing.T) {
		var attempts atomic.Uint32
		res, err := HedgeCall(context.Background(), HedgeParams[uint32]{
			Timeout:     200 * time.Millisecond,
			RetryDelay:  50 * time.Millisecond,
			MaxAttempts: 2,
			Func: func(context.Context) (uint32, error) {
				n := attempts.Add(1)
				time.Sleep(75 * time.Millisecond)
				return n, nil
			},
		})
		require.NoError(t, err)
		require.EqualValues(t, 1, res)
		require.EqualValues(t, 2, attempts.Load())
	})

	t.Run("recoverable error", func(t *testing.T) {
		var recoverableErr = errors.New("recoverable")

		var attempts atomic.Uint32
		res, err := HedgeCall(context.Background(), HedgeParams[uint32]{
			Timeout:     200 * time.Millisecond,
			RetryDelay:  50 * time.Millisecond,
			MaxAttempts: 2,
			IsRecoverable: func(err error) bool {
				return errors.Is(err, recoverableErr)
			},
			Func: func(context.Context) (uint32, error) {
				n := attempts.Add(1)
				if n == 1 {
					return n, recoverableErr
				}
				return n, nil
			},
		})
		require.NoError(t, err)
		require.EqualValues(t, 2, res)
	})

	t.Run("unrecoverable error", func(t *testing.T) {
		var recoverableErr = errors.New("recoverable")
		var unrecoverableErr = errors.New("unrecoverable")

		var attempts atomic.Uint32
		_, err := HedgeCall(context.Background(), HedgeParams[uint32]{
			Timeout:     200 * time.Millisecond,
			RetryDelay:  50 * time.Millisecond,
			MaxAttempts: 3,
			IsRecoverable: func(err error) bool {
				return !errors.Is(err, unrecoverableErr)
			},
			Func: func(context.Context) (uint32, error) {
				n := attempts.Add(1)
				if n == 1 {
					return n, recoverableErr
				}
				return n, unrecoverableErr
			},
		})
		require.ErrorIs(t, err, unrecoverableErr)
		require.EqualValues(t, 2, attempts.Load())
	})

	t.Run("max failures", func(t *testing.T) {
		var attempts atomic.Uint32
		_, err := HedgeCall(context.Background(), HedgeParams[uint32]{
			Timeout:     200 * time.Millisecond,
			RetryDelay:  50 * time.Millisecond,
			MaxAttempts: 2,
			Func: func(context.Context) (uint32, error) {
				n := attempts.Add(1)
				return n, errors.New("failure")
			},
		})
		require.ErrorIs(t, err, ErrMaxAttemptsReached)
		require.EqualValues(t, 2, attempts.Load())
	})
}
