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
	"maps"
	"slices"
	"strings"

	"github.com/pion/logging"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/utils/options"
	"github.com/livekit/protocol/utils/pointer"
)

var (
	pionIgnoredPrefixes = map[string]*prefixSet{
		"ice": {
			"pingAllCandidates called with no candidate pairs",
			"failed to send packet: io: read/write on closed pipe",
			"Ignoring remote candidate with tcpType active",
			"discard message from",
			"Failed to discover mDNS candidate",
			"Failed to read from candidate tcp",
			"remote mDNS candidate added, but mDNS is disabled",
		},
		"pc": {
			"Failed to accept RTCP stream is already closed",
			"Failed to accept RTP stream is already closed",
			"Incoming unhandled RTCP ssrc",
		},
		"tcp_mux": {
			"Error reading first packet from",
			"error closing connection",
		},
		"turn": {
			"error when handling datagram",
			"Failed to send ChannelData from allocation",
			"Failed to handle datagram",
		},
	}
)

type prefixSet []string

func (s *prefixSet) Match(msg string) bool {
	if s == nil {
		return false
	}

	for _, prefix := range *s {
		if strings.HasPrefix(msg, prefix) {
			return true
		}
	}
	return false
}

type PrefixFilter map[string]*prefixSet

func NewPrefixFilter(prefixes map[string][]string) PrefixFilter {
	p := maps.Clone(pionIgnoredPrefixes)
	for scope, set := range prefixes {
		if prev, ok := p[scope]; ok {
			p[scope] = pointer.To(slices.Concat(*prev, prefixSet(set)))
		} else {
			p[scope] = pointer.To(prefixSet(set))
		}
	}
	return p
}

type FilterFunc func(msg string) bool

type Options struct {
	PrefixFilter PrefixFilter
}

type Option func(o *Options)

func WithPrefixFilter(f PrefixFilter) Option {
	return func(o *Options) {
		o.PrefixFilter = f
	}
}

func NewLoggerFactory(l logger.Logger, opts ...Option) logging.LoggerFactory {
	o := Options{
		PrefixFilter: pionIgnoredPrefixes,
	}
	options.Apply(&o, opts)

	if zl, ok := l.(logger.ZapLogger); ok {
		return &zapLoggerFactory{zl, o}
	}
	return &loggerFactory{l, o}
}

// zapLoggerFactory implements logging.LoggerFactory interface for zap loggers
type zapLoggerFactory struct {
	logger  logger.ZapLogger
	options Options
}

func (f *zapLoggerFactory) NewLogger(scope string) logging.LeveledLogger {
	return &zapLogAdapter{
		logger:          f.logger,
		level:           f.logger.ComponentLeveler().ComponentLevel(formatComponent(scope)),
		scope:           scope,
		ignoredPrefixes: f.options.PrefixFilter[scope],
	}
}

// loggerFactory implements logging.LoggerFactory interface for generic loggers
type loggerFactory struct {
	logger  logger.Logger
	options Options
}

func (f *loggerFactory) NewLogger(scope string) logging.LeveledLogger {
	return &logAdapter{
		logger:          f.logger.WithComponent(formatComponent(scope)).WithCallDepth(1),
		ignoredPrefixes: f.options.PrefixFilter[scope],
	}
}

func formatComponent(scope string) string {
	return "pion." + scope
}
