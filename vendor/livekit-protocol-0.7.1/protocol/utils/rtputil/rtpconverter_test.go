package rtputil

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestRTPConverter(t *testing.T) {
	r := NewRTPConverter(90000)
	require.Equal(t, time.Second, r.ToDuration(90000))
	require.Equal(t, 500*time.Millisecond, r.ToDuration(45000))
	require.Equal(t, time.Millisecond, r.ToDuration(90))

	r = NewRTPConverter(48000)
	require.Equal(t, time.Second, r.ToDurationExt(48000))
	require.Equal(t, 48*time.Hour, r.ToDurationExt(24*60*60*2*48000))

	r = NewRTPConverter(44100)
	require.Equal(t, uint32(44100), r.ToRTP(time.Second))
	require.Equal(t, uint32(44), r.ToRTP(time.Millisecond)) // 44.1 gets truncated to 44

	r = NewRTPConverter(8000)
	require.Equal(t, uint64(8000), r.ToRTPExt(time.Second))
	require.Equal(t, uint64(24*60*60*10*8000), r.ToRTPExt(240*time.Hour))
}
