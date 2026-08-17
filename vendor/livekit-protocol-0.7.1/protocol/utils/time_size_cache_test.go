package utils

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/utils/mono"
)

func TestTimeSizeCache(t *testing.T) {
	cache := NewTimeSizeCache[string](TimeSizeCacheParams{
		TTL:     5 * time.Second,
		MaxSize: 10,
	})

	now := mono.Now()
	one := "one"
	cache.AddAt(&one, len(one), now.UnixNano())
	items := cache.Get()
	require.Equal(t, 1, len(items))
	require.Equal(t, one, *items[0])
	require.Equal(t, 3, cache.itemsSize)

	now = now.Add(time.Second)
	two := "two"
	cache.AddAt(&two, len(two), now.UnixNano())
	items = cache.Get()
	require.Equal(t, 2, len(items))
	require.Equal(t, one, *items[0])
	require.Equal(t, two, *items[1])
	require.Equal(t, 6, cache.itemsSize)

	// this should evict the "one" based on size
	now = now.Add(time.Second)
	three := "three"
	cache.AddAt(&three, len(three), now.UnixNano())
	items = cache.Get()
	require.Equal(t, 2, len(items))
	require.Equal(t, two, *items[0])
	require.Equal(t, three, *items[1])
	require.Equal(t, 8, cache.itemsSize)

	// this should evict the "two" based on size
	now = now.Add(time.Second)
	four := "four"
	cache.AddAt(&four, len(four), now.UnixNano())
	items = cache.Get()
	require.Equal(t, 2, len(items))
	require.Equal(t, three, *items[0])
	require.Equal(t, four, *items[1])
	require.Equal(t, 9, cache.itemsSize)

	// this should not evict any element as existing elements should not have aged enough
	now = now.Add(time.Second)
	cache.PruneAt(now.UnixNano())
	items = cache.Get()
	require.Equal(t, 2, len(items))
	require.Equal(t, 9, cache.itemsSize)

	// should evict "three" based on age
	now = now.Add(4 * time.Second)
	cache.PruneAt(now.UnixNano())
	items = cache.Get()
	require.Equal(t, 1, len(items))
	require.Equal(t, 4, cache.itemsSize)

	// should evict all
	now = now.Add(time.Second)
	cache.PruneAt(now.UnixNano())
	items = cache.Get()
	require.Equal(t, 0, len(items))
	require.Equal(t, 0, cache.itemsSize)

	// add something that fits
	cache.Add(&four, len(four))
	items = cache.Get()
	require.Equal(t, 1, len(items))
	require.Equal(t, four, *items[0])
	require.Equal(t, 4, cache.itemsSize)

	// add something which will exceed size threshold by itself,
	// should clean up all items in cache as items are FIFO and the "four" added above
	// gets removed first and the long item added also should get removed as it does not fit.
	reallyLong := "really-long"
	cache.Add(&reallyLong, len(reallyLong))
	items = cache.Get()
	require.Equal(t, 0, len(items))
	require.Equal(t, 0, cache.itemsSize)
}
