package rpc

import (
	"context"
	"fmt"
	reflect "reflect"
	"runtime"
	"slices"
	"testing"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/psrpc"
	"github.com/livekit/psrpc/pkg/middleware"
)

func TestMiddleware(t *testing.T) {
	t.Run("common middleware propagate client request args", func(t *testing.T) {
		InitPSRPCStats(prometheus.Labels{})

		cases := []struct {
			label string
			opt   psrpc.ClientOption
		}{
			{"WithClientLogger", WithClientLogger(logger.GetLogger())},
			{"WithClientMetrics", middleware.WithClientMetrics(PSRPCMetricsObserver{})},
			{"WithClientObservability", WithClientObservability(logger.GetLogger())},
		}

		for _, c := range cases {
			t.Run(c.label, func(t *testing.T) {
				var o psrpc.ClientOpts
				c.opt(&o)
				for _, c := range o.RpcInterceptors {
					ch := make(chan []psrpc.RequestOption, 1)
					call := c(psrpc.RPCInfo{}, func(ctx context.Context, req proto.Message, opts ...psrpc.RequestOption) (proto.Message, error) {
						ch <- opts
						return nil, nil
					})

					expected := []psrpc.RequestOption{func(*psrpc.RequestOpts) {}, func(*psrpc.RequestOpts) {}}
					go call(context.Background(), nil, expected...)

					eqPtr := func(a psrpc.RequestOption) func(psrpc.RequestOption) bool {
						return func(b psrpc.RequestOption) bool {
							return reflect.ValueOf(a).Pointer() == reflect.ValueOf(b).Pointer()
						}
					}

					fp := reflect.ValueOf(c).Pointer()
					f := runtime.FuncForPC(fp)
					file, line := f.FileLine(fp)
					name := fmt.Sprintf("%s:%d %s", file, line, f.Name())

					select {
					case res := <-ch:
						require.True(t, slices.ContainsFunc(res, eqPtr(expected[0])), "failed to receive option 0 from %s", name)
						require.True(t, slices.ContainsFunc(res, eqPtr(expected[1])), "failed to receive option 1 from %s", name)
					case <-time.After(time.Second):
						require.FailNow(t, "timeout")
					}
				}
			})
		}
	})
}

func TestPropagateRequestTimeout(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 42*time.Second)
	defer cancel()
	var ro psrpc.RequestOpts
	WithPropagateRequestTimeout(ctx)(&ro)
	require.InEpsilon(t, 42*time.Second, ro.Timeout, 0.01)
}
