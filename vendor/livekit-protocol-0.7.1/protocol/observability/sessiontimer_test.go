package observability

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestSessionTimer(t *testing.T) {
	t.Run("advance 100ms", func(t *testing.T) {
		ts := time.Now()
		st := NewSessionTimer(ts)

		millis, mins := st.Advance(ts.Add(100 * time.Millisecond))
		require.EqualValues(t, 100, millis)
		require.EqualValues(t, 1, mins)

		millis, mins = st.Advance(ts.Add(200 * time.Millisecond))
		require.EqualValues(t, 100, millis)
		require.EqualValues(t, 0, mins)
	})

	t.Run("advance 2.5m", func(t *testing.T) {
		ts := time.Now()
		st := NewSessionTimer(ts)

		millis, mins := st.Advance(ts.Add(150 * time.Second))
		require.EqualValues(t, 150000, millis)
		require.EqualValues(t, 3, mins)
	})
}
