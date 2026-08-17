package jaeger

import (
	"context"
	"net"
	"net/url"
	"slices"
	"strings"

	"github.com/go-logr/logr"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracehttp"
	sdkresource "go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/utils/must"
)

// Configure Jaeger tracer.
func Configure(ctx context.Context, addr, name string, attrs ...attribute.KeyValue) {
	if !strings.HasPrefix(addr, "http") {
		addr = "http://" + addr
	}
	u, err := url.Parse(addr)
	if err != nil {
		panic(err)
	}
	if _, port, _ := net.SplitHostPort(u.Host); port == "" {
		u.Host = net.JoinHostPort(u.Host, "4318")
	}
	if u.Path == "" {
		u.Path = "/v1/traces"
	}
	attrs = slices.Clip(attrs)
	attrs = append(attrs, attribute.String("service.name", name))

	tp := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(must.Get(otlptracehttp.New(
			ctx,
			otlptracehttp.WithEndpointURL(u.String()),
		))),
		sdktrace.WithResource(must.Get(sdkresource.Merge(
			sdkresource.Default(),
			must.Get(sdkresource.New(
				ctx,
				sdkresource.WithFromEnv(),
				sdkresource.WithHost(),
				sdkresource.WithContainer(),
				sdkresource.WithAttributes(attrs...),
			)),
		))),
	)

	otel.SetLogger(logr.Discard())
	otel.SetErrorHandler(otel.ErrorHandlerFunc(func(err error) {
		logger.Warnw("jaeger error", err)
	}))
	otel.SetTracerProvider(tp)
}
