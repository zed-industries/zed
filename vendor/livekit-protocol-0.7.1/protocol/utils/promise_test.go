package utils

import (
	"errors"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPromise(t *testing.T) {
	t.Run("zero value is usable", func(t *testing.T) {
		var p Promise[bool]

		require.False(t, p.Resolved())
		done := p.Done()
		select {
		case <-done:
			require.FailNow(t, "unresolved done channel should block")
		default:
		}

		p.Resolve(true, nil)

		require.True(t, p.Resolved())
		select {
		case <-done:
		default:
			require.FailNow(t, "resolved done channel should not block")
		}

		require.True(t, p.Result)
	})

	t.Run("promise cannot be resolved twice", func(t *testing.T) {
		p := NewPromise[bool]()
		p.Resolve(false, errors.New("fail"))
		p.Resolve(true, nil)

		require.False(t, p.Result)
		require.Error(t, p.Err)
	})
}
