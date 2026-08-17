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
	"io"

	"go.uber.org/multierr"
)

type Closers []io.Closer

func CombineClosers(cs ...io.Closer) Closers {
	return append([]io.Closer{}, cs...)
}

func (s *Closers) Close() error {
	var err error
	for _, c := range *s {
		if c != nil {
			err = multierr.Append(err, c.Close())
		}
	}

	*s = (*s)[:0]

	return err
}

type CloseFuncType interface {
	~func() error | ~func()
}

func CloseFunc[T CloseFuncType](fn T) io.Closer {
	return closeFunc[T]{fn}
}

type closeFunc[T CloseFuncType] struct {
	fn T
}

func (f closeFunc[T]) Close() error {
	switch fn := any(f.fn).(type) {
	case func() error:
		return fn()
	case func():
		fn()
	}
	return nil
}
