package traceid_test

import (
	"encoding/hex"
	"regexp"
	"runtime"
	"runtime/debug"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/utils/guid"
	"github.com/livekit/protocol/utils/traceid"
)

func TestIDToken(t *testing.T) {
	id := guid.New("TEST_")
	tok := traceid.FromGUID(id)
	h1 := tok.String()
	h2 := hex.EncodeToString([]byte(id[5 : 5+8]))
	require.Equal(t, h1, h2)

	testFunc(t, tok)
}

func testFunc(t testing.TB, tok traceid.ID) {
	h1 := tok.String()
	s := string(debug.Stack())
	runtime.KeepAlive(tok)
	re := regexp.MustCompile(`traceid_test\.testFunc\(.*0x([a-f0-9]+)\)`)
	sub := re.FindStringSubmatch(s)
	require.Len(t, sub, 2)
	h2 := sub[1]
	require.Equal(t, h1, h2)
}
