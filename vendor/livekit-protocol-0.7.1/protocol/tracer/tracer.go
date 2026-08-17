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

package tracer

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/trace"
	"go.opentelemetry.io/otel/trace/noop"
)

type TracerProvider = trace.TracerProvider

type Tracer = trace.Tracer

type Span = trace.Span

var globalTracer Tracer

// SetTracer can be used for your own tracing (for example, with Lightstep).
//
// Deprecated: Use otel.SetTracerProvider.
func SetTracer(t Tracer) {
	globalTracer = t
}

// Start a new span.
//
// Deprecated: Use otel.Tracer for your service, and call Start on it instead.
func Start(ctx context.Context, spanName string, opts ...any) (context.Context, Span) {
	tracer := globalTracer
	if tracer == nil {
		tracer = otel.Tracer("legacy")
	}
	var sopts []trace.SpanStartOption
	for _, opt := range opts {
		switch opt := opt.(type) {
		case trace.SpanStartOption:
			sopts = append(sopts, opt)
		}
	}
	return tracer.Start(ctx, spanName, sopts...)
}

type NoOpTracer = noop.Tracer

type NoOpSpan = noop.Span
