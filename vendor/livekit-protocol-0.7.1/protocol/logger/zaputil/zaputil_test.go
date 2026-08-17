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
	"go.uber.org/atomic"
	"go.uber.org/zap/zapcore"
)

type testStringer string

func (s testStringer) String() string {
	return string(s)
}

type testStringLike string

type testCore struct {
	zapcore.Core
	writeCount *atomic.Int32
}

func (c *testCore) init() {
	if c.writeCount == nil {
		c.writeCount = &atomic.Int32{}
	}
}

func (c *testCore) WriteCount() int {
	c.init()
	return int(c.writeCount.Load())
}

func (c *testCore) With(fields []zapcore.Field) zapcore.Core {
	c.init()
	return &testCore{
		Core:       c.Core.With(fields),
		writeCount: c.writeCount,
	}
}

func (s *testCore) Write(entry zapcore.Entry, fields []zapcore.Field) error {
	s.init()
	s.writeCount.Inc()
	return nil
}
