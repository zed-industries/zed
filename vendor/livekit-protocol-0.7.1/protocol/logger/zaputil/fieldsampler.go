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
	"reflect"
	"sync/atomic"

	"github.com/zeebo/xxh3"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

type FieldSampleRate interface {
	Threshold() uint64
}

func rateToThreshold(rate float64) uint64 {
	return math.MaxUint64 / 10000 * uint64(math.Max(math.Min(rate, 1), 0)*10000)
}

type AtomicFieldSampleRate uint64

func NewAtomicFieldSampleRate(rate float64) *AtomicFieldSampleRate {
	var r AtomicFieldSampleRate
	r.SetRate(rate)
	return &r
}

func (r *AtomicFieldSampleRate) SetRate(rate float64) {
	atomic.StoreUint64((*uint64)(r), rateToThreshold(rate))
}

func (r *AtomicFieldSampleRate) Threshold() uint64 {
	return uint64(atomic.LoadUint64((*uint64)(r)))
}

type FieldSamplerAction int

const (
	OmitSampledLog FieldSamplerAction = iota
	AnnotateSampledLog
)

type FieldSamplerConfig struct {
	FieldName           string
	Rate                FieldSampleRate
	Action              FieldSamplerAction
	AnnotationFieldName string
}

func NewFieldSampler(core zapcore.Core, config FieldSamplerConfig) zapcore.Core {
	return &fieldSampler{
		Core:   core,
		config: config,
	}
}

type fieldSampler struct {
	zapcore.Core
	config FieldSamplerConfig
	hash   uint64
}

var _ zapcore.Core = (*fieldSampler)(nil)

func (s *fieldSampler) With(fields []zapcore.Field) zapcore.Core {
	hash := s.hash
	if h, ok := s.hashSampleField(fields); ok {
		hash = h
	}

	return &fieldSampler{
		Core:   s.Core.With(fields),
		config: s.config,
		hash:   hash,
	}
}

func (s *fieldSampler) Check(ent zapcore.Entry, ce *zapcore.CheckedEntry) *zapcore.CheckedEntry {
	if s.Enabled(ent.Level) {
		return ce.AddCore(ent, s)
	}
	return ce
}

func (s *fieldSampler) hashSampleField(fields []zapcore.Field) (uint64, bool) {
	v, ok := s.findSampleField(fields)
	if !ok {
		return 0, false
	}
	return xxh3.HashString(v), true
}

func (s *fieldSampler) findSampleField(fields []zapcore.Field) (string, bool) {
	for _, f := range fields {
		if f.Key == s.config.FieldName {
			switch f.Type {
			case zapcore.StringType:
				return f.String, true
			case zapcore.ReflectType:
				rv := reflect.ValueOf(f.Interface)
				if rv.Kind() == reflect.String {
					return rv.String(), true
				}
			case zapcore.StringerType:
				if str, ok := f.Interface.(fmt.Stringer); ok {
					return str.String(), true
				}
			}
			return "", false
		}
	}
	return "", false
}

func (s *fieldSampler) test(fields []zapcore.Field) bool {
	if s.hash != 0 {
		return s.hash > s.config.Rate.Threshold()
	}
	if h, ok := s.hashSampleField(fields); ok {
		return h > s.config.Rate.Threshold()
	}
	return false
}

func (s *fieldSampler) Write(entry zapcore.Entry, fields []zapcore.Field) error {
	if s.test(fields) {
		switch s.config.Action {
		case OmitSampledLog:
			return nil
		case AnnotateSampledLog:
			fields = append(fields, zap.Bool(s.config.AnnotationFieldName, true))
		}
	}
	return s.Core.Write(entry, fields)
}
