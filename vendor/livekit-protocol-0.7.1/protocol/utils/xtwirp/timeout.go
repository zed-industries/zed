package xtwirp

import (
	"context"
	"net/http"
	"strconv"
	"time"

	"github.com/twitchtv/twirp"
)

const timeoutHeader = "X-Twirp-Timeout-Ms"

// ClientPassTimout adds context timeout as a Twirp request header.
func ClientPassTimout() twirp.ClientOption {
	return twirp.WithClientInterceptors(func(fnc twirp.Method) twirp.Method {
		return func(ctx context.Context, req any) (any, error) {
			if deadline, ok := ctx.Deadline(); ok {
				dt := time.Until(deadline)
				if dt > 0 {
					h, ok := twirp.HTTPRequestHeaders(ctx)
					if !ok {
						h = make(http.Header)
					}
					h.Add(timeoutHeader, strconv.FormatInt(dt.Milliseconds(), 10))
					var err error
					ctx, err = twirp.WithHTTPRequestHeaders(ctx, h)
					if err != nil {
						return nil, err
					}
				}
			}
			return fnc(ctx, req)
		}
	})
}

// ServerPassTimeout uses context timeout from Twirp request header.
// It requires that Twirp server handler is wrapped with PassHeadersHandler.
func ServerPassTimeout() twirp.ServerOption {
	return twirp.WithServerInterceptors(func(fnc twirp.Method) twirp.Method {
		return func(ctx context.Context, req any) (any, error) {
			if h := GetHeaders(ctx); h != nil {
				if v, err := strconv.ParseInt(h.Get(timeoutHeader), 10, 64); err == nil {
					var cancel context.CancelFunc
					ctx, cancel = context.WithTimeout(ctx, time.Duration(v)*time.Millisecond)
					defer cancel()
				}
			}
			return fnc(ctx, req)
		}
	})
}
