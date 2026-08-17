package xtwirp

import "net/http"

// Server is a minimal interface for a Twirp server.
type Server interface {
	http.Handler
	PathPrefix() string
}

// WrapHandler wraps the Twirp server handler with our custom middleware.
func WrapHandler(s Server) http.Handler {
	wrappers := []func(http.Handler) http.Handler{
		PassHeadersHandler,
	}
	var h http.Handler = s
	for _, wrapper := range wrappers {
		h = wrapper(h)
	}
	return h
}

// RegisterServer registers Twirp server on an HTTP mux.
// It also calls WrapHandler to add default middleware.
func RegisterServer(mux *http.ServeMux, s Server) {
	mux.Handle(s.PathPrefix(), WrapHandler(s))
}
