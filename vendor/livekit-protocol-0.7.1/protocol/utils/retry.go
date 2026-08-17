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
	"math/rand/v2"
	"time"
)

type terminalError struct {
	error
}

func (e terminalError) Unwrap() error {
	return e.error
}

func TerminalError(err error) error {
	return terminalError{err}
}

func ErrIsTerminal(err error) bool {
	var terr terminalError
	return errors.As(err, &terr)
}

func Retry(ctx context.Context, minTime, maxTime, timeout time.Duration, f func(ctx context.Context) error) error {
	retryTime := minTime

	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()

	for {
		err := f(ctx)
		if err == nil || ErrIsTerminal(err) {
			return err
		}

		select {
		case <-ctx.Done():
			return err
		case <-time.After(retryTime):
		}

		retryTime = min(maxTime, time.Duration((rand.Float64()+1)*float64(retryTime)))
	}
}
