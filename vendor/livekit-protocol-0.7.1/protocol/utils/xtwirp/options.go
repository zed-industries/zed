package xtwirp

import "github.com/twitchtv/twirp"

// DefaultClientOptions returns default Twirp client options.
func DefaultClientOptions() []twirp.ClientOption {
	return []twirp.ClientOption{
		ClientPassTimout(),
		ClientPassErrorDetails(),
	}
}

// DefaultServerOptions returns default Twirp server options.
func DefaultServerOptions() []twirp.ServerOption {
	return []twirp.ServerOption{
		ServerPassTimeout(),
		ServerPassErrorDetails(),
	}
}
