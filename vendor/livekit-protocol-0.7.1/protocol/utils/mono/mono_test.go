package mono

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestMonoZero(t *testing.T) {
	ts := time.Time{}
	ts2 := fromTime(ts)
	require.True(t, ts.IsZero())
	require.True(t, ts2.IsZero())
	require.True(t, ts.Equal(ts2))
	require.Equal(t, ts.String(), ts2.String())
}

func TestMono(t *testing.T) {
	t.Cleanup(resetClock) // restore

	ts1 := time.Now()
	ts2 := ts1.Add(time.Second)

	ts1m := fromTime(ts1)
	// emulate a clock reset, +1h jump
	// TODO: use synctest when we switch to Go 1.25
	jumpClock(time.Hour)
	ts2m := fromTime(ts2)

	require.Equal(t, ts2.Sub(ts1), ts2m.Sub(ts1m))
}
