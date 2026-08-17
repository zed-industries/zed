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

	"github.com/livekit/protocol/logger"
)

// implements webrtc.LeveledLogger
type logAdapter struct {
	logger          logger.Logger
	ignoredPrefixes *prefixSet
}

func (l *logAdapter) Trace(msg string) {
	l.logger.Debugw(msg)
}

func (l *logAdapter) Tracef(format string, args ...interface{}) {
	l.logger.Debugw(fmt.Sprintf(format, args...))
}

func (l *logAdapter) Debug(msg string) {
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Debugw(msg)
}

func (l *logAdapter) Debugf(format string, args ...interface{}) {
	msg := fmt.Sprintf(format, args...)
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Debugw(msg)
}

func (l *logAdapter) Info(msg string) {
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Infow(msg)
}

func (l *logAdapter) Infof(format string, args ...interface{}) {
	msg := fmt.Sprintf(format, args...)
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Infow(msg)
}

func (l *logAdapter) Warn(msg string) {
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Warnw(msg, nil)
}

func (l *logAdapter) Warnf(format string, args ...interface{}) {
	msg := fmt.Sprintf(format, args...)
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Warnw(msg, nil)
}

func (l *logAdapter) Error(msg string) {
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Errorw(msg, nil)
}

func (l *logAdapter) Errorf(format string, args ...interface{}) {
	msg := fmt.Sprintf(format, args...)
	if l.ignoredPrefixes.Match(msg) {
		return
	}
	l.logger.Errorw(msg, nil)
}
