package xtwirp

import (
	"context"
	"net/http"

	"github.com/twitchtv/twirp"
)

type twirpHeaders struct{}

func withHeaders(ctx context.Context, h http.Header) context.Context {
	return context.WithValue(ctx, twirpHeaders{}, h)
}

// GetHeaders returns Twirp headers from the context.
func GetHeaders(ctx context.Context) http.Header {
	if h, ok := twirp.HTTPRequestHeaders(ctx); ok {
		return h
	}
	// Ideally we would just use twirp.HTTPRequestHeaders,
	// but it looks like it's not set in the server context.
	//
	// And using twirp.WithHTTPRequestHeaders requires us to clone headers
	// and get rid of the ones that are not allowed to modify for Twirp.
	h, _ := ctx.Value(twirpHeaders{}).(http.Header)
	return h
}

// PassHeadersHandler wraps Twirp server handler to allow passing HTTP headers in the context.
func PassHeadersHandler(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()
		ctx = withHeaders(ctx, r.Header)
		r = r.WithContext(ctx)
		h.ServeHTTP(w, r)
	})
}
