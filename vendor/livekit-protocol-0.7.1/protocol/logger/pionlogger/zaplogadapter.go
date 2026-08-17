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

package pionlogger

import (
	"fmt"

	"go.uber.org/atomic"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"

	"github.com/livekit/protocol/logger"
)

const (
	zapLogAdapterStateCreated = iota
	zapLogAdapterStateBuilding
	zapLogAdapterStateReady
)

// implements webrtc.LeveledLogger
type zapLogAdapter struct {
	state           atomic.Uint32
	logger          logger.Logger
	level           zapcore.LevelEnabler
	scope           string
	ignoredPrefixes *prefixSet
}

func (l *zapLogAdapter) init() {
	for l.state.Load() != zapLogAdapterStateReady {
		if l.state.CompareAndSwap(zapLogAdapterStateCreated, zapLogAdapterStateBuilding) {
			l.logger = l.logger.WithComponent(formatComponent(l.scope)).WithCallDepth(1)
			l.state.Store(zapLogAdapterStateReady)
		}
	}
}

func (l *zapLogAdapter) Trace(msg string) {
	if l.level.Enabled(zap.DebugLevel) {
		l.init()
		l.logger.Debugw(msg)
	}
}

func (l *zapLogAdapter) Tracef(format string, args ...interface{}) {
	if l.level.Enabled(zap.DebugLevel) {
		l.init()
		l.logger.Debugw(fmt.Sprintf(format, args...))
	}
}

func (l *zapLogAdapter) Debug(msg string) {
	if l.level.Enabled(zap.DebugLevel) {
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Debugw(msg)
		}
	}
}

func (l *zapLogAdapter) Debugf(format string, args ...interface{}) {
	if l.level.Enabled(zap.DebugLevel) {
		msg := fmt.Sprintf(format, args...)
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Debugw(msg)
		}
	}
}

func (l *zapLogAdapter) Info(msg string) {
	if l.level.Enabled(zap.InfoLevel) {
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Infow(msg)
		}
	}
}

func (l *zapLogAdapter) Infof(format string, args ...interface{}) {
	if l.level.Enabled(zap.InfoLevel) {
		msg := fmt.Sprintf(format, args...)
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Infow(msg)
		}
	}
}

func (l *zapLogAdapter) Warn(msg string) {
	if l.level.Enabled(zap.WarnLevel) {
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Warnw(msg, nil)
		}
	}
}

func (l *zapLogAdapter) Warnf(format string, args ...interface{}) {
	if l.level.Enabled(zap.WarnLevel) {
		msg := fmt.Sprintf(format, args...)
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Warnw(msg, nil)
		}
	}
}

func (l *zapLogAdapter) Error(msg string) {
	if l.level.Enabled(zap.ErrorLevel) {
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Errorw(msg, nil)
		}
	}
}

func (l *zapLogAdapter) Errorf(format string, args ...interface{}) {
	if l.level.Enabled(zap.ErrorLevel) {
		msg := fmt.Sprintf(format, args...)
		if !l.ignoredPrefixes.Match(msg) {
			l.init()
			l.logger.Errorw(msg, nil)
		}
	}
}
