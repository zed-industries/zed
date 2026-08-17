package interceptors

import (
	"bytes"
	"net/http"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/livekit"
)

func TestCurlPrinter(t *testing.T) {
	var buf bytes.Buffer
	err := printCurl(&buf,
		"http://localhost:8080",
		"example", "Service", "Do",
		http.Header{"X-Test": {"true"}},
		&livekit.Room{Name: "test"},
	)
	require.NoError(t, err)
	require.Equal(t, `curl -X POST \
	-H 'X-Test: true' \
	-H 'Content-Type: application/json' \
	--data '{"name":"test"}' \
	http://localhost:8080/twirp/example.Service/Do
`, buf.String())
}
