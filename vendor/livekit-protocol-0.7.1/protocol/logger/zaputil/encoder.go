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
	"go.uber.org/multierr"
	"go.uber.org/zap/zapcore"
)

type WriteEnabler struct {
	zapcore.WriteSyncer
	zapcore.LevelEnabler
}

func NewWriteEnabler(ws zapcore.WriteSyncer, enab zapcore.LevelEnabler) *WriteEnabler {
	return &WriteEnabler{ws, enab}
}

type discardWriteSyncer struct{}

func (discardWriteSyncer) Write(p []byte) (int, error) { return len(p), nil }
func (discardWriteSyncer) Sync() error                 { return nil }

func NewDiscardWriteEnabler() *WriteEnabler {
	return NewWriteEnabler(discardWriteSyncer{}, zapcore.FatalLevel)
}

func NewEncoderCore(enc zapcore.Encoder, out ...*WriteEnabler) zapcore.Core {
	return &encoderCore{
		enc: enc,
		out: out,
	}
}

type encoderCore struct {
	enc zapcore.Encoder
	out []*WriteEnabler
}

func (c encoderCore) Level() zapcore.Level {
	minLvl := zapcore.FatalLevel
	for _, out := range c.out {
		if lvl := zapcore.LevelOf(out); lvl < minLvl {
			minLvl = lvl
		}
	}
	return minLvl
}

func (c encoderCore) Enabled(lvl zapcore.Level) bool {
	for _, out := range c.out {
		if out.Enabled(lvl) {
			return true
		}
	}
	return false
}

func (c *encoderCore) With(fields []zapcore.Field) zapcore.Core {
	dup := *c
	dup.enc = dup.enc.Clone()
	for _, f := range fields {
		f.AddTo(dup.enc)
	}
	return &dup
}

func (c *encoderCore) Check(ent zapcore.Entry, ce *zapcore.CheckedEntry) *zapcore.CheckedEntry {
	if c.Enabled(ent.Level) {
		return ce.AddCore(ent, c)
	}
	return ce
}

func (c *encoderCore) Write(ent zapcore.Entry, fields []zapcore.Field) error {
	buf, err := c.enc.EncodeEntry(ent, fields)
	if err != nil {
		return err
	}
	for _, out := range c.out {
		if out.Enabled(ent.Level) {
			_, werr := out.Write(buf.Bytes())
			err = multierr.Append(err, werr)
		}
	}
	buf.Free()
	if err != nil {
		return err
	}
	if ent.Level > zapcore.ErrorLevel {
		_ = c.Sync()
	}
	return nil
}

func (c *encoderCore) Sync() error {
	var err error
	for _, out := range c.out {
		err = multierr.Append(err, out.Sync())
	}
	return err
}
