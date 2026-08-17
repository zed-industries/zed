package roomobs

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestTrackLayer(t *testing.T) {
	x, y := UnpackTrackLayer(PackTrackLayer(1920, 1080))
	require.Equal(t, 1920, x)
	require.Equal(t, 1080, y)
}

func TestCountryCode(t *testing.T) {
	require.Equal(t, "us", UnpackCountryCode(PackCountryCode("us")))
}
